/*
 * Copyright (c) 2018 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet

import io.ktor.server.engine.commandLineEnvironment
import io.ktor.server.engine.embeddedServer
import io.ktor.server.jetty.Jetty
import ninja.blacknet.network.Address
import ninja.blacknet.network.Node

object Main {
    @JvmStatic
    fun main(args: Array<String>) {
        Node.listenOn(Address.IPv6_ANY(28453))

        embeddedServer(Jetty, commandLineEnvironment(args)).start(wait = true)
    }
}