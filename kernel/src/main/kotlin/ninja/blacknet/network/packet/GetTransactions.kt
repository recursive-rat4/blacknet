/*
 * Copyright (c) 2018-2024 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.network.packet

import kotlinx.coroutines.runBlocking
import kotlinx.serialization.Serializable
import ninja.blacknet.Kernel
import ninja.blacknet.crypto.Hash
import ninja.blacknet.network.Connection
import ninja.blacknet.network.Node

@Serializable
class GetTransactions(
    private val list: List<Hash>
) : Packet {
    override fun handle(connection: Connection) {
        if (list.size > Transactions.MAX) {
            connection.dos("Invalid GetTransactions size ${list.size}")
            return
        }

        var size = PACKET_HEADER_SIZE_BYTES + 2
        val maxSize = Node.getMinPacketSize() // we don't know actual value, so assume minimum
        val response = ArrayList<ByteArray>(list.size)

        for (hash in list) {
            val value = runBlocking { Kernel.txPool().get(hash) } ?: continue
            val newSize = size + value.size + 4

            if (response.isEmpty()) {
                response.add(value)
                size = newSize
                if (size > maxSize) {
                    connection.sendPacket(PacketType.Transactions, Transactions(response))
                    response.clear()
                    size = PACKET_HEADER_SIZE_BYTES + 2
                }
            } else {
                if (newSize > maxSize) {
                    connection.sendPacket(PacketType.Transactions, Transactions(response))
                    response.clear()
                    size = PACKET_HEADER_SIZE_BYTES + 2
                }
                response.add(value)
                size += value.size + 4
            }
        }

        if (response.size != 0)
            connection.sendPacket(PacketType.Transactions, Transactions(response))
    }
}
