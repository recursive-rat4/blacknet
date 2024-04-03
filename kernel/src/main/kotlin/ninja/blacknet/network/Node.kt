/*
 * Copyright (c) 2018-2024 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.network

import io.github.oshai.kotlinlogging.KotlinLogging
import java.lang.Thread.sleep
import java.math.BigInteger
import java.net.InetSocketAddress
import java.net.ServerSocket
import java.net.SocketException
import java.nio.channels.FileChannel
import java.nio.file.NoSuchFileException
import java.nio.file.StandardOpenOption.READ
import java.util.concurrent.ConcurrentLinkedQueue
import java.util.concurrent.CopyOnWriteArrayList
import java.util.concurrent.CopyOnWriteArraySet
import kotlin.random.Random
import kotlinx.atomicfu.atomic
import kotlinx.coroutines.launch
import kotlinx.serialization.Serializable
import ninja.blacknet.stateDir
import ninja.blacknet.Kernel
import ninja.blacknet.Runtime
import ninja.blacknet.ShutdownHooks
import ninja.blacknet.core.*
import ninja.blacknet.crypto.Hash
import ninja.blacknet.crypto.PoS
import ninja.blacknet.db.LedgerDB
import ninja.blacknet.db.PeerDB
import ninja.blacknet.io.buffered
import ninja.blacknet.io.data
import ninja.blacknet.io.inputStream
import ninja.blacknet.io.replaceFile
import ninja.blacknet.logging.error
import ninja.blacknet.mode
import ninja.blacknet.network.packet.*
import ninja.blacknet.serialization.bbf.binaryFormat
import ninja.blacknet.time.currentTimeMillis
import ninja.blacknet.time.currentTimeSeconds
import ninja.blacknet.util.rotate
import ninja.blacknet.util.startInterruptible

private val logger = KotlinLogging.logger {}

object Node {
    const val NETWORK_TIMEOUT = 90 * 1000L
    const val PROTOCOL_VERSION = 15
    const val MIN_PROTOCOL_VERSION = 12
    private const val DATA_VERSION = 1
    private const val DATA_FILENAME = "node.dat"
    private val router = Router(Kernel.config())
    val nonce = Random.nextLong()
    val connections = CopyOnWriteArrayList<Connection>()
    private val listenAddress = CopyOnWriteArraySet<Address>()
    private val nextPeerId = atomic(1L)
    private val queuedPeers = ConcurrentLinkedQueue<Address>()

    init {
        // All connectors, including listeners and probers
        val connectors = ArrayList<Thread>(Kernel.config().outgoingconnections + 3 + 1)

        if (Kernel.config().listen) {
            try {
                if (!router.isDisabled(Network.IPv4) || !router.isDisabled(Network.IPv6)) {
                    connectors.add(
                        listenOnIP()
                    )
                    if (Kernel.config().upnp) {
                        Runtime.launch { UPnP.forward() }
                    }
                }
            } catch (e: Throwable) {
            }
        }
        if (Kernel.config().tor) {
            if (!Kernel.config().listen || router.isDisabled(Network.IPv4) && router.isDisabled(Network.IPv6))
                connectors.add(
                    listenOn(Network.LOOPBACK(Kernel.config().port))
                )
            connectors.add(
                rotate("Node.router::listenOnTor", router::listenOnTor)
            )
        }
        if (Kernel.config().i2p) {
            connectors.add(
                rotate("Node.router::listenOnI2P", router::listenOnI2P)
            )
        }
        try {
            val file = stateDir.resolve(DATA_FILENAME)
            FileChannel.open(file, READ).inputStream().buffered().data().use { stream ->
                val version = stream.readInt()
                if (version == DATA_VERSION) {
                    val persistent = binaryFormat.decodeFromStream(Persistent.serializer(), stream, true)
                    persistent.peers.forEach { peer ->
                        queuedPeers.offer(peer)
                    }
                } else {
                    logger.warn { "Unknown node data version $version" }
                }
            }
        } catch (e: NoSuchFileException) {
            // first run or unlinked file
        } catch (e: Exception) {
            logger.error(e)
        }
        repeat(Kernel.config().outgoingconnections) {
            connectors.add(
                rotate("Node::connector $it", ::connector)
            )
        }
        connectors.add(
            rotate("Node::prober", ::prober)
        )
        ShutdownHooks.add {
            logger.info { "Unbinding ${connectors.size} connectors" }
            connectors.forEach(Thread::interrupt)
            val persistent = Persistent(ArrayList(Kernel.config().outgoingconnections))
            synchronized(connections) {
                logger.info { "Closing ${connections.size} p2p connections" }
                connections.forEach { connection ->
                    // probers ain't interesting
                    if (connection.state == Connection.State.OUTGOING_CONNECTED)
                        persistent.peers.add(connection.remoteAddress)
                    connection.close()
                }
            }
            logger.info { "Saving node state" }
            replaceFile(stateDir, DATA_FILENAME) {
                writeInt(DATA_VERSION)
                binaryFormat.encodeToStream(Persistent.serializer(), persistent, this)
            }
        }
    }

    fun newPeerId(): Long {
        return nextPeerId.getAndIncrement()
    }

    private fun nonce(network: Network): Long = when (network) {
        Network.IPv4, Network.IPv6 -> nonce
        else -> Random.nextLong()
    }

    fun outgoing(): Int {
        return connections.count {
            when (it.state) {
                Connection.State.OUTGOING_CONNECTED -> true
                else -> false
            }
        }
    }

    fun incoming(includeWaiting: Boolean = false): Int {
        return if (includeWaiting)
            connections.count {
                when (it.state) {
                    Connection.State.INCOMING_CONNECTED -> true
                    Connection.State.INCOMING_WAITING -> true
                    else -> false
                }
            }
        else
            connections.count {
                when (it.state) {
                    Connection.State.INCOMING_CONNECTED -> true
                    else -> false
                }
            }
    }

    fun connected(): Int {
        return connections.count {
            when (it.state) {
                Connection.State.INCOMING_CONNECTED -> true
                Connection.State.OUTGOING_CONNECTED -> true
                else -> false
            }
        }
    }

    fun isOffline(): Boolean {
        return connections.find { it.state.isConnected() } == null
    }

    fun getListenAddress(): Set<Address> {
        return listenAddress
    }

    fun addListenAddress(address: Address) {
        if (listenAddress.add(address))
            PeerDB.contacted(address)
    }

    fun removeListenAddress(address: Address) {
        if (listenAddress.remove(address))
            PeerDB.discontacted(address)
    }

    fun getMaxPacketSize(): Int {
        return LedgerDB.state().maxBlockSize + PoS.BLOCK_RESERVED_SIZE
    }

    fun getMinPacketSize(): Int {
        return PoS.DEFAULT_MAX_BLOCK_SIZE + PoS.BLOCK_RESERVED_SIZE
    }

    fun isInitialSynchronization(): Boolean {
        return ChainFetcher.isSynchronizing() && PoS.guessInitialSynchronization()
    }

    fun listenOn(address: Address): Thread {
        val addr = when (address.network) {
            Network.IPv4, Network.IPv6 -> address.getInetAddress()
            else -> throw NotImplementedError("Not implemented for " + address.network)
        }
        val server = ServerSocket(address.port.toJava(), Kernel.config().incomingconnections, addr)
        logger.info { "Listening on ${address.debugName()}" }
        return startInterruptible("Node::listener ${address.debugName()}") {
            addListenAddress(address)
            try {
                listener(server)
            } catch (e: SocketException) {
                if (!Thread.currentThread().isInterrupted())
                    logger.error(e) { "Lost binding to ${address.debugName()}" }
            } finally {
                removeListenAddress(address)
            }
        }
    }

    private fun listenOnIP(): Thread {
        if (router.isDisabled(Network.IPv4) && router.isDisabled(Network.IPv6))
            throw IllegalStateException("Both IPv4 and IPv6 are disabled")
        if (router.isDisabled(Network.IPv4))
            return listenOn(Address.IPv6_ANY(Kernel.config().port))
        return listenOn(Address.IPv4_ANY(Kernel.config().port))
    }

    fun connectTo(address: Address, v2: Boolean, prober: Boolean = false): Connection {
        val connection = router.connect(address, prober)
        synchronized(connections) {
            connections.add(connection)
            connection.launch()
        }
        if (v2)
            sendHandshake(connection)
        else
            sendVersion(connection, nonce(address.network), prober)
        return connection
    }

    fun sendVersion(connection: Connection, nonce: Long, prober: Boolean) {
        connection.sendPacket(PacketType.Version, if (prober) {
            Version(
                    mode.networkMagic,
                    PROTOCOL_VERSION,
                    currentTimeSeconds(),
                    nonce,
                    UserAgent.prober,
                    Long.MAX_VALUE,
                    ChainAnnounce.GENESIS
            )
        } else {
            val state = LedgerDB.state()
            Version(
                    mode.networkMagic,
                    PROTOCOL_VERSION,
                    currentTimeSeconds(),
                    nonce,
                    UserAgent.string,
                    Kernel.txPool().minFeeRate,
                    ChainAnnounce(state.blockHash, state.cumulativeDifficulty)
            )
        })
    }

    fun sendHandshake(connection: Connection) {
        val hello = Hello().apply {
            magic = mode.networkMagic
            version = PROTOCOL_VERSION
            if (connection.state == Connection.State.OUTGOING_WAITING)
                nonce = nonce(connection.remoteAddress.network) //TODO send only when needed
            agent = if (connection.state == Connection.State.PROBER_WAITING)
                UserAgent.prober
            else
                UserAgent.string
            feeFilter = if (connection.state == Connection.State.PROBER_WAITING)
                Long.MAX_VALUE
            else
                Kernel.txPool().minFeeRate
        }
        connection.sendPacket(PacketType.Hello, hello)
        if (connection.state != Connection.State.PROBER_WAITING) {
            val state = LedgerDB.state()
            connection.sendPacket(PacketType.ChainAnnounce, ChainAnnounce(state.blockHash, state.cumulativeDifficulty))
        }
    }

    fun announceChain(hash: Hash, cumulativeDifficulty: BigInteger, source: Connection? = null): Int {
        Staker.awaitsNextTimeSlot?.interrupt()
        val ann = ChainAnnounce(hash, cumulativeDifficulty)
        return broadcastPacket(PacketType.ChainAnnounce, ann) {
            it != source && it.state.isConnected() && it.lastChain.cumulativeDifficulty < cumulativeDifficulty
        }
    }

    fun broadcastBlock(hash: Hash, bytes: ByteArray): Boolean {
        val (status, n) = ChainFetcher.stakedBlock(hash, bytes)
        if (status == Accepted) {
            if (mode.requiresNetwork)
                logger.info { "Announced to $n peers" }
            return true
        } else {
            logger.info { status.toString() }
            return false
        }
    }

    fun broadcastTx(hash: Hash, bytes: ByteArray): Status {
        val currTime = currentTimeSeconds()
        val (status, fee) = Kernel.txPool().process(hash, bytes, currTime, false)
        if (status == Accepted) {
            connections.forEach {
                if (it.state.isConnected() && it.checkFeeFilter(bytes.size, fee))
                    it.inventory(hash)
            }
        }
        return status
    }

    fun broadcastInv(unfiltered: UnfilteredInvList, source: Connection? = null): Int {
        var n = 0
        val toSend = ArrayList<Hash>(unfiltered.size)
        connections.forEach {
            if (it != source && it.state.isConnected()) {
                for (i in 0 until unfiltered.size) {
                    val (hash, size, fee) = unfiltered[i]
                    if (it.checkFeeFilter(size, fee))
                        toSend.add(hash)
                }
                if (toSend.size != 0) {
                    it.inventory(toSend)
                    toSend.clear()
                    n += 1
                }
            }
        }
        return n
    }

    private fun timeOffset(): Long =
        Kernel.config().outgoingconnections.let { min ->
            connections.fold(
                ArrayList<Long>(min)
            ) { accumulator, element ->
                accumulator.apply {
                    if (element.state == Connection.State.OUTGOING_CONNECTED)
                        add(element.timeOffset)
                }
            }.run {
                if (size >= min) {
                    sort()
                    this[size / 2] // median
                } else {
                    0
                }
            }
        }

    fun warnings(): List<String> {
        val timeOffset = timeOffset()

        return if (timeOffset >- PoS.TIME_SLOT && timeOffset <+ PoS.TIME_SLOT)
            emptyList()
        else
            listOf("Please check your system clock. Many peers report different time.")
    }

    private inline fun broadcastPacket(type: PacketType, packet: Packet, filter: (Connection) -> Boolean = { true }): Int {
        var n = 0
        connections.forEach {
            if (filter(it)) {
                it.sendPacket(type, packet)
                n += 1
            }
        }
        return n
    }

    private fun listener(server: ServerSocket) {
        while (true) {
            val socket = server.accept()
            val remoteAddress = Network.address(socket.remoteSocketAddress as InetSocketAddress)
            val localAddress = Network.address(socket.localSocketAddress as InetSocketAddress)
            if (!localAddress.isLocal())
                addListenAddress(localAddress)
            val connection = Connection(socket, remoteAddress, localAddress, Connection.State.INCOMING_WAITING)
            addIncomingConnection(connection)
        }
    }

    fun addIncomingConnection(connection: Connection) {
        synchronized(connections) {
            if (!haveSlot()) {
                logger.info { "Too many connections, dropping ${connection.debugName()}" }
                connection.close()
                return
            }
            connections.add(connection)
            connection.launch()
        }
    }

    private fun haveSlot(): Boolean {
        return if (incoming(true) < Kernel.config().incomingconnections)
            true
        else
            evictConnection()
    }

    private fun evictConnection(): Boolean {
        val candidates = connections.asSequence().filter { it.state.isIncoming() }
                .sortedBy { if (it.ping != 0L) it.ping else Long.MAX_VALUE }.drop(4)
                .sortedByDescending { it.lastTxTime }.drop(4)
                .sortedByDescending { it.lastBlockTime }.drop(4)
                .sortedBy { it.connectedAt }.drop(4)
                .toMutableList()

        //TODO network groups

        if (candidates.isEmpty())
            return false

        val connection = candidates.random()
        logger.info { "Evicting ${connection.debugName()}" }
        connection.close()
        return true
    }

    private fun connector() {
        val address = queuedPeers.poll()
            ?.let { if (PeerDB.tryContact(it)) it else null }
            ?: PeerDB.getCandidate { _, _ -> true }
        if (address == null) {
            val outgoing = outgoing()
            logger.info { "Don't have candidates in PeerDB. $outgoing connections, max ${Kernel.config().outgoingconnections}" }
            sleep(15 * 60 * 1000L)
            return
        }

        val time = currentTimeMillis()
        var connection: Connection? = null
        try {
            connection = connectTo(address, v2 = true)
            connection.join()
            // try v1 if accepted without reply
            if (connection.getTotalBytesRead() == 0L) {
                connection = connectTo(address, v2 = false)
                connection.join()
            }
        } catch (e: Throwable) {
        } finally {
            if (connection == null || connection.state == Connection.State.OUTGOING_WAITING)
                PeerDB.failed(address, time / 1000L)
            PeerDB.discontacted(address)
        }

        val x = 4 * 1000L - (currentTimeMillis() - time)
        if (x > 0L)
            sleep(x) // 請在繼續之前等待或延遲
    }

    /**
     * Peer network prober is a tool that periodically tries to establish an
     * outgoing connection in order to provide statistic for the peer database
     * prober.
     */
    private fun prober() {
        sleep(4 * 60 * 1000L)

        // Await peer network address announce
        if (PeerDB.size() < PeerDB.MAX_SIZE / 2)
            return

        // Await while connectors are working
        if (outgoing() < Kernel.config().outgoingconnections)
            return

        val time = currentTimeMillis()
        val address = PeerDB.getCandidate { _, entry ->
            (time / 1000L > entry.lastTry + 4 * 60 * 60)
        } ?: return

        var connection: Connection? = null
        try {
            connection = connectTo(address, v2 = true, prober = true)
            connection.join()
            // try v1 if accepted without reply
            if (connection.getTotalBytesRead() == 0L) {
                connection = connectTo(address, v2 = false, prober = true)
                connection.join()
            }
        } catch (e: Throwable) {
        } finally {
            if (connection == null || connection.state == Connection.State.PROBER_WAITING)
                PeerDB.failed(address, time / 1000L)
            PeerDB.discontacted(address)
        }
    }

    @Serializable
    private class Persistent(
        val peers: ArrayList<Address>
    )
}
