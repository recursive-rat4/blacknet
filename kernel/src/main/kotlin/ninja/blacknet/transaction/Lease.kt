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
import ninja.blacknet.crypto.PoS
import ninja.blacknet.crypto.PublicKey
import ninja.blacknet.serialization.LongSerializer

/**
 * 創建合約
 */
@Serializable
class Lease(
        @Serializable(with = LongSerializer::class)
        val amount: Long,
        val to: PublicKey
) : TxData {
    override fun processCoinImpl(tx: Transaction, hash: Hash, dataIndex: Int, coinTx: CoinTx): Status {
        if (amount < PoS.MIN_LEASE) {
            return Invalid("$amount less than minimal ${PoS.MIN_LEASE}")
        }
        val account = coinTx.getAccount(tx.from)!!
        val status = account.credit(amount)
        if (status != Accepted) {
            return status
        }
        coinTx.setAccount(tx.from, account)
        val toAccount = coinTx.getOrCreate(to)
        toAccount.leases.add(AccountState.Lease(tx.from, coinTx.height(), amount))
        coinTx.setAccount(to, toAccount)
        return Accepted
    }

    fun involves(publicKey: PublicKey) = to == publicKey
}
