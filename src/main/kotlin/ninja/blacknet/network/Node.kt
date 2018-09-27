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
import kotlinx.coroutines.experimental.launch
import mu.KotlinLogging
import ninja.blacknet.db.PeerDB
import ninja.blacknet.util.SynchronizedArrayList
import ninja.blacknet.util.delay
import java.net.InetAddress
import java.net.InetSocketAddress
import java.time.Instant
import java.util.*

private val logger = KotlinLogging.logger {}

object Node {
    const val P2P_PORT = 28453
    const val DEFAULT_MAX_PACKET_SIZE = 640000
    const val MIN_CONNECTIONS = 8
    const val NETWORK_TIMEOUT = 60
    const val magic = 0x17895E7D
    const val version = 5
    const val minVersion = 5
    const val agent = "Blacknet"
    val localAddress = Address.IPv6_LOOPBACK(Node.P2P_PORT)
    private val random = Random()
    val nonce = random.nextLong()
    val connections = SynchronizedArrayList<Connection>()
    val listenAddress = SynchronizedArrayList<Address>()

    init {
        launch { connector() }
        launch { pinger() }
        launch { peerAnnouncer() }
        launch { dnsSeeder(true) }
    }

    fun time(): Long {
        return Instant.now().getEpochSecond()
    }

    fun timeMilli(): Long {
        return Instant.now().toEpochMilli()
    }

    fun isTooFarInFuture(time: Long): Boolean {
        return time > Node.time() + 15
    }

    suspend fun outgoing(): Int {
        return connections.sumBy {
            when (it.state) {
                Connection.State.OUTGOING_CONNECTED -> 1
                else -> 0
            }
        }
    }

    suspend fun incoming(): Int {
        return connections.sumBy {
            when (it.state) {
                Connection.State.INCOMING_CONNECTED -> 1
                else -> 0
            }
        }
    }

    suspend fun connected(): Int {
        return connections.sumBy {
            when (it.state) {
                Connection.State.OUTGOING_CONNECTED -> 1
                Connection.State.INCOMING_CONNECTED -> 1
                else -> 0
            }
        }
    }

    fun listenOn(address: Address) {
        val addr = when (address.network) {
            Network.IPv4, Network.IPv6 -> InetSocketAddress(InetAddress.getByAddress(address.bytes.array), address.port)
            else -> throw Exception("not implemented for " + address.network)
        }
        val server = aSocket(ActorSelectorManager(ioCoroutineDispatcher)).tcp().bind(addr)
        logger.info("Listening on $address")
        launch {
            listenAddress.add(address)
            listener(server)
        }
    }

    private suspend fun addConnection(connection: Connection) {
        connections.add(connection)
        connection.invokeOnDisconnect { launch { connections.remove(connection) } }
    }

    suspend fun connectTo(address: Address): Connection {
        val addr = when (address.network) {
            Network.IPv4, Network.IPv6 -> InetSocketAddress(InetAddress.getByAddress(address.bytes.array), address.port)
            else -> throw Exception("not implemented for " + address.network)
        }
        val socket = aSocket(ActorSelectorManager(ioCoroutineDispatcher)).tcp().connect(addr)
        val connection = Connection(socket, address, Connection.State.OUTGOING_WAITING)
        addConnection(connection)
        sendVersion(connection)
        return connection
    }

    fun sendVersion(connection: Connection) {
        val v = Version(magic, version, time(), nonce, agent)
        connection.sendPacket(v)
    }

    private suspend fun listener(server: ServerSocket) {
        while (true) {
            val socket = server.accept()
            val address = Network.address(socket.remoteAddress as InetSocketAddress)
            val connection = Connection(socket, address, Connection.State.INCOMING_WAITING)
            addConnection(connection)
        }
    }

    private suspend fun connector() {
        if (PeerDB.isEmpty()) {
            logger.info("PeerDB is empty. Waiting for dns seeder.")
            delay(DNS_TIMEOUT)
        }

        while (true) {
            if (outgoing() >= MIN_CONNECTIONS)
                delay(NETWORK_TIMEOUT)

            val filter = connections.map { it.remoteAddress }.plus(listenAddress.clone())

            val address = PeerDB.getCandidate(filter)
            if (address == null) {
                logger.info("Don't have candidates in PeerDB")
                delay(PeerDB.NETWORK_TIMEOUT)
                dnsSeeder(false)
                continue
            }

            try {
                connectTo(address)
                PeerDB.connected(address)
            } catch (e: Throwable) {
                PeerDB.failed(address)
            } finally {
                PeerDB.commit()
            }

            delay(NETWORK_TIMEOUT) //TODO
        }
    }

    private suspend fun pinger() {
        while (true) {
            delay(NETWORK_TIMEOUT)

            val currTime = time()
            connections.forEach {
                if (it.state.isWaiting()) {
                    if (currTime > it.connectedAt + NETWORK_TIMEOUT)
                        it.close()
                } else {
                    if (it.pingRequest == null) {
                        val id = random.nextInt()
                        val ping = Ping(id)
                        it.pingRequest = Connection.PingRequest(id, timeMilli())
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
            delay(5 * 60)

            val randomPeers = PeerDB.getRandom(Peers.MAX)
            if (randomPeers.size == 0)
                continue

            val myAddress = listenAddress.filter { !it.isLocal() }
            if (myAddress.size > 0) {
                val i = random.nextInt(randomPeers.size * 500)
                if (i < randomPeers.size)
                    randomPeers[i] = myAddress[random.nextInt(myAddress.size)]
            }

            val peers = Peers(randomPeers)

            connections.forEach {
                if (it.state.isConnected())
                    it.sendPacket(peers)
            }
        }
    }

    private val DNS_TIMEOUT = 5
    private val DNS_SEEDER_DELAY = 11
    private suspend fun dnsSeeder(delay: Boolean) {
        if (delay && !PeerDB.isEmpty()) {
            delay(DNS_SEEDER_DELAY)
            if (connected() >= 2) {
                logger.info("P2P peers available. Skipped DNS seeding.")
                return
            }
        }

        logger.info("Requesting DNS seeds.")

        val seeds = "dnsseed.blacknet.ninja"
        try {
            val response = InetAddress.getAllByName(seeds)
            val peers = response.map { Network.address(it, P2P_PORT) }.filter { !it.isLocal() }
            PeerDB.add(peers, localAddress)
            PeerDB.commit()
        } catch (e: Throwable) {
        }
    }
}