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
import mu.KotlinLogging

private val logger = KotlinLogging.logger {}

@Serializable
class Version(
        private val magic: Int,
        private val version: Int,
        private val time: Long,
        private val nonce: Long,
        private val agent: String
) : Packet {
    override suspend fun process(connection: Connection) {
        connection.timeOffset = Server.time() - time

        if (magic != Server.magic || version < Server.minVersion || nonce == Server.nonce) {
            connection.close()
            return
        }

        if (connection.state == Connection.State.INCOMING_WAITING) {
            val myVersion = Version(Server.magic, Server.version, Server.time(), Server.nonce, Server.agent)
            //TODO send
            connection.state = Connection.State.INCOMING_CONNECTED
            logger.info("Accepted connection from ${connection.remoteAddress}")
        } else {
            connection.state = Connection.State.OUTGOING_CONNECTED
            logger.info("Connected to ${connection.remoteAddress}")
        }
    }
}