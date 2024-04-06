/*
 * Copyright (c) 2018-2024 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.transaction

import kotlinx.serialization.Serializable
import ninja.blacknet.core.*
import ninja.blacknet.crypto.Hash
import ninja.blacknet.crypto.PublicKey
import ninja.blacknet.serialization.LongSerializer

/**
 * 取消合約
 */
@Serializable
class CancelLease(
        @Serializable(with = LongSerializer::class)
        val amount: Long,
        val to: PublicKey,
        val height: Int
) : TxData {
    override fun processCoinImpl(tx: Transaction, hash: Hash, dataIndex: Int, coinTx: CoinTx): Status {
        val toAccount = coinTx.getAccount(to)
        if (toAccount == null) {
            return Invalid("Account not found")
        }
        if (toAccount.leases.remove(AccountState.Lease(tx.from, height, amount))) {
            coinTx.setAccount(to, toAccount)
            val account = coinTx.getAccount(tx.from)!!
            account.debit(coinTx.height(), amount)
            coinTx.setAccount(tx.from, account)
            return Accepted
        }
        return Invalid("Lease not found")
    }

    fun involves(publicKey: PublicKey) = to == publicKey
}
