/*
 * Copyright (c) 2018 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.network

import kotlinx.coroutines.experimental.launch
import mu.KotlinLogging
import ninja.blacknet.Config
import ninja.blacknet.Config.p2pport
import org.bitlet.weupnp.GatewayDiscover

private val logger = KotlinLogging.logger {}

object UPnP {
    fun forwardAsync() {
        launch { forward() }
    }

    suspend fun forward() {
        logger.info("Looking for UPnP Gateway")
        val discover = GatewayDiscover()
        discover.discover()

        val gateway = discover.getValidGateway()
        if (gateway == null) {
            logger.info("No valid gateway found")
            return
        }

        logger.info("Sending port mapping request")
        if (!gateway.addPortMapping(Config[p2pport], Config[p2pport], gateway.localAddress.hostAddress, "TCP", Node.agent)) {
            logger.info("Port mapping failed")
            return
        }

        var address: Address? = null
        try {
            address = Network.parse(gateway.externalIPAddress, Config[p2pport])
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
            gateway.deletePortMapping(Config[p2pport], "TCP")
        })
    }
}