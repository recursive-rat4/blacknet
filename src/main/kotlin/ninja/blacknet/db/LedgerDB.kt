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
import kotlinx.serialization.Serializable
import kotlinx.serialization.json.JSON
import kotlinx.serialization.list
import mu.KotlinLogging
import ninja.blacknet.core.*
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
    private val supply = db.atomicLong("supply").createOrOpen()
    private val blockSizes = db.indexTreeList("blockSizes", Serializer.INTEGER).createOrOpen()

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
                set(publicKey, account)
                supply += i.balance
            }

            addSupply(supply)
            blockSizes.add(0)
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

    fun supply(): Long {
        return supply.get()
    }

    fun accounts(): Int {
        return accounts.size
    }

    override fun get(key: PublicKey): AccountState? {
        return accounts[key]
    }

    override fun set(key: PublicKey, state: AccountState) {
        accounts[key] = state
    }

    override fun addSupply(amount: Long) {
        supply.set(supply.get() + amount)
    }

    override fun checkFee(size: Int, amount: Long) = amount >= 0

    override fun checkSequence(key: PublicKey, seq: Int): Boolean {
        val account = get(key) ?: return false
        return account.seq == seq
    }

    fun getMaxBlockSize(): Int {
        return maxBlockSize
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

        height.set(height.get() + 1)
        blockHash.set(hash)
        blockSizes.add(size)

        var fees = 0L
        for (bytes in block.transactions) {
            val tx = Transaction.deserialize(bytes.array)
            if (tx == null) {
                logger.info("deserialization failed")
                return false
            }
            if (!processTransaction(tx, hash, bytes.array.size)) {
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
        addSupply(reward)
        generator.debit(height(), reward + fees)
        set(block.generator, generator)
        maxBlockSize = calcMaxBlockSize()

        return true
    }
}