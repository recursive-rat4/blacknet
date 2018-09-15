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
import mu.KotlinLogging
import ninja.blacknet.serialization.BlacknetOutput

private val logger = KotlinLogging.logger {}

@Serializable
class Version(
        private val magic: Int,
        private val version: Int,
        private val time: Long,
        private val nonce: Long,
        private val agent: String
) : Packet {
    override fun serialize(): ByteReadPacket {
        val out = BlacknetOutput()
        out.write(this)
        return out.build()
    }

    override fun getType(): Int {
        return PacketType.Version.ordinal
    }

    override fun process(connection: Connection) {
        connection.timeOffset = Node.time() - time

        if (magic != Node.magic || version < Node.minVersion || nonce == Node.nonce) {
            connection.close()
            return
        }

        if (connection.state == Connection.State.INCOMING_WAITING) {
            Node.sendVersion(connection)
            connection.state = Connection.State.INCOMING_CONNECTED
            logger.info("Accepted connection from ${connection.remoteAddress}")
        } else {
            connection.state = Connection.State.OUTGOING_CONNECTED
            logger.info("Connected to ${connection.remoteAddress}")
        }
    }
}