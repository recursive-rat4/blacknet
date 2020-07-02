/*
 * Copyright (c) 2018-2019 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.packet

import io.ktor.utils.io.core.ByteReadPacket
import kotlinx.serialization.Serializable
import ninja.blacknet.core.TxPool
import ninja.blacknet.crypto.Hash
import ninja.blacknet.network.Connection
import ninja.blacknet.network.Node
import ninja.blacknet.serialization.BinaryEncoder

@Serializable
class GetTransactions(
        private val list: ArrayList<Hash>
) : Packet {
    override fun serialize(): ByteReadPacket = BinaryEncoder.toPacket(serializer(), this)

    override fun getType() = PacketType.GetTransactions

    override suspend fun process(connection: Connection) {
        if (list.size > Transactions.MAX) {
            connection.dos("Invalid GetTransactions size ${list.size}")
            return
        }

        var size = PACKET_HEADER_SIZE_BYTES + 2
        val maxSize = Node.getMinPacketSize() // we don't know actual value, so assume minimum
        val response = ArrayList<ByteArray>(list.size)

        for (hash in list) {
            val value = TxPool.get(hash) ?: continue
            val newSize = size + value.size + 4

            if (response.isEmpty()) {
                response.add(value)
                size = newSize
                if (size > maxSize) {
                    connection.sendPacket(Transactions(response))
                    response.clear()
                    size = PACKET_HEADER_SIZE_BYTES + 2
                }
            } else {
                if (newSize > maxSize) {
                    connection.sendPacket(Transactions(response))
                    response.clear()
                    size = PACKET_HEADER_SIZE_BYTES + 2
                }
                response.add(value)
                size += value.size + 4
            }
        }

        if (response.size != 0)
            connection.sendPacket(Transactions(response))
    }
}
