/*
 * Copyright (c) 2018 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.db

import com.google.common.io.Resources
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.Serializable
import kotlinx.serialization.json.JSON
import kotlinx.serialization.list
import mu.KotlinLogging
import ninja.blacknet.core.*
import ninja.blacknet.crypto.BigInt
import ninja.blacknet.crypto.Hash
import ninja.blacknet.crypto.PublicKey
import org.mapdb.DBMaker
import org.mapdb.Serializer
import kotlin.math.max

private val logger = KotlinLogging.logger {}

object LedgerDB : Ledger {
    private val db = DBMaker.fileDB("ledger.db").transactionEnable().fileMmapEnable().closeOnJvmShutdown().make()
    private val accounts = db.hashMap("accounts", PublicKeySerializer, AccountStateSerializer).createOrOpen()
    private val height = db.atomicInteger("height").createOrOpen()
    private val blockHash = db.atomicVar("blockHash", HashSerializer, Hash.ZERO).createOrOpen()
    private val blockTime = db.atomicLong("blockTime").createOrOpen()
    private val cumulativeDifficulty = db.atomicVar("cumulativeDifficulty", BigIntSerializer, BigInt.ZERO).createOrOpen()
    private val supply = db.atomicLong("supply").createOrOpen()
    private val undo = db.hashMap("undo", HashSerializer, UndoSerializer).createOrOpen()
    private val blockSizes = db.indexTreeList("blockSizes", Serializer.INTEGER).createOrOpen()
    private val nxtrng = db.indexTreeList("nxtrng", HashSerializer).createOrOpen()

    private var maxBlockSize: Int

    init {
        maxBlockSize = calcMaxBlockSize()

        @Serializable
        class Entry(val publicKey: String, val balance: Long)

        if (accounts.isEmpty()) {
            val url = Resources.getResource("genesis.json")
            val genesis = Resources.toString(url, Charsets.UTF_8)
            val list = JSON.parse(Entry.serializer().list, genesis)

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
            nxtrng.add(Hash.ZERO)
            commit()
            logger.info("loaded genesis.json ${accounts()} accounts, supply = ${supply()}")
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

    fun blockTime(): Long {
        return blockTime.get()
    }

    fun cumulativeDifficulty(): BigInt {
        return cumulativeDifficulty.get()
    }

    fun supply(): Long {
        return supply.get()
    }

    fun accounts(): Int {
        return accounts.size
    }

    override suspend fun get(key: PublicKey): AccountState? {
        return accounts[key]
    }

    override suspend fun set(key: PublicKey, state: AccountState) {
        accounts[key] = state
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

    override fun checkFee(size: Int, amount: Long) = amount >= 0

    override suspend fun checkSequence(key: PublicKey, seq: Int): Boolean {
        val account = get(key) ?: return false
        return account.seq == seq
    }

    fun maxBlockSize(): Int {
        return maxBlockSize
    }

    fun nxtrng(): Hash {
        return nxtrng.last()!!
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

    suspend fun processBlock(hash: Hash, block: Block, size: Int): Boolean {
        if (block.previous != blockHash()) {
            logger.error("not on current chain")
            return false
        }
        if (block.time <= blockTime()) {
            logger.info("timestamp is too early")
            return false
        }

        val undo = UndoBlock(blockTime(), supply(), UndoList())

        height.set(height.get() + 1)
        blockHash.set(hash)
        blockTime.set(block.time)
        blockSizes.add(size)
        nxtrng.add(PoS.nxtrng(nxtrng(), block.generator))
        //TODO cumulative difficulty

        var fees = 0L
        for (bytes in block.transactions) {
            val tx = Transaction.deserialize(bytes.array)
            if (tx == null) {
                logger.info("deserialization failed")
                return false
            }
            if (!processTransaction(tx, DataType.TxHash(bytes.array), bytes.array.size, undo.accounts)) {
                logger.info("invalid transaction")
                return false
            }
            fees += tx.fee
        }

        val generator = get(block.generator)
        if (generator == null) {
            logger.error("block generator not found")
            return false
        }
        val reward = PoS.reward(supply())
        undo.accounts.add(Pair(block.generator, generator.copy()))
        this.undo[hash] = undo
        addSupply(reward)
        generator.debit(height(), reward + fees)
        set(block.generator, generator)
        maxBlockSize = calcMaxBlockSize()

        return true
    }

    suspend fun undoBlock() {
        val hash = blockHash()
        val undo = this.undo[hash]!!

        val height = height.get()
        this.height.set(height - 1)
        //TODO blockHash.set()
        blockTime.set(undo.blockTime)
        blockSizes.removeAt(height)
        nxtrng.removeAt(height)

        setSupply(undo.supply)
        for (i in undo.accounts.reversed()) {
            val key = i.first
            val state = i.second
            if (state.isEmpty())
                accounts.remove(key)
            else
                accounts[key] = state
        }

        this.undo.remove(hash)
    }
}
