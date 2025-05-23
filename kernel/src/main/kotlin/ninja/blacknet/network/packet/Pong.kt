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
import ninja.blacknet.network.Connection

@Serializable
class Pong(
    private val response: Int,
) : Packet {
    override fun handle(connection: Connection) {
        val (challenge, requestTime) = connection.pingRequest ?: return connection.dos("Unexpected packet Pong")

        val solution = if (connection.version >= Ping.MIN_VERSION)
            solve(challenge)
        else if (connection.version == 13)
            solveV1(challenge)
        else
            challenge

        if (response != solution) {
            connection.dos("Invalid Pong")
            return
        }

        connection.ping = connection.lastPacketTime - requestTime
        connection.pingRequest = null
    }
}
