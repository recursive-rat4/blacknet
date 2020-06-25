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
import kotlinx.serialization.json.JsonArray
import kotlinx.serialization.json.JsonElement
import kotlinx.serialization.json.JsonObject
import ninja.blacknet.core.Transaction
import ninja.blacknet.crypto.Address
import ninja.blacknet.crypto.Hash
import ninja.blacknet.db.WalletDB
import ninja.blacknet.serialization.Json
import ninja.blacknet.transaction.MultiData
import ninja.blacknet.transaction.TxData
import ninja.blacknet.transaction.TxType

@Serializable
class TransactionInfo(
        val hash: String,
        val size: Int,
        val signature: String,
        val from: String,
        val seq: Int,
        val referenceChain: String,
        val fee: String,
        val data: List<DataInfo>
) {
    constructor(tx: Transaction, hash: Hash, size: Int, filter: List<WalletDB.TransactionDataType>? = null) : this(
            hash.toString(),
            size,
            tx.signature.toString(),
            Address.encode(tx.from),
            tx.seq,
            tx.referenceChain.toString(),
            tx.fee.toString(),
            data(tx.type, tx.data.array, filter)
    )

    @Serializable
    class DataInfo(
            val type: Int,
            val dataIndex: Int,
            val data: JsonElement
    )

    companion object {
        fun data(type: Byte, bytes: ByteArray, filter: List<WalletDB.TransactionDataType>?): List<DataInfo> {
            val data = if (type == TxType.Generated.type) {
                listOf(DataInfo(type.toUByte().toInt(), 0, JsonObject(emptyMap())))
            } else if (type != TxType.MultiData.type) {
                listOf(DataInfo(type.toUByte().toInt(), 0, TxData.deserialize(type, bytes).toJson()))
            } else {
                val multiData = MultiData.deserialize(bytes)
                val list = ArrayList<DataInfo>(multiData.multiData.size)
                if (filter == null) {
                    for (index in 0 until multiData.multiData.size) {
                        val (dataType, dataBytes) = multiData.multiData[index]
                        list.add(DataInfo(dataType.toUByte().toInt(), index + 1, TxData.deserialize(dataType, dataBytes.array).toJson()))
                    }
                } else {
                    for (i in 0 until filter.size) {
                        val dataIndex = filter[i].dataIndex.toInt()
                        val (dataType, dataBytes) = multiData.multiData[dataIndex - 1]
                        list.add(DataInfo(dataType.toUByte().toInt(), dataIndex, TxData.deserialize(dataType, dataBytes.array).toJson()))
                    }
                }
                list
            }
            return data
        }
    }
}
