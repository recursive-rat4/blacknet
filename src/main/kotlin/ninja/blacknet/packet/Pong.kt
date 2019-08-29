/*
 * Copyright (c) 2018 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.packet

import kotlinx.io.core.ByteReadPacket
import kotlinx.serialization.Serializable
import ninja.blacknet.network.Connection
import ninja.blacknet.network.Runtime
import ninja.blacknet.serialization.BinaryEncoder

@Serializable
internal class Pong(private val id: Int) : Packet {
    override fun serialize(): ByteReadPacket = BinaryEncoder.toPacket(serializer(), this)

    override fun getType() = PacketType.Pong

    override suspend fun process(connection: Connection) {
        val (pingId, pingTime) = connection.pingRequest ?: return connection.dos("unexpected Pong")

        if (pingId != id) {
            connection.dos("invalid Pong id")
            return
        }

        connection.ping = Runtime.timeMilli() - pingTime
        connection.pingRequest = null
    }
}
