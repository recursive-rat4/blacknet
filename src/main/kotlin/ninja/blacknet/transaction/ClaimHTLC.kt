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
import ninja.blacknet.crypto.Address
import ninja.blacknet.crypto.Hash
import ninja.blacknet.serialization.BinaryDecoder
import ninja.blacknet.serialization.BinaryEncoder
import ninja.blacknet.serialization.Json
import ninja.blacknet.serialization.SerializableByteArray

@Serializable
class ClaimHTLC(
        val id: Hash,
        val preimage: SerializableByteArray
) : TxData {
    override fun getType() = TxType.ClaimHTLC
    override fun serialize() = BinaryEncoder.toBytes(serializer(), this)
    override fun toJson() = Json.toJson(Info.serializer(), Info(this))

    override fun processImpl(tx: Transaction, hash: Hash, dataIndex: Int, ledger: Ledger): Status {
        val htlc = ledger.getHTLC(id)
        if (htlc == null) {
            return Invalid("HTLC not found")
        }
        if (tx.from != htlc.to) {
            return Invalid("Invalid sender")
        }
        if (!htlc.verifyHashLock(preimage)) {
            return Invalid("Invalid hash lock")
        }

        val account = ledger.get(tx.from)!!
        account.debit(ledger.height(), htlc.lot)
        ledger.set(tx.from, account)
        ledger.removeHTLC(id)
        return Accepted
    }

    fun involves(ids: Set<Hash>) = ids.contains(id)

    companion object {
        fun deserialize(bytes: ByteArray): ClaimHTLC = BinaryDecoder(bytes).decode(serializer())
    }

    @Suppress("unused")
    @Serializable
    class Info(
            val id: String,
            val preimage: String
    ) {
        constructor(data: ClaimHTLC) : this(
                Address.encodeId(Address.HTLC, data.id),
                data.preimage.toString()
        )
    }
}
