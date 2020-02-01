/*
 * Copyright (c) 2018-2019 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.packet

import kotlinx.io.core.BytePacketBuilder
import kotlinx.io.core.ByteReadPacket
import ninja.blacknet.network.Connection
import ninja.blacknet.serialization.BinaryDecoder

const val PACKET_HEADER_SIZE_BYTES = 4

interface Packet {
    fun serialize(): ByteReadPacket
    fun getType(): PacketType
    suspend fun process(connection: Connection)

    fun build(): ByteReadPacket {
        val s = serialize()
        val b = BytePacketBuilder()
        b.writeInt(s.remaining.toInt() + 4)
        b.writeInt(getType().ordinal)
        b.writePacket(s)
        return b.build()
    }

    companion object {
        fun deserialize(type: Int, bytes: ByteReadPacket): Packet {
            val serializer = PacketType.getSerializer(type)
            return BinaryDecoder(bytes).decode(serializer)
        }
    }
}
