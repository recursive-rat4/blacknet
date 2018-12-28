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
import io.ktor.network.sockets.openReadChannel
import io.ktor.network.sockets.openWriteChannel
import io.ktor.util.error
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.io.ByteReadChannel
import kotlinx.coroutines.io.ByteWriteChannel
import kotlinx.coroutines.io.readUTF8Line
import kotlinx.coroutines.io.writeStringUtf8
import kotlinx.coroutines.launch
import mu.KotlinLogging
import net.i2p.data.Destination
import ninja.blacknet.Config
import ninja.blacknet.Config.i2psamhost
import ninja.blacknet.Config.i2psamport
import ninja.blacknet.Config.port
import java.io.File
import kotlin.coroutines.CoroutineContext
import kotlin.random.Random

private val logger = KotlinLogging.logger {}

object I2PSAM : CoroutineScope {
    override val coroutineContext: CoroutineContext = Dispatchers.Default
    private val selector = ActorSelectorManager(Dispatchers.IO)
    private val sessionId = generateId()
    private var privateKey = "TRANSIENT"
    val sam: Address?
    var localAddress: Address? = null

    init {
        if (Config.contains(i2psamhost) && Config.contains(i2psamport))
            sam = Network.resolve(Config[i2psamhost], Config[i2psamport])
        else
            sam = null

        try {
            privateKey = File("db/privateKey.i2p").readText()
        } catch (e: Throwable) {
        }
    }

    fun haveSession(): Boolean {
        return localAddress != null
    }

    private suspend fun connectToSAM(): Pair<ByteReadChannel, ByteWriteChannel> {
        val socket = aSocket(selector).tcp().connect(sam!!.getSocketAddress())
        val readChannel = socket.openReadChannel()
        val writeChannel = socket.openWriteChannel(true)

        val answer = request(readChannel, writeChannel, "HELLO VERSION MIN=3.2\n")
        checkResult(answer)

        return Pair(readChannel, writeChannel)
    }

    suspend fun createSession() {
        val channels = connectToSAM()

        val answer = request(channels.first, channels.second, "SESSION CREATE STYLE=STREAM ID=$sessionId DESTINATION=$privateKey SIGNATURE_TYPE=EdDSA_SHA512_Ed25519\n")
        checkResult(answer)

        val dest = getValue(answer, "DESTINATION") ?: throw I2PException("Invalid response")
        val bytes = Destination(dest).getHash().getData()
        localAddress = Address(Network.I2P, Config[port], bytes)

        if (privateKey == "TRANSIENT")
            savePrivateKey(dest)

        launch {
            while (true) {
                val message = channels.first.readUTF8Line() ?: break

                if (message.startsWith("PING")) {
                    channels.second.writeStringUtf8("PONG" + message.drop(4) + '\n')
                }
            }
        }
    }

    suspend fun connect(address: Address): Pair<ByteReadChannel, ByteWriteChannel> {
        val channels = connectToSAM()

        val destination = lookup(channels.first, channels.second, address.getAddressString())

        val answer = request(channels.first, channels.second, "STREAM CONNECT ID=$sessionId DESTINATION=$destination\n")
        checkResult(answer)

        return channels
    }

    suspend fun accept(): Accepted? {
        val channels = connectToSAM()

        val answer = request(channels.first, channels.second, "STREAM ACCEPT ID=$sessionId\n")
        try {
            checkResult(answer)
        } catch (e: Throwable) {
            logger.info("STREAM ACCEPT failed: ${e.message}")
            return null
        }

        while (true) {
            val message = channels.first.readUTF8Line() ?: break

            if (message.startsWith("PING")) {
                channels.second.writeStringUtf8("PONG" + message.drop(4) + '\n')
            } else {
                val bytes = Destination(message.takeWhile { it != ' ' }).getHash().getData()
                val remoteAddress = Address(Network.I2P, Config[port], bytes)
                return Accepted(channels.first, channels.second, remoteAddress)
            }
        }

        return null
    }

    private suspend fun lookup(readChannel: ByteReadChannel, writeChannel: ByteWriteChannel, name: String): String {
        val answer = request(readChannel, writeChannel, "NAMING LOOKUP NAME=$name\n")
        checkResult(answer)
        return getValue(answer, "VALUE")!!
    }

    private fun checkResult(answer: String?) {
        if (answer == null)
            throw I2PException("Connection closed")
        val result = getValue(answer, "RESULT")
        if (result == null)
            throw I2PException("No RESULT")
        if (result.isEmpty())
            throw I2PException("Empty RESULT")
        if (result != "OK") {
            val message = getValue(answer, "MESSAGE")
            if (message != null && !message.isEmpty())
                throw I2PException("$result: $message")
            else
                throw I2PException(result)
        }
    }

    private suspend fun request(readChannel: ByteReadChannel, writeChannel: ByteWriteChannel, request: String): String? {
        writeChannel.writeStringUtf8(request)
        return readChannel.readUTF8Line()
    }

    private fun generateId(): String {
        val size = 8
        val alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
        val builder = StringBuilder()
        for (i in 1..size) {
            val rnd = Random.nextInt(alphabet.length)
            builder.append(alphabet[rnd])
        }
        return builder.toString()
    }

    private fun getValue(answer: String?, key: String): String? {
        if (answer == null) return null
        val keyPattern = ' ' + key + '='
        val i = answer.indexOf(keyPattern)
        if (i == -1)
            return null
        val valueStart = i + keyPattern.length
        if (valueStart == answer.length)
            return String()
        if (answer[valueStart] == '"')
            return answer.substring(valueStart + 1, answer.indexOf('"', valueStart + 1))
        val valueEnd = answer.indexOf(' ', valueStart)
        if (valueEnd == -1)
            return answer.substring(valueStart)
        return answer.substring(valueStart, valueEnd)
    }

    class Accepted(val readChannel: ByteReadChannel, val writeChannel: ByteWriteChannel, val remoteAddress: Address)
    class I2PException(message: String) : RuntimeException(message)

    private fun savePrivateKey(dest: String) {
        privateKey = dest
        logger.info("Saving I2P destination to db")
        try {
            File("db/privateKey.i2p").writeText(privateKey)
        } catch (e: Throwable) {
            logger.error(e)
        }
    }
}
