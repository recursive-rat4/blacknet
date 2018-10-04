/*
 * Copyright (c) 2018 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.db

import mu.KotlinLogging
import ninja.blacknet.core.*
import ninja.blacknet.crypto.Hash
import ninja.blacknet.crypto.Mnemonic
import ninja.blacknet.crypto.PublicKey
import org.mapdb.DBMaker

private val logger = KotlinLogging.logger {}

object LedgerDB : Ledger {
    private val db = DBMaker.fileDB("ledger.db").transactionEnable().fileMmapEnable().closeOnJvmShutdown().make()
    private val accounts = db.hashMap("accounts", PublicKeySerializer, AccountStateSerializer).createOrOpen()
    private val height = db.atomicInteger("height").createOrOpen()
    private val blockHash = db.atomicVar("blockHash", HashSerializer, Hash.ZERO).createOrOpen()
    private val supply = db.atomicLong("supply").createOrOpen()

    init {
        if (accounts.isEmpty()) {
            val supply = 1000000000L * PoS.COIN
            val mnemonic = "unit visual denial donor twice sure trim blast sniff topple december pill"
            // publicKey a11188d9e5087156c3e7f2deeccfb9879a46d8fc49579c8fc3205319a4e31f0f
            val publicKey = Mnemonic.fromString(mnemonic)!!.toPublicKey()
            val account = AccountState.create(supply)
            set(publicKey, account)
            addSupply(supply)
            commit()
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

    suspend fun processBlock(hash: Hash, block: Block): Boolean {
        if (block.previous != blockHash()) {
            logger.error("not on current chain")
            return false
        }

        height.set(height.get() + 1)
        blockHash.set(hash)

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

        return true
    }
}