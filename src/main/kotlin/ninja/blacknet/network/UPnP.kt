/*
 * Copyright (c) 2018 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.network

import mu.KotlinLogging
import ninja.blacknet.Config
import ninja.blacknet.Config.port
import org.bitlet.weupnp.GatewayDiscover

private val logger = KotlinLogging.logger {}

object UPnP {
    suspend fun forward() {
        logger.info("Looking for UPnP Gateway")
        val discover = GatewayDiscover()
        discover.discover()

        val gateway = discover.getValidGateway()
        if (gateway == null) {
            logger.info("No valid UPnP Gateway found")
            return
        }

        logger.info("Sending port mapping request")
        if (!gateway.addPortMapping(Config[port], Config[port], gateway.localAddress.hostAddress, "TCP", Bip14.client)) {
            logger.info("Port mapping failed")
            return
        }

        var address: Address? = null
        try {
            address = Network.parse(gateway.externalIPAddress, Config[port])
        } catch (e: Throwable) {
        }
        if (address != null) {
            Node.listenAddress.add(address)
            logger.info("Mapped to $address")
        } else {
            logger.info("Mapped to unknown external address")
        }

        Runtime.getRuntime().addShutdownHook(Thread {
            logger.info("Removing port mapping")
            gateway.deletePortMapping(Config[port], "TCP")
        })
    }
}