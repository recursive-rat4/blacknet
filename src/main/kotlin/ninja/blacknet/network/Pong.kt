/*
 * Copyright (c) 2018 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.network

import kotlinx.io.core.ByteReadPacket
import kotlinx.serialization.Serializable
import ninja.blacknet.serialization.BlacknetOutput

@Serializable
class Pong(private val id: Int) : Packet {
    override fun serialize(): ByteReadPacket {
        val out = BlacknetOutput()
        out.write(this)
        return out.build()
    }

    override fun getType(): Int {
        return PacketType.Pong.ordinal
    }

    override suspend fun process(connection: Connection) {
        val request = connection.pingRequest
        if (request == null) {
            connection.dos("unexpected Pong")
            return
        }
        if (request.id != id) {
            connection.dos("invalid Pong id")
            return
        }
        connection.ping = Node.timeMilli() - request.time
        connection.pingRequest = null
    }
}