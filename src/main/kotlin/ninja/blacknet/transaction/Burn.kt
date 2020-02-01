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
import ninja.blacknet.core.*
import ninja.blacknet.crypto.Hash
import ninja.blacknet.serialization.BinaryDecoder
import ninja.blacknet.serialization.BinaryEncoder
import ninja.blacknet.serialization.Json
import ninja.blacknet.serialization.SerializableByteArray

@Serializable
class Burn(
        val amount: Long,
        val message: SerializableByteArray
) : TxData {
    override fun getType() = TxType.Burn
    override fun serialize() = BinaryEncoder.toBytes(serializer(), this)
    override fun toJson() = Json.toJson(Info.serializer(), Info(this))

    override fun processImpl(tx: Transaction, hash: Hash, dataIndex: Int, ledger: Ledger): Status {
        if (amount == 0L) {
            return Invalid("Invalid amount")
        }
        val account = ledger.get(tx.from)!!
        val status = account.credit(amount)
        if (status != Accepted) {
            return status
        }
        ledger.set(tx.from, account)
        ledger.addSupply(-amount)
        return Accepted
    }

    companion object {
        fun deserialize(bytes: ByteArray): Burn = BinaryDecoder.fromBytes(bytes).decode(serializer())
    }

    @Suppress("unused")
    @Serializable
    class Info(
            val amount: String,
            val message: String
    ) {
        constructor(data: Burn) : this(
                data.amount.toString(),
                data.message.toString()
        )
    }
}
