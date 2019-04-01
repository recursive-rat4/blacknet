/*
 * Copyright (c) 2019 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.api

import kotlinx.serialization.Serializable
import kotlinx.serialization.json.JsonElement
import ninja.blacknet.core.Transaction
import ninja.blacknet.crypto.Address
import ninja.blacknet.crypto.Hash
import ninja.blacknet.transaction.*

@Serializable
class TransactionInfo(
        val hash: String,
        val size: Int,
        val signature: String,
        val from: String,
        val seq: Int,
        val blockHash: String,
        val fee: Long,
        val type: Byte,
        val data: JsonElement
) {
    constructor(tx: Transaction, hash: Hash, size: Int, data: JsonElement) : this(
            hash.toString(),
            size,
            tx.signature.toString(),
            Address.encode(tx.from),
            tx.seq,
            tx.blockHash.toString(),
            tx.fee,
            tx.type,
            data
    )

    companion object {
        fun get(tx: Transaction, hash: Hash, size: Int): TransactionInfo {
            return TransactionInfo(tx, hash, size, data(tx.type, tx.data.array))
        }

        private fun data(type: Byte, bytes: ByteArray): JsonElement {
            val txData = TxData.deserialize(type, bytes) ?: throw RuntimeException("Deserialization error")
            return when (type) {
                TxType.Transfer.type -> APIServer.json.toJson(Transfer.serializer(), txData as Transfer)
                TxType.Burn.type -> APIServer.json.toJson(Burn.serializer(), txData as Burn)
                TxType.Lease.type -> APIServer.json.toJson(Lease.serializer(), txData as Lease)
                TxType.CancelLease.type -> APIServer.json.toJson(CancelLease.serializer(), txData as CancelLease)
                TxType.Bundle.type -> APIServer.json.toJson(Bundle.serializer(), txData as Bundle)
                TxType.CreateHTLC.type -> APIServer.json.toJson(CreateHTLC.serializer(), txData as CreateHTLC)
                TxType.UnlockHTLC.type -> APIServer.json.toJson(UnlockHTLC.serializer(), txData as UnlockHTLC)
                TxType.RefundHTLC.type -> APIServer.json.toJson(RefundHTLC.serializer(), txData as RefundHTLC)
                TxType.SpendHTLC.type -> APIServer.json.toJson(SpendHTLC.serializer(), txData as SpendHTLC)
                TxType.CreateMultisig.type -> APIServer.json.toJson(CreateMultisig.serializer(), txData as CreateMultisig)
                TxType.SpendMultisig.type -> APIServer.json.toJson(SpendMultisig.serializer(), txData as SpendMultisig)
                else -> throw RuntimeException("Unknown tx type:$type")
            }
        }
    }
}
