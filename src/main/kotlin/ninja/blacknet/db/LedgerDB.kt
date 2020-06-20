/*
 * Copyright (c) 2018-2019 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.db

import com.google.common.collect.Maps.newHashMapWithExpectedSize
import kotlinx.coroutines.runBlocking
import kotlinx.coroutines.sync.withLock
import kotlinx.serialization.*
import kotlinx.serialization.builtins.*
import kotlinx.serialization.json.JsonOutput
import mu.KotlinLogging
import ninja.blacknet.Config
import ninja.blacknet.contract.HashTimeLockContractId
import ninja.blacknet.contract.MultiSignatureLockContractId
import ninja.blacknet.core.*
import ninja.blacknet.crypto.*
import ninja.blacknet.dataDir
import ninja.blacknet.serialization.BinaryDecoder
import ninja.blacknet.serialization.BinaryEncoder
import ninja.blacknet.serialization.notSupportedDecoderError
import ninja.blacknet.serialization.notSupportedEncoderError
import ninja.blacknet.util.buffered
import ninja.blacknet.util.data
import ninja.blacknet.util.toByteArray
import java.io.File
import java.util.ArrayDeque //TODO check kotlin.collections.ArrayDeque
import kotlin.math.max
import kotlin.math.min

private val logger = KotlinLogging.logger {}

object LedgerDB {
    private const val VERSION = 8
    private val ACCOUNT_KEY = DBKey(1, PublicKey.SIZE_BYTES)
    private val CHAIN_KEY = DBKey(2, Hash.SIZE_BYTES)
    private val HTLC_KEY = DBKey(3, HashTimeLockContractId.SIZE_BYTES)
    private val MULTISIG_KEY = DBKey(4, MultiSignatureLockContractId.SIZE_BYTES)
    private val UNDO_KEY = DBKey(5, Hash.SIZE_BYTES)
    private val SIZES_KEY = DBKey(6, 0)
    private val SNAPSHOT_KEY = DBKey(7, Int.SIZE_BYTES)
    private val SNAPSHOTHEIGHTS_KEY = DBKey(8, 0)
    private val STATE_KEY = DBKey(9, 0)
    private val VERSION_KEY = DBKey(10, 0)

    @Serializable
    internal class State(
            val height: Int,
            val blockHash: Hash,
            val blockTime: Long,
            val difficulty: BigInt,
            val cumulativeDifficulty: BigInt,
            val supply: Long,
            val nxtrng: Hash,
            val rollingCheckpoint: Hash,
            val maxBlockSize: Int,
            val upgraded: Short,
            val forkV2: Short
    ) {
    }

    @Volatile
    private lateinit var state: State

    private val blockSizes = ArrayDeque<Int>(PoS.BLOCK_SIZE_SPAN)
    private val snapshotHeights = HashSet<Int>()

    private fun loadGenesisState() {
        val batch = LevelDB.createWriteBatch()

        var supply = 0L
        Genesis.balances.forEach { (publicKey, balance) ->
            val account = AccountState.create(balance)
            batch.put(ACCOUNT_KEY, publicKey.bytes, BinaryEncoder.toBytes(AccountState.serializer(), account))
            supply += balance
        }

        val chainIndex = ChainIndex(Hash.ZERO, Hash.ZERO, 0, 0, 0)
        batch.put(CHAIN_KEY, Hash.ZERO.bytes, BinaryEncoder.toBytes(ChainIndex.serializer(), chainIndex))

        blockSizes.add(0)
        writeBlockSizes(batch)

        val state = State(
                0,
                Hash.ZERO,
                Genesis.TIME,
                PoS.INITIAL_DIFFICULTY,
                BigInt.ZERO,
                supply,
                Hash.ZERO,
                Hash.ZERO,
                PoS.DEFAULT_MAX_BLOCK_SIZE,
                0,
                0)
        batch.put(STATE_KEY, BinaryEncoder.toBytes(State.serializer(), state))
        this.state = state

        setVersion(batch)

        batch.write()
    }

    private fun setVersion(batch: LevelDB.WriteBatch) {
        val version = BinaryEncoder()
        version.encodeVarInt(VERSION)
        batch.put(VERSION_KEY, version.toBytes())
    }

    private fun writeBlockSizes(batch: LevelDB.WriteBatch) {
        val encoder = BinaryEncoder()
        encoder.encodeVarInt(blockSizes.size)
        for (size in blockSizes)
            encoder.encodeVarInt(size)
        batch.put(SIZES_KEY, encoder.toBytes())
    }

    private fun writeSnapshotHeights(batch: LevelDB.WriteBatch) {
        val encoder = BinaryEncoder()
        encoder.encodeVarInt(snapshotHeights.size)
        for (height in snapshotHeights)
            encoder.encodeVarInt(height)
        batch.put(SNAPSHOTHEIGHTS_KEY, encoder.toBytes())
    }

    init {
        val snapshotHeightsBytes = LevelDB.get(SNAPSHOTHEIGHTS_KEY)
        if (snapshotHeightsBytes != null) {
            val decoder = BinaryDecoder(snapshotHeightsBytes)
            val size = decoder.decodeVarInt()
            for (i in 0 until size)
                snapshotHeights.add(decoder.decodeVarInt())
        }

        val blockSizesBytes = LevelDB.get(SIZES_KEY)
        if (blockSizesBytes != null) {
            val decoder = BinaryDecoder(blockSizesBytes)
            val size = decoder.decodeVarInt()
            for (i in 0 until size)
                blockSizes.addLast(decoder.decodeVarInt())
        }

        val stateBytes = LevelDB.get(STATE_KEY)
        if (stateBytes != null) {
            val versionBytes = LevelDB.get(VERSION_KEY)!!
            val version = BinaryDecoder(versionBytes).decodeVarInt()

            if (version == VERSION) {
                val state = BinaryDecoder(stateBytes).decode(LedgerDB.State.serializer())
                logger.info("Blockchain height ${state.height}")
                this.state = state
            } else if (version in 1 until VERSION) {
                logger.info("Reindexing blockchain...")

                runBlocking {
                    val blockHashes = ArrayList<Hash>(500000)
                    var index = getChainIndex(Hash.ZERO)!!
                    while (index.next != Hash.ZERO) {
                        blockHashes.add(index.next)
                        index = getChainIndex(index.next)!!
                    }
                    logger.info("Found ${blockHashes.size} blocks")

                    clear()

                    for (i in 0 until blockHashes.size) {
                        val hash = blockHashes[i]
                        val (block, size) = BlockDB.blockImpl(hash)!!
                        val batch = LevelDB.createWriteBatch()
                        val txDb = Update(batch, block.version, hash, block.previous, block.time, size, block.generator)
                        val (status, _) = processBlockImpl(txDb, hash, block, size)
                        if (status != Accepted) {
                            batch.close()
                            logger.error("process block failed")
                            break
                        }
                        pruneImpl(batch)
                        txDb.commitImpl()
                        if (i != 0 && i % 50000 == 0)
                            logger.info("Processed $i blocks")
                    }

                    logger.info("Finished reindex at height ${state.height}")
                }
            } else {
                throw RuntimeException("Unknown database version $version")
            }
        } else {
            loadGenesisState()
        }

        val bootstrap = File(dataDir, "bootstrap.dat")
        if (bootstrap.exists()) {
            runBlocking {
                logger.info("Found bootstrap")
                var n = 0

                val stream = bootstrap.inputStream().buffered().data()
                try {
                    while (true) {
                        val size = stream.readInt()
                        val bytes = ByteArray(size)
                        stream.readFully(bytes)

                        val hash = Block.hash(bytes)
                        val status = BlockDB.processImpl(hash, bytes)
                        if (status == Accepted) {
                            if (++n % 50000 == 0)
                                logger.info("Processed $n blocks")
                            pruneImpl()
                        } else if (status !is AlreadyHave) {
                            logger.info("$status block $hash")
                            break
                        }
                    }
                } catch (e: Throwable) {
                    logger.debug { e }
                } finally {
                    stream.close()
                }

                val f = File(dataDir, "bootstrap.dat.old")
                f.delete()
                bootstrap.renameTo(f)

                logger.info("Imported $n blocks")
            }
        }
    }

    internal fun state(): State {
        return state
    }

    fun forkV2(): Boolean {
        return if (Config.instance.regtest)
            true
        else
            state.forkV2 == (PoS.MATURITY + 1).toShort()
    }

    fun chainContains(hash: Hash): Boolean {
        return LevelDB.contains(CHAIN_KEY, hash.bytes)
    }

    fun scheduleSnapshotImpl(height: Int): Boolean {
        if (height <= state.height)
            return false
        if (snapshotHeights.add(height)) {
            val batch = LevelDB.createWriteBatch()
            writeSnapshotHeights(batch)
            batch.write()
        }
        return true
    }

    fun getSnapshot(height: Int): Snapshot? {
        return LevelDB.get(SNAPSHOT_KEY, height.toByteArray())?.let { bytes ->
            BinaryDecoder(bytes).decode(Snapshot.serializer())
        }
    }

    internal fun getNextRollingCheckpoint(): Hash {
        val state = state
        if (state.rollingCheckpoint != Hash.ZERO) {
            val chainIndex = getChainIndex(state.rollingCheckpoint)!!
            return chainIndex.next
        } else {
            if (state.height < PoS.MATURITY + 1)
                return Hash.ZERO
            val checkpoint = state.height - PoS.MATURITY
            var chainIndex = getChainIndex(state.blockHash)!!
            while (chainIndex.height != checkpoint + 1)
                chainIndex = getChainIndex(chainIndex.previous)!!
            return chainIndex.previous
        }
    }

    private fun getAccountBytes(key: PublicKey): ByteArray? {
        return LevelDB.get(ACCOUNT_KEY, key.bytes)
    }

    fun get(key: PublicKey): AccountState? {
        return getAccountBytes(key)?.let { bytes ->
            BinaryDecoder(bytes).decode(AccountState.serializer())
        }
    }

    private fun getUndo(hash: Hash): UndoBlock {
        return BinaryDecoder(LevelDB.get(UNDO_KEY, hash.bytes)!!).decode(UndoBlock.serializer())
    }

    fun getChainIndex(hash: Hash): ChainIndex? {
        return LevelDB.get(CHAIN_KEY, hash.bytes)?.let { bytes ->
            BinaryDecoder(bytes).decode(ChainIndex.serializer())
        }
    }

    fun checkReferenceChain(hash: Hash): Boolean {
        return hash == Hash.ZERO || chainContains(hash)
    }

    suspend fun getNextBlockHashes(start: Hash, max: Int): List<Hash>? = BlockDB.mutex.withLock {
        var chainIndex = getChainIndex(start) ?: return@withLock null
        val result = ArrayList<Hash>(max)
        while (true) {
            val hash = chainIndex.next
            if (hash == Hash.ZERO)
                break
            result.add(hash)
            if (result.size == max)
                break
            chainIndex = getChainIndex(chainIndex.next)!!
        }
        return result
    }

    private fun getHTLCBytes(id: HashTimeLockContractId): ByteArray? {
        return LevelDB.get(HTLC_KEY, id.hash.bytes)
    }

    fun getHTLC(id: HashTimeLockContractId): HTLC? {
        return getHTLCBytes(id)?.let { bytes ->
            BinaryDecoder(bytes).decode(HTLC.serializer())
        }
    }

    private fun getMultisigBytes(id: MultiSignatureLockContractId): ByteArray? {
        return LevelDB.get(MULTISIG_KEY, id.hash.bytes)
    }

    fun getMultisig(id: MultiSignatureLockContractId): Multisig? {
        return getMultisigBytes(id)?.let { bytes ->
            BinaryDecoder(bytes).decode(Multisig.serializer())
        }
    }

    internal suspend fun processBlockImpl(txDb: Update, hash: Hash, block: Block, size: Int): Pair<Status, List<Hash>> {
        val state = state
        if (block.previous != state.blockHash) {
            logger.error("$hash not on current chain ${state.blockHash} previous ${block.previous}")
            return Pair(NotOnThisChain(block.previous.toString()), emptyList())
        }
        if (size > state.maxBlockSize) {
            return Pair(Invalid("Too large block $size bytes, maximum ${state.maxBlockSize}"), emptyList())
        }
        if (block.time <= state.blockTime) {
            return Pair(Invalid("Timestamp is too early"), emptyList())
        }
        var generator = txDb.get(block.generator)
        if (generator == null) {
            return Pair(Invalid("Block generator not found"), emptyList())
        }
        val height = txDb.height()
        val txHashes = ArrayList<Hash>(block.transactions.size)

        val pos = PoS.check(block.time, block.generator, state.nxtrng, state.difficulty, state.blockTime, generator.stakingBalance(height))
        if (pos != Accepted) {
            return Pair(pos, emptyList())
        }

        txDb.set(block.generator, generator)

        var fees = 0L
        for (index in 0 until block.transactions.size) {
            val bytes = block.transactions[index]
            val tx = BinaryDecoder(bytes.array).decode(Transaction.serializer())
            val txHash = Transaction.hash(bytes.array)
            val status = txDb.processTransactionImpl(tx, txHash, bytes.array.size)
            if (status != Accepted) {
                return Pair(notAccepted("Transaction $index", status), emptyList())
            }
            txHashes.add(txHash)
            fees += tx.fee

            WalletDB.processTransaction(txHash, tx, bytes.array, block.time, height, txDb.batch)
        }

        generator = txDb.get(block.generator)!!

        val mint = PoS.mint(state.supply)
        val generated = mint + fees

        val prevIndex = getChainIndex(block.previous)!!
        prevIndex.next = hash
        prevIndex.nextSize = size
        txDb.prevIndex = prevIndex
        txDb.chainIndex = ChainIndex(block.previous, Hash.ZERO, 0, height, generated)

        txDb.addSupply(mint)
        generator.debit(height, generated)
        txDb.set(block.generator, generator)

        WalletDB.processBlock(hash, block, height, generated, txDb.batch)

        return Pair(Accepted, txHashes)
    }

    private suspend fun undoBlockImpl(): Hash {
        val state = state
        val batch = LevelDB.createWriteBatch()
        val hash = state.blockHash
        val chainIndex = getChainIndex(hash)!!
        val undo = getUndo(hash)

        blockSizes.removeLast()
        blockSizes.addFirst(undo.blockSize)
        writeBlockSizes(batch)

        val height = state.height - 1
        val blockHash = chainIndex.previous
        val maxBlockSize = PoS.maxBlockSize(blockSizes)
        val newState = State(
                height,
                blockHash,
                undo.blockTime,
                undo.difficulty,
                undo.cumulativeDifficulty,
                undo.supply,
                undo.nxtrng,
                undo.rollingCheckpoint,
                maxBlockSize,
                undo.upgraded,
                undo.forkV2)
        this.state = newState
        batch.put(STATE_KEY, BinaryEncoder.toBytes(State.serializer(), newState))

        val prevIndex = getChainIndex(chainIndex.previous)!!
        prevIndex.next = Hash.ZERO
        prevIndex.nextSize = 0
        batch.put(CHAIN_KEY, chainIndex.previous.bytes, BinaryEncoder.toBytes(ChainIndex.serializer(), prevIndex))
        batch.delete(CHAIN_KEY, hash.bytes)

        undo.accounts.forEach { (key, bytes) ->
            if (bytes.array.isNotEmpty())
                batch.put(ACCOUNT_KEY, key.bytes, bytes.array)
            else
                batch.delete(ACCOUNT_KEY, key.bytes)
        }
        undo.htlcs.forEach { (id, bytes) ->
            if (bytes.array.isNotEmpty())
                batch.put(HTLC_KEY, id.hash.bytes, bytes.array)
            else
                batch.delete(HTLC_KEY, id.hash.bytes)
        }
        undo.multisigs.forEach { (id, bytes) ->
            if (bytes.array.isNotEmpty())
                batch.put(MULTISIG_KEY, id.hash.bytes, bytes.array)
            else
                batch.delete(MULTISIG_KEY, id.hash.bytes)
        }

        batch.delete(UNDO_KEY, hash.bytes)

        WalletDB.disconnectBlock(hash, batch)

        batch.write()

        return hash
    }

    internal suspend fun rollbackToImpl(hash: Hash): List<Hash> {
        val result = ArrayList<Hash>()
        do {
            result.add(undoBlockImpl())
        } while (state.blockHash != hash)
        return result
    }

    internal suspend fun undoRollbackImpl(rollbackTo: Hash, list: List<Hash>): List<Hash> {
        val toRemove = if (state.blockHash != rollbackTo) rollbackToImpl(rollbackTo) else emptyList()

        list.asReversed().forEach { hash ->
            val block = BlockDB.blockImpl(hash)
            if (block == null) {
                logger.error("$hash not found")
                return toRemove
            }

            val batch = LevelDB.createWriteBatch()
            val txDb = LedgerDB.Update(batch, block.first.version, hash, block.first.previous, block.first.time, block.second, block.first.generator)
            val (status, _) = processBlockImpl(txDb, hash, block.first, block.second)
            if (status != Accepted) {
                batch.close()
                logger.error("$status block $hash")
                return toRemove
            }
            txDb.commitImpl()
        }

        return toRemove
    }

    internal fun pruneImpl() {
        val batch = LevelDB.createWriteBatch()
        pruneImpl(batch)
        batch.write()
    }

    private fun pruneImpl(batch: LevelDB.WriteBatch) {
        var chainIndex = getChainIndex(state.rollingCheckpoint)!!
        while (true) {
            val hash = chainIndex.previous
            if (!LevelDB.contains(UNDO_KEY, hash.bytes))
                break
            batch.delete(UNDO_KEY, hash.bytes)
            if (hash == Hash.ZERO)
                break
            chainIndex = getChainIndex(hash)!!
        }
    }

    private fun clear() {
        val batch = LevelDB.createWriteBatch()
        val iterator = LevelDB.iterator()
        iterator.seekToFirst()
        while (iterator.hasNext()) {
            val entry = iterator.next()
            if (ACCOUNT_KEY % entry ||
                    CHAIN_KEY % entry ||
                    HTLC_KEY % entry ||
                    MULTISIG_KEY % entry ||
                    UNDO_KEY % entry ||
                    SIZES_KEY % entry ||
                    SNAPSHOT_KEY % entry ||
                    STATE_KEY % entry ||
                    VERSION_KEY % entry) {
                batch.delete(entry.key)
            }
        }
        iterator.close()
        batch.write()

        blockSizes.clear()

        loadGenesisState()
    }

    fun warnings(): List<String> {
        return if (state.upgraded < PoS.MATURITY / 2)
            emptyList()
        else
            listOf("This version is obsolete, upgrade required!")
    }

    suspend fun check(): Check = BlockDB.mutex.withLock {
        var supply = 0L
        val result = Check(false, 0, 0, 0)
        iterateImpl(
                { _, account ->
                    supply += account.totalBalance()
                    result.accounts += 1
                },
                { _, htlc ->
                    supply += htlc.amount
                    result.htlcs += 1
                },
                { _, multisig ->
                    supply += multisig.amount()
                    result.multisigs += 1
                }
        )
        if (supply == state.supply)
            result.result = true
        return@withLock result
    }

    @Serializable
    class Check(
            var result: Boolean,
            var accounts: Int,
            var htlcs: Int,
            var multisigs: Int
    )

    private fun iterateImpl(
            account: (PublicKey, AccountState) -> Unit,
            htlc: (HashTimeLockContractId, HTLC) -> Unit,
            multisig: (MultiSignatureLockContractId, Multisig) -> Unit
    ) {
        val iterator = LevelDB.iterator()
        if (LevelDB.seek(iterator, ACCOUNT_KEY)) {
            while (iterator.hasNext()) {
                val entry = iterator.next()
                val key = ACCOUNT_KEY - entry ?: break
                account(PublicKey(key), BinaryDecoder(entry.value).decode(AccountState.serializer()))
            }
        }
        if (LevelDB.seek(iterator, HTLC_KEY)) {
            while (iterator.hasNext()) {
                val entry = iterator.next()
                val key = HTLC_KEY - entry ?: break
                htlc(HashTimeLockContractId(key), BinaryDecoder(entry.value).decode(HTLC.serializer()))
            }
        }
        if (LevelDB.seek(iterator, MULTISIG_KEY)) {
            while (iterator.hasNext()) {
                val entry = iterator.next()
                val key = MULTISIG_KEY - entry ?: break
                multisig(MultiSignatureLockContractId(key), BinaryDecoder(entry.value).decode(Multisig.serializer()))
            }
        }
        iterator.close()
    }

    internal class Update(
            val batch: LevelDB.WriteBatch,
            private val blockVersion: Int,
            private val blockHash: Hash,
            private val blockPrevious: Hash,
            private val blockTime: Long,
            private val blockSize: Int,
            private val blockGenerator: PublicKey,
            private val state: State = LedgerDB.state,
            private val height: Int = state.height + 1,
            private var supply: Long = state.supply,
            private val rollingCheckpoint: Hash = LedgerDB.getNextRollingCheckpoint(),
            private val accounts: MutableMap<PublicKey, AccountState> = HashMap(),
            private val htlcs: MutableMap<HashTimeLockContractId, HTLC?> = HashMap(),
            private val multisigs: MutableMap<MultiSignatureLockContractId, Multisig?> = HashMap(),
            private val undo: UndoBlock = UndoBlock(
                    state.blockTime,
                    state.difficulty,
                    state.cumulativeDifficulty,
                    state.supply,
                    state.nxtrng,
                    state.rollingCheckpoint,
                    state.upgraded,
                    blockSizes.peekFirst(),
                    ArrayList(),
                    ArrayList(),
                    ArrayList(),
                    state.forkV2
            ),
            var chainIndex: ChainIndex? = null,
            var prevIndex: ChainIndex? = null
    ) : Ledger {
        override fun addSupply(amount: Long) {
            supply += amount
        }

        override fun checkReferenceChain(hash: Hash): Boolean {
            return LedgerDB.checkReferenceChain(hash)
        }

        override fun checkFee(size: Int, amount: Long): Boolean {
            return amount >= 0
        }

        override fun blockTime(): Long {
            return blockTime
        }

        override fun height(): Int {
            return height
        }

        override fun get(key: PublicKey): AccountState? {
            val account = accounts.get(key)
            return if (account != null) {
                account
            } else {
                val bytes = getAccountBytes(key)
                return if (bytes != null) {
                    val dbAccount = BinaryDecoder(bytes).decode(AccountState.serializer())
                    if (!dbAccount.prune(height))
                        undo.add(key, bytes)
                    else
                        undo.add(key, BinaryEncoder.toBytes(AccountState.serializer(), dbAccount))
                    dbAccount
                } else {
                    bytes
                }
            }
        }

        override fun getOrCreate(key: PublicKey): AccountState {
            val account = get(key)
            return if (account != null) {
                account
            } else {
                undo.add(key, null)
                AccountState.create()
            }
        }

        override fun set(key: PublicKey, state: AccountState) {
            accounts.set(key, state)
        }

        override fun addHTLC(id: HashTimeLockContractId, htlc: HTLC) {
            undo.addHTLC(id, null)
            htlcs.put(id, htlc)
        }

        override fun getHTLC(id: HashTimeLockContractId): HTLC? {
            return if (!htlcs.containsKey(id)) {
                getHTLCBytes(id).also { bytes ->
                    undo.addHTLC(id, bytes)
                }?.let { bytes ->
                    BinaryDecoder(bytes).decode(HTLC.serializer())
                }
            } else {
                htlcs.get(id)
            }
        }

        override fun removeHTLC(id: HashTimeLockContractId) {
            htlcs.put(id, null)
        }

        override fun addMultisig(id: MultiSignatureLockContractId, multisig: Multisig) {
            undo.addMultisig(id, null)
            multisigs.put(id, multisig)
        }

        override fun getMultisig(id: MultiSignatureLockContractId): Multisig? {
            return if (!multisigs.containsKey(id)) {
                getMultisigBytes(id).also { bytes ->
                    undo.addMultisig(id, bytes)
                }?.let { bytes ->
                    BinaryDecoder(bytes).decode(Multisig.serializer())
                }
            } else {
                multisigs.get(id)
            }
        }

        override fun removeMultisig(id: MultiSignatureLockContractId) {
            multisigs.put(id, null)
        }

        fun commitImpl() {
            val state = state

            if (blockSizes.size == PoS.BLOCK_SIZE_SPAN)
                blockSizes.removeFirst()
            blockSizes.addLast(blockSize)
            writeBlockSizes(batch)

            val difficulty = PoS.nextDifficulty(undo.difficulty, undo.blockTime, blockTime)
            val cumulativeDifficulty = PoS.cumulativeDifficulty(undo.cumulativeDifficulty, difficulty)
            val nxtrng = PoS.nxtrng(state.nxtrng, blockGenerator)
            val maxBlockSize = PoS.maxBlockSize(blockSizes)
            val upgraded = if (blockVersion.toUInt() > Block.VERSION.toUInt()) min(state.upgraded + 1, PoS.MATURITY + 1) else max(state.upgraded - 1, 0)
            val forkV2 = if (blockVersion.toUInt() >= 2.toUInt()) min(state.forkV2 + 1, PoS.MATURITY + 1) else max(state.forkV2 - 1, 0)
            val newState = State(
                    height,
                    blockHash,
                    blockTime,
                    difficulty,
                    cumulativeDifficulty,
                    supply,
                    nxtrng,
                    rollingCheckpoint,
                    maxBlockSize,
                    upgraded.toShort(),
                    forkV2.toShort())
            LedgerDB.state = newState
            batch.put(STATE_KEY, BinaryEncoder.toBytes(State.serializer(), newState))

            batch.put(UNDO_KEY, blockHash.bytes, BinaryEncoder.toBytes(UndoBlock.serializer(), undo))
            batch.put(CHAIN_KEY, blockPrevious.bytes, BinaryEncoder.toBytes(ChainIndex.serializer(), prevIndex!!))
            batch.put(CHAIN_KEY, blockHash.bytes, BinaryEncoder.toBytes(ChainIndex.serializer(), chainIndex!!))
            for ((key, account) in accounts)
                batch.put(ACCOUNT_KEY, key.bytes, BinaryEncoder.toBytes(AccountState.serializer(), account))
            for ((id, htlc) in htlcs)
                if (htlc != null)
                    batch.put(HTLC_KEY, id.hash.bytes, BinaryEncoder.toBytes(HTLC.serializer(), htlc))
                else
                    batch.delete(HTLC_KEY, id.hash.bytes)
            for ((id, multisig) in multisigs)
                if (multisig != null)
                    batch.put(MULTISIG_KEY, id.hash.bytes, BinaryEncoder.toBytes(Multisig.serializer(), multisig))
                else
                    batch.delete(MULTISIG_KEY, id.hash.bytes)

            batch.write()

            if (snapshotHeights.contains(height))
                snapshotImpl()
        }
    }

    @Serializable
    class Snapshot(
            private val balances: HashMap<PublicKey, Long> = HashMap()
    ) {
        fun supply(): Long {
            var supply = 0L
            balances.forEach { (_, balance) -> supply += balance }
            return supply
        }

        fun credit(publicKey: PublicKey, amount: Long) {
            if (amount != 0L) {
                val balance = balances.get(publicKey) ?: 0
                balances.put(publicKey, balance + amount)
            }
        }

        @Serializer(forClass = Snapshot::class)
        companion object {
            override fun deserialize(decoder: Decoder): Snapshot {
                return when (decoder) {
                    is BinaryDecoder -> {
                        val size = decoder.decodeVarInt()
                        val balances = newHashMapWithExpectedSize<PublicKey, Long>(size)
                        for (i in 0 until size)
                            balances.put(PublicKey(decoder.decodeFixedByteArray(PublicKey.SIZE_BYTES)), decoder.decodeVarLong())
                        Snapshot(balances)
                    }
                    else -> throw notSupportedDecoderError(decoder, this)
                }
            }

            override fun serialize(encoder: Encoder, value: Snapshot) {
                when (encoder) {
                    is BinaryEncoder -> {
                        encoder.encodeVarInt(value.balances.size)
                        value.balances.forEach { (publicKey, balance) ->
                            encoder.encodeFixedByteArray(publicKey.bytes)
                            encoder.encodeVarLong(balance)
                        }
                    }
                    is JsonOutput -> {
                        val balances = newHashMapWithExpectedSize<String, String>(value.balances.size)
                        value.balances.forEach { (publicKey, balance) ->
                            balances.put(publicKey.toString(), balance.toString())
                        }
                        @Suppress("NAME_SHADOWING")
                        val encoder = encoder.beginStructure(descriptor)
                        encoder.encodeSerializableElement(descriptor, 0, MapSerializer(String.serializer(), String.serializer()), balances)
                        encoder.endStructure(descriptor)
                    }
                    else -> throw notSupportedEncoderError(encoder, this)
                }
            }
        }
    }

    private fun snapshotImpl() {
        val state = state
        val snapshot = Snapshot()

        iterateImpl(
                { publicKey, account ->
                    snapshot.credit(publicKey, account.balance())
                    account.leases.forEach { lease ->
                        snapshot.credit(lease.publicKey, lease.amount)
                    }
                },
                { _, htlc ->
                    snapshot.credit(htlc.from, htlc.amount)
                },
                { _, multisig ->
                    multisig.deposits.forEach { (publicKey, amount) ->
                        snapshot.credit(publicKey, amount)
                    }
                }
        )

        if (snapshot.supply() != state.supply)
            logger.error("Snapshot supply does not match ledger")

        val batch = LevelDB.createWriteBatch()
        batch.put(SNAPSHOT_KEY, state.height.toByteArray(), BinaryEncoder.toBytes(Snapshot.serializer(), snapshot))
        batch.write()
    }
}
