/*
 * Copyright (c) 2018 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.api

import io.ktor.application.Application
import io.ktor.application.call
import io.ktor.application.install
import io.ktor.features.DefaultHeaders
import io.ktor.response.respond
import io.ktor.routing.get
import io.ktor.routing.routing
import kotlinx.serialization.json.JSON
import kotlinx.serialization.list
import ninja.blacknet.db.PeerDB
import ninja.blacknet.network.Node

fun Application.main() {
    install(DefaultHeaders)

    routing {
        get("/") {
            call.respond("It works\n")
        }

        get("/peerinfo") {
            val ret = ArrayList<PeerInfo>()
            Node.connections.forEach { ret.add(PeerInfo(it)) }
            call.respond(JSON.indented.stringify(PeerInfo.serializer().list, ret))
        }

        get("/nodeinfo") {
            val listening = Node.listenAddress.clone().map { it.toString() }
            val ret = NodeInfo(Node.agent, Node.version, Node.outgoing(), Node.incoming(), listening)
            call.respond(JSON.indented.stringify(ret))
        }

        get("/peerdb") {
            val peers = PeerDB.getAll().map { it.toString() }
            val ret = PeerDBInfo(peers.size, peers)
            call.respond(JSON.indented.stringify(ret))
        }
    }
}