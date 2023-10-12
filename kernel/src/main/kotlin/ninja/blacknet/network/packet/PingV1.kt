/*
 * Copyright (c) 2018-2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.network.packet

import kotlinx.serialization.Serializable
import ninja.blacknet.crypto.Blake2b.buildHash
import ninja.blacknet.mode
import ninja.blacknet.network.Connection
import ninja.blacknet.network.Node
import ninja.blacknet.util.fromBytes

@Serializable
class PingV1(
    val challenge: Int
) : Packet {
    override suspend fun process(connection: Connection) {
        connection.sendPacket(PacketType.Pong, Pong(if (connection.version == 13) solveV1(challenge) else challenge))
        val lastPacketTime = connection.lastPacketTime
        val lastPingTime = connection.lastPingTime
        connection.lastPingTime = lastPacketTime
        if (lastPacketTime > lastPingTime + Node.NETWORK_TIMEOUT / 2)
            Unit
        else
            connection.dos("Too many ping requests")
    }
}

fun solveV1(challenge: Int): Int {
    val hash = buildHash {
        encodeInt(mode.networkMagic)
        encodeInt(challenge)
    }
    return Int.fromBytes(hash[0], hash[1], hash[2], hash[3])
}
