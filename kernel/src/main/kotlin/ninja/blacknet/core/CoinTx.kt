/*
 * Copyright (c) 2018-2024 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.core

import ninja.blacknet.contract.HashTimeLockContractId
import ninja.blacknet.contract.MultiSignatureLockContractId
import ninja.blacknet.crypto.Hash
import ninja.blacknet.crypto.PublicKey

interface CoinTx {
    fun addSupply(amount: Long)
    fun checkAnchor(hash: Hash): Boolean
    fun blockHash(): Hash
    fun blockTime(): Long
    fun height(): Int
    fun getAccount(key: PublicKey): AccountState?
    fun getOrCreate(key: PublicKey): AccountState
    fun setAccount(key: PublicKey, state: AccountState)
    fun addHTLC(id: HashTimeLockContractId, htlc: HTLC)
    fun getHTLC(id: HashTimeLockContractId): HTLC?
    fun removeHTLC(id: HashTimeLockContractId)
    fun addMultisig(id: MultiSignatureLockContractId, multisig: Multisig)
    fun getMultisig(id: MultiSignatureLockContractId): Multisig?
    fun removeMultisig(id: MultiSignatureLockContractId)

    fun processTransactionImpl(tx: Transaction, hash: Hash): Status {
        if (!tx.verifySignature(hash)) {
            return Invalid("Invalid signature")
        }
        if (!checkAnchor(tx.anchor)) {
            return NotReachableVertex(tx.anchor.toString())
        }
        if (tx.fee < 0) {
            return Invalid("Negative fee")
        }
        val data = tx.data()
        return data.processCoin(tx, hash, this)
    }
}
