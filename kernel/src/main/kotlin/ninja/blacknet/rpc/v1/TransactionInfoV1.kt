/*
 * Copyright (c) 2019-2024 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

@file:Suppress("DEPRECATION")

package ninja.blacknet.rpc.v1

import kotlinx.serialization.Serializable
import ninja.blacknet.core.Transaction
import ninja.blacknet.crypto.Address
import ninja.blacknet.crypto.Hash
import ninja.blacknet.crypto.SignatureSerializer
import ninja.blacknet.serialization.bbf.binaryFormat
import ninja.blacknet.transaction.*

@Serializable
class TransactionInfoV1(
        val hash: String,
        val size: Int,
        val signature: String,
        val from: String,
        val seq: Int,
        val blockHash: String,
        val fee: Long,
        val type: UByte,
        val data: String
) {
    constructor(tx: Transaction, hash: Hash, size: Int) : this(
            hash.toString(),
            size,
            SignatureSerializer.encode(tx.signature),
            Address.encode(tx.from.bytes),
            tx.seq,
            tx.anchor.toString(),
            tx.fee,
            tx.type,
            data(tx.type, tx.data)
    )

    companion object {
        fun fromBytes(bytes: ByteArray): TransactionInfoV1 {
            val hash = Transaction.hash(bytes)
            return TransactionInfoV1(binaryFormat.decodeFromByteArray(Transaction.serializer(), bytes), hash, bytes.size)
        }

        private fun data(type: UByte, bytes: ByteArray): String {
            val serializer = TxType.getSerializer<TxData>(type)
            val txData = binaryFormat.decodeFromByteArray(serializer, bytes)
            return when (type) {
                TxType.Transfer.type -> Json.stringify(TransferInfo.serializer(), TransferInfo(txData as Transfer))
                TxType.Burn.type -> Json.stringify(Burn.serializer(), txData as Burn)
                TxType.Lease.type -> Json.stringify(Lease.serializer(), txData as Lease)
                TxType.CancelLease.type -> Json.stringify(CancelLease.serializer(), txData as CancelLease)
                TxType.BApp.type -> Json.stringify(BApp.serializer(), txData as BApp)
                else -> "Unknown type:$type data:$bytes"
            }
        }
    }
}
