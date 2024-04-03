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
import java.lang.Thread.sleep
import java.net.ConnectException
import java.net.InetSocketAddress
import java.net.Socket
import ninja.blacknet.Config

//UPSTREAM https://github.com/Kotlin/KEEP/issues/348
private val logger = KotlinLogging.logger {}
private const val INIT_TIMEOUT = 1 * 60 * 1000L
private const val MAX_TIMEOUT = 2 * 60 * 60 * 1000L

class Router(
    private val config: Config,
) {
    private val socksProxy: Address?
    private val torProxy: Address?
    private val sam: I2PSAM

    init {
        if (config.proxyhost != null && config.proxyport != null)
            socksProxy = Network.resolve(config.proxyhost, config.proxyport)
        else
            socksProxy = null

        if (config.torhost != null && config.torport != null)
            torProxy = Network.resolve(config.torhost, config.torport)
        else
            torProxy = null

        sam = I2PSAM(config)
    }

    fun isDisabled(network: Network): Boolean {
        return !when (network) {
            Network.IPv4 -> config.ipv4
            Network.IPv6 -> config.ipv6
            Network.TORv2, Network.TORv3 -> config.tor
            Network.I2P -> config.i2p
        }
    }

    fun connect(address: Address, prober: Boolean): Connection {
        if (isDisabled(address.network)) throw RuntimeException("${address.network} is disabled")
        val state = if (prober) Connection.State.PROBER_WAITING else Connection.State.OUTGOING_WAITING
        when (address.network) {
            Network.IPv4, Network.IPv6 -> {
                if (socksProxy != null) {
                    val socket = Socks5.connect(socksProxy, address)
                    return Connection(socket, address, socksProxy, state)
                } else {
                    val socket = Socket(address.getInetAddress(), address.port.toJava())
                    val localAddress = Network.address(socket.localSocketAddress as InetSocketAddress)
                    if (config.listen && !localAddress.isLocal())
                        Node.addListenAddress(Address(localAddress.network, config.port, localAddress.bytes))
                    return Connection(socket, address, localAddress, state)
                }
            }
            Network.TORv3 -> {
                if (torProxy == null) throw RuntimeException("Tor proxy is not set")
                val socket = Socks5.connect(torProxy, address)
                return Connection(socket, address, torProxy, state)
            }
            Network.I2P -> {
                val c = sam.connect(address)
                return Connection(c.socket, address, sam.session().second, state)
            }
            Network.TORv2 -> {
                throw RuntimeException("${address.network} is obsolete")
            }
        }
    }

    private var torTimeout = INIT_TIMEOUT
    private var i2pTimeout = INIT_TIMEOUT

    fun listenOnTor() {
        try {
            val (vThread, localAddress) = TorController.listen()

            logger.info { "Listening on ${localAddress.debugName()}" }
            Node.addListenAddress(localAddress)

            try {
                vThread.join()
            } catch (e: InterruptedException) {
                vThread.interrupt()
                throw e
            } finally {
                Node.removeListenAddress(localAddress)
            }

            logger.info { "Lost connection to tor controller" }

            torTimeout = INIT_TIMEOUT
        } catch (e: ConnectException) {
            logger.debug { "Can't connect to tor controller: ${e.message}" }
        }

        sleep(torTimeout)
        torTimeout = minOf(torTimeout * 2, MAX_TIMEOUT)
    }

    fun listenOnI2P() {
        try {
            val (_, localAddress, closeable) = sam.createSession()

            logger.info { "Listening on ${localAddress.debugName()}" }
            Node.addListenAddress(localAddress)

            while (true) {
                val a = try {
                    sam.accept()
                } catch (e: Throwable) {
                    closeable.close()
                    break
                }
                val connection = Connection(a.socket, a.remoteAddress, localAddress, Connection.State.INCOMING_WAITING)
                Node.addIncomingConnection(connection)
            }

            Node.removeListenAddress(localAddress)
            logger.info { "I2P SAM session closed" }

            i2pTimeout = INIT_TIMEOUT
        } catch (e: I2PSAM.NotConfigured) {
            logger.info { e.message }
            return
        } catch (e: I2PSAM.I2PException) {
            logger.info { e.message }
        } catch (e: ConnectException) {
            logger.debug { "Can't connect to I2P SAM: ${e.message}" }
        }

        sleep(i2pTimeout)
        i2pTimeout = minOf(i2pTimeout * 2, MAX_TIMEOUT)
    }
}
