/*
 * Copyright (c) 2019 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.db

import kotlinx.coroutines.launch
import kotlinx.coroutines.sync.Mutex
import kotlinx.coroutines.sync.withLock
import kotlinx.serialization.*
import kotlinx.serialization.json.JsonElement
import kotlinx.serialization.json.JsonLiteral
import kotlinx.serialization.json.JsonOutput
import mu.KotlinLogging
import ninja.blacknet.Runtime
import ninja.blacknet.api.APIServer
import ninja.blacknet.core.*
import ninja.blacknet.crypto.*
import ninja.blacknet.network.Node
import ninja.blacknet.packet.UnfilteredInvList
import ninja.blacknet.serialization.BinaryDecoder
import ninja.blacknet.serialization.BinaryEncoder
import ninja.blacknet.serialization.Json
import ninja.blacknet.transaction.CancelLease
import ninja.blacknet.transaction.Lease
import ninja.blacknet.transaction.TxType
import ninja.blacknet.transaction.WithdrawFromLease
import ninja.blacknet.util.delay
import ninja.blacknet.util.startsWith
import ninja.blacknet.util.withUnlock

private val logger = KotlinLogging.logger {}

object WalletDB {
    private const val DELAY = 30 * 60
    private const val VERSION = 5
    internal val mutex = Mutex()
    private val KEYS_KEY = "keys".toByteArray()
    private val TX_KEY = "tx".toByteArray()
    private val VERSION_KEY = "version".toByteArray()
    private val WALLET_KEY = "wallet".toByteArray()
    private val wallets = HashMap<PublicKey, Wallet>()

    private fun setVersion(batch: LevelDB.WriteBatch) {
        val version = BinaryEncoder()
        version.encodeVarInt(VERSION)
        batch.put(WALLET_KEY, VERSION_KEY, version.toBytes())
    }

    init {
        val keysBytes = LevelDB.get(WALLET_KEY, KEYS_KEY)
        val versionBytes = LevelDB.get(WALLET_KEY, VERSION_KEY)

        val version = if (versionBytes != null) {
            BinaryDecoder.fromBytes(versionBytes).decodeVarInt()
        } else {
            1
        }

        if (version == VERSION || version == 4 || version == 3 || version == 2) {
            if (keysBytes != null) {
                var txns = 0
                val decoder = BinaryDecoder.fromBytes(keysBytes)
                for (i in 0 until keysBytes.size step PublicKey.SIZE) {
                    val publicKey = PublicKey(decoder.decodeFixedByteArray(PublicKey.SIZE))
                    val walletBytes = LevelDB.get(WALLET_KEY, publicKey.bytes)!!
                    val wallet = if (version >= 5) Wallet.deserialize(walletBytes) else Wallet.updateV1(walletBytes)
                    txns += wallet.transactions.size
                    wallets.put(publicKey, wallet)
                }
                if (version == 3)
                    updateV3()
                if (version < 5) {
                    logger.info("Updating WalletDB")
                    val batch = LevelDB.createWriteBatch()
                    wallets.forEach { (publicKey, wallet) ->
                        batch.put(WALLET_KEY, publicKey.bytes, wallet.serialize())
                    }
                    setVersion(batch)
                    batch.write()
                }
                if (wallets.size == 1)
                    logger.info("Loaded wallet with $txns transactions")
                else
                    logger.info("Loaded ${wallets.size} wallets with $txns transactions")
            } else if (version < VERSION) {
                val batch = LevelDB.createWriteBatch()
                setVersion(batch)
                batch.write()
            }
        } else if (version == 1) {
            val batch = LevelDB.createWriteBatch()
            if (keysBytes != null) {
                logger.info("Clearing WalletDB...")
                clear(batch)
            }
            setVersion(batch)
            batch.write()
        } else {
            throw RuntimeException("Unknown database version $version")
        }

        Runtime.launch { broadcaster() }
    }

    private suspend fun broadcaster() {
        while (true) {
            delay(DELAY)

            if (Node.isOffline())
                continue

            val inv = UnfilteredInvList()

            mutex.withLock {
                wallets.forEach { (_, wallet) ->
                    val unconfirmed = ArrayList<Triple<Hash, ByteArray, Int>>()

                    wallet.transactions.forEach { (hash, txData) ->
                        if (txData.height == 0 && txData.type != TxType.Generated.type) {
                            val bytes = getTransactionImpl(hash)!!
                            val tx = Transaction.deserialize(bytes)
                            unconfirmed.add(Triple(hash, bytes, tx.seq))
                        }
                    }

                    unconfirmed.sortBy { (_, _, seq) -> seq }

                    unconfirmed.forEach { (hash, bytes, _) ->
                        val (status, fee) = TxPool.processTx(hash, bytes)
                        if (status == Accepted || status == AlreadyHave) {
                            inv.add(Pair(hash, fee))
                        } else {
                            logger.debug { "$status tx $hash" }
                        }
                    }
                }
            }

            if (inv.isNotEmpty()) {
                logger.info("Broadcasting ${inv.size} transactions")
                Node.broadcastInv(inv)
            }
        }
    }

    suspend fun getConfirmations(hash: Hash): Int? = mutex.withLock {
        wallets.forEach { (_, wallet) ->
            val data = wallet.transactions.get(hash)
            if (data != null) {
                if (data.height == 0) return@withLock 0
                return@withLock LedgerDB.height() - data.height
            }
        }
        return@withLock null
    }

    suspend fun getSequence(publicKey: PublicKey): Int = mutex.withLock {
        return@withLock getWalletImpl(publicKey).seq
    }

    suspend fun getTransaction(hash: Hash): ByteArray? = mutex.withLock {
        return@withLock getTransactionImpl(hash)
    }

    private fun getTransactionImpl(hash: Hash): ByteArray? {
        return LevelDB.get(TX_KEY, hash.bytes)
    }

    suspend fun disconnectBlock(blockHash: Hash, batch: LevelDB.WriteBatch) = mutex.withLock {
        if (wallets.isEmpty()) return@withLock

        val (block, _) = BlockDB.blockImpl(blockHash)!!
        val txHashes = block.transactions.map { Transaction.Hasher(it.array) }

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
            batch.put(WALLET_KEY, publicKey.bytes, wallet.serialize())
        }
    }

    suspend fun processBlock(hash: Hash, block: Block?, height: Int, generated: Long, batch: LevelDB.WriteBatch) = mutex.withLock {
        wallets.forEach { (publicKey, wallet) ->
            processBlockImpl(publicKey, wallet, hash, block, height, generated, batch, false)
        }
    }

    private suspend fun processBlockImpl(publicKey: PublicKey, wallet: Wallet, hash: Hash, block: Block?, height: Int, generated: Long, batch: LevelDB.WriteBatch, rescan: Boolean) {
        if (height != 0) {
            if (block!!.generator == publicKey) {
                val tx = Transaction.generated(publicKey, height, hash, generated)
                val txBytes = tx.serialize()
                val txHash = hash // re-use block hash as hash of Generated tx
                processTransactionImpl(publicKey, wallet, txHash, tx, txBytes, block.time, height, batch, rescan)
            }
        } else {
            val balance = LedgerDB.genesisBlock.get(publicKey) ?: return

            val tx = Transaction.generated(publicKey, height, hash, balance)
            val txBytes = tx.serialize()
            val txHash = Transaction.Hasher(txBytes)
            processTransactionImpl(publicKey, wallet, txHash, tx, txBytes, LedgerDB.GENESIS_TIME, height, batch, rescan)
        }
    }

    suspend fun processTransaction(hash: Hash, tx: Transaction, bytes: ByteArray, time: Long) = mutex.withLock {
        val batch = LevelDB.createWriteBatch()
        processTransactionImpl(hash, tx, bytes, time, 0, batch)
        batch.write()
    }

    suspend fun processTransaction(hash: Hash, tx: Transaction, bytes: ByteArray, time: Long, height: Int, batch: LevelDB.WriteBatch) = mutex.withLock {
        processTransactionImpl(hash, tx, bytes, time, height, batch)
    }

    private suspend fun processTransactionImpl(hash: Hash, tx: Transaction, bytes: ByteArray, time: Long, height: Int, batch: LevelDB.WriteBatch) {
        var store = !LevelDB.contains(TX_KEY, hash.bytes)

        wallets.forEach { (publicKey, wallet) ->
            if (processTransactionImpl(publicKey, wallet, hash, tx, bytes, time, height, batch, false, store))
                store = false
        }
    }

    private suspend fun processTransactionImpl(publicKey: PublicKey, wallet: Wallet, hash: Hash, tx: Transaction, bytes: ByteArray, time: Long, height: Int, batch: LevelDB.WriteBatch, rescan: Boolean, store: Boolean = true): Boolean {
        var added = false
        var updated = false

        val from = tx.from == publicKey
        if (from || tx.data().involves(publicKey)) {
            val txData = wallet.transactions.get(hash)
            if (txData == null) {
                wallet.transactions.put(hash, TransactionData(tx.type, time, height))
                if (from && tx.type != TxType.Generated.type) {
                    if (tx.seq == wallet.seq)
                        wallet.seq += 1
                    else
                        logger.warn("Out of order sequence ${tx.seq} ${wallet.seq} $hash")
                }
                added = true
            } else if (txData.height != height) {
                txData.height = height
                updated = true
            }
        }

        if (added) {
            if (!rescan) {
                APIServer.walletNotify(tx, hash, time, bytes.size, publicKey)
                batch.put(WALLET_KEY, publicKey.bytes, wallet.serialize())
            }
            if (store)
                batch.put(TX_KEY, hash.bytes, bytes)
            return true
        } else if (updated) {
            if (!rescan)
                batch.put(WALLET_KEY, publicKey.bytes, wallet.serialize())
            return true
        } else {
            return false
        }
    }

    @Serializable
    class TransactionData(
            val type: Byte,
            val time: Long,
            var height: Int
    ) {
        fun toJson() = Json.toJson(serializer(), this)

        @Serializer(forClass = TransactionData::class)
        companion object {
            override fun serialize(encoder: Encoder, obj: TransactionData) {
                when (encoder) {
                    is JsonOutput -> {
                        @Suppress("NAME_SHADOWING")
                        val encoder = encoder.beginStructure(descriptor)
                        encoder.encodeSerializableElement(descriptor, 0, Int.serializer(), obj.type.toUByte().toInt())
                        encoder.encodeSerializableElement(descriptor, 1, Long.serializer(), obj.time)
                        encoder.encodeSerializableElement(descriptor, 2, Int.serializer(), obj.height)
                        encoder.endStructure(descriptor)
                    }
                    else -> throw RuntimeException("Unsupported encoder")
                }
            }
        }
    }

    @Serializable
    class Wallet(
            var seq: Int = 0,
            val transactions: HashMap<Hash, TransactionData> = HashMap()
    ) {
        fun serialize(): ByteArray = BinaryEncoder.toBytes(serializer(), this)

        internal fun toV1(): WalletV1 {
            val wallet = WalletV1(seq)
            transactions.forEach { (hash, txData) ->
                wallet.transactions.add(JsonLiteral(hash.toString()))
                wallet.transactions.add(txData.toJson())
            }
            return wallet
        }

        @Serializer(forClass = Wallet::class)
        companion object {
            fun deserialize(bytes: ByteArray): Wallet = BinaryDecoder.fromBytes(bytes).decode(serializer())

            override fun deserialize(decoder: Decoder): Wallet {
                return when (decoder) {
                    is BinaryDecoder -> {
                        val seq = decoder.decodeVarInt()
                        val size = decoder.decodeVarInt()
                        val wallet = Wallet(seq, HashMap(size * 2))
                        for (i in 0 until size)
                            wallet.transactions.put(Hash(decoder.decodeFixedByteArray(Hash.SIZE)), TransactionData(decoder.decodeByte(), decoder.decodeVarLong(), decoder.decodeVarInt()))
                        wallet
                    }
                    else -> throw RuntimeException("Unsupported decoder")
                }
            }

            override fun serialize(encoder: Encoder, obj: Wallet) {
                when (encoder) {
                    is BinaryEncoder -> {
                        encoder.encodeVarInt(obj.seq)
                        encoder.encodeVarInt(obj.transactions.size)
                        for ((hash, data) in obj.transactions) {
                            encoder.encodeFixedByteArray(hash.bytes)
                            encoder.encodeByte(data.type)
                            encoder.encodeVarLong(data.time)
                            encoder.encodeVarInt(data.height)
                        }
                    }
                    else -> throw RuntimeException("Unsupported encoder")
                }
            }

            internal fun updateV1(bytes: ByteArray): Wallet {
                val decoder = BinaryDecoder.fromBytes(bytes)
                val seq = decoder.decodeVarInt()
                val size = decoder.decodeVarInt()
                val wallet = Wallet(seq, HashMap(size * 2))
                for (i in 0 until size) {
                    val hash = Hash(decoder.decodeFixedByteArray(Hash.SIZE))
                    val tx = Transaction.deserialize(getTransactionImpl(hash)!!)
                    wallet.transactions.put(hash, TransactionData(tx.type, decoder.decodeVarLong(), decoder.decodeVarInt()))
                }
                return wallet
            }
        }
    }

    @Serializable
    internal class WalletV1(val seq: Int, val transactions: ArrayList<JsonElement> = ArrayList())

    private fun addWalletImpl(batch: LevelDB.WriteBatch, publicKey: PublicKey, wallet: Wallet) {
        wallets.put(publicKey, wallet)
        batch.put(WALLET_KEY, publicKey.bytes, wallet.serialize())
        val encoder = BinaryEncoder()
        wallets.forEach { (publicKey, _) -> encoder.encodeFixedByteArray(publicKey.bytes) }
        val keysBytes = encoder.toBytes()
        batch.put(WALLET_KEY, KEYS_KEY, keysBytes)
    }

    internal suspend fun getWalletImpl(publicKey: PublicKey, rescan: Boolean = true): Wallet {
        var wallet = wallets.get(publicKey)
        if (wallet != null)
            return wallet

        logger.debug { "Creating new wallet for ${Address.encode(publicKey)}" }
        val batch = LevelDB.createWriteBatch()
        wallet = Wallet()

        if (!rescan) {
            addWalletImpl(batch, publicKey, wallet)
            batch.write()
            return wallet
        }

        mutex.withUnlock {
            BlockDB.mutex.withLock {
                mutex.withLock {
                    var hash = Hash.ZERO
                    var index = LedgerDB.getChainIndex(hash)!!
                    val height = LedgerDB.height()
                    val n = height - index.height + 1
                    if (n > 0) {
                        logger.info("Rescanning $n blocks...")
                        do {
                            rescanBlockImpl(publicKey, wallet, hash, index.height, index.generated, batch)
                            hash = index.next
                            index = LedgerDB.getChainIndex(hash)!!
                        } while (index.height != height)
                    }
                }
            }
        }

        addWalletImpl(batch, publicKey, wallet)
        batch.write()
        return wallet
    }

    suspend fun getOutLeases(publicKey: PublicKey): List<AccountState.Lease> = mutex.withLock {
        val wallet = getWalletImpl(publicKey)
        val leases = ArrayList<AccountState.Lease>()
        val transactions = ArrayList<Pair<Transaction, TransactionData>>()

        wallet.transactions.forEach { (hash, txData) ->
            when (txData.type) {
                TxType.Lease.type,
                TxType.CancelLease.type,
                TxType.WithdrawFromLease.type -> {
                    val tx = Transaction.deserialize(getTransactionImpl(hash)!!)
                    transactions.add(Pair(tx, txData))
                }
            }
        }

        transactions.sortBy { (tx, _) -> tx.seq }

        transactions.forEach { (tx, txData) ->
            if (tx.type == TxType.Lease.type) {
                val data = Lease.deserialize(tx.data.array)
                leases.add(AccountState.Lease(data.to, txData.height, data.amount))
            } else if (tx.type == TxType.CancelLease.type) {
                val data = CancelLease.deserialize(tx.data.array)
                if (!leases.remove(AccountState.Lease(data.to, data.height, data.amount)))
                    logger.warn("Lease not found")
            } else if (tx.type == TxType.WithdrawFromLease.type) {
                val data = WithdrawFromLease.deserialize(tx.data.array)
                val lease = leases.find { it.publicKey == data.to && it.height == data.height && it.amount == data.amount }
                if (lease != null)
                    lease.amount -= data.withdraw
                else
                    logger.warn("Lease not found")
            }
        }

        return@withLock leases
    }

    fun getCheckpoint(): Hash {
        return if (!PoS.guessInitialSynchronization())
            LedgerDB.rollingCheckpoint()
        else
            Hash.ZERO
    }

    private suspend fun rescanBlockImpl(publicKey: PublicKey, wallet: Wallet, hash: Hash, height: Int, generated: Long, batch: LevelDB.WriteBatch) {
        if (height != 0) {
            val block = BlockDB.blockImpl(hash)!!.first
            processBlockImpl(publicKey, wallet, hash, block, height, generated, batch, true)
            for (bytes in block.transactions) {
                val tx = Transaction.deserialize(bytes.array)
                val txHash = Transaction.Hasher(bytes.array)
                processTransactionImpl(publicKey, wallet, txHash, tx, bytes.array, block.time, height, batch, true)
            }
        } else {
            processBlockImpl(publicKey, wallet, hash, null, height, generated, batch, true)
        }
    }

    private fun clear(batch: LevelDB.WriteBatch = LevelDB.createWriteBatch()) {
        val iterator = LevelDB.iterator()
        iterator.seekToFirst()
        while (iterator.hasNext()) {
            val entry = iterator.next()
            if (entry.key.startsWith(WALLET_KEY) ||
                    entry.key.startsWith(TX_KEY)) {
                batch.delete(entry.key)
            }
        }
        iterator.close()

        wallets.clear()
    }

    private fun updateV3() {
        class Update(val publicKey: PublicKey, val hash: Hash, val newHash: Hash, val bytes: ByteArray)

        val toUpdate = ArrayList<Update>()

        wallets.forEach { (publicKey, wallet) ->
            wallet.transactions.forEach { (hash, txData) ->
                val bytes = getTransactionImpl(hash)!!
                val tx = Transaction.deserialize(bytes)
                if (tx.type == TxType.Generated.type && tx.referenceChain != Hash.ZERO) {
                    if (txData.height != 0 && !LedgerDB.chainContains(tx.referenceChain))
                        txData.height = 0

                    toUpdate.add(Update(publicKey, hash, tx.referenceChain, bytes))
                }
            }
        }

        if (toUpdate.isEmpty())
            return

        logger.info("Updating ${toUpdate.size} Generated transactions")

        val updatedTx = HashMap<Hash, Pair<Hash, ByteArray>>()
        val batch = LevelDB.createWriteBatch()

        toUpdate.forEach { update ->
            val wallet = wallets.get(update.publicKey)!!
            val txData = wallet.transactions.remove(update.hash)!!
            wallet.transactions.put(update.newHash, txData)
            updatedTx.put(update.hash, Pair(update.newHash, update.bytes))
        }

        updatedTx.forEach { (hash, pair) ->
            batch.delete(TX_KEY, hash.bytes)
            batch.put(TX_KEY, pair.first.bytes, pair.second)
        }

        batch.write()
    }
}
