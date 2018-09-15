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
import io.ktor.util.error
import kotlinx.coroutines.experimental.channels.ClosedReceiveChannelException
import kotlinx.coroutines.experimental.channels.LinkedListChannel
import kotlinx.coroutines.experimental.io.readPacket
import kotlinx.coroutines.experimental.launch
import kotlinx.io.core.BytePacketBuilder
import kotlinx.io.core.ByteReadPacket
import mu.KotlinLogging
import ninja.blacknet.serialization.BlacknetInput

private val logger = KotlinLogging.logger {}

class Connection(private val socket: Socket, val remoteAddress: Address, var state: State) {
    private val readChannel = socket.openReadChannel()
    private val writeChannel = socket.openWriteChannel(true)
    private val sendChannel = LinkedListChannel<ByteReadPacket>()
    val connectedAt = Node.time()

    var timeOffset: Long = 0
    var pingRequest: PingRequest? = null
    var ping: Long? = null
    private var dosScore: Int = 0

    init {
        launch { receiver() }
        launch { sender() }
    }

    private suspend fun receiver() {
        try {
            while (true) {
                val len = readChannel.readInt()
                val bytes = readChannel.readPacket(len)
                val type = bytes.readInt()

                if ((state.isWaiting() && type != 0) || (state.isConnected() && type == 0))
                    break

                val serializer = PacketType.getSerializer(type)
                if (serializer == null) {
                    logger.info("unknown packet type $type")
                    continue
                }
                val packet = BlacknetInput(bytes).read(serializer)
                if (bytes.remaining > 0) {
                    bytes.release()
                    dos(1, "trailing data in packet")
                    continue
                }
                packet.process(this)
            }
        } catch (e: ClosedReceiveChannelException) {
        } catch (e: Throwable) {
            logger.error(e)
        } finally {
            close()
            Node.disconnected(this)
        }
    }

    private suspend fun sender() {
        try {
            for (packet in sendChannel)
                writeChannel.writePacket(packet)
        } catch (e: Throwable) {
            logger.error(e)
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

    fun dos(score: Int, reason: String) {
        dosScore += score
        logger.warn("DoS: $dosScore $reason ${socket.remoteAddress}")
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