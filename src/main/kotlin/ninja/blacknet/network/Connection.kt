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
import mu.KotlinLogging

private val logger = KotlinLogging.logger {}

class Connection(private val socket: Socket, var state: State) {
    val remoteAddress = socket.remoteAddress

    var timeOffset: Long = 0
    var pingRequest: PingRequest? = null
    var ping: Long? = null
    private var dosScore: Int = 0

    suspend fun loop() {
        //TODO
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
    }
}