/*
 * Copyright (c) 2018-2024 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.network

import io.ktor.network.sockets.ASocket
import io.ktor.network.sockets.aSocket
import io.ktor.network.sockets.openReadChannel
import io.ktor.network.sockets.openWriteChannel
import io.ktor.utils.io.ByteReadChannel
import io.ktor.utils.io.ByteWriteChannel
import io.ktor.utils.io.cancel
import io.ktor.utils.io.close
import io.ktor.utils.io.writeFully

/**
 * 代理客戶端
 *
 * RFC 1928 SOCKS Protocol Version 5
 * RFC 1929 Username/Password Authentication for SOCKS V5
 */
object Socks5 {
    suspend fun connect(proxy: Address, destination: Address): Triple<ASocket, ByteReadChannel, ByteWriteChannel> {
        val socket = aSocket(Network.selector).tcp().connect(proxy.getSocketAddress())
        val readChannel = socket.openReadChannel()
        val writeChannel = socket.openWriteChannel()
        val connection = Triple(socket, readChannel, writeChannel)

        writeChannel.writeByte(VERSION)
        writeChannel.writeByte(1) // number of authentication methods supported
        writeChannel.writeByte(NO_AUTHENTICATION)
        writeChannel.flush()

        if (readChannel.readByte() != VERSION) {
            connection.exception("Unknown socks version")
        }
        if (readChannel.readByte() != NO_AUTHENTICATION) {
            connection.exception("Socks auth not accepted")
        }

        writeChannel.writeByte(VERSION)
        writeChannel.writeByte(TCP_CONNECTION)
        writeChannel.writeByte(0) // reserved
        when (destination.network) {
            Network.IPv4 -> {
                writeChannel.writeByte(IPv4_ADDRESS)
                writeChannel.writeFully(destination.bytes)
            }
            Network.IPv6 -> {
                writeChannel.writeByte(IPv6_ADDRESS)
                writeChannel.writeFully(destination.bytes)
            }
            Network.TORv2, Network.TORv3 -> {
                val bytes = destination.getAddressString().toByteArray(Charsets.US_ASCII)
                if (bytes.size < 1 || bytes.size > 255)
                    connection.exception("Invalid length of domain name")
                writeChannel.writeByte(DOMAIN_NAME)
                writeChannel.writeByte(bytes.size.toByte())
                writeChannel.writeFully(bytes)
            }
            else -> connection.exception("Not implemented for ${destination.network}")
        }
        writeChannel.writeUShort(destination.port.value)
        writeChannel.flush()

        if (readChannel.readByte() != VERSION) {
            connection.exception("Unknown socks version")
        }
        if (readChannel.readByte() != REQUEST_GRANTED) {
            connection.exception("Connection failed")
        }
        if (readChannel.readByte() != 0.toByte()) {
            connection.exception("Invalid socks response")
        }
        val addrType = readChannel.readByte()
        when (addrType) {
            IPv4_ADDRESS -> readChannel.skip(4 + 2)
            IPv6_ADDRESS -> readChannel.skip(16 + 2)
            DOMAIN_NAME -> readChannel.skip(readChannel.readByte().toInt() + 2)
            else -> connection.exception("Unknown socks response")
        }

        return connection
    }

    private fun Triple<ASocket, ByteReadChannel, ByteWriteChannel>.exception(message: String) {
        val socket = first; val readChannel = second; val writeChannel = third;
        socket.close()
        readChannel.cancel()
        writeChannel.close()
        throw RuntimeException(message)
    }

    private const val VERSION = 5.toByte()
    private const val NO_AUTHENTICATION = 0.toByte()
    private const val USERNAME_PASSWORD_AUTHENTICATION = 2.toByte()
    private const val NO_ACCEPTABLE_METHODS = 255.toByte()
    private const val TCP_CONNECTION = 1.toByte()
    private const val REQUEST_GRANTED = 0.toByte()
    private const val IPv4_ADDRESS = 1.toByte()
    private const val DOMAIN_NAME = 3.toByte()
    private const val IPv6_ADDRESS = 4.toByte()

    private suspend fun ByteReadChannel.skip(num: Int) {
        readFully(ByteArray(num), 0, num)
    }

    private suspend fun ByteWriteChannel.writeUShort(value: UShort) {
        writeShort(value.toShort())
    }
}
