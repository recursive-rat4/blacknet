/*
 * Copyright (c) 2018-2024 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.transaction

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import ninja.blacknet.core.*
import ninja.blacknet.crypto.Hash
import ninja.blacknet.crypto.PaymentId
import ninja.blacknet.crypto.PublicKey
import ninja.blacknet.serialization.LongSerializer

@Serializable
class Transfer(
        @Serializable(with = LongSerializer::class)
        val amount: Long,
        val to: PublicKey,
        @SerialName("message")
        val paymentId: PaymentId
) : TxData {
    override fun processCoinImpl(tx: Transaction, hash: Hash, dataIndex: Int, coinTx: CoinTx): Status {
        val account = coinTx.getAccount(tx.from)!!
        val status = account.credit(amount)
        if (status != Accepted) {
            return status
        }
        coinTx.setAccount(tx.from, account)
        val toAccount = coinTx.getOrCreate(to)
        toAccount.debit(coinTx.height(), amount)
        coinTx.setAccount(to, toAccount)
        return Accepted
    }

    fun involves(publicKey: PublicKey) = to == publicKey
}
