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
        Node.listenOn(Address.IPv6_ANY(Node.P2P_PORT))

        /* Launch Blacknet API web-server on port 8283
         * using Ktor.
         * http://localhost:8283
         *
         * Blacknet API web-server logic is implemented in
         * ninja.blacknet.api.APIServerKt.main
         *
         * Ktor is a framework for building asynchronous servers and clients
         * in connected systems using the powerful Kotlin programming language.
         * https://ktor.io/
         *
         * Ktor configuration is stored in
         * resources/application.conf
         * https://ktor.io/servers/engine.html
         *
         */
        embeddedServer(Jetty, commandLineEnvironment(args)).start(wait = true)
    }
}