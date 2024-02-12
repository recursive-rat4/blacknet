/*
 * Copyright (c) 2018-2023 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.network.packet

import io.github.oshai.kotlinlogging.KotlinLogging
import kotlinx.serialization.KSerializer
import kotlinx.serialization.Serializable
import kotlinx.serialization.builtins.serializer
import ninja.blacknet.db.PeerDB
import ninja.blacknet.mode
import ninja.blacknet.network.Connection
import ninja.blacknet.network.Node
import ninja.blacknet.network.UserAgent
import ninja.blacknet.serialization.ByteArraySerializer
import ninja.blacknet.serialization.bbf.binaryFormat

private val logger = KotlinLogging.logger {}

@Serializable
class Hello(
    private val data: HashMap<Byte, @Serializable(ByteArraySerializer::class) ByteArray> = HashMap()
) : Packet {
    var magic: Int? by Delegate(MAGIC, Int.serializer())
    var version: Int? by Delegate(VERSION, Int.serializer())
    var nonce: Long? by Delegate(NONCE, Long.serializer())
    var agent: String? by Delegate(AGENT, String.serializer())
    var feeFilter: Long? by Delegate(FEE_FILTER, Long.serializer())

    override suspend fun process(connection: Connection) {
        magic?.let {
            if (it != mode.networkMagic) {
                // connection from another network
                connection.close()
                return
            }
        }
        // if not provided, the oldest supported version may be tried
        connection.version = version ?: Node.MIN_PROTOCOL_VERSION
        agent?.let {
            connection.agent = UserAgent.sanitize(it)
        }
        feeFilter?.let {
            connection.feeFilter = it
        }

        if (connection.version < Node.MIN_PROTOCOL_VERSION) {
            logger.info { "Obsolete protocol version ${connection.version} ${connection.debugName()} ${connection.agent}" }
            connection.close()
            return
        }

        when (connection.state) {
            Connection.State.INCOMING_WAITING -> {
                nonce?.let {
                    if (it == Node.nonce) {
                        // connected to self or bad luck
                        connection.close()
                        return
                    }
                }
                Node.sendHandshake(connection)
                logger.info { "Accepted connection from ${connection.debugName()} ${connection.agent}" }
                connection.state = Connection.State.INCOMING_CONNECTED
            }
            Connection.State.OUTGOING_WAITING -> {
                logger.info { "Connected to ${connection.debugName()} ${connection.agent}" }
                connection.state = Connection.State.OUTGOING_CONNECTED
                PeerDB.connected(connection.remoteAddress, connection.connectedAt, connection.agent, prober = false)
            }
            Connection.State.PROBER_WAITING -> {
                // keeping track of online peers
                connection.state = Connection.State.PROBER_CONNECTED
                connection.close()
                PeerDB.connected(connection.remoteAddress, connection.connectedAt, connection.agent, prober = true)
                return
            }
            else -> {
                // this condition should never be reached
                throw IllegalStateException("${connection.debugName()} ${connection.state}")
            }
        }
    }

    private class Delegate<T>(
        private val key: Byte,
        private val serializer: KSerializer<T>,
    ) {
        operator fun getValue(thisRef: Hello, property: Any): T? =
            thisRef.data.get(key)?.let {
                binaryFormat.decodeFromByteArray(serializer, it)
            }

        operator fun setValue(thisRef: Hello, property: Any, value: T?) =
            if (value != null)
                thisRef.data.put(key, binaryFormat.encodeToByteArray(serializer, value))
            else
                thisRef.data.remove(key)
    }

    companion object {
        const val MIN_VERSION = 15
    }
}

private const val MAGIC = 128.toByte()
private const val VERSION = 129.toByte()
private const val NONCE = 130.toByte()
private const val AGENT = 131.toByte()
private const val FEE_FILTER = 132.toByte()
