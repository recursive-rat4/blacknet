/*
 * Copyright (c) 2018-2024 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.network

import java.net.Socket
import ninja.blacknet.io.data
import ninja.blacknet.io.writeByte
import ninja.blacknet.io.writeUShort

/**
 * 代理客戶端
 *
 * RFC 1928 SOCKS Protocol Version 5
 * RFC 1929 Username/Password Authentication for SOCKS V5
 */
object Socks5 {
    fun connect(proxy: Address, destination: Address): Socket {
        val socket = Socket(proxy.getInetAddress(), proxy.port.toJava())
        val inputStream = socket.inputStream.data()
        val outputStream = socket.outputStream.data()

        outputStream.writeByte(VERSION)
        outputStream.writeByte(1) // number of authentication methods supported
        outputStream.writeByte(NO_AUTHENTICATION)
        outputStream.flush()

        if (inputStream.readByte() != VERSION) {
            socket.exception("Unknown socks version")
        }
        if (inputStream.readByte() != NO_AUTHENTICATION) {
            socket.exception("Socks auth not accepted")
        }

        outputStream.writeByte(VERSION)
        outputStream.writeByte(TCP_CONNECTION)
        outputStream.writeByte(0) // reserved
        when (destination.network) {
            Network.IPv4 -> {
                outputStream.writeByte(IPv4_ADDRESS)
                outputStream.write(destination.bytes)
            }
            Network.IPv6 -> {
                outputStream.writeByte(IPv6_ADDRESS)
                outputStream.write(destination.bytes)
            }
            Network.TORv2, Network.TORv3 -> {
                val bytes = destination.getAddressString().toByteArray(Charsets.US_ASCII)
                if (bytes.size < 1 || bytes.size > 255)
                    socket.exception("Invalid length of domain name")
                outputStream.writeByte(DOMAIN_NAME)
                outputStream.writeByte(bytes.size.toByte())
                outputStream.write(bytes)
            }
            else -> socket.exception("Not implemented for ${destination.network}")
        }
        outputStream.writeUShort(destination.port.value)
        outputStream.flush()

        if (inputStream.readByte() != VERSION) {
            socket.exception("Unknown socks version")
        }
        if (inputStream.readByte() != REQUEST_GRANTED) {
            socket.exception("Connection failed")
        }
        if (inputStream.readByte() != 0.toByte()) {
            socket.exception("Invalid socks response")
        }
        val addrType = inputStream.readByte()
        when (addrType) {
            IPv4_ADDRESS -> inputStream.skipNBytes(4 + 2)
            IPv6_ADDRESS -> inputStream.skipNBytes(16 + 2)
            DOMAIN_NAME -> inputStream.skipNBytes(inputStream.readByte().toLong() + 2)
            else -> socket.exception("Unknown socks response")
        }

        return socket
    }

    private fun Socket.exception(message: String) {
        close()
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
}
