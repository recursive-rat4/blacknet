/*
 * Copyright (c) 2018 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.db

import com.google.common.collect.Maps.newHashMapWithExpectedSize
import com.google.common.io.Resources
import com.google.common.primitives.Ints
import kotlinx.coroutines.runBlocking
import kotlinx.coroutines.sync.withLock
import kotlinx.serialization.*
import kotlinx.serialization.internal.HashMapSerializer
import kotlinx.serialization.json.JsonOutput
import mu.KotlinLogging
import ninja.blacknet.Config
import ninja.blacknet.core.*
import ninja.blacknet.crypto.BigInt
import ninja.blacknet.crypto.Hash
import ninja.blacknet.crypto.PoS
import ninja.blacknet.crypto.PublicKey
import ninja.blacknet.network.Network
import ninja.blacknet.serialization.BinaryDecoder
import ninja.blacknet.serialization.BinaryEncoder
import ninja.blacknet.serialization.Json
import ninja.blacknet.util.buffered
import ninja.blacknet.util.data
import ninja.blacknet.util.startsWith
import java.io.File
import java.util.ArrayDeque
import kotlin.math.max
import kotlin.math.min

private val logger = KotlinLogging.logger {}

object LedgerDB {
    private const val VERSION = 6
    private val ACCOUNT_KEY = "account".toByteArray()
    private val CHAIN_KEY = "chain".toByteArray()
    private val HTLC_KEY = "htlc".toByteArray()
    private val MULTISIG_KEY = "multisig".toByteArray()
    private val UNDO_KEY = "undo".toByteArray()
    private val SIZES_KEY = "ledgersizes".toByteArray()
    private val SNAPSHOT_KEY = "ledgersnapshot".toByteArray()
    private val SNAPSHOTHEIGHTS_KEY = "ledgersnapshotheights".toByteArray()
    private val STATE_KEY = "ledgerstate".toByteArray()
    private val VERSION_KEY = "ledgerversion".toByteArray()

    const val GENESIS_TIME = 1545555600L
    private fun genesisState() = State(0, Hash.ZERO, GENESIS_TIME, PoS.INITIAL_DIFFICULTY, BigInt.ZERO, 0, Hash.ZERO, Hash.ZERO, 0, 0)

    val genesisBlock by lazy {
        val map = HashMap<PublicKey, Long>()

        val genesis = Resources.toString(Resources.getResource("genesis.json"), Charsets.UTF_8)
        val entries = Json.parse(GenesisJsonEntry.serializer().list, genesis)
        entries.forEach {
            val publicKey = PublicKey.fromString(it.publicKey)!!
            map.put(publicKey, it.balance)
        }

        map
    }

    @Serializable
    private class GenesisJsonEntry(val publicKey: String, val balance: Long)

    @Serializable
    private class State(
            @Volatile
            var height: Int,
            @Volatile
            var blockHash: Hash,
            @Volatile
            var blockTime: Long,
            @Volatile
            var difficulty: BigInt,
            @Volatile
            var cumulativeDifficulty: BigInt,
            @Volatile
            var supply: Long,
            @Volatile
            var nxtrng: Hash,
            @Volatile
            var rollingCheckpoint: Hash,
            @Volatile
            var upgraded: Int,
            @Volatile
            var forkV2: Int
    ) {
        fun serialize(): ByteArray = BinaryEncoder.toBytes(serializer(), this)

        companion object {
            fun deserialize(bytes: ByteArray): State = BinaryDecoder.fromBytes(bytes).decode(serializer())
        }
    }

    private lateinit var state: State

    private val blockSizes = ArrayDeque<Int>(PoS.BLOCK_SIZE_SPAN)
    const val DEFAULT_MAX_BLOCK_SIZE = 100000
    const val MAX_BLOCK_SIZE = Int.MAX_VALUE - Network.RESERVED
    private var maxBlockSize: Int
    private val snapshotHeights = HashSet<Int>()

    private fun loadGenesisState() {
        state = genesisState()

        val batch = LevelDB.createWriteBatch()

        var supply = 0L
        genesisBlock.forEach { (publicKey, balance) ->
            val account = AccountState.create(balance)
            batch.put(ACCOUNT_KEY, publicKey.bytes, account.serialize())
            supply += balance
        }

        val chainIndex = ChainIndex(Hash.ZERO, Hash.ZERO, 0, 0, 0)
        batch.put(CHAIN_KEY, Hash.ZERO.bytes, chainIndex.serialize())

        blockSizes.add(0)
        writeBlockSizes(batch)
        state.supply = supply
        batch.put(STATE_KEY, state.serialize())

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
            val decoder = BinaryDecoder.fromBytes(snapshotHeightsBytes)
            val size = decoder.decodeVarInt()
            for (i in 0 until size)
                snapshotHeights.add(decoder.decodeVarInt())
        }

        val blockSizesBytes = LevelDB.get(SIZES_KEY)
        if (blockSizesBytes != null) {
            val decoder = BinaryDecoder.fromBytes(blockSizesBytes)
            val size = decoder.decodeVarInt()
            for (i in 0 until size)
                blockSizes.addLast(decoder.decodeVarInt())
        }
        maxBlockSize = calcMaxBlockSize()

        val stateBytes = LevelDB.get(STATE_KEY)
        if (stateBytes != null) {
            val versionBytes = LevelDB.get(VERSION_KEY)!!
            val version = BinaryDecoder.fromBytes(versionBytes).decodeVarInt()

            if (version == VERSION) {
                state = LedgerDB.State.deserialize(stateBytes)
                logger.info("Blockchain height ${state.height}")
            } else if (version in 1..5) {
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
                        val txHashes = processBlockImpl(txDb, hash, block, size)
                        if (txHashes == null) {
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

        val bootstrap = File(Config.dataDir, "bootstrap.dat")
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

                        val hash = Block.Hasher(bytes)
                        val status = BlockDB.process(hash, bytes)
                        if (status == DataDB.Status.ACCEPTED) {
                            if (++n % 50000 == 0)
                                logger.info("Processed $n blocks")
                            prune()
                        } else if (status != DataDB.Status.ALREADY_HAVE) {
                            logger.info("$status block $hash")
                            break
                        }
                    }
                } catch (e: Throwable) {
                    logger.debug { e }
                } finally {
                    stream.close()
                }

                val f = File(Config.dataDir, "bootstrap.dat.old")
                f.delete()
                bootstrap.renameTo(f)

                logger.info("Imported $n blocks")
            }
        }
    }

    fun height(): Int {
        return state.height
    }

    fun blockHash(): Hash {
        return state.blockHash
    }

    fun blockTime(): Long {
        return state.blockTime
    }

    fun difficulty(): BigInt {
        return state.difficulty
    }

    fun cumulativeDifficulty(): BigInt {
        return state.cumulativeDifficulty
    }

    fun supply(): Long {
        return state.supply
    }

    fun nxtrng(): Hash {
        return state.nxtrng
    }

    fun rollingCheckpoint(): Hash {
        return state.rollingCheckpoint
    }

    fun upgraded(): Int {
        return state.upgraded
    }

    fun forkV2(): Boolean {
        return state.forkV2 == PoS.MATURITY + 1
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
        val bytes = LevelDB.get(SNAPSHOT_KEY, Ints.toByteArray(height)) ?: return null
        return Snapshot.deserialize(bytes)
    }

    internal fun getNextRollingCheckpoint(): Hash {
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

    fun get(key: PublicKey): AccountState? {
        val bytes = LevelDB.get(ACCOUNT_KEY, key.bytes) ?: return null
        return AccountState.deserialize(bytes)
    }

    private fun getUndo(hash: Hash): UndoBlock {
        return UndoBlock.deserialize(LevelDB.get(UNDO_KEY, hash.bytes)!!)
    }

    fun getChainIndex(hash: Hash): ChainIndex? {
        val bytes = LevelDB.get(CHAIN_KEY, hash.bytes) ?: return null
        return ChainIndex.deserialize(bytes)
    }

    fun checkReferenceChain(hash: Hash): Boolean {
        return hash == Hash.ZERO || chainContains(hash)
    }

    fun maxBlockSize(): Int {
        return maxBlockSize
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

    fun getBlockNumber(hash: Hash): Int? {
        val bytes = LevelDB.get(CHAIN_KEY, hash.bytes) ?: return null
        return ChainIndex.deserialize(bytes).height
    }

    fun getHTLC(id: Hash): HTLC? {
        val bytes = LevelDB.get(HTLC_KEY, id.bytes) ?: return null
        return HTLC.deserialize(bytes)
    }

    fun getMultisig(id: Hash): Multisig? {
        val bytes = LevelDB.get(MULTISIG_KEY, id.bytes) ?: return null
        return Multisig.deserialize(bytes)
    }

    private fun calcMaxBlockSize(): Int {
        if (blockSizes.size < PoS.BLOCK_SIZE_SPAN)
            return DEFAULT_MAX_BLOCK_SIZE
        val iterator = blockSizes.iterator()
        val sizes = Array(PoS.BLOCK_SIZE_SPAN) { iterator.next() }
        sizes.sort()
        val median = sizes[PoS.BLOCK_SIZE_SPAN / 2]
        val size = median * 2
        if (size < 0 || size > MAX_BLOCK_SIZE)
            return MAX_BLOCK_SIZE
        else if (size < DEFAULT_MAX_BLOCK_SIZE)
            return DEFAULT_MAX_BLOCK_SIZE
        else
            return size
    }

    internal suspend fun processBlockImpl(txDb: Update, hash: Hash, block: Block, size: Int): ArrayList<Hash>? {
        if (block.previous != state.blockHash) {
            logger.error("not on current chain")
            return null
        }
        if (size > maxBlockSize) {
            logger.info("too large block $size bytes, maximum ${maxBlockSize()}")
            return null
        }
        if (block.time <= state.blockTime) {
            logger.info("timestamp is too early")
            return null
        }
        var generator = txDb.get(block.generator)
        if (generator == null) {
            logger.info("block generator not found")
            return null
        }
        val height = txDb.height()
        val txHashes = ArrayList<Hash>(block.transactions.size)

        if (!PoS.check(block.time, block.generator, txDb.undo.nxtrng, txDb.undo.difficulty, txDb.undo.blockTime, generator.stakingBalance(height))) {
            logger.info("invalid proof of stake")
            return null
        }

        txDb.set(block.generator, generator)

        var fees = 0L
        for (bytes in block.transactions) {
            val tx = Transaction.deserialize(bytes.array)
            val txHash = Transaction.Hasher(bytes.array)
            val status = txDb.processTransactionImpl(tx, txHash, bytes.array.size)
            if (status != DataDB.Status.ACCEPTED) {
                logger.info("$status tx $txHash")
                return null
            }
            txHashes.add(txHash)
            fees += tx.fee

            WalletDB.processTransaction(txHash, tx, bytes.array, block.time, height, txDb.batch)
        }

        generator = txDb.get(block.generator)!!

        val reward = if (forkV2()) PoS.reward(txDb.undo.supply) else PoS.reward(state.supply)
        val generated = reward + fees

        val prevIndex = getChainIndex(block.previous)!!
        prevIndex.next = hash
        prevIndex.nextSize = size
        txDb.prevIndex = prevIndex
        txDb.chainIndex = ChainIndex(block.previous, Hash.ZERO, 0, height, generated)

        txDb.addSupply(reward)
        generator.debit(height, generated)
        txDb.set(block.generator, generator)

        WalletDB.processBlock(hash, block, height, generated, txDb.batch)

        return txHashes
    }

    private suspend fun undoBlockImpl(): Hash {
        val batch = LevelDB.createWriteBatch()
        val hash = state.blockHash
        val chainIndex = getChainIndex(hash)!!
        val undo = getUndo(hash)

        val height = state.height
        state.height = height - 1
        state.cumulativeDifficulty = undo.cumulativeDifficulty
        state.supply = undo.supply
        state.blockHash = chainIndex.previous
        state.blockTime = undo.blockTime
        state.difficulty = undo.difficulty
        blockSizes.removeLast()
        blockSizes.addFirst(undo.blockSize)
        state.nxtrng = undo.nxtrng
        state.rollingCheckpoint = undo.rollingCheckpoint
        state.upgraded = undo.upgraded
        state.forkV2 = undo.forkV2
        batch.put(STATE_KEY, state.serialize())
        writeBlockSizes(batch)

        val prevIndex = getChainIndex(chainIndex.previous)!!
        prevIndex.next = Hash.ZERO
        prevIndex.nextSize = 0
        batch.put(CHAIN_KEY, chainIndex.previous.bytes, prevIndex.serialize())
        batch.delete(CHAIN_KEY, hash.bytes)

        undo.accounts.forEach {
            val key = it.first
            val state = it.second
            if (!state.isEmpty())
                batch.put(ACCOUNT_KEY, key.bytes, state.serialize())
            else
                batch.delete(ACCOUNT_KEY, key.bytes)
        }
        undo.htlcs.forEach {
            val id = it.first
            val htlc = it.second
            if (htlc != null)
                batch.put(HTLC_KEY, id.bytes, htlc.serialize())
            else
                batch.delete(HTLC_KEY, id.bytes)
        }
        undo.multisigs.forEach {
            val id = it.first
            val multisig = it.second
            if (multisig != null)
                batch.put(MULTISIG_KEY, id.bytes, multisig.serialize())
            else
                batch.delete(MULTISIG_KEY, id.bytes)
        }

        batch.delete(UNDO_KEY, hash.bytes)

        WalletDB.disconnectBlock(hash, batch)

        batch.write()

        return hash
    }

    internal suspend fun rollbackTo(hash: Hash): ArrayList<Hash> = BlockDB.mutex.withLock {
        return@withLock rollbackToImpl(hash, false)
    }

    private suspend fun rollbackToImpl(hash: Hash, allowZero: Boolean): ArrayList<Hash> {
        val i = getBlockNumber(hash) ?: return ArrayList()
        var n = state.height - i
        if (allowZero && n == 0)
            return ArrayList()
        if (n <= 0) throw RuntimeException("Rollback of $n blocks")
        val result = ArrayList<Hash>(n)
        while (n-- > 0)
            result.add(undoBlockImpl())
        return result
    }

    internal suspend fun undoRollback(hash: Hash, list: ArrayList<Hash>): ArrayList<Hash> = BlockDB.mutex.withLock {
        val toRemove = rollbackToImpl(hash, true)

        list.asReversed().forEach { hash ->
            val block = BlockDB.blockImpl(hash)
            if (block == null) {
                logger.error("block not found")
                return@withLock toRemove
            }

            val batch = LevelDB.createWriteBatch()
            val txDb = LedgerDB.Update(batch, block.first.version, hash, block.first.previous, block.first.time, block.second, block.first.generator)
            val txHashes = processBlockImpl(txDb, hash, block.first, block.second)
            if (txHashes == null) {
                batch.close()
                logger.error("process block failed")
                return@withLock toRemove
            }
            txDb.commitImpl()
        }

        return@withLock toRemove
    }

    internal suspend fun prune() = BlockDB.mutex.withLock {
        val batch = LevelDB.createWriteBatch()
        pruneImpl(batch)
        batch.write()
    }

    internal fun pruneImpl(batch: LevelDB.WriteBatch) {
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
            if (entry.key.startsWith(ACCOUNT_KEY) ||
                    entry.key.startsWith(CHAIN_KEY) ||
                    entry.key.startsWith(HTLC_KEY) ||
                    entry.key.startsWith(MULTISIG_KEY) ||
                    entry.key.startsWith(UNDO_KEY) ||
                    entry.key!!.contentEquals(SIZES_KEY) ||
                    entry.key.startsWith(SNAPSHOT_KEY) ||
                    entry.key!!.contentEquals(STATE_KEY) ||
                    entry.key!!.contentEquals(VERSION_KEY)) {
                batch.delete(entry.key)
            }
        }
        iterator.close()
        batch.write()

        blockSizes.clear()

        loadGenesisState()
    }

    fun warnings(): List<String> {
        if (state.upgraded > PoS.MATURITY / 2)
            return listOf("This version is obsolete, upgrade required!")

        return emptyList()
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
            htlc: (Hash, HTLC) -> Unit,
            multisig: (Hash, Multisig) -> Unit
    ) {
        val iterator = LevelDB.iterator()
        if (LevelDB.seek(iterator, ACCOUNT_KEY)) {
            while (iterator.hasNext()) {
                val entry = iterator.next()
                if (entry.key.startsWith(ACCOUNT_KEY))
                    account(PublicKey(LevelDB.sliceKey(entry, ACCOUNT_KEY)), AccountState.deserialize(entry.value))
                else
                    break
            }
        }
        if (LevelDB.seek(iterator, HTLC_KEY)) {
            while (iterator.hasNext()) {
                val entry = iterator.next()
                if (entry.key.startsWith(HTLC_KEY))
                    htlc(Hash(LevelDB.sliceKey(entry, HTLC_KEY)), HTLC.deserialize(entry.value))
                else
                    break
            }
        }
        if (LevelDB.seek(iterator, MULTISIG_KEY)) {
            while (iterator.hasNext()) {
                val entry = iterator.next()
                if (entry.key.startsWith(MULTISIG_KEY))
                    multisig(Hash(LevelDB.sliceKey(entry, MULTISIG_KEY)), Multisig.deserialize(entry.value))
                else
                    break
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
            private val height: Int = state.height + 1,
            private var supply: Long = state.supply,
            private val rollingCheckpoint: Hash = LedgerDB.getNextRollingCheckpoint(),
            private val accounts: MutableMap<PublicKey, AccountState> = HashMap(),
            private val htlcs: MutableMap<Hash, HTLC?> = HashMap(),
            private val multisigs: MutableMap<Hash, Multisig?> = HashMap(),
            val undo: UndoBuilder = UndoBuilder(
                    state.blockTime,
                    state.difficulty,
                    state.cumulativeDifficulty,
                    state.supply,
                    state.nxtrng,
                    state.rollingCheckpoint,
                    state.upgraded,
                    blockSizes.peekFirst(),
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
            if (account != null) {
                undo.add(key, account)
                return account
            }
            val dbAccount = LedgerDB.get(key)
            if (dbAccount != null) {
                dbAccount.prune(height)
                undo.add(key, dbAccount)
            }
            return dbAccount
        }

        override fun getOrCreate(key: PublicKey): AccountState {
            val account = get(key) ?: AccountState.create()
            undo.add(key, account)
            return account
        }

        override fun set(key: PublicKey, state: AccountState) {
            accounts.set(key, state)
        }

        override fun addHTLC(id: Hash, htlc: HTLC) {
            undo.addHTLC(id, null)
            htlcs.put(id, htlc)
        }

        override fun getHTLC(id: Hash): HTLC? {
            val htlc = if (!htlcs.containsKey(id))
                LedgerDB.getHTLC(id)
            else
                htlcs.get(id)
            undo.addHTLC(id, htlc)
            return htlc
        }

        override fun removeHTLC(id: Hash) {
            htlcs.put(id, null)
        }

        override fun addMultisig(id: Hash, multisig: Multisig) {
            undo.addMultisig(id, null)
            multisigs.put(id, multisig)
        }

        override fun getMultisig(id: Hash): Multisig? {
            val multisig = if (!multisigs.containsKey(id))
                LedgerDB.getMultisig(id)
            else
                multisigs.get(id)
            undo.addMultisig(id, multisig)
            return multisig
        }

        override fun removeMultisig(id: Hash) {
            multisigs.put(id, null)
        }

        fun commitImpl() {
            batch.put(UNDO_KEY, blockHash.bytes, undo.build().serialize())
            batch.put(CHAIN_KEY, blockPrevious.bytes, prevIndex!!.serialize())
            batch.put(CHAIN_KEY, blockHash.bytes, chainIndex!!.serialize())
            for (account in accounts)
                batch.put(ACCOUNT_KEY, account.key.bytes, account.value.serialize())
            for (htlc in htlcs)
                if (htlc.value != null)
                    batch.put(HTLC_KEY, htlc.key.bytes, htlc.value!!.serialize())
                else
                    batch.delete(HTLC_KEY, htlc.key.bytes)
            for (multisig in multisigs)
                if (multisig.value != null)
                    batch.put(MULTISIG_KEY, multisig.key.bytes, multisig.value!!.serialize())
                else
                    batch.delete(MULTISIG_KEY, multisig.key.bytes)

            state.blockHash = blockHash
            state.blockTime = blockTime
            state.height = height
            state.supply = supply
            state.nxtrng = PoS.nxtrng(state.nxtrng, blockGenerator)
            state.rollingCheckpoint = rollingCheckpoint
            state.upgraded = if (blockVersion.toUInt() > Block.VERSION.toUInt()) min(++state.upgraded, PoS.MATURITY + 1) else max(--state.upgraded, 0)
            state.forkV2 = if (blockVersion.toUInt() >= 2.toUInt()) min(++state.forkV2, PoS.MATURITY + 1) else max(--state.forkV2, 0)
            val difficulty = PoS.nextDifficulty(undo.difficulty, undo.blockTime, blockTime)
            state.difficulty = difficulty
            state.cumulativeDifficulty = PoS.cumulativeDifficulty(undo.cumulativeDifficulty, difficulty)
            batch.put(STATE_KEY, state.serialize())

            if (blockSizes.size == PoS.BLOCK_SIZE_SPAN)
                blockSizes.removeFirst()
            blockSizes.addLast(blockSize)
            maxBlockSize = calcMaxBlockSize()
            writeBlockSizes(batch)

            batch.write()

            if (snapshotHeights.contains(height))
                snapshotImpl()
        }
    }

    @Serializable
    class Snapshot(
            private val balances: HashMap<PublicKey, Long> = HashMap()
    ) {
        fun serialize(): ByteArray = BinaryEncoder.toBytes(serializer(), this)

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
            fun deserialize(bytes: ByteArray): Snapshot = BinaryDecoder.fromBytes(bytes).decode(serializer())

            override fun deserialize(decoder: Decoder): Snapshot {
                return when (decoder) {
                    is BinaryDecoder -> {
                        val size = decoder.decodeVarInt()
                        val balances = newHashMapWithExpectedSize<PublicKey, Long>(size)
                        for (i in 0 until size)
                            balances.put(PublicKey(decoder.decodeFixedByteArray(PublicKey.SIZE)), decoder.decodeVarLong())
                        Snapshot(balances)
                    }
                    else -> throw RuntimeException("Unsupported decoder")
                }
            }

            override fun serialize(encoder: Encoder, obj: Snapshot) {
                when (encoder) {
                    is BinaryEncoder -> {
                        encoder.encodeVarInt(obj.balances.size)
                        obj.balances.forEach { (publicKey, balance) ->
                            encoder.encodeFixedByteArray(publicKey.bytes)
                            encoder.encodeVarLong(balance)
                        }
                    }
                    is JsonOutput -> {
                        val balances = newHashMapWithExpectedSize<String, String>(obj.balances.size)
                        obj.balances.forEach { (publicKey, balance) ->
                            balances.put(publicKey.toString(), balance.toString())
                        }
                        @Suppress("NAME_SHADOWING")
                        val encoder = encoder.beginStructure(descriptor)
                        encoder.encodeSerializableElement(descriptor, 0, HashMapSerializer(String.serializer(), String.serializer()), balances)
                        encoder.endStructure(descriptor)
                    }
                    else -> throw RuntimeException("Unsupported encoder")
                }
            }
        }
    }

    private fun snapshotImpl() {
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
        batch.put(SNAPSHOT_KEY, Ints.toByteArray(state.height), snapshot.serialize())
        batch.write()
    }
}
