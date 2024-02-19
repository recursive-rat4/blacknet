/*
 * Copyright (c) 2018-2024 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.db

import io.github.oshai.kotlinlogging.KotlinLogging
import java.math.BigInteger
import java.util.ArrayDeque //TODO check kotlin.collections.ArrayDeque
import kotlin.concurrent.withLock
import kotlin.math.max
import kotlin.math.min
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.Serializable
import kotlinx.serialization.builtins.ListSerializer
import kotlinx.serialization.builtins.SetSerializer
import ninja.blacknet.Kernel
import ninja.blacknet.Mode.*
import ninja.blacknet.contract.HashTimeLockContractId
import ninja.blacknet.contract.MultiSignatureLockContractId
import ninja.blacknet.core.*
import ninja.blacknet.crypto.*
import ninja.blacknet.mode
import ninja.blacknet.serialization.LongSerializer
import ninja.blacknet.serialization.bbf.binaryFormat
import ninja.blacknet.serialization.VarIntSerializer
import ninja.blacknet.serialization.VarLongSerializer
import ninja.blacknet.util.toByteArray

private val logger = KotlinLogging.logger {}

object LedgerDB {
    private const val VERSION = 10
    private val ACCOUNT_KEY = DBKey(1, PublicKey.SIZE_BYTES)
    internal val CHAIN_KEY = DBKey(2, Hash.SIZE_BYTES)
    private val HTLC_KEY = DBKey(3, HashTimeLockContractId.SIZE_BYTES)
    private val MULTISIG_KEY = DBKey(4, MultiSignatureLockContractId.SIZE_BYTES)
    private val UNDO_KEY = DBKey(5, Hash.SIZE_BYTES)
    private val SIZES_KEY = DBKey(6, 0)
    private val SNAPSHOT_KEY = DBKey(7, Int.SIZE_BYTES)
    private val SNAPSHOTHEIGHTS_KEY = DBKey(8, 0)
    private val STATE_KEY = DBKey(9, 0)
    private val VERSION_KEY = DBKey(10, 0)

    val chainIndexes = DBView(LevelDB, CHAIN_KEY, ChainIndex.serializer(), binaryFormat)
    private val undos = DBView(LevelDB, UNDO_KEY, UndoBlock.serializer(), binaryFormat)

    @Serializable
    internal class State(
            val height: Int,
            val blockHash: Hash,
            val blockTime: Long,
            @Serializable(with = BigIntegerSerializer::class)
            val difficulty: BigInteger,
            @Serializable(with = BigIntegerSerializer::class)
            val cumulativeDifficulty: BigInteger,
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
            val account = AccountState()
            account.stake = balance
            batch.put(ACCOUNT_KEY, publicKey.bytes, binaryFormat.encodeToByteArray(AccountState.serializer(), account))
            supply += balance
        }

        val chainIndex = ChainIndex(Hash.ZERO, Hash.ZERO, 0, 0, 0L)
        batch.put(CHAIN_KEY, Genesis.BLOCK_HASH.bytes, binaryFormat.encodeToByteArray(ChainIndex.serializer(), chainIndex))

        blockSizes.add(0)
        writeBlockSizes(batch)

        val state = State(
                0,
                Genesis.BLOCK_HASH,
                Genesis.TIME,
                PoS.INITIAL_DIFFICULTY,
                Genesis.CUMULATIVE_DIFFICULTY,
                supply,
                Hash.ZERO,
                Genesis.BLOCK_HASH,
                PoS.DEFAULT_MAX_BLOCK_SIZE,
                0,
                0)
        batch.put(STATE_KEY, binaryFormat.encodeToByteArray(State.serializer(), state))
        this.state = state

        setVersion(batch)

        batch.write()
    }

    private fun setVersion(batch: LevelDB.WriteBatch) {
        val versionBytes = binaryFormat.encodeToByteArray(VarIntSerializer, VERSION)
        batch.put(VERSION_KEY, versionBytes)
    }

    private fun writeBlockSizes(batch: LevelDB.WriteBatch) {
        val blockSizesList = blockSizes.toList() //TODO ArrayDequeSerializer
        val blockSizesBytes = binaryFormat.encodeToByteArray(ListSerializer(VarIntSerializer), blockSizesList)
        batch.put(SIZES_KEY, blockSizesBytes)
    }

    private fun writeSnapshotHeights(batch: LevelDB.WriteBatch) {
        val snapshotHeightsBytes = binaryFormat.encodeToByteArray(SetSerializer(VarIntSerializer), snapshotHeights)
        batch.put(SNAPSHOTHEIGHTS_KEY, snapshotHeightsBytes)
    }

    init {
        val snapshotHeightsBytes = LevelDB.get(SNAPSHOTHEIGHTS_KEY)
        if (snapshotHeightsBytes != null) {
            binaryFormat.decodeFromByteArray(SetSerializer(VarIntSerializer), snapshotHeightsBytes).forEach { height ->
                snapshotHeights.add(height)
            }
        }

        val blockSizesBytes = LevelDB.get(SIZES_KEY)
        if (blockSizesBytes != null) {
            //TODO ArrayDequeSerializer
            binaryFormat.decodeFromByteArray(ListSerializer(VarIntSerializer), blockSizesBytes).forEach { blockSize ->
                blockSizes.addLast(blockSize)
            }
        }

        val stateBytes = LevelDB.get(STATE_KEY)
        if (stateBytes != null) {
            val versionBytes = LevelDB.get(VERSION_KEY)!!
            val version = binaryFormat.decodeFromByteArray(VarIntSerializer, versionBytes)

            if (version == VERSION) {
                val state = binaryFormat.decodeFromByteArray(LedgerDB.State.serializer(), stateBytes)
                logger.info { "Ledger height ${state.height}" }
                this.state = state
            } else if (version in 1 until VERSION) {
                logger.info { "Reindexing ledger..." }

                runBlocking {
                    val blockHashes = ArrayList<Hash>(500000)
                    var index = chainIndexes.getOrThrow(Genesis.BLOCK_HASH.bytes)
                    while (index.next != Hash.ZERO) {
                        blockHashes.add(index.next)
                        index = chainIndexes.getOrThrow(index.next.bytes)
                    }
                    logger.info { "Found ${blockHashes.size} blocks" }

                    clear()

                    for (i in 0 until blockHashes.size) {
                        val hash = blockHashes[i]
                        val (block, size) = Kernel.blockDB().blocks.getWithSizeOrThrow(hash.bytes)
                        val batch = LevelDB.createWriteBatch()
                        val txDb = Update(batch, block.version, hash, block.previous, block.time, size, block.generator)
                        val (status, _) = processBlockImpl(txDb, hash, block, size)
                        if (status != Accepted) {
                            batch.close()
                            logger.error { "process block failed" }
                            break
                        }
                        pruneImpl(batch)
                        txDb.commitImpl()
                        if (i != 0 && i % 50000 == 0)
                            logger.info { "Processed $i blocks" }
                    }

                    logger.info { "Finished reindex at height ${state.height}" }
                }
            } else {
                throw Error("Unknown database version $version")
            }
        } else {
            loadGenesisState()
        }

        Bootstrap.import()
    }

    internal fun state(): State {
        return state
    }

    fun forkV2(): Boolean = when (mode) {
        MainNet -> state.forkV2 == (PoS.UPGRADE_THRESHOLD + 1).toShort()
        TestNet -> throw NotImplementedError("$mode fork activation is missing")
        SigNet -> throw NotImplementedError("$mode fork activation is missing")
        RegTest -> true
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
            binaryFormat.decodeFromByteArray(Snapshot.serializer(), bytes)
        }
    }

    internal fun getNextRollingCheckpoint(): Hash {
        val state = state
        if (state.rollingCheckpoint != Genesis.BLOCK_HASH) {
            val chainIndex = chainIndexes.getOrThrow(state.rollingCheckpoint.bytes)
            return chainIndex.next
        } else {
            if (state.height < PoS.ROLLBACK_LIMIT + 1)
                return Genesis.BLOCK_HASH
            val checkpoint = state.height - PoS.ROLLBACK_LIMIT
            var chainIndex = chainIndexes.getOrThrow(state.blockHash.bytes)
            while (chainIndex.height != checkpoint + 1)
                chainIndex = chainIndexes.getOrThrow(chainIndex.previous.bytes)
            return chainIndex.previous
        }
    }

    private fun getAccountBytes(key: PublicKey): ByteArray? {
        return LevelDB.get(ACCOUNT_KEY, key.bytes)
    }

    fun get(key: PublicKey): AccountState? {
        return getAccountBytes(key)?.let { bytes ->
            binaryFormat.decodeFromByteArray(AccountState.serializer(), bytes)
        }
    }

    fun checkAnchor(hash: Hash): Boolean {
        return hash == Genesis.BLOCK_HASH || chainIndexes.contains(hash.bytes)
    }

    fun getNextBlockHashes(start: Hash, max: Int): List<Hash>? = Kernel.blockDB().reentrant.readLock().withLock {
        var chainIndex = chainIndexes.get(start.bytes) ?: return@withLock null
        val result = ArrayList<Hash>(max)
        while (true) {
            val hash = chainIndex.next
            if (hash == Hash.ZERO)
                break
            result.add(hash)
            if (result.size == max)
                break
            chainIndex = chainIndexes.getOrThrow(chainIndex.next.bytes)
        }
        return result
    }

    private fun getHTLCBytes(id: HashTimeLockContractId): ByteArray? {
        return LevelDB.get(HTLC_KEY, id.bytes)
    }

    fun getHTLC(id: HashTimeLockContractId): HTLC? {
        return getHTLCBytes(id)?.let { bytes ->
            binaryFormat.decodeFromByteArray(HTLC.serializer(), bytes)
        }
    }

    private fun getMultisigBytes(id: MultiSignatureLockContractId): ByteArray? {
        return LevelDB.get(MULTISIG_KEY, id.bytes)
    }

    fun getMultisig(id: MultiSignatureLockContractId): Multisig? {
        return getMultisigBytes(id)?.let { bytes ->
            binaryFormat.decodeFromByteArray(Multisig.serializer(), bytes)
        }
    }

    internal suspend fun processBlockImpl(txDb: Update, hash: Hash, block: Block, size: Int): Pair<Status, List<Hash>> {
        val state = state
        if (block.previous != state.blockHash) {
            logger.error { "$hash not on current chain ${state.blockHash} previous ${block.previous}" }
            return Pair(NotOnThisChain(block.previous.toString()), emptyList())
        }
        if (size > state.maxBlockSize) {
            return Pair(Invalid("Too large block $size bytes, maximum ${state.maxBlockSize}"), emptyList())
        }
        if (block.time <= state.blockTime) {
            return Pair(Invalid("Timestamp is too early"), emptyList())
        }
        var generator = txDb.getAccount(block.generator)
        if (generator == null) {
            return Pair(Invalid("Block generator not found"), emptyList())
        }
        val height = txDb.height()
        val txHashes = ArrayList<Hash>(block.transactions.size)

        val pos = PoS.check(block.time, block.generator, state.nxtrng, state.difficulty, state.blockTime, generator.stakingBalance(height))
        if (pos != Accepted) {
            return Pair(pos, emptyList())
        }

        txDb.setAccount(block.generator, generator)

        var fees = 0L
        for (index in 0 until block.transactions.size) {
            val bytes = block.transactions[index]
            val tx = binaryFormat.decodeFromByteArray(Transaction.serializer(), bytes)
            val txHash = Transaction.hash(bytes)
            val status = txDb.processTransactionImpl(tx, txHash)
            if (status != Accepted) {
                return Pair(notAccepted("Transaction $index", status), emptyList())
            }
            txHashes.add(txHash)
            fees += tx.fee

            WalletDB.processTransaction(txHash, tx, bytes, block.time, height, txDb.batch)
        }

        generator = txDb.getAccount(block.generator)!!

        val mint = PoS.mint(state.supply)
        val generated = mint + fees

        val prevIndex = chainIndexes.getOrThrow(block.previous.bytes)
        prevIndex.next = hash
        prevIndex.nextSize = size
        txDb.prevIndex = prevIndex
        txDb.chainIndex = ChainIndex(block.previous, Hash.ZERO, 0, height, generated)

        txDb.addSupply(mint)
        generator.debit(height, generated)
        txDb.setAccount(block.generator, generator)

        WalletDB.processBlock(hash, block, height, generated, txDb.batch)

        return Pair(Accepted, txHashes)
    }

    private suspend fun undoBlockImpl(): Hash {
        val state = state
        val batch = LevelDB.createWriteBatch()
        val hash = state.blockHash
        val chainIndex = chainIndexes.getOrThrow(hash.bytes)
        val undo = undos.getOrThrow(hash.bytes)

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
        batch.put(STATE_KEY, binaryFormat.encodeToByteArray(State.serializer(), newState))

        val prevIndex = chainIndexes.getOrThrow(chainIndex.previous.bytes)
        prevIndex.next = Hash.ZERO
        prevIndex.nextSize = 0
        batch.put(CHAIN_KEY, chainIndex.previous.bytes, binaryFormat.encodeToByteArray(ChainIndex.serializer(), prevIndex))
        batch.delete(CHAIN_KEY, hash.bytes)

        undo.accounts.forEach { (key, bytes) ->
            if (bytes != null)
                batch.put(ACCOUNT_KEY, key.bytes, bytes)
            else
                batch.delete(ACCOUNT_KEY, key.bytes)
        }
        undo.htlcs.forEach { (id, bytes) ->
            if (bytes != null)
                batch.put(HTLC_KEY, id.bytes, bytes)
            else
                batch.delete(HTLC_KEY, id.bytes)
        }
        undo.multisigs.forEach { (id, bytes) ->
            if (bytes != null)
                batch.put(MULTISIG_KEY, id.bytes, bytes)
            else
                batch.delete(MULTISIG_KEY, id.bytes)
        }
        //TODO undo bapps

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
            val (block, size) = Kernel.blockDB().blocks.getWithSize(hash.bytes) ?: return toRemove.also {
                logger.error { "$hash not found" }
            }

            val batch = LevelDB.createWriteBatch()
            val txDb = LedgerDB.Update(batch, block.version, hash, block.previous, block.time, size, block.generator)
            val (status, _) = processBlockImpl(txDb, hash, block, size)
            if (status != Accepted) {
                batch.close()
                logger.error { "$status block $hash}" }
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
        var chainIndex = chainIndexes.getOrThrow(state.rollingCheckpoint.bytes)
        while (true) {
            val hash = chainIndex.previous
            if (!LevelDB.contains(UNDO_KEY, hash.bytes))
                break
            batch.delete(UNDO_KEY, hash.bytes)
            if (hash == Hash.ZERO)
                break
            chainIndex = chainIndexes.getOrThrow(hash.bytes)
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
        return if (state.upgraded < PoS.UPGRADE_THRESHOLD / 2)
            emptyList()
        else
            listOf("This version is obsolete, upgrade required!")
    }

    fun check(): Check = Kernel.blockDB().reentrant.readLock().withLock {
        val result = Check(false, 0, 0, 0, state.supply, 0L)
        iterateImpl(
            { _, account ->
                result.actualSupply += account.totalBalance()
                result.accounts += 1
            },
            { _, htlc ->
                result.actualSupply += htlc.amount
                result.htlcs += 1
            },
            { _, multisig ->
                result.actualSupply += multisig.amount()
                result.multisigs += 1
            }
        )
        if (result.actualSupply == result.expectedSupply)
            result.result = true
        return@withLock result
    }

    @Serializable
    class Check(
        var result: Boolean,
        var accounts: Int,
        var htlcs: Int,
        var multisigs: Int,
        @Serializable(with = LongSerializer::class)
        val expectedSupply: Long,
        @Serializable(with = LongSerializer::class)
        var actualSupply: Long,
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
                val key = PublicKey(ACCOUNT_KEY - entry ?: break)
                account(key, binaryFormat.decodeFromByteArray(AccountState.serializer(), entry.value))
            }
        }
        if (LevelDB.seek(iterator, HTLC_KEY)) {
            while (iterator.hasNext()) {
                val entry = iterator.next()
                val key = HashTimeLockContractId(HTLC_KEY - entry ?: break)
                htlc(key, binaryFormat.decodeFromByteArray(HTLC.serializer(), entry.value))
            }
        }
        if (LevelDB.seek(iterator, MULTISIG_KEY)) {
            while (iterator.hasNext()) {
                val entry = iterator.next()
                val key = MultiSignatureLockContractId(MULTISIG_KEY - entry ?: break)
                multisig(key, binaryFormat.decodeFromByteArray(Multisig.serializer(), entry.value))
            }
        }
        iterator.close()
    }

    internal class Update(
            val batch: LevelDB.WriteBatch,
            private val blockVersion: UInt,
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
                    state.forkV2,
                    ArrayList()
            ),
            var chainIndex: ChainIndex? = null,
            var prevIndex: ChainIndex? = null
    ) : Ledger {
        override fun addSupply(amount: Long) {
            supply += amount
        }

        override fun checkAnchor(hash: Hash): Boolean {
            return LedgerDB.checkAnchor(hash)
        }

        override fun blockHash(): Hash {
            return blockHash
        }

        override fun blockTime(): Long {
            return blockTime
        }

        override fun height(): Int {
            return height
        }

        override fun getAccount(key: PublicKey): AccountState? {
            val account = accounts.get(key)
            return if (account != null) {
                account
            } else {
                val bytes = getAccountBytes(key)
                return if (bytes != null) {
                    val dbAccount = binaryFormat.decodeFromByteArray(AccountState.serializer(), bytes)
                    if (!dbAccount.prune(height))
                        undo.add(key, bytes)
                    else
                        undo.add(key, binaryFormat.encodeToByteArray(AccountState.serializer(), dbAccount))
                    dbAccount
                } else {
                    bytes
                }
            }
        }

        override fun getOrCreate(key: PublicKey): AccountState {
            val account = getAccount(key)
            return if (account != null) {
                account
            } else {
                undo.add(key, null)
                AccountState()
            }
        }

        override fun setAccount(key: PublicKey, state: AccountState) {
            accounts.put(key, state)
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
                    binaryFormat.decodeFromByteArray(HTLC.serializer(), bytes)
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
                    binaryFormat.decodeFromByteArray(Multisig.serializer(), bytes)
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
            val upgraded = if (blockVersion > Block.VERSION) min(state.upgraded + 1, PoS.UPGRADE_THRESHOLD + 1) else max(state.upgraded - 1, 0)
            val forkV2 = if (blockVersion >= 2u) min(state.forkV2 + 1, PoS.UPGRADE_THRESHOLD + 1) else max(state.forkV2 - 1, 0)
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
            batch.put(STATE_KEY, binaryFormat.encodeToByteArray(State.serializer(), newState))

            batch.put(UNDO_KEY, blockHash.bytes, binaryFormat.encodeToByteArray(UndoBlock.serializer(), undo))
            batch.put(CHAIN_KEY, blockPrevious.bytes, binaryFormat.encodeToByteArray(ChainIndex.serializer(), prevIndex!!))
            batch.put(CHAIN_KEY, blockHash.bytes, binaryFormat.encodeToByteArray(ChainIndex.serializer(), chainIndex!!))
            for ((key, account) in accounts)
                batch.put(ACCOUNT_KEY, key.bytes, binaryFormat.encodeToByteArray(AccountState.serializer(), account))
            for ((id, htlc) in htlcs)
                if (htlc != null)
                    batch.put(HTLC_KEY, id.bytes, binaryFormat.encodeToByteArray(HTLC.serializer(), htlc))
                else
                    batch.delete(HTLC_KEY, id.bytes)
            for ((id, multisig) in multisigs)
                if (multisig != null)
                    batch.put(MULTISIG_KEY, id.bytes, binaryFormat.encodeToByteArray(Multisig.serializer(), multisig))
                else
                    batch.delete(MULTISIG_KEY, id.bytes)

            batch.write()

            if (snapshotHeights.contains(height))
                snapshotImpl()
        }
    }

    @Serializable
    class Snapshot(
            private val balances: HashMap<
                PublicKey,
                @Serializable(VarLongSerializer::class) Long
            > = HashMap()
    ) {
        fun supply(): Long {
            var supply = 0L
            balances.forEach { (_, balance) -> supply += balance }
            return supply
        }

        fun credit(publicKey: PublicKey, amount: Long) {
            if (amount != 0L) {
                val balance = balances.get(publicKey) ?: 0L
                balances.put(publicKey, balance + amount)
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
            logger.error { "Snapshot supply does not match ledger" }

        val batch = LevelDB.createWriteBatch()
        batch.put(SNAPSHOT_KEY, state.height.toByteArray(), binaryFormat.encodeToByteArray(Snapshot.serializer(), snapshot))
        batch.write()
    }
}
