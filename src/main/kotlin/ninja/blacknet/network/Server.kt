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
import io.ktor.network.sockets.aSocket
import io.ktor.network.util.ioCoroutineDispatcher
import kotlinx.coroutines.experimental.launch
import kotlinx.coroutines.experimental.runBlocking
import mu.KotlinLogging
import java.net.InetSocketAddress
import java.time.Instant
import java.util.*

private val logger = KotlinLogging.logger {}

object Server {
    const val magic = 0x17895E7D
    const val version = 5
    const val minVersion = 5
    const val agent = "Blacknet"
    val nonce = Random().nextLong()

    fun time(): Long {
        return Instant.now().getEpochSecond()
    }

    fun start() {
        runBlocking {
            val server = aSocket(ActorSelectorManager(ioCoroutineDispatcher)).tcp().bind(InetSocketAddress("127.0.0.1", 28453))
            logger.info("Started server at ${server.localAddress}")

            while (true) {
                val socket = server.accept()
                val connection = Connection(socket, Connection.State.INCOMING_WAITING)
                launch {
                    connection.loop()
                }
            }
        }
    }
}