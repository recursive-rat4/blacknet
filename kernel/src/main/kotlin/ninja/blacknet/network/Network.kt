/*
 * Copyright (c) 2018-2024 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.network

import com.google.common.net.InetAddresses
import java.net.InetAddress
import java.net.InetSocketAddress
import ninja.blacknet.codec.base.Base32

enum class Network(val type: Byte, val addrSize: Int) {
    IPv4(128, 4),
    IPv6(129, 16),
    TORv2(130, 10),
    TORv3(131, 32),
    I2P(132, 32),
    ;

    constructor(type: Int, addrSize: Int) : this(type.toByte(), addrSize)

    companion object {
        const val TOR_SUFFIX = ".onion"
        const val I2P_SUFFIX = ".b32.i2p"
        val IPv4_LOOPBACK_BYTES = byteArrayOf(127, 0, 0, 1)
        val IPv6_ANY_BYTES = ByteArray(Network.IPv6.addrSize)
        val IPv6_LOOPBACK_BYTES = byteArrayOf(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)
        fun LOOPBACK(port: Port) = Address.IPv4_LOOPBACK(port)

        fun get(type: Byte): Network {
            return when (type) {
                IPv4.type -> IPv4
                IPv6.type -> IPv6
                TORv2.type -> TORv2
                TORv3.type -> TORv3
                I2P.type -> I2P
                else -> throw RuntimeException("Unknown network type $type")
            }
        }

        fun address(inet: InetSocketAddress): Address {
            return address(inet.getAddress(), Port(inet.port))
        }

        fun address(inet: InetAddress, port: Port): Address {
            val bytes = inet.getAddress()
            return when (bytes.size) {
                IPv6.addrSize -> Address(IPv6, port, bytes)
                IPv4.addrSize -> Address(IPv4, port, bytes)
                else -> throw RuntimeException("Unknown IP address length ${bytes.size}")
            }
        }

        fun parse(string: String?, port: Port): Address? {
            if (string == null) return null
            val host = string.lowercase()

            if (host.endsWith(TOR_SUFFIX)) {
                if (host.length == 16 + TOR_SUFFIX.length) {
                    val decoded = Base32.decode(host.substring(0, 16))
                    if (decoded != null && decoded.size == TORv2.addrSize) {
                        return Address(TORv2, port, decoded)
                    }
                } else if (host.length == 56 + TOR_SUFFIX.length) {
                    val decoded = Base32.decode(host.substring(0, 56))
                    if (decoded != null && decoded.size == 35 && decoded[34] == TorController.V3) {
                        val bytes = decoded.copyOf(32)
                        val checksum = TorController.checksum(bytes, TorController.V3)
                        if (checksum[0] == decoded[32] && checksum[1] == decoded[33]) {
                            return Address(TORv3, port, bytes)
                        }
                    }
                }
                return null
            }

            if (host.endsWith(I2P_SUFFIX)) {
                if (host.length == 52 + I2P_SUFFIX.length) {
                    val decoded = Base32.decode(host.substring(0, 52))
                    if (decoded != null && decoded.size == I2P.addrSize) {
                        return Address(I2P, port, decoded)
                    }
                }
                return null
            }

            var parsed: InetAddress? = null
            try {
                parsed = InetAddresses.forString(host)
            } catch (e: Throwable) {
            }
            if (parsed != null)
                return address(parsed, port)
            return null
        }

        fun parsePort(string: String): Port? {
            return try {
                Port(string.toUShort())
            } catch (e: Throwable) {
                null
            }
        }

        fun resolve(string: String, port: Port): Address? {
            return resolveAll(string, port).firstOrNull()
        }

        fun resolveAll(string: String, port: Port): List<Address> {
            val parsed = parse(string, port)
            if (parsed != null)
                return listOf(parsed)

            try {
                return InetAddress.getAllByName(string).map { Network.address(it, port) }
            } catch (e: Throwable) {
                return emptyList()
            }
        }
    }
}
