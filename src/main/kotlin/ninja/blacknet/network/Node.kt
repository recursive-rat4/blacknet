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
import io.ktor.network.sockets.ServerSocket
import io.ktor.network.sockets.aSocket
import io.ktor.network.util.ioCoroutineDispatcher
import kotlinx.coroutines.experimental.delay
import kotlinx.coroutines.experimental.launch
import mu.KotlinLogging
import ninja.blacknet.core.SynchronizedArrayList
import ninja.blacknet.db.PeerDB
import java.net.Inet6Address
import java.net.InetAddress
import java.net.InetSocketAddress
import java.time.Instant
import java.util.*

private val logger = KotlinLogging.logger {}

object Node {
    const val magic = 0x17895E7D
    const val version = 5
    const val minVersion = 5
    const val agent = "Blacknet"
    val nonce = Random().nextLong()
    val connections = SynchronizedArrayList<Connection>()

    init {
        launch { pinger() }
        launch { peerAnnouncer() }
    }

    fun time(): Long {
        return Instant.now().getEpochSecond()
    }

    fun timeMilli(): Long {
        return Instant.now().toEpochMilli()
    }

    fun listenOn(address: Address) {
        val addr = when (address.network) {
            Network.IPv4, Network.IPv6 -> InetSocketAddress(InetAddress.getByAddress(address.bytes.array), address.port)
            else -> throw Exception("not implemented for " + address.network)
        }

        val server = aSocket(ActorSelectorManager(ioCoroutineDispatcher)).tcp().bind(addr)
        logger.info("Listening on $address")
        launch { listener(server) }
    }

    suspend fun connectTo(address: Address): Connection {
        val addr = when (address.network) {
            Network.IPv4, Network.IPv6 -> InetSocketAddress(InetAddress.getByAddress(address.bytes.array), address.port)
            else -> throw Exception("not implemented for " + address.network)
        }
        val socket = aSocket(ActorSelectorManager(ioCoroutineDispatcher)).tcp().connect(addr)
        val connection = Connection(socket, address, Connection.State.OUTGOING_WAITING)
        Node.connections.add(connection)
        Node.sendVersion(connection)
        return connection
    }

    fun sendVersion(connection: Connection) {
        val v = Version(magic, version, time(), nonce, agent)
        connection.sendPacket(v)
    }

    private suspend fun listener(server: ServerSocket) {
        while (true) {
            val socket = server.accept()
            val inet = socket.remoteAddress as InetSocketAddress
            val addr = inet.getAddress()
            val network = if (addr is Inet6Address) Network.IPv6 else Network.IPv4
            val address = Address(network, inet.port, addr.getAddress())
            val connection = Connection(socket, address, Connection.State.INCOMING_WAITING)
            connections.add(connection)
        }
    }

    private suspend fun pinger() {
        val timeout = 60
        val random = Random()
        while (true) {
            delay(timeout * 1000)

            connections.forEach {
                if (it.state.isWaiting()) {
                    if (time() > it.connectedAt + timeout)
                        it.close()
                } else {
                    if (it.pingRequest == null) {
                        val id = random.nextInt()
                        val ping = Ping(id)
                        it.pingRequest = Connection.PingRequest(id, Node.timeMilli())
                        it.sendPacket(ping)
                    } else {
                        logger.info("Disconnecting ${it.remoteAddress} on ping timeout")
                        it.close()
                    }
                }
            }
        }
    }

    private suspend fun peerAnnouncer() {
        while (true) {
            delay(5 * 60 * 1000)

            val randomPeers = PeerDB.getRandom(Peers.MAX)
            if (randomPeers.size == 0)
                continue

            val peers = Peers(randomPeers)

            connections.forEach {
                if (it.state.isConnected())
                    it.sendPacket(peers)
            }
        }
    }

    suspend fun disconnected(connection: Connection) {
        connections.remove(connection)
    }
}