/*
 * Copyright (c) 2018 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.network

import io.ktor.network.sockets.ServerSocket
import io.ktor.network.sockets.aSocket
import io.ktor.network.sockets.openReadChannel
import io.ktor.network.sockets.openWriteChannel
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.io.cancel
import kotlinx.coroutines.io.close
import kotlinx.coroutines.launch
import mu.KotlinLogging
import ninja.blacknet.Config
import ninja.blacknet.Config.dnsseed
import ninja.blacknet.Config.incomingconnections
import ninja.blacknet.Config.mintxfee
import ninja.blacknet.Config.outgoingconnections
import ninja.blacknet.Config.port
import ninja.blacknet.core.DataDB.Status
import ninja.blacknet.core.DataType
import ninja.blacknet.core.PoS
import ninja.blacknet.core.TxPool
import ninja.blacknet.crypto.BigInt
import ninja.blacknet.crypto.Hash
import ninja.blacknet.crypto.PrivateKey
import ninja.blacknet.db.BlockDB
import ninja.blacknet.db.LedgerDB
import ninja.blacknet.db.PeerDB
import ninja.blacknet.util.SynchronizedArrayList
import ninja.blacknet.util.SynchronizedHashSet
import ninja.blacknet.util.delay
import java.math.BigDecimal
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

    suspend fun incoming(includeWaiting: Boolean = false): Int {
        return connections.sumBy {
            if (includeWaiting)
                when (it.state) {
                    Connection.State.INCOMING_CONNECTED -> 1
                    Connection.State.INCOMING_WAITING -> 1
                    else -> 0
                }
            else
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

    fun isSynchronizing(): Boolean {
        return ChainFetcher.isSynchronizing()
    }

    fun listenOn(address: Address) {
        val addr = when (address.network) {
            Network.IPv4, Network.IPv6 -> address.getSocketAddress()
            else -> throw NotImplementedError("not implemented for " + address.network)
        }
        val server = aSocket(Network.selector).tcp().bind(addr)
        logger.info("Listening on $address")
        launch {
            listenAddress.add(address)
            listener(server)
        }
    }

    fun listenOnIP() {
        if (Network.IPv4.isDisabled() && Network.IPv6.isDisabled())
            return
        if (Network.IPv4.isDisabled())
            return Node.listenOn(Address.IPv6_ANY(Config[port]))
        Node.listenOn(Address.IPv4_ANY(Config[port]))
    }

    fun listenOnTor() {
        launch {
            val address = Network.listenOnTor()
            if (address != null) {
                logger.info("Listening on $address")
                listenAddress.add(address)
            }
        }
    }

    fun listenOnI2P() {
        launch {
            val address = Network.listenOnI2P()
            if (address != null) {
                logger.info("Listening on $address")
                listenAddress.add(address)
                i2plistener()
            }
        }
    }

    fun disconnected(connection: Connection) = launch {
        connections.remove(connection)
    }

    suspend fun connectTo(address: Address) {
        val connection = Network.connect(address)
        connections.add(connection)
        sendVersion(connection)
    }

    fun sendVersion(connection: Connection) {
        val blockHash = if (isSynchronizing()) Hash.ZERO else LedgerDB.blockHash()
        val cumulativeDifficulty = if (isSynchronizing()) BigInt.ZERO else LedgerDB.cumulativeDifficulty()
        val v = Version(magic, version, time(), nonce, agent, minTxFee, blockHash, cumulativeDifficulty)
        connection.sendPacket(v)
    }

    suspend fun broadcastBlock(hash: Hash, bytes: ByteArray): Boolean {
        val status = BlockDB.process(hash, bytes)
        if (status == Status.ACCEPTED) {
            val inv = InvList()
            inv.add(Pair(DataType.Block, hash))
            broadcastPacket(Inventory(inv))
            return true
        } else {
            logger.info("$status block $hash")
            return false
        }
    }

    suspend fun broadcastTx(hash: Hash, bytes: ByteArray, fee: Long): Boolean {
        val status = TxPool.process(hash, bytes)
        if (status == Status.ACCEPTED) {
            val inv = InvList()
            inv.add(Pair(DataType.Transaction, hash))
            val packet = Inventory(inv)
            broadcastPacket(packet) {
                it.feeFilter <= fee
            }
            return true
        } else {
            logger.info("$status tx $hash")
            return false
        }
    }

    suspend fun broadcastInv(inv: InvList, filter: Connection): Boolean {
        //TODO feeFilter
        broadcastPacket(Inventory(inv)) {
            it != filter
        }
        return true
    }

    private suspend fun broadcastPacket(packet: Packet, filter: (Connection) -> Boolean = { true }) {
        val bytes = packet.build()
        connections.forEach {
            if (it.state.isConnected() && filter(it))
                it.sendPacket(bytes.copy())
        }
        bytes.release()
    }

    private suspend fun listener(server: ServerSocket) {
        while (true) {
            val socket = server.accept()
            val remoteAddress = Network.address(socket.remoteAddress as InetSocketAddress)
            if (!haveSlot()) {
                logger.info("Too many connections, dropping $remoteAddress")
                socket.close()
                continue
            }
            val localAddress = Network.address(socket.localAddress as InetSocketAddress)
            if (!localAddress.isLocal())
                listenAddress.add(localAddress)
            val connection = Connection(socket.openReadChannel(), socket.openWriteChannel(true), remoteAddress, localAddress, Connection.State.INCOMING_WAITING)
            connections.add(connection)
        }
    }

    private suspend fun i2plistener() {
        while (true) {
            val c = I2PSAM.accept() ?: continue
            if (!haveSlot()) {
                logger.info("Too many connections, dropping ${c.remoteAddress}")
                c.readChannel.cancel()
                c.writeChannel.close()
                continue
            }
            val connection = Connection(c.readChannel, c.writeChannel, c.remoteAddress, I2PSAM.localAddress!!, Connection.State.INCOMING_WAITING)
            connections.add(connection)
        }
    }

    private suspend fun haveSlot(): Boolean {
        if (incoming(true) < Config[incomingconnections])
            return true
        return evictConnection()
    }

    private suspend fun evictConnection(): Boolean {
        val candidates = connections.copy().asSequence()
                .sortedBy { if (it.ping != 0L) it.ping else Long.MAX_VALUE }.drop(4)
                .sortedByDescending { it.lastTxTime }.drop(4)
                .sortedByDescending { it.lastBlockTime }.drop(4)
                .sortedBy { it.connectedAt }.drop(4)
                .toMutableList()

        //TODO network groups

        if (candidates.isEmpty())
            return false

        val connection = candidates.random()
        logger.info("Evicting ${connection.remoteAddress}")
        connection.close()
        return true
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
                logger.info("Don't have candidates in PeerDB. ${outgoing()} connections, max ${Config[outgoingconnections]}")
                delay(PeerDB.DELAY)
                dnsSeeder(false)
                continue
            }

            PeerDB.attempt(address)
            PeerDB.commit()

            try {
                connectTo(address)
            } catch (e: Throwable) {
            }

            delay(NETWORK_TIMEOUT) //TODO
        }
    }

    private suspend fun pinger() {
        while (true) {
            delay(NETWORK_TIMEOUT)

            val currTime = timeMilli()
            connections.forEach {
                if (it.state.isWaiting()) {
                    if (currTime > it.connectedAt + NETWORK_TIMEOUT * 1000)
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
            delay(7 * 60)

            val randomPeers = PeerDB.getRandom(Peers.MAX)
            if (randomPeers.size == 0)
                continue

            val myAddress = listenAddress.filter { !it.isLocal() && !it.isPrivate() }
            if (myAddress.isNotEmpty()) {
                val i = Random.nextInt(randomPeers.size * 500)
                if (i < randomPeers.size)
                    randomPeers[i] = myAddress[Random.nextInt(myAddress.size)]
            }

            broadcastPacket(Peers(randomPeers))
        }
    }

    suspend fun startStaker(privateKey: PrivateKey): Boolean {
        val publicKey = privateKey.toPublicKey()
        if (LedgerDB.get(publicKey) == null) {
            logger.info("account not found")
            return false
        }
        launch { PoS.staker(privateKey, publicKey) }
        return true
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
            val peers = Network.resolveAll(seeds, DEFAULT_P2P_PORT)
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