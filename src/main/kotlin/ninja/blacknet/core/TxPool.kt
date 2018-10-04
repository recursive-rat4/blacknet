/*
 * Copyright (c) 2018 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.core

import ninja.blacknet.crypto.Hash
import ninja.blacknet.crypto.PublicKey
import ninja.blacknet.db.LedgerDB
import ninja.blacknet.network.Node

object TxPool : MemPool(), Ledger {
    private val accounts = HashMap<PublicKey, AccountState>()

    fun getSequence(key: PublicKey): Int {
        val account = accounts[key]
        if (account != null)
            return account.seq
        return LedgerDB.get(key)?.seq ?: 0
    }

    override fun checkFee(size: Int, amount: Long): Boolean {
        //TODO
        return amount >= Node.MIN_FEE
    }

    override fun checkSequence(key: PublicKey, seq: Int): Boolean {
        val account = accounts[key]
        if (account != null)
            return account.seq == seq
        return LedgerDB.checkSequence(key, seq)
    }

    override fun get(key: PublicKey): AccountState? {
        val account = accounts[key]
        if (account != null)
            return account
        return LedgerDB.get(key)
    }

    override fun set(key: PublicKey, state: AccountState) {
        accounts[key] = state
    }

    override suspend fun processImpl(hash: Hash, bytes: ByteArray): Boolean {
        if (processTransaction(hash, bytes)) {
            add(hash, bytes)
            return true
        }
        return false
    }

    override fun addSupply(amount: Long) {}
    override fun height() = -1
}