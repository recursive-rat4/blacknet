/*
 * Copyright (c) 2018-2019 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.packet

import kotlinx.io.core.ByteReadPacket
import kotlinx.serialization.Serializable
import ninja.blacknet.core.DataType
import ninja.blacknet.core.TxPool
import ninja.blacknet.crypto.Hash
import ninja.blacknet.db.BlockDB
import ninja.blacknet.network.Connection
import ninja.blacknet.network.Node
import ninja.blacknet.serialization.BinaryEncoder
import ninja.blacknet.serialization.SerializableByteArray

@Serializable
internal class GetData(private val list: List<Pair<Byte, Hash>>) : Packet {
    override fun serialize(): ByteReadPacket = BinaryEncoder.toPacket(serializer(), this)

    override fun getType() = PacketType.GetData

    override suspend fun process(connection: Connection) {
        if (list.size > Transactions.MAX) {
            connection.dos("invalid GetData size")
            return
        }

        var size = PACKET_HEADER_SIZE + 2
        val maxSize = Node.getMinPacketSize() // we don't know actual value, so assume minimum
        val response = ArrayList<Pair<Byte, SerializableByteArray>>()

        loop@ for (i in list) {
            val type = i.first
            val hash = i.second

            val value = when (type) {
                DataType.Transaction -> TxPool.get(hash) ?: continue@loop
                DataType.Block -> BlockDB.get(hash) ?: continue@loop
                else -> continue@loop
            }
            val newSize = size + value.size + 4

            if (response.isEmpty()) {
                response.add(Pair(type, SerializableByteArray(value)))
                size = newSize
                if (size > maxSize) {
                    connection.sendPacket(Data(response))
                    response.clear()
                    size = PACKET_HEADER_SIZE + 2
                }
            } else {
                if (newSize > maxSize) {
                    connection.sendPacket(Data(response))
                    response.clear()
                    size = PACKET_HEADER_SIZE + 2
                }
                response.add(Pair(type, SerializableByteArray(value)))
                size += value.size + 4
            }
        }

        if (response.size == 0)
            return

        connection.sendPacket(Data(response))
    }
}
