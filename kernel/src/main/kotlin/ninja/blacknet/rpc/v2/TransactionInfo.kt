/*
 * Copyright (c) 2019-2024 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.rpc.v2

import kotlinx.serialization.KSerializer
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import kotlinx.serialization.json.JsonElement
import kotlinx.serialization.json.JsonObject
import ninja.blacknet.core.Transaction
import ninja.blacknet.crypto.Hash
import ninja.blacknet.crypto.PublicKey
import ninja.blacknet.crypto.SignatureSerializer
import ninja.blacknet.db.WalletDB
import ninja.blacknet.serialization.bbf.binaryFormat
import ninja.blacknet.serialization.json.json
import ninja.blacknet.serialization.LongSerializer
import ninja.blacknet.transaction.Batch
import ninja.blacknet.transaction.TxData
import ninja.blacknet.transaction.TxType

@Serializable
class TransactionInfo(
        val hash: Hash,
        val size: Int,
        @Serializable(with = SignatureSerializer::class)
        val signature: ByteArray,
        val from: PublicKey,
        val seq: Int,
        @SerialName("referenceChain")
        val anchor: Hash,
        @Serializable(with = LongSerializer::class)
        val fee: Long,
        val data: List<DataInfo>
) {
    constructor(tx: Transaction, hash: Hash, size: Int, filter: List<WalletDB.TransactionDataType>? = null) : this(
            hash,
            size,
            tx.signature,
            tx.from,
            tx.seq,
            tx.anchor,
            tx.fee,
            data(tx.type, tx.data, filter)
    )

    @Serializable
    class DataInfo(
            val type: UByte,
            val dataIndex: Int,
            val data: JsonElement
    )

    companion object {
        fun data(type: UByte, bytes: ByteArray, filter: List<WalletDB.TransactionDataType>?): List<DataInfo> {
            val data = if (type == TxType.Generated.type) {
                listOf(DataInfo(type, 0, JsonObject(emptyMap())))
            } else if (type != TxType.Batch.type) {
                val serializer = TxType.getSerializer<TxData>(type)
                val data = binaryFormat.decodeFromByteArray(serializer, bytes)
                listOf(DataInfo(type, 0, json.encodeToJsonElement(serializer, data)))
            } else {
                val multiData = binaryFormat.decodeFromByteArray(Batch.serializer(), bytes)
                val list = ArrayList<DataInfo>(multiData.multiData.size)
                if (filter == null) {
                    for (index in 0 until multiData.multiData.size) {
                        val (dataType, dataBytes) = multiData.multiData[index]
                        val serializer = TxType.getSerializer<TxData>(dataType)
                        val data = binaryFormat.decodeFromByteArray(serializer, dataBytes)
                        list.add(DataInfo(dataType, index + 1, json.encodeToJsonElement(serializer, data)))
                    }
                } else {
                    for (i in 0 until filter.size) {
                        val dataIndex = filter[i].dataIndex.toInt()
                        val (dataType, dataBytes) = multiData.multiData[dataIndex - 1]
                        val serializer = TxType.getSerializer<TxData>(dataType)
                        val data = binaryFormat.decodeFromByteArray(serializer, dataBytes)
                        list.add(DataInfo(dataType, dataIndex, json.encodeToJsonElement(serializer, data)))
                    }
                }
                list
            }
            return data
        }
    }
}
