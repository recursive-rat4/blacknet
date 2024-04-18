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
import java.io.IOException
import java.lang.Thread.sleep
import java.math.BigInteger
import java.net.Socket
import java.util.concurrent.LinkedBlockingQueue
import kotlin.random.Random
import kotlinx.atomicfu.atomic
import kotlinx.serialization.KSerializer
import kotlinx.serialization.SerializationException
import ninja.blacknet.Kernel
import ninja.blacknet.crypto.Hash
import ninja.blacknet.db.PeerDB
import ninja.blacknet.io.buffered
import ninja.blacknet.io.counted
import ninja.blacknet.io.data
import ninja.blacknet.io.delimited
import ninja.blacknet.network.packet.*
import ninja.blacknet.serialization.bbf.binaryFormat
import ninja.blacknet.time.currentTimeMillis
import ninja.blacknet.time.currentTimeSeconds
import ninja.blacknet.util.startInterruptible

private val logger = KotlinLogging.logger {}

class Connection(
    private val socket: Socket,
    val remoteAddress: Address,
    val localAddress: Address,
    var state: State,
) {
    private val countedInput = socket.inputStream.counted()
    private val countedOutput = socket.outputStream.counted()
    private val delimitedInput = countedInput.buffered().delimited()
    private val inputStream = delimitedInput.data()
    private val outputStream = countedOutput.buffered().data()
    private val vThreads = ArrayList<Thread>()

    private val closed = atomic(false)
    private val dosScore = atomic(0)
    private val sendQueueSize = atomic(0L)
    private val sendQueue = LinkedBlockingQueue<QueuedPacket>()
    private val inventoryMonitor = Any()
    private var inventoryToSend = ArrayList<Hash>(Inventory.SEND_MAX)
    val connectedAt = currentTimeSeconds()

    @Volatile
    var lastPacketTime: Long = 0
    @Volatile
    var lastBlock: BlockAnnounce = BlockAnnounce.GENESIS
    @Volatile
    var lastBlockTime: Long = 0
    @Volatile
    var lastTxTime: Long = 0
    @Volatile
    var lastPingTime: Long = 0
    @Volatile
    var lastInvSentTime: Long = 0
    @Volatile
    var timeOffset: Long = 0
    @Volatile
    var ping: Long = 0
    @Volatile
    var pingRequest: Pair<Int, Long>? = null
    @Volatile
    var requestedDifficulty: BigInteger = BigInteger.ZERO

    fun getTotalBytesRead(): Long = countedInput.bytesRead
    fun getTotalBytesWritten(): Long = countedOutput.bytesWritten

    inline val requestedBlocks: Boolean
        get() = requestedDifficulty != BigInteger.ZERO

    val peerId: Long = Node.newPeerId()
    var version: Int = 0
    var agent: String = ""
    var feeFilter: Long = 0

    fun launch() {
        vThreads.add(startInterruptible("Connection::pinger ${debugName()}", ::pinger))
        vThreads.add(startInterruptible("Connection::whisperer ${debugName()}", ::whisperer))
        vThreads.add(startInterruptible("Connection::pusher ${debugName()}", ::pusher))
        vThreads.add(startInterruptible("Connection::receiver ${debugName()}", ::receiver))
        vThreads.add(startInterruptible("Connection::sender ${debugName()}", ::sender))
    }

    fun join() {
        vThreads.forEach(Thread::join)
    }

    private fun receiver() {
        try {
            while (true) {
                val size = inputStream.readInt()
                if (size > Node.getMaxPacketSize()) {
                    if (state.isConnected()) {
                        logger.info { "Too long packet $size max ${Node.getMaxPacketSize()} Disconnecting ${debugName()}" }
                    }
                    close()
                }
                delimitedInput.begin(size)
                val type = inputStream.readInt()
                val serializer = PacketType.getSerializer<Packet>(type)
                val packet = binaryFormat.decodeFromStream(serializer, inputStream, false)
                delimitedInput.end()

                if (state.isConnected()) {
                    if (type == PacketType.Version.ordinal || type == PacketType.Hello.ordinal)
                        break
                } else {
                    if (type != PacketType.Version.ordinal && type != PacketType.Hello.ordinal)
                        break
                }

                lastPacketTime = currentTimeMillis()
                logger.debug { "Received ${packet::class.simpleName} from ${debugName()}" }
                packet.handle(this)
            }
        } catch (e: SerializationException) {
            logger.debug(e) { "Exception in receiver ${debugName()}" }
        } catch (e: IOException) {
            logger.debug(e) { "Exception in receiver ${debugName()}" }
        } finally {
            close()
        }
    }

    private fun sender() {
        try {
            while (true) {
                val e = sendQueue.take()
                logger.debug { "Sending ${e.type} to ${debugName()}" }
                outputStream.writeInt(e.size + PACKET_HEADER_SIZE_BYTES)
                outputStream.writeInt(e.type.ordinal)
                binaryFormat.encodeToStream(e.serializer, e.packet, outputStream)
                outputStream.flush()
                sendQueueSize -= e.size.toLong()
            }
        } catch (e: SerializationException) {
            logger.error(e) { "Exception in sender ${debugName()}" }
        } catch (e: IOException) {
            logger.debug(e) { "Exception in sender ${debugName()}" }
        } finally {
            close()
        }
    }

    fun inventory(inv: Hash) = synchronized(inventoryMonitor) {
        inventoryToSend.add(inv)
        if (inventoryToSend.size == Inventory.SEND_MAX) {
            sendInventoryImpl(currentTimeMillis())
        }
    }

    fun inventory(inv: ArrayList<Hash>): Unit = synchronized(inventoryMonitor) {
        val newSize = inventoryToSend.size + inv.size
        if (newSize < Inventory.SEND_MAX) {
            inventoryToSend.addAll(inv)
        } else if (newSize > Inventory.SEND_MAX) {
            val n = Inventory.SEND_MAX - inventoryToSend.size
            for (i in 0 until n)
                inventoryToSend.add(inv[i])
            sendInventoryImpl(currentTimeMillis())
            for (i in n until inv.size)
                inventoryToSend.add(inv[i])
        } else {
            inventoryToSend.addAll(inv)
            sendInventoryImpl(currentTimeMillis())
        }
    }

    private fun sendInventory(time: Long) = synchronized(inventoryMonitor) {
        if (inventoryToSend.size != 0) {
            sendInventoryImpl(time)
        }
    }

    private fun sendInventoryImpl(time: Long) {
        sendPacket(PacketType.Inventory, Inventory(inventoryToSend))
        inventoryToSend = ArrayList(Inventory.SEND_MAX)
        lastInvSentTime = time
    }

    fun sendPacket(type: PacketType, packet: Packet) {
        val serializer = PacketType.getSerializer<Packet>(type.ordinal)
        val size = binaryFormat.computeSize(serializer, packet)
        //TODO review threshold
        if (sendQueueSize.addAndGet(size.toLong()) <= Node.getMaxPacketSize() * 10) {
            sendQueue.offer(QueuedPacket(serializer, packet, type, size))
        } else {
            logger.info { "Disconnecting ${debugName()} on send queue overflow" }
            close()
        }
    }

    fun dos(reason: String) {
        val score = dosScore.incrementAndGet()
        if (score == 100)
            close()
        logger.info { "$reason ${debugName()} DoS $score" }
    }

    fun dosScore(): Int {
        return dosScore.value
    }

    fun close() {
        if (closed.compareAndSet(false, true)) {
            socket.close()
            inputStream.close()
            outputStream.close()

            synchronized(Node.connections) {
                Node.connections.remove(this@Connection)
            }

            when (state) {
                State.INCOMING_CONNECTED, State.OUTGOING_CONNECTED -> {
                    BlockFetcher.disconnected(this@Connection)
                }
                State.OUTGOING_WAITING, State.PROBER_WAITING -> {
                }
                State.INCOMING_WAITING, State.PROBER_CONNECTED -> {
                }
            }

            vThreads.forEach(Thread::interrupt)
        }
    }

    fun isClosed(): Boolean {
        return closed.value
    }

    @Suppress("UNUSED_PARAMETER")
    fun checkFeeFilter(size: Int, fee: Long): Boolean {
        //FIXME use size
        return feeFilter <= fee
    }

    fun debugName(): String {
        return if (Kernel.config().logips)
            remoteAddress.toString()
        else
            "peer $peerId"
    }

    enum class State {
        INCOMING_CONNECTED,
        INCOMING_WAITING,
        OUTGOING_CONNECTED,
        OUTGOING_WAITING,
        PROBER_CONNECTED,
        PROBER_WAITING,
        ;

        fun isConnected(): Boolean {
            return this == INCOMING_CONNECTED || this == OUTGOING_CONNECTED
                    // this == PROBER_CONNECTED
        }

        fun isIncoming(): Boolean {
            return this == INCOMING_CONNECTED || this == INCOMING_WAITING
        }

        fun isOutgoing(): Boolean {
            return this == OUTGOING_CONNECTED || this == OUTGOING_WAITING
                    || this == PROBER_CONNECTED || this == PROBER_WAITING
        }
    }

    private fun pinger() {
        sleep(Node.NETWORK_TIMEOUT)

        if (state.isConnected()) {
            sleep(Random.nextLong(Node.NETWORK_TIMEOUT))
            pingPong()
        } else {
            close()
            return
        }

        while (true) {
            val currTime = currentTimeMillis()
            val nextPing = lastPacketTime + Node.NETWORK_TIMEOUT
            val d = nextPing - currTime
            if (d > 0) {
                sleep(d)
                continue
            } else {
                pingPong()
            }
        }
    }

    private fun pingPong() {
        sendPing()
        sleep(Node.NETWORK_TIMEOUT)
        if (pingRequest == null)
            return
        logger.info { "Disconnecting ${debugName()} on ping timeout" }
        close()
    }

    private fun sendPing() {
        val challenge = Random.nextInt()
        pingRequest = Pair(challenge, currentTimeMillis())
        if (version >= Ping.MIN_VERSION)
            sendPacket(PacketType.Ping, Ping(challenge, currentTimeSeconds()))
        else
            sendPacket(PacketType.PingV1, PingV1(challenge))
    }

    private fun whisperer() {
        sleep(Random.nextLong(10 * 60 * 1000L, 20 * 60 * 1000L))

        while (true) {
            val n = Random.nextInt(Peers.MAX + 1)
            val randomPeers = PeerDB.getRandom(n)
            if (randomPeers.size > 0)
                sendPacket(PacketType.Peers, Peers(randomPeers))

            sleep(Random.nextLong(4 * 60 * 60 * 1000L, 20 * 60 * 60 * 1000L))
        }
    }

    private fun pusher() {
        while (!state.isConnected()) {
            sleep(Inventory.SEND_TIMEOUT)
        }
        while (true) {
            val currTime = currentTimeMillis()
            if (currTime >= lastInvSentTime + Inventory.SEND_TIMEOUT) {
                sendInventory(currTime)
            }
            sleep(Inventory.SEND_TIMEOUT)
        }
    }

    private class QueuedPacket(
        val serializer: KSerializer<Packet>,
        val packet: Packet,
        val type: PacketType,
        val size: Int,
    )
}
