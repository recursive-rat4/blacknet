/*
 * Copyright (c) 2018-2019 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.network

import io.ktor.network.sockets.ASocket
import io.ktor.utils.io.*
import io.ktor.utils.io.core.ByteReadPacket
import io.ktor.utils.io.core.readInt
import io.ktor.utils.io.errors.IOException
import kotlinx.atomicfu.atomic
import kotlinx.coroutines.*
import kotlinx.coroutines.CancellationException
import kotlinx.coroutines.channels.Channel
import kotlinx.coroutines.channels.ClosedReceiveChannelException
import kotlinx.coroutines.sync.withLock
import mu.KotlinLogging
import ninja.blacknet.Config
import ninja.blacknet.Runtime
import ninja.blacknet.crypto.Hash
import ninja.blacknet.db.PeerDB
import ninja.blacknet.packet.*
import ninja.blacknet.time.SystemClock
import ninja.blacknet.time.delay
import ninja.blacknet.time.milliseconds.MilliSeconds
import ninja.blacknet.time.milliseconds.hours
import ninja.blacknet.time.milliseconds.minutes
import ninja.blacknet.time.milliseconds.nextTime
import ninja.blacknet.util.SynchronizedArrayList
import kotlin.coroutines.CoroutineContext
import kotlin.random.Random

private val logger = KotlinLogging.logger {}

class Connection(
        private val socket: ASocket,
        private val readChannel: ByteReadChannel,
        private val writeChannel: ByteWriteChannel,
        val remoteAddress: Address,
        val localAddress: Address,
        var state: State
) : CoroutineScope {
    val job = Job()
    override val coroutineContext: CoroutineContext = Runtime.coroutineContext + job

    private val closed = atomic(false)
    private val dosScore = atomic(0)
    private val sendChannel: Channel<ByteReadPacket> = Channel(Channel.UNLIMITED)
    private val inventoryToSend = SynchronizedArrayList<Hash>(Inventory.SEND_MAX)
    val connectedAt = SystemClock.seconds

    @Volatile
    var lastPacketTime: MilliSeconds = MilliSeconds.ZERO
    @Volatile
    var totalBytesRead: Long = 0
    @Volatile
    var totalBytesWritten: Long = 0
    @Volatile
    var lastChain: ChainAnnounce = ChainAnnounce.GENESIS
    @Volatile
    var lastBlockTime: MilliSeconds = MilliSeconds.ZERO
    @Volatile
    var lastTxTime: MilliSeconds = MilliSeconds.ZERO
    @Volatile
    var lastInvSentTime: MilliSeconds = MilliSeconds.ZERO
    @Volatile
    var ping: MilliSeconds = MilliSeconds.ZERO
    @Volatile
    internal var pingRequest: Pair<Int, MilliSeconds>? = null
    @Volatile
    internal var requestedBlocks: Boolean = false

    var peerId: Long = 0
    var version: Int = 0
    var agent: String = ""
    var feeFilter: Long = 0
    var timeOffset: Long = 0

    fun launch() {
        launch { Pinger.implementation(this@Connection) }
        launch { peerAnnouncer() }
        launch { inventoryBroadcaster() }
        launch { receiver() }
        launch { sender() }
    }

    private suspend fun receiver() {
        try {
            while (true) {
                val bytes = recvPacket()
                val type = bytes.readInt()

                if (state.isConnected()) {
                    if (type == PacketType.Version.ordinal)
                        break
                } else {
                    if (type != PacketType.Version.ordinal)
                        break
                }

                val packet = try {
                    Packet.deserialize(type, bytes)
                } catch (e: Throwable) {
                    dos("Deserialization failed: ${e.message}")
                    continue
                }
                logger.debug { "Received ${packet.getType()} from ${debugName()}" }
                packet.process(this)
            }
        } catch (e: ClosedReceiveChannelException) {
        } catch (e: CancellationException) {
        } catch (e: IOException) {
        } catch (e: Throwable) {
            logger.error("Exception in receiver ${debugName()}", e)
        } finally {
            close()
        }
    }

    private suspend fun recvPacket(): ByteReadPacket {
        val size = readChannel.readInt()
        if (size > Node.getMaxPacketSize()) {
            if (state.isConnected()) {
                logger.info("Too long packet $size max ${Node.getMaxPacketSize()} Disconnecting ${debugName()}")
            }
            close()
        }
        val result = readChannel.readPacket(size, 0)
        lastPacketTime = SystemClock.milliseconds
        totalBytesRead += size + 4
        return result
    }

    private suspend fun sender() {
        try {
            for (packet in sendChannel) {
                val size = packet.remaining
                writeChannel.writePacket(packet)
                totalBytesWritten += size
            }
        } catch (e: ClosedWriteChannelException) {
        } catch (e: CancellationException) {
        } catch (e: Throwable) {
            logger.error("Exception in sender ${debugName()}", e)
        } finally {
            close()
        }
    }

    suspend fun inventory(inv: Hash) = inventoryToSend.mutex.withLock {
        inventoryToSend.list.add(inv)
        if (inventoryToSend.list.size == Inventory.SEND_MAX) {
            sendInventoryImpl(SystemClock.milliseconds)
        }
    }

    suspend fun inventory(inv: ArrayList<Hash>): Unit = inventoryToSend.mutex.withLock {
        val newSize = inventoryToSend.list.size + inv.size
        if (newSize < Inventory.SEND_MAX) {
            inventoryToSend.list.addAll(inv)
        } else if (newSize > Inventory.SEND_MAX) {
            val n = Inventory.SEND_MAX - inventoryToSend.list.size
            for (i in 0 until n)
                inventoryToSend.list.add(inv[i])
            sendInventoryImpl(SystemClock.milliseconds)
            for (i in n until inv.size)
                inventoryToSend.list.add(inv[i])
        } else {
            inventoryToSend.list.addAll(inv)
            sendInventoryImpl(SystemClock.milliseconds)
        }
    }

    private suspend fun sendInventory(time: MilliSeconds) = inventoryToSend.mutex.withLock {
        if (inventoryToSend.list.size != 0) {
            sendInventoryImpl(time)
        }
    }

    private fun sendInventoryImpl(time: MilliSeconds) {
        sendPacket(Inventory(inventoryToSend.list))
        inventoryToSend.list.clear()
        lastInvSentTime = time
    }

    fun sendPacket(packet: Packet) {
        logger.debug { "Sending ${packet.getType()} to ${debugName()}" }
        sendChannel.offer(packet.build())
    }

    internal fun sendPacket(bytes: ByteReadPacket) {
        sendChannel.offer(bytes)
    }

    fun dos(reason: String) {
        val score = dosScore.incrementAndGet()
        if (score == 100)
            close()
        logger.info("$reason ${debugName()} DoS $score")
    }

    fun dosScore(): Int {
        return dosScore.value
    }

    fun close() {
        if (closed.compareAndSet(false, true)) {
            socket.close()
            readChannel.cancel()
            writeChannel.close()
            Runtime.launch {
                Node.connections.remove(this@Connection)

                when (state) {
                    State.INCOMING_CONNECTED, State.OUTGOING_CONNECTED -> {
                        ChainFetcher.disconnected(this@Connection)
                    }
                    State.OUTGOING_WAITING -> {
                        PeerDB.failed(remoteAddress, connectedAt)
                    }
                    State.INCOMING_WAITING -> {
                    }
                }

                cancel()
                sendChannel.cancel()
                job.cancel()
            }
        }
    }

    fun isClosed(): Boolean {
        return closed.value
    }

    fun debugName(): String {
        return if (Config.instance.logips)
            remoteAddress.toString()
        else
            "peer $peerId"
    }

    enum class State {
        INCOMING_CONNECTED,
        INCOMING_WAITING,
        OUTGOING_CONNECTED,
        OUTGOING_WAITING;

        fun isConnected(): Boolean {
            return this == INCOMING_CONNECTED || this == OUTGOING_CONNECTED
        }

        fun isIncoming(): Boolean {
            return this == INCOMING_CONNECTED || this == INCOMING_WAITING
        }

        fun isOutgoing(): Boolean {
            return this == OUTGOING_CONNECTED || this == OUTGOING_WAITING
        }
    }

    private suspend fun peerAnnouncer() {
        delay(Random.nextTime(10.minutes, 20.minutes))

        while (true) {
            val n = Random.nextInt(Peers.MAX) + 1

            val randomPeers = PeerDB.getRandom(n)
            if (randomPeers.size == 0)
                continue

            val myAddress = Node.listenAddress.filterToList { !it.isLocal() && !it.isPrivate() && !PeerDB.contains(it) }
            if (myAddress.size != 0) {
                val i = Random.nextInt(randomPeers.size * 4)
                if (i < randomPeers.size) {
                    randomPeers[i] = myAddress[Random.nextInt(myAddress.size)]
                    logger.info("Whispering ${randomPeers[i].debugName()} to ${debugName()}")
                }
            }

            sendPacket(Peers(randomPeers))

            delay(Random.nextTime(4.hours, 20.hours))
        }
    }

    private suspend fun inventoryBroadcaster() {
        while (!state.isConnected()) {
            delay(Inventory.SEND_TIMEOUT)
        }
        while (true) {
            val currTime = SystemClock.milliseconds
            if (currTime >= lastInvSentTime + Inventory.SEND_TIMEOUT) {
                sendInventory(currTime)
            }
            delay(Inventory.SEND_TIMEOUT)
        }
    }
}
