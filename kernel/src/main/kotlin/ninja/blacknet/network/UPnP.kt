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
import ninja.blacknet.Kernel
import ninja.blacknet.ShutdownHooks
import ninja.blacknet.Version
import org.bitlet.weupnp.GatewayDevice
import org.bitlet.weupnp.GatewayDiscover

private val logger = KotlinLogging.logger {}

object UPnP {
    private const val PROTOCOL = "TCP"

    fun forward() {
        logger.info { "Looking for UPnP Gateway" }
        val discover = GatewayDiscover()
        discover.discover()

        val gateway = discover.getValidGateway()
        if (gateway == null) {
            logger.info { "No valid UPnP Gateway found" }
            return
        }

        logger.info { "Sending port mapping request" }
        if (!gateway.addPortMapping(Kernel.config().port.toJava(), Kernel.config().port.toJava(), gateway.localAddress.hostAddress, PROTOCOL, Version.name)) {
            logger.info { "Port mapping failed" }
            return
        }

        var address: Address? = null
        try {
            address = Network.parse(gateway.externalIPAddress, Kernel.config().port)
        } catch (e: Throwable) {
        }
        if (address != null) {
            Node.addListenAddress(address)
            logger.info { "Mapped to ${address.debugName()}" }
        } else {
            logger.info { "Mapped to unknown external address" }
        }

        ShutdownHooks.add {
            remove(gateway)
        }
    }

    private fun remove(gateway: GatewayDevice) {
        logger.info { "Removing port mapping" }
        gateway.deletePortMapping(Kernel.config().port.toJava(), PROTOCOL)
    }
}
