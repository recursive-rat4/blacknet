/*
 * Copyright (c) 2018-2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.core

import ninja.blacknet.crypto.HashSerializer

interface Ledger {
    fun addSupply(amount: Long)
    fun checkReferenceChain(hash: ByteArray): Boolean
    fun blockTime(): Long
    fun height(): Int
    fun getAccount(key: ByteArray): AccountState?
    fun getOrCreate(key: ByteArray): AccountState
    fun setAccount(key: ByteArray, state: AccountState)
    fun addHTLC(id: ByteArray, htlc: HTLC)
    fun getHTLC(id: ByteArray): HTLC?
    fun removeHTLC(id: ByteArray)
    fun addMultisig(id: ByteArray, multisig: Multisig)
    fun getMultisig(id: ByteArray): Multisig?
    fun removeMultisig(id: ByteArray)

    fun processTransactionImpl(tx: Transaction, hash: ByteArray): Status {
        if (!tx.verifySignature(hash)) {
            return Invalid("Invalid signature")
        }
        if (!checkReferenceChain(tx.referenceChain)) {
            return NotOnThisChain(HashSerializer.encode(tx.referenceChain))
        }
        if (tx.fee < 0) {
            return Invalid("Negative fee")
        }
        val data = tx.data()
        return data.process(tx, hash, this)
    }
}
