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
import io.ktor.network.sockets.openReadChannel
import io.ktor.network.sockets.openWriteChannel
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.launch
import mu.KotlinLogging
import ninja.blacknet.Config
import ninja.blacknet.Config.dnsseed
import ninja.blacknet.Config.incomingconnections
import ninja.blacknet.Config.mintxfee
import ninja.blacknet.Config.outgoingconnections
import ninja.blacknet.core.DataType
import ninja.blacknet.core.PoS
import ninja.blacknet.core.TxPool
import ninja.blacknet.crypto.BigInt
import ninja.blacknet.crypto.Hash
import ninja.blacknet.db.LedgerDB
import ninja.blacknet.db.PeerDB
import ninja.blacknet.util.SynchronizedArrayList
import ninja.blacknet.util.SynchronizedHashSet
import ninja.blacknet.util.delay
import java.math.BigDecimal
import java.net.InetAddress
import java.net.InetSocketAddress
import java.time.Instant
import kotlin.coroutines.CoroutineContext
import kotlin.random.Random

private val logger = KotlinLogging.logger {}

object Node : CoroutineScope {
    const val DEFAULT_P2P_PORT = 28453
    const val NETWORK_TIMEOUT = 60
    const val magic = 0x17895E7D
    const val version = 5
    const val minVersion = 5
    const val agent = "Blacknet"
    override val coroutineContext: CoroutineContext = Dispatchers.Default
    val nonce = Random.nextLong()
    val connections = SynchronizedArrayList<Connection>()
    val listenAddress = SynchronizedHashSet<Address>()
    var minTxFee = parseAmount(Config[mintxfee])

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

    suspend fun isOffline(): Boolean {
        return connected() == 0
    }

    fun getMaxPacketSize(): Int {
        return LedgerDB.maxBlockSize() + 100
    }

    fun isInitialBlockDownload(): Boolean {
        return false //TODO
    }

    fun listenOn(address: Address) {
        val addr = when (address.network) {
            Network.IPv4, Network.IPv6 -> InetSocketAddress(InetAddress.getByAddress(address.bytes.array), address.port)
            else -> throw NotImplementedError("not implemented for " + address.network)
        }
        val server = aSocket(ActorSelectorManager(Dispatchers.IO)).tcp().bind(addr)
        logger.info("Listening on $address")
        launch {
            listenAddress.add(address)
            listener(server)
        }
    }

    fun listenOnTor() {
        launch {
            val address = Network.listenOnTor()
            if (address != null) {
                logger.info("Listening on $address")
                Node.listenAddress.add(address)
            }
        }
    }

    suspend fun disconnected(connection: Connection) {
        connections.remove(connection)
    }

    suspend fun connectTo(address: Address) {
        val connection = Network.connect(address)
        connections.add(connection)
        sendVersion(connection)
    }

    fun sendVersion(connection: Connection) {
        val blockHash = if (isInitialBlockDownload()) Hash.ZERO else LedgerDB.blockHash()
        val cumulativeDifficulty = if (isInitialBlockDownload()) BigInt.ZERO else LedgerDB.cumulativeDifficulty()
        val v = Version(magic, version, time(), nonce, agent, minTxFee, blockHash, cumulativeDifficulty)
        connection.sendPacket(v)
    }

    suspend fun broadcastTx(hash: Hash, bytes: ByteArray, fee: Long): Boolean {
        if (TxPool.process(hash, bytes)) {
            val inv = InvList()
            inv.add(Pair(DataType.Transaction, hash))
            val packet = Inventory(inv).build()
            connections.forEach {
                if (it.state.isConnected() && it.feeFilter <= fee)
                    it.sendPacket(packet)
            }
            return true
        }
        return false
    }

    private suspend fun broadcastPacket(packet: Packet) {
        val bytes = packet.build()
        connections.forEach {
            if (it.state.isConnected())
                it.sendPacket(bytes)
        }
    }

    private suspend fun listener(server: ServerSocket) {
        while (true) {
            val socket = server.accept()
            if (incoming() >= Config[incomingconnections]) {
                socket.close()
                continue
            }
            val remoteAddress = Network.address(socket.remoteAddress as InetSocketAddress)
            val localAddress = Network.address(socket.localAddress as InetSocketAddress)
            if (!localAddress.isLocal())
                listenAddress.add(localAddress)
            val connection = Connection(socket.openReadChannel(), socket.openWriteChannel(true), remoteAddress, localAddress, Connection.State.INCOMING_WAITING)
            connections.add(connection)
        }
    }

    private suspend fun connector() {
        if (PeerDB.isEmpty()) {
            logger.info("PeerDB is empty.")
            delay(DNS_TIMEOUT)
        }

        while (true) {
            if (outgoing() >= Config[outgoingconnections]) {
                delay(NETWORK_TIMEOUT)
                continue
            }

            val filter = connections.map { it.remoteAddress }.plus(listenAddress.toList())

            val address = PeerDB.getCandidate(filter)
            if (address == null) {
                logger.info("Don't have candidates in PeerDB")
                delay(PeerDB.DELAY)
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
                        val id = Random.nextInt()
                        it.pingRequest = Connection.PingRequest(id, timeMilli())
                        it.sendPacket(Ping(id))
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
            if (myAddress.isNotEmpty()) {
                val i = Random.nextInt(randomPeers.size * 500)
                if (i < randomPeers.size)
                    randomPeers[i] = myAddress[Random.nextInt(myAddress.size)]
            }

            broadcastPacket(Peers(randomPeers))
        }
    }

    private const val DNS_TIMEOUT = 5
    private const val DNS_SEEDER_DELAY = 11
    private suspend fun dnsSeeder(delay: Boolean) {
        if (!Config[dnsseed])
            return

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
            val peers = Network.resolveAll(seeds, DEFAULT_P2P_PORT).filter { !it.isLocal() }
            PeerDB.add(peers, Address.LOOPBACK)
            PeerDB.commit()
        } catch (e: Throwable) {
        }
    }

    private fun parseAmount(string: String): Long {
        val n = (BigDecimal(string) * BigDecimal(PoS.COIN)).longValueExact()
        if (n < 0) throw RuntimeException("Negative amount")
        return n
    }
}