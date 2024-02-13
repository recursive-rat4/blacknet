/*
 * Copyright (c) 2018-2024 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.network.packet

import io.ktor.utils.io.core.BytePacketBuilder
import io.ktor.utils.io.core.ByteReadPacket
import io.ktor.utils.io.core.writeInt
import ninja.blacknet.network.Connection
import ninja.blacknet.serialization.bbf.binaryFormat

const val PACKET_HEADER_SIZE_BYTES = 4

interface Packet {
    fun handle(connection: Connection)
}

fun <T : Packet> buildPacket(type: PacketType, packet: T): ByteReadPacket {
    val serializer = PacketType.getSerializer<T>(type.ordinal)
    val payload = binaryFormat.encodeToPacket(serializer, packet)
    val builder = BytePacketBuilder()
    builder.writeInt(payload.remaining.toInt() + 4)
    builder.writeInt(type.ordinal)
    builder.writePacket(payload)
    return builder.build()
}
