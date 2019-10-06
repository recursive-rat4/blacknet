/*
 * Copyright (c) 2018 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.core

import mu.KotlinLogging
import ninja.blacknet.crypto.Hash
import ninja.blacknet.crypto.PublicKey
import ninja.blacknet.transaction.TxType

private val logger = KotlinLogging.logger {}

interface Ledger {
    fun addSupply(amount: Long)
    fun checkBlockHash(hash: Hash): Boolean
    fun checkFee(size: Int, amount: Long): Boolean
    fun blockTime(): Long
    fun height(): Int
    fun get(key: PublicKey): AccountState?
    fun getOrCreate(key: PublicKey): AccountState
    fun set(key: PublicKey, state: AccountState)
    fun addHTLC(id: Hash, htlc: HTLC)
    fun getHTLC(id: Hash): HTLC?
    fun removeHTLC(id: Hash)
    fun addMultisig(id: Hash, multisig: Multisig)
    fun getMultisig(id: Hash): Multisig?
    fun removeMultisig(id: Hash)

    suspend fun processTransactionImpl(tx: Transaction, hash: Hash, size: Int): DataDB.Status {
        if (!tx.verifySignature(hash)) {
            logger.info("invalid signature")
            return DataDB.Status.INVALID
        }
        if (!checkBlockHash(tx.blockHash)) {
            logger.info("not valid on this chain")
            return DataDB.Status.NOT_ON_THIS_CHAIN
        }
        if (!checkFee(size, tx.fee)) {
            logger.info("too low fee ${tx.fee}")
            return DataDB.Status.INVALID
        }
        if (tx.type == TxType.Generated.type) {
            logger.info("Generated as individual tx")
            return DataDB.Status.INVALID
        }
        val data = tx.data()
        return data.process(tx, hash, this)
    }
}
