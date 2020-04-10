/*
 * Copyright (c) 2018-2019 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.api

import io.ktor.application.call
import io.ktor.http.HttpStatusCode
import io.ktor.response.respond
import io.ktor.routing.Route
import io.ktor.routing.get
import kotlinx.serialization.builtins.list
import ninja.blacknet.coding.toHex
import ninja.blacknet.core.Transaction
import ninja.blacknet.core.TxPool
import ninja.blacknet.crypto.Hash
import ninja.blacknet.messageOrDefault
import ninja.blacknet.network.Network
import ninja.blacknet.network.Node

fun Route.node() {
    get("/api/v2/peers") {
        call.respondJson(PeerInfo.serializer().list, PeerInfo.getAll())
    }

    get("/api/v2/node") {
        call.respondJson(NodeInfo.serializer(), NodeInfo.get())
    }

    get("/api/v2/txpool") {
        call.respondJson(TxPoolInfo.serializer(), TxPoolInfo.get())
    }

    get("/api/v2/txpool/transaction/{hash}/{raw?}") {
        val hash = Hash.fromString(call.parameters["hash"]) ?: return@get call.respond(HttpStatusCode.BadRequest, "Invalid hash")
        val raw = call.parameters["raw"]?.toBoolean() ?: false

        val result = TxPool.get(hash)
        if (result != null) {
            if (raw)
                return@get call.respond(result.toHex())

            val tx = Transaction.deserialize(result)
            call.respondJson(TransactionInfo.serializer(), TransactionInfo(tx, hash, result.size))
        } else {
            call.respond(HttpStatusCode.BadRequest, "Transaction not found")
        }
    }

    get("/api/v2/addpeer/{address}/{port?}/{force?}") {
        val port = call.parameters["port"]?.let { Network.parsePort(it) ?: return@get call.respond(HttpStatusCode.BadRequest, "Invalid port") } ?: Node.DEFAULT_P2P_PORT
        val address = Network.parse(call.parameters["address"], port) ?: return@get call.respond(HttpStatusCode.BadRequest, "Invalid address")
        val force = call.parameters["force"]?.toBoolean() ?: false

        try {
            val connection = Node.connections.find { it.remoteAddress == address }
            if (force || connection == null) {
                Node.connectTo(address)
                call.respond(true.toString())
            } else {
                call.respond(HttpStatusCode.BadRequest, "Already connected on ${connection.localAddress}")
            }
        } catch (e: Throwable) {
            call.respond(HttpStatusCode.BadRequest, e.messageOrDefault())
        }
    }

    get("/api/v2/disconnectpeerbyaddress/{address}/{port?}/{force?}") {
        val port = call.parameters["port"]?.let { Network.parsePort(it) ?: return@get call.respond(HttpStatusCode.BadRequest, "Invalid port") } ?: Node.DEFAULT_P2P_PORT
        val address = Network.parse(call.parameters["address"], port) ?: return@get call.respond(HttpStatusCode.BadRequest, "Invalid address")
        @Suppress("UNUSED_VARIABLE")
        val force = call.parameters["force"]?.toBoolean() ?: false

        val connection = Node.connections.find { it.remoteAddress == address }
        if (connection != null) {
            connection.close()
            call.respond(true.toString())
        } else {
            call.respond(false.toString())
        }
    }

    get("/api/v2/disconnectpeer/{id}/{force?}") {
        val id = call.parameters["id"]?.toLongOrNull() ?: return@get call.respond(HttpStatusCode.BadRequest, "Invalid id")
        @Suppress("UNUSED_VARIABLE")
        val force = call.parameters["force"]?.toBoolean() ?: false

        val connection = Node.connections.find { it.peerId == id }
        if (connection != null) {
            connection.close()
            call.respond(true.toString())
        } else {
            call.respond(false.toString())
        }
    }
}
