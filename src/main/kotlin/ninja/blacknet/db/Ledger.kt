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
import ninja.blacknet.core.Block
import ninja.blacknet.core.DataType
import ninja.blacknet.core.Transaction
import ninja.blacknet.core.TxType
import ninja.blacknet.crypto.Hash
import ninja.blacknet.crypto.PublicKey
import ninja.blacknet.serialization.BlacknetInput
import org.mapdb.DBMaker

private val logger = KotlinLogging.logger {}

object Ledger {
    private val db = DBMaker.fileDB("ledger.db").transactionEnable().fileMmapEnable().closeOnJvmShutdown().make()
    private val accounts = db.hashMap("accounts", PublicKeySerializer, AccountStateSerializer).createOrOpen()
    private val height = db.atomicInteger("height").createOrOpen()
    private val blockHash = db.atomicVar("blockHash", HashSerializer, Hash.ZERO).createOrOpen()
    private val supply = db.atomicLong("supply").createOrOpen()

    fun commit() {
        db.commit()
    }

    fun rollback() {
        db.rollback()
    }

    fun height(): Int {
        return height.get()
    }

    fun blockHash(): Hash {
        return blockHash.get()
    }

    fun supply(): Long {
        return supply.get()
    }

    fun addSupply(amount: Long) {
        supply.set(supply.get() + amount)
    }

    fun accounts(): Int {
        return accounts.size
    }

    fun get(key: PublicKey): AccountState? {
        return accounts[key]
    }

    fun set(key: PublicKey, state: AccountState) {
        accounts[key] = state
    }

    fun checkSequence(key: PublicKey, seq: Int): Boolean {
        val account = get(key) ?: return false
        return account.seq == seq
    }

    fun processBlock(block: Block): Boolean {
        if (block.previous != blockHash()) {
            logger.error("not on current chain")
            return false
        }
        for (bytes in block.transactions) {
            val tx = Transaction.deserialize(bytes.array)
            if (tx == null) {
                logger.info("deserialization failed")
                return false
            }
            if (!checkSequence(tx.from, tx.seq)) {
                logger.info("invalid sequence number")
                return false
            }
            val hash = DataType.Transaction.hash(bytes.array)
            if (!tx.verifySignature(hash)) {
                logger.info("invalid signature")
                return false
            }
            if (tx.fee < 0) {
                logger.info("negative fee")
                return false
            }
            if (!processTransaction(tx)) {
                logger.info("invalid transaction")
                return false
            }
        }
        //TODO pos
        return true
    }

    private fun processTransaction(tx: Transaction): Boolean {
        val serializer = TxType.getSerializer(tx.type)
        if (serializer == null) {
            logger.info("unknown transaction type ${tx.type}")
            return false
        }
        val data = BlacknetInput.fromBytes(tx.data.array).deserialize(serializer)
        if (data == null) {
            logger.info("deserialization failed")
            return false
        }
        return data.process(tx)
    }
}