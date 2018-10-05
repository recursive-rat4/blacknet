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
import io.ktor.network.util.ioCoroutineDispatcher
import kotlinx.coroutines.experimental.io.ByteReadChannel
import kotlinx.coroutines.experimental.io.ByteWriteChannel
import mu.KotlinLogging
import net.freehaven.tor.control.TorControlCommands.HS_ADDRESS
import net.freehaven.tor.control.TorControlConnection
import net.freehaven.tor.control.TorControlError
import ninja.blacknet.crypto.Base32
import ninja.blacknet.util.toHex
import java.net.ConnectException
import java.net.Inet6Address
import java.net.InetAddress
import java.net.InetSocketAddress

private val logger = KotlinLogging.logger {}

enum class Network(val addrSize: Int) {
    IPv4(4),
    IPv6(16),
    TORv2(10),
    TORv3(32),
    I2P(32),
    ;

    fun getAddressString(address: Address): String {
        return when (this) {
            IPv4 -> InetSocketAddress(InetAddress.getByAddress(address.bytes.array), address.port).getHostString()
            IPv6 -> '[' + InetSocketAddress(InetAddress.getByAddress(address.bytes.array), address.port).getHostString() + ']'
            TORv2, TORv3 -> Base32.encode(address.bytes.array) + ".onion"
            else -> name + ' ' + address.bytes.array.toHex()
        }
    }

    fun isLocal(address: Address): Boolean {
        return when (this) {
            IPv4 -> isLocalIPv4(address.bytes.array)
            IPv6 -> isLocalIPv6(address.bytes.array)
            TORv2, TORv3 -> false
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

        suspend fun connect(address: Address): Pair<ByteReadChannel, ByteWriteChannel> {
            when (address.network) {
                IPv4, IPv6 -> {
                    val addr = InetSocketAddress(InetAddress.getByAddress(address.bytes.array), address.port)
                    val socket = aSocket(ActorSelectorManager(ioCoroutineDispatcher)).tcp().connect(addr)
                    return Pair(socket.openReadChannel(), socket.openWriteChannel())
                }
                TORv2, TORv3 -> {
                    val proxy = InetSocketAddress("localhost", 9050)
                    return Socks5(proxy).connect(address)
                }
                else -> throw NotImplementedError("not implemented for " + address.network)
            }
        }

        fun listenOnTor(): Address? {
            try {
                val s = java.net.Socket("localhost", 9051)
                val tor = TorControlConnection(s)
                tor.launchThread(true)
                tor.authenticate(ByteArray(0));

                val port = HashMap<Int, String?>()
                port[Node.P2P_PORT] = null

                val response = tor.addOnion(port)
                val string = response[HS_ADDRESS]!!
                val bytes = Base32.decode(string)!!

                val type = when (bytes.size) {
                    TORv2.addrSize -> TORv2
                    TORv3.addrSize -> TORv3
                    else -> throw TorControlError("Unknown KeyType")
                }
                return Address(type, Node.P2P_PORT, bytes)
            } catch (e: ConnectException) {
                logger.info("Can't connect to tor controller")
            } catch (e: TorControlError) {
                logger.info("Tor " + e.message)
            } finally {
            }
            return null
        }
    }
}