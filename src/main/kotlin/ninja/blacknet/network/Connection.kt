/*
 * Copyright (c) 2018 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.network

import io.ktor.network.sockets.Socket
import io.ktor.network.sockets.openReadChannel
import io.ktor.network.sockets.openWriteChannel
import kotlinx.coroutines.experimental.CompletionHandler
import kotlinx.coroutines.experimental.Job
import kotlinx.coroutines.experimental.JobCancellationException
import kotlinx.coroutines.experimental.channels.ClosedReceiveChannelException
import kotlinx.coroutines.experimental.channels.LinkedListChannel
import kotlinx.coroutines.experimental.io.readPacket
import kotlinx.coroutines.experimental.launch
import kotlinx.io.IOException
import kotlinx.io.core.BytePacketBuilder
import kotlinx.io.core.ByteReadPacket
import mu.KotlinLogging
import ninja.blacknet.serialization.BlacknetInput

private val logger = KotlinLogging.logger {}

class Connection(private val socket: Socket, val remoteAddress: Address, var state: State) {
    private val job: Job
    private val readChannel = socket.openReadChannel()
    private val writeChannel = socket.openWriteChannel(true)
    private val sendChannel = LinkedListChannel<ByteReadPacket>()
    val connectedAt = Node.time()

    var version: Int = 0
    var agent: String = ""
    var timeOffset: Long = 0
    var pingRequest: PingRequest? = null
    var ping: Long = 0
    var dosScore: Int = 0

    init {
        job = launch { receiver() }
        launch { sender() }
    }

    fun invokeOnDisconnect(handler: CompletionHandler) {
        job.invokeOnCompletion(true, true,  handler)
    }

    private suspend fun receiver() {
        try {
            while (true) {
                val bytes = recvPacket()
                val type = bytes.readInt()

                if ((state.isWaiting() && type != 0) || (state.isConnected() && type == 0))
                    break

                val serializer = PacketType.getSerializer(type)
                if (serializer == null) {
                    logger.info("unknown packet type $type")
                    continue
                }
                val packet = BlacknetInput(bytes).deserialize(serializer)
                if (packet == null) {
                    dos("deserialization failed")
                    continue
                }
                packet.process(this)
            }
        } catch (e: ClosedReceiveChannelException) {
        } catch (e: JobCancellationException) {
        } catch (e: Throwable) {
            logger.error("Exception in receiver $remoteAddress", e)
        } finally {
            close()
        }
    }

    private suspend fun recvPacket(): ByteReadPacket {
        try {
            val size = readChannel.readInt()
            if (size > Node.DEFAULT_MAX_PACKET_SIZE) {
                logger.info("Too long packet $size max ${Node.DEFAULT_MAX_PACKET_SIZE} Disconnecting $remoteAddress")
                close()
            }
            return readChannel.readPacket(size)
        } catch (e: IOException) {
            throw ClosedReceiveChannelException(e.message)
        }
    }

    private suspend fun sender() {
        try {
            for (packet in sendChannel)
                writeChannel.writePacket(packet)
        } catch (e: Throwable) {
            logger.error("Exception in sender $remoteAddress",e)
        } finally {
            close()
        }
    }

    fun sendPacket(p: Packet) {
        val s = p.serialize()
        val b = BytePacketBuilder()
        b.writeInt(s.remaining.toInt() + 4)
        b.writeInt(p.getType())
        b.writePacket(s)
        sendChannel.offer(b.build())
    }

    fun dos(reason: String) {
        dosScore++
        logger.info("DoS: $dosScore $reason $remoteAddress")
        if (dosScore >= 100)
            close()
    }

    fun close() {
        socket.close()
    }

    class PingRequest(val id: Int, val time: Long)

    enum class State {
        INCOMING_WAITING,
        INCOMING_CONNECTED,
        OUTGOING_WAITING,
        OUTGOING_CONNECTED;

        fun isConnected(): Boolean {
            return this == INCOMING_CONNECTED || this == OUTGOING_CONNECTED
        }

        fun isWaiting(): Boolean {
            return this == INCOMING_WAITING || this == OUTGOING_WAITING
        }
    }
}