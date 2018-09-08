/*
 * Copyright (c) 2018 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.network

import kotlinx.serialization.Serializable

@Serializable
class Pong(private val id: Int) : Packet {
    override suspend fun process(connection: Connection) {
        val request = connection.pingRequest
        if (request == null) {
            connection.dos(1, "unexpected pong")
            return
        }
        if (request.id != id) {
            connection.dos(1, "invalid pong id")
            return
        }
        connection.ping = Server.time() - request.time
        connection.pingRequest = null
    }
}