/*
 * Copyright (c) 2018-2024 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.network.packet

import kotlinx.serialization.Serializable
import ninja.blacknet.mode
import ninja.blacknet.network.Connection
import ninja.blacknet.network.Node
import ninja.blacknet.time.currentTimeSeconds

@Serializable
class Ping(
    private val challenge: Int,
    private val time: Long,
) : Packet {
    override fun handle(connection: Connection) {
        connection.timeOffset = time - currentTimeSeconds()

        connection.sendPacket(PacketType.Pong, Pong(solve(challenge)))

        val lastPacketTime = connection.lastPacketTime
        val lastPingTime = connection.lastPingTime
        connection.lastPingTime = lastPacketTime
        if (lastPacketTime > lastPingTime + Node.NETWORK_TIMEOUT / 2)
            Unit
        else
            connection.dos("Too many ping requests")
    }

    companion object {
        const val MIN_VERSION = 14
    }
}

fun solve(challenge: Int): Int {
    return challenge xor mode.networkMagic
}
