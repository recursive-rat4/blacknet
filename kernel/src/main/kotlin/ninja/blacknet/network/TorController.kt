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
import java.io.BufferedReader
import java.io.BufferedWriter
import java.net.InetAddress
import java.net.Socket
import java.nio.file.Files
import java.nio.file.StandardCopyOption.ATOMIC_MOVE
import kotlin.io.path.readText
import ninja.blacknet.Kernel
import ninja.blacknet.dataDir
import ninja.blacknet.crypto.HashEncoder.Companion.buildHash
import ninja.blacknet.crypto.encodeByteArray
import ninja.blacknet.io.replaceFile
import ninja.blacknet.time.currentTimeSeconds
import ninja.blacknet.util.startInterruptible

private val logger = KotlinLogging.logger {}

object TorController {
    private var privateKey = "NEW:BEST"

    init {
        try {
            val file = dataDir.resolve("privateKey.tor")
            val readPrivateKey = file.readText()
            if (readPrivateKey.startsWith("RSA1024:")) {
                logger.info { "Migration to Tor addresses version 3" }
                val newName = "privateKey.${currentTimeSeconds()}.tor"
                Files.move(file, dataDir.resolve(newName), ATOMIC_MOVE)
                logger.info { "Renamed private key file to $newName" }
            } else {
                privateKey = readPrivateKey
            }
        } catch (e: Throwable) {
        }
    }

    class Connection(
        val socket: Socket,
        val bufferedReader: BufferedReader,
        val bufferedWriter: BufferedWriter,
    ) {
        fun authenticate() {
            bufferedWriter.write("AUTHENTICATE\r\n")
            bufferedWriter.flush()
            val replyLine = bufferedReader.readLine()
            when (replyLine) {
                "250 OK" -> Unit
                null -> throw RuntimeException("Tor controller connection unexpectedly closed")
                else -> throw RuntimeException("Unknown Tor reply line $replyLine")
            }
        }

        fun addOnion(): Pair<String?, String?> {
            bufferedWriter.write("ADD_ONION $privateKey Port=${Kernel.config().port}\r\n")
            bufferedWriter.flush()
            var serviceID: String? = null
            var newPrivateKey: String? = null
            while (true) {
                val replyLine = bufferedReader.readLine()
                if (replyLine == null)
                    throw RuntimeException("Tor controller connection unexpectedly closed")
                else if (replyLine == "250 OK")
                    break
                else if (replyLine.startsWith("250-ServiceID="))
                    serviceID = replyLine.drop(14)
                else if (replyLine.startsWith("250-PrivateKey="))
                    newPrivateKey = replyLine.drop(15)
                else if (!replyLine.startsWith("250-"))
                    throw RuntimeException("Unknown Tor reply line $replyLine")
            }
            return Pair(serviceID, newPrivateKey)
        }

        fun close() {
            socket.close()
            bufferedReader.close()
            bufferedWriter.close()
        }

        fun exception(message: String): Nothing {
            close()
            throw RuntimeException(message)
        }
    }

    fun listen(): Pair<Thread, Address> {
        //TODO configure host
        val socket = Socket(InetAddress.getByAddress(Network.IPv4_LOOPBACK_BYTES), Kernel.config().torcontrol.toJava())
        val connection = Connection(socket, socket.getInputStream().bufferedReader(), socket.getOutputStream().bufferedWriter())
        //TODO cookie, password
        connection.authenticate()
        val (serviceID, newPrivateKey) = connection.addOnion()
        val address = Network.parse(serviceID + Network.TOR_SUFFIX, Kernel.config().port) ?: connection.exception("Failed to parse Onion Service ID $serviceID")
        require(address.network == Network.TORv2 || address.network == Network.TORv3)

        if (privateKey.startsWith("NEW:"))
            savePrivateKey(newPrivateKey ?: connection.exception("Failed to get new private key"))

        return Pair(startInterruptible("TorController::listen") {
            val replyLine = connection.bufferedReader.readLine()
            if (replyLine != null)
                connection.exception("Unknown Tor reply line $replyLine")
            else
                connection.close()
        }, address)
    }

    private fun savePrivateKey(privKey: String) {
        privateKey = privKey
        logger.info { "Saving Tor private key" }
        replaceFile(dataDir, "privateKey.tor") {
            write(privateKey.toByteArray(Charsets.UTF_8))
        }
    }

    private const val CHECKSUM_CONST = ".onion checksum"
    const val V3: Byte = 3

    fun checksum(bytes: ByteArray, version: Byte): ByteArray {
        return buildHash("SHA3-256") {
            encodeString(CHECKSUM_CONST)
            encodeByteArray(bytes)
            encodeByte(version)
        }.copyOf(2)
    }
}
