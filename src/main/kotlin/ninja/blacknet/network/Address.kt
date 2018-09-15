/*
 * Copyright (c) 2018 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.network

import kotlinx.serialization.Serializable
import ninja.blacknet.serialization.SerializableByteArray
import java.net.InetAddress
import java.net.InetSocketAddress

@Serializable
class Address(
        val network: Network,
        val port: Int,
        val bytes: SerializableByteArray
) {
    constructor(network: Network, port: Int, bytes: ByteArray) : this(network, port, SerializableByteArray(bytes))

    fun checkSize(): Boolean {
        return bytes.size() == network.addrSize
    }

    private fun getAddressString(): String {
        return when (network) {
            Network.IPv4 -> InetSocketAddress(InetAddress.getByAddress(bytes.array), port).getHostString()
            Network.IPv6 -> '[' + InetSocketAddress(InetAddress.getByAddress(bytes.array), port).getHostString() + ']'
            else -> bytes.toString()
        }
    }

    override fun toString(): String {
        return network.name + ' ' + getAddressString() + ':' + port
    }

    companion object {
        fun IPv4_ANY(port: Int) = Address(Network.IPv4, port, SerializableByteArray(Network.IPv4.addrSize))
        fun IPv6_ANY(port: Int) = Address(Network.IPv6, port, SerializableByteArray(Network.IPv6.addrSize))
    }
}