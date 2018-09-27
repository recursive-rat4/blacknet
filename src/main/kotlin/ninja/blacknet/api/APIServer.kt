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
import io.ktor.http.HttpStatusCode
import io.ktor.response.respond
import io.ktor.routing.get
import io.ktor.routing.routing
import kotlinx.serialization.json.JSON
import kotlinx.serialization.list
import ninja.blacknet.core.Block
import ninja.blacknet.core.TxPool
import ninja.blacknet.crypto.Hash
import ninja.blacknet.db.BlockDB
import ninja.blacknet.db.Ledger
import ninja.blacknet.db.PeerDB
import ninja.blacknet.network.Node

fun Application.main() {
    install(DefaultHeaders)

    routing {
        get("/") {
            call.respond("It works\n")
        }

        get("/peerinfo") {
            val ret = Node.connections.map { PeerInfo(it) }
            call.respond(JSON.indented.stringify(PeerInfo.serializer().list, ret))
        }

        get("/nodeinfo") {
            val listening = Node.listenAddress.map { it.toString() }
            val ret = NodeInfo(Node.agent, Node.version, Node.outgoing(), Node.incoming(), listening)
            call.respond(JSON.indented.stringify(ret))
        }

        get("/peerdb") {
            val peers = PeerDB.getAll().map { it.toString() }
            val ret = PeerDBInfo(peers.size, peers)
            call.respond(JSON.indented.stringify(ret))
        }

        get("/blockdb") {
            val ret = BlockDBInfo(BlockDB.size())
            call.respond(JSON.indented.stringify(ret))
        }

        get("/blockdb/get/{hash}") {
            val string = call.parameters["hash"]
            if (string != null) {
                val hash = Hash.fromString(string)
                if (hash != null) {
                    val bytes = BlockDB.get(hash)
                    if (bytes != null) {
                        val block = Block.deserialize(bytes) ?: throw RuntimeException("invalid block in db")
                        val ret = BlockInfo(block)
                        call.respond(JSON.indented.stringify(ret))
                    } else {
                        call.respond(HttpStatusCode.NotFound, "block not found")
                    }
                } else {
                    call.respond(HttpStatusCode.BadRequest, "invalid hash")
                }
            } else {
                call.respond(HttpStatusCode.BadRequest, "hash is not specified")
            }
        }

        get("/ledger") {
            val ret = LedgerInfo(Ledger.height(), Ledger.blockHash().toString())
            call.respond(JSON.indented.stringify(ret))
        }

        get("/txpool") {
            val tx = TxPool.mapHashes { toString() }
            val ret = TxPoolInfo(TxPool.size(), TxPool.dataSize(), tx)
            call.respond(JSON.indented.stringify(ret))
        }
    }
}