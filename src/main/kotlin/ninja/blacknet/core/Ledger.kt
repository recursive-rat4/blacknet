/*
 * Copyright (c) 2018-2019 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.core

import ninja.blacknet.crypto.Hash
import ninja.blacknet.crypto.PublicKey
import ninja.blacknet.db.LedgerDB.forkV2
import ninja.blacknet.transaction.TxType

interface Ledger {
    fun addSupply(amount: Long)
    fun checkReferenceChain(hash: Hash): Boolean
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

    suspend fun processTransactionImpl(tx: Transaction, hash: Hash, size: Int): Status {
        if (!tx.verifySignature(hash)) {
            return Invalid("Invalid signature")
        }
        if (!checkReferenceChain(tx.referenceChain)) {
            return NotOnThisChain
        }
        if (!checkFee(size, tx.fee)) {
            return Invalid("Too low fee ${tx.fee}")
        }
        if (tx.type == TxType.MultiData.type) {
            if (!forkV2()) {
                return Invalid("MultiData before forkV2")
            }
        }
        if (tx.type == TxType.WithdrawFromLease.type) {
            if (!forkV2()) {
                return Invalid("WithdrawFromLease before forkV2")
            }
        }
        if (tx.type == TxType.ClaimHTLC.type) {
            if (!forkV2()) {
                return Invalid("ClaimHTLC before forkV2")
            }
        }
        if (tx.type == TxType.UnlockHTLC.type) {
            if (forkV2()) {
                return Invalid("UnlockHTLC after forkV2")
            }
        }
        if (tx.type == TxType.SpendHTLC.type) {
            if (forkV2()) {
                return Invalid("SpendHTLC after forkV2")
            }
        }
        if (tx.type == TxType.Generated.type) {
            return Invalid("Generated as individual tx")
        }
        val data = tx.data()
        return data.process(tx, hash, this)
    }
}
