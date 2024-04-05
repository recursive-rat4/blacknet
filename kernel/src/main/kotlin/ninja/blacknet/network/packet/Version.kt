/*
 * Copyright (c) 2018-2024 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.network.packet

import io.github.oshai.kotlinlogging.KotlinLogging
import kotlinx.serialization.Serializable
import ninja.blacknet.db.PeerDB
import ninja.blacknet.mode
import ninja.blacknet.network.*
import ninja.blacknet.time.currentTimeSeconds

private val logger = KotlinLogging.logger {}

@Serializable
class Version(
    private val magic: Int,
    private val version: Int,
    private val time: Long,
    private val nonce: Long,
    private val agent: String,
    private val feeFilter: Long,
    private val chainAnnounce: ChainAnnounce
) : Packet {
    override fun handle(connection: Connection) {
        if (magic != mode.networkMagic) {
            // connection from another network
            connection.close()
            return
        }

        connection.timeOffset = time - currentTimeSeconds()
        connection.version = version
        connection.agent = UserAgent.sanitize(agent)
        connection.feeFilter = feeFilter
        connection.lastChain = chainAnnounce

        if (version < Node.MIN_PROTOCOL_VERSION) {
            logger.info { "Obsolete protocol version $version ${connection.debugName()} ${connection.agent}" }
            connection.close()
            return
        }

        if (connection.state == Connection.State.INCOMING_WAITING) {
            if (nonce != Node.nonce) {
                // echo the nonce
                Node.sendVersion(connection, nonce, prober = false)
                logger.info { "Accepted connection from ${connection.debugName()} ${connection.agent}" }
                connection.state = Connection.State.INCOMING_CONNECTED
            } else {
                // connected to self or bad luck
                connection.close()
                return
            }
        } else if (connection.state == Connection.State.OUTGOING_WAITING) {
            logger.info { "Connected to ${connection.debugName()} ${connection.agent}" }
            connection.state = Connection.State.OUTGOING_CONNECTED
            PeerDB.connected(connection.remoteAddress, connection.connectedAt, connection.agent, prober = false)
        } else if (connection.state == Connection.State.PROBER_WAITING) {
            // keeping track of online peers
            connection.state = Connection.State.PROBER_CONNECTED
            connection.close()
            PeerDB.connected(connection.remoteAddress, connection.connectedAt, connection.agent, prober = true)
            return
        } else {
            // this condition should never be reached
            throw IllegalStateException("${connection.state}")
        }

        // got anything?
        ChainFetcher.offer(connection, chainAnnounce)
    }
}
