/*
 * Copyright (c) 2018 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.db

import io.ktor.util.error
import kotlinx.coroutines.runBlocking
import kotlinx.coroutines.sync.Mutex
import kotlinx.coroutines.sync.withLock
import kotlinx.serialization.Serializable
import kotlinx.serialization.json.Json
import kotlinx.serialization.list
import mu.KotlinLogging
import ninja.blacknet.core.*
import ninja.blacknet.crypto.BigInt
import ninja.blacknet.crypto.Hash
import ninja.blacknet.crypto.PublicKey
import org.mapdb.DBMaker
import org.mapdb.Serializer
import java.io.File
import kotlin.math.max

private val logger = KotlinLogging.logger {}

object LedgerDB : Ledger {
    private val mutex = Mutex()
    private val db = DBMaker.fileDB("db/ledger").transactionEnable().closeOnJvmShutdown().make()
    private val accounts = db.hashMap("accounts", PublicKeySerializer, AccountStateSerializer).createOrOpen()
    private val height = db.atomicInteger("height").createOrOpen()
    private val blockHash = db.atomicVar("blockHash", HashSerializer, Hash.ZERO).createOrOpen()
    private val blockTime = db.atomicLong("blockTime").createOrOpen()
    private val difficulty = db.atomicVar("difficulty", BigIntSerializer, BigInt.ZERO).createOrOpen()
    private val cumulativeDifficulty = db.atomicVar("cumulativeDifficulty", BigIntSerializer, BigInt.ZERO).createOrOpen()
    private val supply = db.atomicLong("supply").createOrOpen()
    private val undo = db.hashMap("undo", HashSerializer, UndoSerializer).createOrOpen()
    private val blockSizes = db.indexTreeList("blockSizes", Serializer.INTEGER).createOrOpen()
    private val nxtrng = db.atomicVar("nxtrng", HashSerializer, Hash.ZERO).createOrOpen()
    private val chain = db.indexTreeList("chain", HashSerializer).createOrOpen()
    private val chainIndex = db.hashMap("chainIndex", HashSerializer, Serializer.INTEGER).createOrOpen()
    private val htlcs = db.hashMap("htlcs", HashSerializer, HTLCSerializer).createOrOpen()
    private val multisigs = db.hashMap("multisigs", HashSerializer, MultisigSerializer).createOrOpen()
    private val updatedV1 = db.atomicBoolean("updatedV1", false).createOrOpen()

    private var maxBlockSize: Int

    init {
        val rescanHashes = ArrayList<Hash>()
        val rescanBlocks = ArrayList<ByteArray>()

        if (!updatedV1.get()) {
            logger.info("Rescanning blockchain...")
            runBlocking {
                rescanHashes.ensureCapacity(height())
                rescanBlocks.ensureCapacity(height())
                try {
                    for (i in 1..height()) {
                        rescanHashes.add(chain[i]!!)
                        rescanBlocks.add(BlockDB.get(chain[i]!!)!!)
                    }
                } catch (e: Throwable) {
                    logger.error(e)
                }
                logger.info("Loaded ${rescanBlocks.size} blocks")

                accounts.clear()
                height.set(0)
                blockHash.set(Hash.ZERO)
                blockTime.set(0)
                difficulty.set(BigInt.ZERO)
                cumulativeDifficulty.set(BigInt.ZERO)
                supply.set(0)
                undo.clear()
                blockSizes.clear()
                nxtrng.set(Hash.ZERO)
                chain.clear()
                chainIndex.clear()
                htlcs.clear()
                multisigs.clear()
                commit()
                BlockDB.clear()
            }
        }

        maxBlockSize = calcMaxBlockSize()

        @Serializable
        class Entry(val publicKey: String, val balance: Long)

        if (accounts.isEmpty()) {
            val genesis = File("config/genesis.json").readText()
            val list = Json.parse(Entry.serializer().list, genesis)

            var supply = 0L
            for (i in list) {
                val publicKey = PublicKey.fromString(i.publicKey)!!
                val account = AccountState.create(i.balance)
                runBlocking {
                    set(publicKey, account)
                }
                supply += i.balance
            }

            addSupply(supply)
            blockSizes.add(0)
            chain.add(Hash.ZERO)
            chainIndex[Hash.ZERO] = 0
            blockTime.set(1545555600)
            difficulty.set(PoS.INITIAL_DIFFICULTY)
            updatedV1.set(true)
            commit()
            logger.info("loaded genesis.json ${accounts()} accounts, supply = ${supply()}")
        }

        if (rescanBlocks.isNotEmpty()) {
            runBlocking {
                for (i in 0 until rescanBlocks.size) {
                    if (BlockDB.process(rescanHashes[i], rescanBlocks[i]) != DataDB.Status.ACCEPTED)
                        break
                    if (i % 5000 == 4999) {
                        logger.info("Rescanned 5000 blocks")
                        prune()
                    }
                }
            }
        }
    }

    fun commit() {
        db.commit()
    }

    fun rollback() {
        db.rollback()
    }

    override fun height(): Int {
        return height.get()
    }

    fun blockHash(): Hash {
        return blockHash.get()
    }

    override fun blockTime(): Long {
        return blockTime.get()
    }

    fun difficulty(): BigInt {
        return difficulty.get()
    }

    fun cumulativeDifficulty(): BigInt {
        return cumulativeDifficulty.get()
    }

    fun getRollingCheckpoint(): Hash {
        val height = height()
        if (height < PoS.MATURITY)
            return Hash.ZERO
        return chain[height - PoS.MATURITY]!!
    }

    fun supply(): Long {
        return supply.get()
    }

    fun accounts(): Int {
        return accounts.size
    }

    fun htlcs(): Int {
        return htlcs.size
    }

    fun multisigs(): Int {
        return multisigs.size
    }

    override suspend fun get(key: PublicKey): AccountState? {
        return accounts[key]
    }

    override suspend fun set(key: PublicKey, state: AccountState) {
        accounts[key] = state
    }

    private fun remove(key: PublicKey) {
        accounts.remove(key)
    }

    override fun addSupply(amount: Long) {
        supply.set(supply.get() + amount)
    }

    private fun setSupply(amount: Long) {
        supply.set(amount)
    }

    override fun addUndo(hash: Hash, undo: UndoBlock) {
        this.undo[hash] = undo
    }

    private fun removeUndo(hash: Hash) {
        this.undo.remove(hash)
    }

    override fun checkBlockHash(hash: Hash) = hash == Hash.ZERO || chainIndex.containsKey(hash)
    override fun checkFee(size: Int, amount: Long) = amount >= 0

    fun maxBlockSize(): Int {
        return maxBlockSize
    }

    fun nxtrng(): Hash {
        return nxtrng.get()
    }

    fun getBlockHash(index: Int): Hash? {
        return chain.getOrNull(index)
    }

    suspend fun getNextBlockHashes(start: Hash, max: Int): ArrayList<Hash> = mutex.withLock {
        var i = getBlockNumber(start) ?: return ArrayList()
        val ret = ArrayList<Hash>(max)
        while (true) {
            i++
            val hash = chain.getOrNull(i) ?: break
            ret.add(hash)
            if (ret.size >= max)
                break
        }
        return ret
    }

    fun getBlockNumber(hash: Hash): Int? {
        return chainIndex[hash]
    }

    override fun addHTLC(id: Hash, htlc: HTLC) {
        htlcs[id] = htlc
    }

    override fun getHTLC(id: Hash): HTLC? {
        return htlcs[id]
    }

    override fun removeHTLC(id: Hash) {
        htlcs.remove(id)
    }

    override fun addMultisig(id: Hash, multisig: Multisig) {
        multisigs[id] = multisig
    }

    override fun getMultisig(id: Hash): Multisig? {
        return multisigs[id]
    }

    override fun removeMultisig(id: Hash) {
        multisigs.remove(id)
    }

    private fun calcMaxBlockSize(): Int {
        val default = 100000
        val height = height()
        if (height < PoS.BLOCK_SIZE_SPAN)
            return default
        val sizes = Array(PoS.BLOCK_SIZE_SPAN) { blockSizes[height - it]!! }
        sizes.sort()
        val median = sizes[PoS.BLOCK_SIZE_SPAN / 2]
        return max(default, median * 2)
    }

    suspend fun processBlock(hash: Hash, block: Block, size: Int, txHashes: ArrayList<Hash>): Boolean = mutex.withLock {
        return@withLock processBlockUnlocked(hash, block, size, txHashes)
    }

    private suspend fun processBlockUnlocked(hash: Hash, block: Block, size: Int, txHashes: ArrayList<Hash>): Boolean {
        if (block.previous != blockHash()) {
            logger.error("not on current chain")
            return false
        }
        if (size > maxBlockSize()) {
            logger.info("too large block $size bytes, maximum ${maxBlockSize()}")
            return false
        }
        if (block.time <= blockTime()) {
            logger.info("timestamp is too early")
            return false
        }
        var generator = get(block.generator)
        if (generator == null) {
            logger.info("block generator not found")
            return false
        }

        val undo = UndoBlock(
                blockTime(),
                difficulty(),
                cumulativeDifficulty(),
                supply(),
                nxtrng(),
                UndoList(),
                UndoHTLCList(),
                UndoMultisigList())

        if (!PoS.check(block.time, block.generator, undo.nxtrng, undo.difficulty, undo.blockTime, generator.stakingBalance(height()))) {
            logger.info("invalid proof of stake")
            return false
        }

        val height = height.get() + 1
        this.height.set(height)
        blockHash.set(hash)
        blockTime.set(block.time)
        blockSizes.add(size)
        nxtrng.set(PoS.nxtrng(nxtrng(), block.generator))
        chain.add(hash)
        chainIndex[hash] = height
        val difficulty = PoS.nextDifficulty(undo.difficulty, undo.blockTime, block.time)
        this.difficulty.set(difficulty)
        cumulativeDifficulty.set(PoS.cumulativeDifficulty(undo.cumulativeDifficulty, difficulty))

        var fees = 0L
        for (bytes in block.transactions) {
            val tx = Transaction.deserialize(bytes.array)
            if (tx == null) {
                logger.info("deserialization failed")
                return false
            }
            val txHash = Transaction.Hasher(bytes.array)
            if (!processTransaction(tx, txHash, bytes.array.size, undo)) {
                logger.info("invalid transaction")
                return false
            }
            txHashes.add(txHash)
            fees += tx.fee
        }

        generator = get(block.generator)!!
        undo.accounts.add(Pair(block.generator, generator.copy()))
        val reward = PoS.reward(supply())
        addUndo(hash, undo)
        addSupply(reward)
        generator.prune(height())
        generator.debit(height(), reward + fees)
        set(block.generator, generator)
        maxBlockSize = calcMaxBlockSize()

        return true
    }

    private suspend fun undoBlock(): Hash {
        val hash = blockHash()
        val undo = this.undo[hash]!!

        val height = height.get()
        this.height.set(height - 1)
        cumulativeDifficulty.set(undo.cumulativeDifficulty)
        blockHash.set(chain[height - 1])
        blockTime.set(undo.blockTime)
        difficulty.set(undo.difficulty)
        blockSizes.removeAt(height)
        nxtrng.set(undo.nxtrng)
        chain.removeAt(height)
        chainIndex.remove(hash)

        setSupply(undo.supply)
        undo.accounts.asReversed().forEach {
            val key = it.first
            val state = it.second
            if (state.isEmpty())
                remove(key)
            else
                set(key, state)
        }
        undo.htlcs.asReversed().forEach {
            val id = it.first
            val htlc = it.second
            if (htlc != null)
                addHTLC(id, htlc)
            else
                removeHTLC(id)
        }
        undo.multisigs.asReversed().forEach {
            val id = it.first
            val multisig = it.second
            if (multisig != null)
                addMultisig(id, multisig)
            else
                removeMultisig(id)
        }

        removeUndo(hash)
        return hash
    }

    suspend fun rollbackTo(hash: Hash): ArrayList<Hash> = mutex.withLock {
        return@withLock rollbackToUnlocked(hash)
    }

    private suspend fun rollbackToUnlocked(hash: Hash): ArrayList<Hash> {
        val i = getBlockNumber(hash) ?: return ArrayList()
        val height = height()
        var n = height - i
        val ret = ArrayList<Hash>(n)
        while (n-- > 0)
            ret.add(undoBlock())
        return ret
    }

    suspend fun undoRollback(hash: Hash, list: ArrayList<Hash>): ArrayList<Hash> = mutex.withLock {
        val toRemove = rollbackToUnlocked(hash)

        list.asReversed().forEach {
            val block = BlockDB.block(it)
            if (block == null) {
                logger.error("block not found")
                return@withLock toRemove
            }
            val txHashes = ArrayList<Hash>(block.first.transactions.size)
            if (!processBlockUnlocked(it, block.first, block.second, txHashes)) {
                logger.error("process block failed")
                return@withLock toRemove
            }
            TxPool.remove(txHashes)
        }

        return@withLock toRemove
    }

    suspend fun prune() = mutex.withLock {
        var height = height() - PoS.MATURITY
        while (height > 0) {
            val hash = chain[height]!!
            if (!undo.containsKey(hash))
                break
            removeUndo(hash)
            height--
        }
    }
}
