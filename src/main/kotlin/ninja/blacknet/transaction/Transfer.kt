/*
 * Copyright (c) 2018-2019 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.transaction

import kotlinx.serialization.Serializable
import kotlinx.serialization.json.JsonElement
import ninja.blacknet.core.*
import ninja.blacknet.crypto.Address
import ninja.blacknet.crypto.Hash
import ninja.blacknet.crypto.Message
import ninja.blacknet.crypto.PublicKey
import ninja.blacknet.serialization.BinaryDecoder
import ninja.blacknet.serialization.BinaryEncoder
import ninja.blacknet.serialization.Json

@Serializable
class Transfer(
        val amount: Long,
        val to: PublicKey,
        val message: Message
) : TxData {
    override fun getType() = TxType.Transfer
    override fun serialize() = BinaryEncoder.toBytes(serializer(), this)
    override fun toJson() = Json.toJson(Info.serializer(), Info(this))

    override fun processImpl(tx: Transaction, hash: Hash, dataIndex: Int, ledger: Ledger): Status {
        val account = ledger.get(tx.from)!!
        val status = account.credit(amount)
        if (status != Accepted) {
            return status
        }
        ledger.set(tx.from, account)
        val toAccount = ledger.getOrCreate(to)
        toAccount.debit(ledger.height(), amount)
        ledger.set(to, toAccount)
        return Accepted
    }

    fun involves(publicKey: PublicKey) = to == publicKey

    companion object {
        fun deserialize(bytes: ByteArray): Transfer = BinaryDecoder(bytes).decode(serializer())
    }

    @Suppress("unused")
    @Serializable
    class Info(
            val amount: String,
            val to: String,
            val message: JsonElement
    ) {
        constructor(data: Transfer) : this(
                data.amount.toString(),
                Address.encode(data.to),
                data.message.toJson()
        )
    }
}
