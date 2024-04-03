/*
 * Copyright (c) 2018-2024 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.network

import com.google.common.base.Utf8
import io.github.oshai.kotlinlogging.KotlinLogging
import java.io.Closeable
import java.io.InputStream
import java.io.OutputStream
import java.net.Socket
import java.net.SocketException
import java.nio.file.Files
import java.nio.file.StandardCopyOption.ATOMIC_MOVE
import java.util.Arrays
import kotlin.io.path.readText
import kotlin.random.Random
import ninja.blacknet.Config
import ninja.blacknet.dataDir
import ninja.blacknet.codec.base.Base64
import ninja.blacknet.crypto.HashEncoder.Companion.buildHash
import ninja.blacknet.crypto.encodeByteArray
import ninja.blacknet.io.replaceFile
import ninja.blacknet.util.startInterruptible

private val logger = KotlinLogging.logger {}

// https://geti2p.net/en/docs/api/samv3
class I2PSAM(
    private val config: Config,
) {
    private var privateKey = "TRANSIENT"
    private val sam: Address?
    @Volatile
    private var session: Triple<String, Address, Closeable>? = null

    init {
        if (config.i2psamhost != null && config.i2psamport != null)
            sam = Network.resolve(config.i2psamhost, config.i2psamport)
        else
            sam = null

        try {
            val file = dataDir.resolve("privateKey.i2p")
            val lastModified = Files.getLastModifiedTime(file).toMillis()
            if (lastModified != 0L && lastModified < 1550000000000) {
                Files.move(file, dataDir.resolve("privateKey.$lastModified.i2p"), ATOMIC_MOVE)
                logger.info { "Renamed private key file to privateKey.$lastModified.i2p" }
            } else {
                privateKey = file.readText()
            }
        } catch (e: Throwable) {
        }
    }

    fun session(): Triple<String, Address, Closeable> {
        return session ?: throw I2PException("session is not available")
    }

    private fun connectToSAM(): Connection {
        val socket = sam?.run { Socket(getInetAddress(), port.toJava()) } ?: throw NotConfigured
        val connection = Connection(socket, socket.getInputStream(), socket.getOutputStream())

        val answer = request(connection, "HELLO VERSION MIN=3.2\n")
        connection.checkResult(answer)

        return connection
    }

    fun createSession(): Triple<String, Address, Closeable> {
        val connection = connectToSAM()

        val sessionId = generateId()

        // i2cp.leaseSetEncType 0 for connectivity with [Node.PROTOCOL_VERSION] <= 15
        val answer = request(connection, "SESSION CREATE STYLE=STREAM ID=$sessionId DESTINATION=$privateKey SIGNATURE_TYPE=EdDSA_SHA512_Ed25519 i2cp.leaseSetEncType=4,0\n")
        connection.checkResult(answer)
        val privateKey = getValue(answer, "DESTINATION") ?: throw I2PException("invalid response")

        val destination = lookup(connection, "ME")
        val localAddress = Address(Network.I2P, config.port, hash(destination))

        if (this.privateKey == "TRANSIENT")
            savePrivateKey(privateKey)

        val newSession = Triple(sessionId, localAddress, connection.socket)
        session = newSession

        startInterruptible("I2PSAM::createSession") {
            try {
                while (true) {
                    val message = connection.inputStream.readUTF8Line() ?: break

                    if (message.startsWith("PING")) {
                        connection.outputStream.writeStringUtf8("PONG" + message.drop(4) + '\n')
                        connection.outputStream.flush()
                    }
                }
            } catch (e: SocketException) {
                //XXX probably should be interruptible instead of closeable
            } finally {
                session = null
            }
        }

        return newSession
    }

    fun connect(address: Address): Connection {
        val (sessionId, _) = session()

        val connection = connectToSAM()

        val destination = lookup(connection, address.getAddressString())

        val answer = request(connection, "STREAM CONNECT ID=$sessionId DESTINATION=$destination\n")
        connection.checkResult(answer)

        return connection
    }

    fun accept(): Accepted {
        val (sessionId, _) = session()

        val connection = connectToSAM()

        val answer = request(connection, "STREAM ACCEPT ID=$sessionId\n")
        try {
            connection.checkResult(answer)
        } catch (e: Throwable) {
            logger.info { "STREAM ACCEPT failed: ${e.message}" }
            throw e
        }

        while (true) {
            val message = connection.inputStream.readUTF8Line() ?: throw I2PException("connection closed")

            if (message.startsWith("PING")) {
                connection.outputStream.writeStringUtf8("PONG" + message.drop(4) + '\n')
                connection.outputStream.flush()
            } else {
                val destination = message.takeWhile { it != ' ' }
                val remoteAddress = Address(Network.I2P, config.port, hash(destination))
                return Accepted(connection.socket, connection.inputStream, connection.outputStream, remoteAddress)
            }
        }
    }

    private fun lookup(connection: Connection, name: String): String {
        val answer = request(connection, "NAMING LOOKUP NAME=$name\n")
        connection.checkResult(answer)
        return getValue(answer, "VALUE")!!
    }

    private fun request(connection: Connection, request: String): String? {
        logger.debug { request.dropLast(1) }
        connection.outputStream.writeStringUtf8(request)
        connection.outputStream.flush()
        val answer = connection.inputStream.readUTF8Line()
        logger.debug { "$answer" }
        return answer
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

    class Connection(val socket: Socket, val inputStream: InputStream, val outputStream: OutputStream) {
        internal fun checkResult(answer: String?) {
            if (answer == null)
                exception("connection closed")
            val result = getValue(answer, "RESULT")
            if (result == null) {
                exception("No RESULT")
            } else if (result.isEmpty()) {
                exception("Empty RESULT")
            } else if (result != "OK") {
                val message = getValue(answer, "MESSAGE")
                if (message != null && !message.isEmpty())
                    exception("$result $message")
                else
                    exception(result)
            }
        }

        private fun exception(message: String): Nothing {
            socket.close()
            inputStream.close()
            outputStream.close()
            throw I2PException(message)
        }
    }

    class Accepted(val socket: Socket, val inputStream: InputStream, val outputStream: OutputStream, val remoteAddress: Address)
    open class I2PException(message: String) : RuntimeException("I2P SAM $message")
    object NotConfigured : I2PException("is not configured")

    private fun savePrivateKey(dest: String) {
        privateKey = dest
        logger.info { "Saving I2P private key" }
        replaceFile(dataDir, "privateKey.i2p") {
            write(privateKey.toByteArray(Charsets.UTF_8))
        }
    }

    companion object {
        internal fun hash(destination: String): ByteArray {
            val base64 = destination.replace('-', '+').replace('~', '/') //XXX optimize?
            val decoded = Base64.decode(base64)
            return buildHash("SHA-256") { encodeByteArray(decoded) }
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

        internal fun InputStream.readUTF8Line(): String? {
            // Because java.io.InputStreamReader buffers...
            val builder = StringBuilder(80)
            val scratch = ByteArray(4)
            var eos = false
            var i = 0
            while (true) {
                val read = read()
                if (read == -1) {
                    eos = true
                    break
                }
                scratch[i++] = read.toByte()
                if (Utf8.isWellFormed(scratch, 0, i)) {
                    val str = String(scratch, 0, i)
                    if (str == "\n")
                        break
                    builder.append(str)
                    Arrays.fill(scratch, 0, i, 0)
                    i = 0
                    continue
                }
                if (i == 4)
                    throw RuntimeException("Malformed UTF-8 character")
            }
            return if (builder.isNotEmpty())
                builder.toString()
            else if (!eos)
                ""
            else
                null
        }
        internal fun OutputStream.writeStringUtf8(string: String): Unit = write(string.toByteArray(Charsets.UTF_8))
    }
}
