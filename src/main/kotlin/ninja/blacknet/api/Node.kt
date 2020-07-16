/*
 * Copyright (c) 2018-2019 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.api

import io.ktor.application.ApplicationCall
import io.ktor.application.call
import io.ktor.http.HttpStatusCode
import io.ktor.response.respond
import io.ktor.routing.Route
import io.ktor.routing.get
import kotlinx.serialization.Serializable
import kotlinx.serialization.builtins.list
import ninja.blacknet.coding.toHex
import ninja.blacknet.core.Transaction
import ninja.blacknet.core.TxPool
import ninja.blacknet.crypto.HashSerializer
import ninja.blacknet.ktor.requests.Request
import ninja.blacknet.ktor.requests.get
import ninja.blacknet.network.Network
import ninja.blacknet.network.Node
import ninja.blacknet.network.toPort
import ninja.blacknet.serialization.BinaryDecoder

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

    @Serializable
    class TxPoolTransaction(
            @Serializable(with = HashSerializer::class)
            val hash: ByteArray,
            val raw: Boolean = false
    ) : Request {
        override suspend fun handle(call: ApplicationCall): Unit {
            val result = TxPool.get(hash)
            return if (result != null) {
                if (raw)
                    return call.respond(result.toHex())

                val tx = BinaryDecoder(result).decode(Transaction.serializer())
                call.respondJson(TransactionInfo.serializer(), TransactionInfo(tx, hash, result.size))
            } else {
                call.respond(HttpStatusCode.BadRequest, "Transaction not found")
            }
        }
    }

    get(TxPoolTransaction.serializer(), "/api/v2/txpool/transaction")
    get(TxPoolTransaction.serializer(), "/api/v2/txpool/transaction/{hash}/{raw?}")

    @Serializable
    class AddPeer(
            val port: String = Node.DEFAULT_P2P_PORT.toPort().toString(),
            val address: String,
            val force: Boolean = false
    ) : Request {
        override suspend fun handle(call: ApplicationCall): Unit {
            val port = Network.parsePort(port) ?: return call.respond(HttpStatusCode.BadRequest, "Invalid port")
            val address = Network.parse(address, port) ?: return call.respond(HttpStatusCode.BadRequest, "Invalid address")

            val connection = Node.connections.find { it.remoteAddress == address }
            return if (force || connection == null) {
                Node.connectTo(address)
                call.respond(true.toString())
            } else {
                call.respond(HttpStatusCode.BadRequest, "Already connected on ${connection.localAddress}")
            }
        }
    }

    get(AddPeer.serializer(), "/api/v2/addpeer")
    get(AddPeer.serializer(), "/api/v2/addpeer/{address}/{port?}/{force?}")

    @Serializable
    class DisconnectPeerByAddress(
            val port: String = Node.DEFAULT_P2P_PORT.toPort().toString(),
            val address: String,
            @Suppress("unused")
            val force: Boolean = false
    ) : Request {
        override suspend fun handle(call: ApplicationCall): Unit {
            val port = Network.parsePort(port) ?: return call.respond(HttpStatusCode.BadRequest, "Invalid port")
            val address = Network.parse(address, port) ?: return call.respond(HttpStatusCode.BadRequest, "Invalid address")

            val connection = Node.connections.find { it.remoteAddress == address }
            return if (connection != null) {
                connection.close()
                call.respond(true.toString())
            } else {
                call.respond(false.toString())
            }
        }
    }

    get(DisconnectPeerByAddress.serializer(), "/api/v2/disconnectpeerbyaddress")
    get(DisconnectPeerByAddress.serializer(), "/api/v2/disconnectpeerbyaddress/{address}/{port?}/{force?}")

    @Serializable
    class DisconnectPeer(
            val id: Long,
            @Suppress("unused")
            val force: Boolean = false
    ) : Request {
        override suspend fun handle(call: ApplicationCall): Unit {

            val connection = Node.connections.find { it.peerId == id }
            return if (connection != null) {
                connection.close()
                call.respond(true.toString())
            } else {
                call.respond(false.toString())
            }
        }
    }

    get(DisconnectPeer.serializer(), "/api/v2/disconnectpeer")
    get(DisconnectPeer.serializer(), "/api/v2/disconnectpeer/{id}/{force?}")
}
