/*
 * Copyright (c) 2018 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.network

import ninja.blacknet.util.toHex
import java.net.Inet6Address
import java.net.InetAddress
import java.net.InetSocketAddress

enum class Network(val addrSize: Int) {
    IPv4(4),
    IPv6(16),
    TORv3(32),
    I2P(32),
    ;

    fun getAddressString(address: Address): String {
        return when (this) {
            IPv4 -> InetSocketAddress(InetAddress.getByAddress(address.bytes.array), address.port).getHostString()
            IPv6 -> '[' + InetSocketAddress(InetAddress.getByAddress(address.bytes.array), address.port).getHostString() + ']'
            else -> name + ' ' + address.bytes.array.toHex()
        }
    }

    fun isLocal(address: Address): Boolean {
        return when (this) {
            IPv4 -> isLocalIPv4(address.bytes.array)
            IPv6 -> isLocalIPv6(address.bytes.array)
            TORv3 -> false
            I2P -> false
        }
    }

    private fun isLocalIPv4(bytes: ByteArray): Boolean {
        return bytes[0] == 0.toByte() || bytes[0] == 127.toByte()
    }

    private fun isLocalIPv6(bytes: ByteArray): Boolean {
        return bytes.contentEquals(Network.IPv6_ANY_BYTES) || bytes.contentEquals(Network.IPv6_LOOPBACK_BYTES)
    }

    companion object {
        val IPv6_ANY_BYTES = ByteArray(Network.IPv6.addrSize)
        val IPv6_LOOPBACK_BYTES = byteArrayOf(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1)

        fun address(inet: InetSocketAddress): Address {
            return address(inet.getAddress(), inet.port)
        }

        fun address(inet: InetAddress, port: Int): Address {
            val network = if (inet is Inet6Address) Network.IPv6 else Network.IPv4
            return Address(network, port, inet.getAddress())
        }
    }
}