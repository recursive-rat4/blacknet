/*
 * Copyright (c) 2018 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.network

import io.ktor.network.selector.ActorSelectorManager
import io.ktor.network.sockets.aSocket
import io.ktor.network.util.ioCoroutineDispatcher
import kotlinx.coroutines.experimental.delay
import kotlinx.coroutines.experimental.launch
import kotlinx.coroutines.experimental.runBlocking
import mu.KotlinLogging
import java.net.InetSocketAddress
import java.time.Instant
import java.util.*
import kotlin.collections.ArrayList

private val logger = KotlinLogging.logger {}

object Node {
    const val magic = 0x17895E7D
    const val version = 5
    const val minVersion = 5
    const val agent = "Blacknet"
    val nonce = Random().nextLong()
    private val connections = ArrayList<Connection>()

    init {
        launch { pinger() }
    }

    fun time(): Long {
        return Instant.now().getEpochSecond()
    }

    fun listen() {
        runBlocking {
            val server = aSocket(ActorSelectorManager(ioCoroutineDispatcher)).tcp().bind(InetSocketAddress("127.0.0.1", 28453))
            logger.info("Listening at ${server.localAddress}")

            while (true) {
                val socket = server.accept()
                val connection = Connection(socket, Connection.State.INCOMING_WAITING)
                connections.add(connection)
            }
        }
    }

    suspend fun connectTo(addr: InetSocketAddress): Connection {
        val socket = aSocket(ActorSelectorManager(ioCoroutineDispatcher)).tcp().connect(addr)
        val connection = Connection(socket, Connection.State.OUTGOING_WAITING)
        Node.connections.add(connection)
        Node.sendVersion(connection)
        return connection
    }

    fun sendVersion(connection: Connection) {
        val v = Version(magic, version, time(), nonce, agent)
        connection.sendPacket(v)
    }

    private suspend fun pinger() {
        val timeout = 60
        val random = Random()
        while (true) {
            for (connection in connections) {
                if (connection.state.isWaiting()) {
                    if (time() > connection.connectedAt + timeout)
                        connection.close()
                    continue
                }
                if (connection.pingRequest == null) {
                    val id = random.nextInt()
                    val ping = Ping(id)
                    connection.pingRequest = Connection.PingRequest(id, time())
                    connection.sendPacket(ping)
                } else {
                    logger.info("Disconnecting ${connection.remoteAddress} on ping timeout")
                    connection.close()
                }
            }
            delay(timeout * 1000)
        }
    }

    fun disconnected(connection: Connection) {
        connections.remove(connection)
    }
}