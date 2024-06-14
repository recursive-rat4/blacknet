/*
 * Copyright (c) 2019-2024 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.db

import io.github.oshai.kotlinlogging.KotlinLogging
import java.lang.Thread.sleep
import java.nio.channels.FileChannel
import java.nio.file.Files
import java.nio.file.StandardOpenOption.CREATE
import java.nio.file.StandardOpenOption.TRUNCATE_EXISTING
import java.nio.file.StandardOpenOption.WRITE
import java.util.concurrent.locks.ReentrantReadWriteLock
import kotlin.concurrent.withLock
import kotlinx.atomicfu.locks.ReentrantLock
import kotlinx.serialization.Serializable
import kotlinx.serialization.builtins.SetSerializer
import ninja.blacknet.Kernel
import ninja.blacknet.ShutdownHooks
import ninja.blacknet.contract.HashTimeLockContractId
import ninja.blacknet.contract.MultiSignatureLockContractId
import ninja.blacknet.core.*
import ninja.blacknet.crypto.*
import ninja.blacknet.dataDir
import ninja.blacknet.io.data
import ninja.blacknet.io.outputStream
import ninja.blacknet.network.Node
import ninja.blacknet.network.packet.UnfilteredInvList
import ninja.blacknet.serialization.*
import ninja.blacknet.serialization.bbf.binaryFormat
import ninja.blacknet.signal.Signal6
import ninja.blacknet.time.currentTimeSeconds
import ninja.blacknet.transaction.*
import ninja.blacknet.util.rotate
import ninja.blacknet.util.withUnlock

private val logger = KotlinLogging.logger {}

object WalletDB {
    private const val VERSION = 9
    internal val txLock = ReentrantLock(true)
    internal val reentrant = ReentrantReadWriteLock()
    private val PUBLIC_KEYS_KEY = DBKey(64, 0)
    private val TX_KEY = DBKey(65, Hash.SIZE_BYTES)
    private val VERSION_KEY = DBKey(66, 0)
    private val WALLET_KEY = DBKey(67, PublicKey.SIZE_BYTES)
    private val wallets = HashMap<PublicKey, Wallet>()

    val txNotify = Signal6<Transaction, Hash, Long, Int, PublicKey, List<TransactionDataType>>()

    private fun setVersion(batch: LevelDB.WriteBatch) {
        val versionBytes = binaryFormat.encodeToByteArray(VarIntSerializer, VERSION)
        batch.put(VERSION_KEY, versionBytes)
    }

    init {
        val versionBytes = LevelDB.get(VERSION_KEY)

        val version = if (versionBytes != null) {
            binaryFormat.decodeFromByteArray(VarIntSerializer, versionBytes)
        } else {
            1
        }

        if (version == VERSION) {
            val publicKeysBytes = LevelDB.get(PUBLIC_KEYS_KEY)
            if (publicKeysBytes != null) {
                var txns = 0
                val publicKeys = binaryFormat.decodeFromByteArray(SetSerializer(PublicKey.serializer()), publicKeysBytes)
                publicKeys.forEach { publicKey ->
                    val walletBytes = LevelDB.get(WALLET_KEY, publicKey.bytes)!!
                    val wallet = binaryFormat.decodeFromByteArray(Wallet.serializer(), walletBytes)
                    txns += wallet.transactions.size
                    wallets.put(publicKey, wallet)
                }
                if (wallets.size == 1)
                    logger.info { "Loaded wallet with $txns transactions" }
                else
                    logger.info { "Loaded ${wallets.size} wallets with $txns transactions" }
            }
        } else if (version in 1 until VERSION) {
            LevelDB.createWriteBatch().use { batch ->
                clear(batch)
                setVersion(batch)
                batch.write()
            }
        } else {
            throw Error("Unknown database version $version")
        }

        val vThread = rotate("WalletDB::announcer", ::announcer)

        ShutdownHooks.add {
            logger.info { "Silencing WalletDB announcer" }
            vThread.interrupt()
        }
    }

    private fun announcer() {
        val allUnconfirmed = ArrayList<ArrayList<Triple<Hash, ByteArray, Int>>>()

        reentrant.readLock().withLock {
            wallets.forEach { (_, wallet) ->
                val unconfirmed = ArrayList<Triple<Hash, ByteArray, Int>>()

                wallet.transactions.forEach { (hash, txData) ->
                    if (txData.height == 0 && txData.types[0].type != TxType.Generated.type.toUByte()) {
                        val bytes = getTransactionImpl(hash)!!
                        val tx = binaryFormat.decodeFromByteArray(Transaction.serializer(), bytes)
                        unconfirmed.add(Triple(hash, bytes, tx.seq))
                    }
                }

                if (unconfirmed.isNotEmpty()) {
                    allUnconfirmed.add(unconfirmed)
                }
            }
        }

        val currTime = currentTimeSeconds()
        val inv = UnfilteredInvList()
        var poolAccepted = 0

        allUnconfirmed.forEach { unconfirmed ->
            unconfirmed.sortBy { (_, _, seq) -> seq }

            unconfirmed.forEach { (hash, bytes, _) ->
                val (status, fee) = Kernel.txPool().process(hash, bytes, currTime, false)
                when (status) {
                    Accepted -> {
                        poolAccepted += 1
                        inv.add(Triple(hash, bytes.size, fee))
                    }
                    is AlreadyHave -> {
                        inv.add(Triple(hash, bytes.size, fee))
                    }
                    else -> {
                        logger.debug { "$status $hash" }
                    }
                }
            }
        }

        if (poolAccepted != 0) {
            logger.info { "Added ${inv.size} transactions to pool" }
        }

        if (inv.isNotEmpty()) {
            val n = Node.broadcastInv(inv)
            logger.info { "Announced ${inv.size} transactions to $n peers" }
        }

        sleep(30 * 60 * 1000L)
    }

    fun getConfirmations(hash: Hash): Int? = Kernel.blockDB().reentrant.readLock().withLock {
        reentrant.readLock().withLock<Int?> {
            wallets.forEach { (_, wallet) ->
                val data = wallet.transactions.get(hash)
                if (data != null) {
                    return data.confirmationsImpl(CoinDB.state())
                } else {
                    Unit
                }
            }
            return null
        }
    }

    fun getConfirmations(publicKey: PublicKey, hash: Hash): Int? = Kernel.blockDB().reentrant.readLock().withLock {
        return reentrant.readLock().withLock {
            val wallet = wallets.get(publicKey)
            if (wallet != null) {
                val txData = wallet.transactions.get(hash)
                if (txData != null) {
                    txData.confirmationsImpl(CoinDB.state())
                } else {
                    txData
                }
            } else {
                wallet
            }
        }
    }

    fun getSequence(publicKey: PublicKey): Int = reentrant.readLock().withLock {
        val wallet = getWalletImpl(publicKey)
        val seq = wallet.seq
        return@withLock if (seq < Kernel.config().seqthreshold)
            seq
        else
            throw RuntimeException("Wallet reached sequence threshold")
    }

    internal fun getTransactionImpl(hash: Hash): ByteArray? {
        return LevelDB.get(TX_KEY, hash.bytes)
    }

    fun disconnectBlock(blockHash: Hash, batch: LevelDB.WriteBatch) = reentrant.writeLock().withLock {
        if (wallets.isEmpty()) return@withLock

        val block = Kernel.blockDB().blocks.getOrThrow(blockHash.bytes)
        val txHashes = block.transactions.map { Transaction.hash(it) }

        val updated = HashMap<PublicKey, Wallet>(wallets.size)
        wallets.forEach { (publicKey, wallet) ->
            val generated = wallet.transactions.get(blockHash)
            if (generated != null) {
                generated.height = 0
                updated.put(publicKey, wallet)
            }
            txHashes.forEach { hash ->
                val tx = wallet.transactions.get(hash)
                if (tx != null) {
                    tx.height = 0
                    updated.put(publicKey, wallet)
                }
            }
        }

        updated.forEach { (publicKey, wallet) ->
            batch.put(WALLET_KEY, publicKey.bytes, binaryFormat.encodeToByteArray(Wallet.serializer(), wallet))
        }
    }

    fun processBlock(hash: Hash, block: Block?, height: Int, generated: Long, batch: LevelDB.WriteBatch) = reentrant.writeLock().withLock {
        wallets.forEach { (publicKey, wallet) ->
            processBlockImpl(publicKey, wallet, hash, block, height, generated, batch, false)
        }
    }

    private fun processBlockImpl(publicKey: PublicKey, wallet: Wallet, hash: Hash, block: Block?, height: Int, generated: Long, batch: LevelDB.WriteBatch, rescan: Boolean) {
        if (height != 0) {
            if (block!!.generator == publicKey) {
                val tx = Transaction.generated(publicKey, height, hash, generated)
                val txBytes = binaryFormat.encodeToByteArray(Transaction.serializer(), tx)
                val txHash = hash // re-use block hash as hash of Generated tx
                processTransactionImpl(publicKey, wallet, txHash, tx, txBytes, block.time, height, batch, rescan)
            }
        } else {
            val balance = Genesis.balances.get(publicKey) ?: return

            val tx = Transaction.generated(publicKey, height, hash, balance)
            val txBytes = binaryFormat.encodeToByteArray(Transaction.serializer(), tx)
            val txHash = Transaction.hash(txBytes)
            processTransactionImpl(publicKey, wallet, txHash, tx, txBytes, Genesis.TIME, height, batch, rescan)
        }
    }

    fun processTransaction(hash: Hash, tx: Transaction, bytes: ByteArray, time: Long) = reentrant.writeLock().withLock {
        LevelDB.createWriteBatch().use { batch ->
            processTransactionImpl(hash, tx, bytes, time, 0, batch)
            batch.write()
        }
    }

    fun processTransaction(hash: Hash, tx: Transaction, bytes: ByteArray, time: Long, height: Int, batch: LevelDB.WriteBatch) = reentrant.writeLock().withLock {
        processTransactionImpl(hash, tx, bytes, time, height, batch)
    }

    private fun processTransactionImpl(hash: Hash, tx: Transaction, bytes: ByteArray, time: Long, height: Int, batch: LevelDB.WriteBatch) {
        var store = !LevelDB.contains(TX_KEY, hash.bytes)

        wallets.forEach { (publicKey, wallet) ->
            if (processTransactionImpl(publicKey, wallet, hash, tx, bytes, time, height, batch, false, store))
                store = false
        }
    }

    private fun processTransactionDataImpl(publicKey: PublicKey, wallet: Wallet, hash: Hash, dataIndex: Int, type: Byte, bytes: ByteArray, height: Int, from: Boolean): Boolean {
        return when (type) {
            TxType.Transfer.type -> {
                if (from) {
                    true
                } else {
                    binaryFormat.decodeFromByteArray(Transfer.serializer(), bytes).involves(publicKey)
                }
            }
            TxType.Burn.type -> {
                from
            }
            TxType.Lease.type -> {
                val data = binaryFormat.decodeFromByteArray(Lease.serializer(), bytes)
                if (from) {
                    wallet.outLeases.add(AccountState.Lease(data.to, height, data.amount))
                    true
                } else {
                    data.involves(publicKey)
                }
            }
            TxType.CancelLease.type -> {
                val data = binaryFormat.decodeFromByteArray(CancelLease.serializer(), bytes)
                if (from) {
                    if (!wallet.outLeases.remove(AccountState.Lease(data.to, data.height, data.amount)))
                        logger.warn { "Lease not found" }
                    true
                } else {
                    data.involves(publicKey)
                }
            }
            TxType.BApp.type -> {
                from
            }
            TxType.CreateHTLC.type -> {
                val data = binaryFormat.decodeFromByteArray(CreateHTLC.serializer(), bytes)
                if (from || data.involves(publicKey)) {
                    wallet.htlcs.add(data.id(hash, dataIndex))
                    true
                } else {
                    false
                }
            }
            TxType.RefundHTLC.type -> {
                val data = binaryFormat.decodeFromByteArray(RefundHTLC.serializer(), bytes)
                if (from || data.involves(wallet.htlcs)) {
                    wallet.htlcs.remove(data.id)
                    true
                } else {
                    false
                }
            }
            TxType.CreateMultisig.type -> {
                val data = binaryFormat.decodeFromByteArray(CreateMultisig.serializer(), bytes)
                if (from || data.involves(publicKey)) {
                    wallet.multisigs.add(data.id(hash, dataIndex))
                    true
                } else {
                    false
                }
            }
            TxType.SpendMultisig.type -> {
                val data = binaryFormat.decodeFromByteArray(SpendMultisig.serializer(), bytes)
                if (from || data.involves(wallet.multisigs)) {
                    wallet.multisigs.remove(data.id)
                    true
                } else {
                    false
                }
            }
            TxType.WithdrawFromLease.type -> {
                val data = binaryFormat.decodeFromByteArray(WithdrawFromLease.serializer(), bytes)
                if (from) {
                    val lease = wallet.outLeases.find { it.publicKey == data.to && it.height == data.height && it.amount == data.amount }
                    if (lease != null)
                        lease.amount -= data.withdraw
                    else
                        logger.warn { "Lease not found" }
                    true
                } else {
                    data.involves(publicKey)
                }
            }
            TxType.ClaimHTLC.type -> {
                val data = binaryFormat.decodeFromByteArray(ClaimHTLC.serializer(), bytes)
                if (from || data.involves(wallet.htlcs)) {
                    wallet.htlcs.remove(data.id)
                    true
                } else {
                    false
                }
            }
            else -> {
                logger.warn { "Unexpected TxType $type" }
                from
            }
        }
    }

    private fun processTransactionImpl(publicKey: PublicKey, wallet: Wallet, hash: Hash, tx: Transaction, bytes: ByteArray, time: Long, height: Int, batch: LevelDB.WriteBatch, rescan: Boolean, store: Boolean = true): Boolean {
        val from = tx.from == publicKey
        val txData = wallet.transactions.get(hash)
        if (txData == null) {
            val types = if (tx.type == TxType.Generated.type) {
                listOf(TransactionDataType(tx.type.toUByte(), 0u))
            } else {
                if (from) {
                    if (tx.seq == wallet.seq)
                        wallet.seq += 1
                    else
                        logger.warn { "Out of order sequence ${tx.seq} ${wallet.seq} $hash" }
                }
                if (tx.type != TxType.Batch.type) {
                    if (processTransactionDataImpl(publicKey, wallet, hash, 0, tx.type, tx.data, height, from))
                        listOf(TransactionDataType(tx.type.toUByte(), 0u))
                    else
                        emptyList()
                } else {
                    val data = binaryFormat.decodeFromByteArray(Batch.serializer(), bytes)
                    val types = ArrayList<TransactionDataType>(data.multiData.size)
                    for (index in 0 until data.multiData.size) {
                        val (dataType, dataBytes) = data.multiData[index]
                        val dataIndex = index + 1
                        if (processTransactionDataImpl(publicKey, wallet, hash, dataIndex, dataType, dataBytes, height, from))
                            types.add(TransactionDataType(dataType.toUByte(), dataIndex.toUByte()))
                    }
                    types
                }
            }
            if (types.isNotEmpty()) {
                wallet.transactions.put(hash, TransactionData(types, time, height))

                if (!rescan) {
                    txNotify(tx, hash, time, bytes.size, publicKey, types)
                    batch.put(WALLET_KEY, publicKey.bytes, binaryFormat.encodeToByteArray(Wallet.serializer(), wallet))
                }
                if (store) {
                    batch.put(TX_KEY, hash.bytes, bytes)
                }

                return true
            } else {
                return false
            }
        } else if (txData.height != height) {
            //TODO Batch
            if (tx.type == TxType.Lease.type && from) {
                val data = binaryFormat.decodeFromByteArray(Lease.serializer(), tx.data)
                val lease = wallet.outLeases.find {
                    it.publicKey == data.to &&
                    it.height == txData.height &&
                    it.amount == data.amount
                }
                if (lease != null)
                    lease.height = height
                else
                    logger.warn { "Lease not found" }
            }

            txData.height = height

            if (!rescan) {
                batch.put(WALLET_KEY, publicKey.bytes, binaryFormat.encodeToByteArray(Wallet.serializer(), wallet))
            }

            return true
        } else {
            return false
        }
    }

    @Serializable
    class TransactionDataType(
        val type: UByte,
        val dataIndex: UByte,
    ) {
    }

    @Serializable
    class TransactionData(
            // 交易數據類型和交易數據索引號列表
            val types: List<TransactionDataType>,
            // 交易被接收了的時間戳
            @Serializable(with = VarLongSerializer::class)
            val time: Long,
            // 包含交易的區塊的高度
            @Serializable(with = VarIntSerializer::class)
            var height: Int
    ) {
        internal fun confirmationsImpl(state: CoinDB.State): Int {
            return if (height != 0)
                state.height - height + 1
            else
                0
        }
    }

    @Serializable
    class Wallet(
            @Serializable(with = VarIntSerializer::class)
            var seq: Int = 0,
            val htlcs: HashSet<HashTimeLockContractId> = HashSet(),
            val multisigs: HashSet<MultiSignatureLockContractId> = HashSet(),
            val outLeases: ArrayList<AccountState.Lease> = ArrayList(),
            val transactions: HashMap<Hash, TransactionData> = HashMap()
    )

    private fun addWalletImpl(batch: LevelDB.WriteBatch, publicKey: PublicKey, wallet: Wallet) {
        wallets.put(publicKey, wallet)
        batch.put(WALLET_KEY, publicKey.bytes, binaryFormat.encodeToByteArray(Wallet.serializer(), wallet))
        val publicKeys = wallets.keys
        val publicKeysBytes = binaryFormat.encodeToByteArray(SetSerializer(PublicKey.serializer()), publicKeys)
        batch.put(PUBLIC_KEYS_KEY, publicKeysBytes)
    }

    internal fun getWalletImpl(publicKey: PublicKey): Wallet {
        wallets.get(publicKey)?.let { return it }

        logger.debug { "Creating new wallet for ${Address.encode(publicKey.bytes)}" }
        return LevelDB.createWriteBatch().use { batch ->
            val wallet = Wallet()

            reentrant.readLock().withUnlock {
                Kernel.blockDB().reentrant.readLock().withLock {
                    reentrant.writeLock().withLock {
                        var hash = Genesis.BLOCK_HASH
                        var index = CoinDB.blockIndexes.getOrThrow(hash.bytes)
                        val height = CoinDB.state().height
                        val n = height - index.height + 1
                        if (n > 0) {
                            logger.info { "Rescanning $n blocks..." }
                            do {
                                rescanBlockImpl(publicKey, wallet, hash, index.height, index.generated, batch)
                                hash = index.next
                                index = CoinDB.blockIndexes.getOrThrow(hash.bytes)
                            } while (index.height != height)
                            logger.info { "Finished rescan" }
                        }
                        //TODO 重掃描交易池
                        addWalletImpl(batch, publicKey, wallet)
                        batch.write()
                    }
                }
            }

            wallet
        }
    }

    fun anchor(): Hash {
        return if (!PoS.guessInitialSynchronization())
            CoinDB.state().rollingCheckpoint
        else
            Genesis.BLOCK_HASH
    }

    private fun rescanBlockImpl(publicKey: PublicKey, wallet: Wallet, hash: Hash, height: Int, generated: Long, batch: LevelDB.WriteBatch) {
        if (height != 0) {
            val block = Kernel.blockDB().blocks.getOrThrow(hash.bytes)
            processBlockImpl(publicKey, wallet, hash, block, height, generated, batch, true)
            for (bytes in block.transactions) {
                val tx = binaryFormat.decodeFromByteArray(Transaction.serializer(), bytes)
                val txHash = Transaction.hash(bytes)
                processTransactionImpl(publicKey, wallet, txHash, tx, bytes, block.time, height, batch, true)
            }
        } else {
            processBlockImpl(publicKey, wallet, hash, null, height, generated, batch, true)
        }
    }

    private fun clear(batch: LevelDB.WriteBatch) {
        val backupDir = dataDir.resolve("walletdb.backup.${currentTimeSeconds()}")
        Files.createDirectories(backupDir)
        logger.info { "Saving backup to $backupDir" }
        wallets.clear()

        batch.delete(PUBLIC_KEYS_KEY)

        val iterator = LevelDB.iterator()

        if (LevelDB.seek(iterator, TX_KEY)) {
            val file = backupDir.resolve("transactions")
            val stream = FileChannel.open(file, CREATE, TRUNCATE_EXISTING, WRITE).outputStream().buffered().data()

            while (iterator.hasNext()) {
                val entry = iterator.next()
                if (TX_KEY % entry) {
                    val bytes = entry.value
                    stream.writeInt(bytes.size)
                    stream.write(bytes, 0, bytes.size)
                    batch.delete(entry.key)
                } else {
                    break
                }
            }

            stream.close()
        }

        if (LevelDB.seek(iterator, WALLET_KEY)) {
            while (iterator.hasNext()) {
                val entry = iterator.next()
                val key = WALLET_KEY - entry ?: break
                val file = backupDir.resolve(Address.encode(key))
                val stream = FileChannel.open(file, CREATE, TRUNCATE_EXISTING, WRITE).outputStream().buffered().data()

                val bytes = entry.value
                stream.write(bytes, 0, bytes.size)
                batch.delete(entry.key)

                stream.close()
            }
        }

        iterator.close()
    }
}
