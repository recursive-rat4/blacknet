/*
 * Copyright (c) 2018-2024 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.rpc.v2

import io.ktor.server.routing.Route
import kotlinx.serialization.Serializable
import kotlinx.serialization.builtins.ListSerializer
import ninja.blacknet.Kernel
import ninja.blacknet.codec.base.Base16
import ninja.blacknet.codec.base.encode
import ninja.blacknet.core.Transaction
import ninja.blacknet.crypto.Hash
import ninja.blacknet.db.PeerDB
import ninja.blacknet.mode
import ninja.blacknet.network.Network
import ninja.blacknet.network.Node
import ninja.blacknet.rpc.requests.*
import ninja.blacknet.rpc.v1.NodeInfo
import ninja.blacknet.rpc.v1.TxPoolInfo
import ninja.blacknet.serialization.bbf.binaryFormat
import ninja.blacknet.util.startInterruptible

@Serializable
class Peers : Request {
    override fun handle(): TextContent {
        return respondJson(ListSerializer(PeerInfo.serializer()), PeerInfo.getAll())
    }
}

@Serializable
class NodeRequest : Request {
    override fun handle(): TextContent {
        return respondJson(NodeInfo.serializer(), NodeInfo.get())
    }
}

@Serializable
class TxPoolRequest : Request {
    override fun handle(): TextContent {
        return respondJson(TxPoolInfo.serializer(), TxPoolInfo.get())
    }
}

@Serializable
class TxPoolTransaction(
    val hash: Hash,
    val raw: Boolean = false
) : Request {
    override fun handle(): TextContent {
        val result = Kernel.txPool().get(hash)
        return if (result != null) {
            if (raw)
                return respondText(Base16.encode(result))

            val tx = binaryFormat.decodeFromByteArray(Transaction.serializer(), result)
            respondJson(TransactionInfo.serializer(), TransactionInfo(tx, hash, result.size))
        } else {
            respondError("Transaction not found")
        }
    }
}

@Serializable
class AddPeer(
    val port: String = mode.defaultP2PPort.toString(),
    val address: String,
    val force: Boolean = false, // ignored
) : Request {
    override fun handle(): TextContent {
        val port = Network.parsePort(port) ?: return respondError("Invalid port")
        val address = Network.parse(address, port) ?: return respondError("Invalid address")

        return if (PeerDB.tryContact(address)) {
            val connection = try {
                Node.connectTo(address, v2 = false)
            } catch (e: Throwable) {
                PeerDB.discontacted(address)
                throw e
            }
            startInterruptible("AddPeer::discontactor ${connection.debugName()}") {
                connection.join()
                PeerDB.discontacted(address)
            }
            respondText(true.toString())
        } else if (address.isLocal() || address.isPrivate()) {
            Node.connectTo(address, v2 = false)
            respondText(true.toString())
        } else {
            respondError("Already in contact")
        }
    }
}

@Serializable
class DisconnectPeerByAddress(
    val port: String = mode.defaultP2PPort.toString(),
    val address: String,
    @Suppress("unused")
    val force: Boolean = false
) : Request {
    override fun handle(): TextContent {
        val port = Network.parsePort(port) ?: return respondError("Invalid port")
        val address = Network.parse(address, port) ?: return respondError("Invalid address")

        val connection = Node.connections.find { it.remoteAddress == address }
        return if (connection != null) {
            connection.close()
            respondText(true.toString())
        } else {
            respondText(false.toString())
        }
    }
}

@Serializable
class DisconnectPeer(
    val id: Long,
    @Suppress("unused")
    val force: Boolean = false
) : Request {
    override fun handle(): TextContent {
        val connection = Node.connections.find { it.peerId == id }
        return if (connection != null) {
            connection.close()
            respondText(true.toString())
        } else {
            respondText(false.toString())
        }
    }
}

fun Route.node() {
    get(Peers.serializer(), "/api/v2/peers")

    get(NodeRequest.serializer(), "/api/v2/node")

    get(TxPoolRequest.serializer(), "/api/v2/txpool")

    get(TxPoolTransaction.serializer(), "/api/v2/txpool/transaction")
    get(TxPoolTransaction.serializer(), "/api/v2/txpool/transaction/{hash}/{raw?}")

    get(AddPeer.serializer(), "/api/v2/addpeer")
    get(AddPeer.serializer(), "/api/v2/addpeer/{address}/{port?}/{force?}")

    get(DisconnectPeerByAddress.serializer(), "/api/v2/disconnectpeerbyaddress")
    get(DisconnectPeerByAddress.serializer(), "/api/v2/disconnectpeerbyaddress/{address}/{port?}/{force?}")

    get(DisconnectPeer.serializer(), "/api/v2/disconnectpeer")
    get(DisconnectPeer.serializer(), "/api/v2/disconnectpeer/{id}/{force?}")
}
