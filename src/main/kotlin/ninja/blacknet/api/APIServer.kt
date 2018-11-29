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
import io.ktor.http.cio.websocket.CloseReason
import io.ktor.http.cio.websocket.Frame
import io.ktor.http.cio.websocket.close
import io.ktor.http.cio.websocket.readText
import io.ktor.response.respond
import io.ktor.routing.get
import io.ktor.routing.post
import io.ktor.routing.routing
import io.ktor.websocket.WebSockets
import io.ktor.websocket.webSocket
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.channels.SendChannel
import kotlinx.coroutines.launch
import kotlinx.serialization.json.JSON
import kotlinx.serialization.list
import ninja.blacknet.core.*
import ninja.blacknet.crypto.*
import ninja.blacknet.network.Node
import ninja.blacknet.serialization.SerializableByteArray
import ninja.blacknet.transaction.*
import ninja.blacknet.util.SynchronizedArrayList
import kotlin.coroutines.CoroutineContext

object APIServer : CoroutineScope {
    override val coroutineContext: CoroutineContext = Dispatchers.Default
    val blockNotify = SynchronizedArrayList<SendChannel<Frame>>()
    val transactionNotify = SynchronizedArrayList<Pair<SendChannel<Frame>, PublicKey>>()

    suspend fun blockNotify(hash: Hash) {
        blockNotify.forEach {
            launch {
                try {
                    it.send(Frame.Text(hash.toString()))
                } finally {
                }
            }
        }
    }

    suspend fun transactionNotify(hash: Hash, pubkey: PublicKey) {
        transactionNotify.forEach {
            launch {
                try {
                    if (it.second == pubkey)
                        it.first.send(Frame.Text(hash.toString()))
                } finally {
                }
            }
        }
    }
}

fun Application.main() {
    install(DefaultHeaders)
    install(WebSockets)

    routing {
        get("/") {
            call.respond("It works\n")
        }

        webSocket("/api/v1/notify/block") {
            try {
                APIServer.blockNotify.add(outgoing)
                while (true) {
                    incoming.receive()
                }
            } finally {
                APIServer.blockNotify.remove(outgoing)
            }
        }

        webSocket("/api/v1/notify/transaction") {
            try {
                while (true) {
                    val string = (incoming.receive() as Frame.Text).readText()
                    val pubkey = Address.decode(string) ?: return@webSocket this.close(CloseReason(CloseReason.Codes.PROTOCOL_ERROR, "invalid account"))
                    APIServer.transactionNotify.add(Pair(outgoing, pubkey))
                }
            } finally {
                APIServer.transactionNotify.removeIf { it.first == outgoing }
            }
        }

        get("/api/v1/peerinfo") {
            call.respond(JSON.indented.stringify(PeerInfo.serializer().list, PeerInfo.getAll()))
        }

        get("/api/v1/nodeinfo") {
            call.respond(JSON.indented.stringify(NodeInfo.serializer(), NodeInfo.get()))
        }

        get("/api/v1/peerdb") {
            call.respond(JSON.indented.stringify(PeerDBInfo.serializer(), PeerDBInfo.get()))
        }

        get("/api/v1/blockdb") {
            call.respond(JSON.indented.stringify(BlockDBInfo.serializer(), BlockDBInfo.get()))
        }

        get("/api/v1/blockdb/get/{hash}") {
            val hash = Hash.fromString(call.parameters["hash"]) ?: return@get call.respond(HttpStatusCode.BadRequest, "invalid hash")
            val ret = BlockInfo.get(hash)
            if (ret != null)
                call.respond(JSON.indented.stringify(BlockInfo.serializer(), ret))
            else
                call.respond(HttpStatusCode.NotFound, "block not found")
        }

        get("/api/v1/ledger") {
            call.respond(JSON.indented.stringify(LedgerInfo.serializer(), LedgerInfo.get()))
        }

        get("/api/v1/ledger/get/{account}") {
            val pubkey = Address.decode(call.parameters["account"]) ?: return@get call.respond(HttpStatusCode.BadRequest, "invalid account")
            val ret = AccountInfo.get(pubkey)
            if (ret != null)
                call.respond(JSON.indented.stringify(AccountInfo.serializer(), ret))
            else
                call.respond(HttpStatusCode.NotFound, "account not found")
        }

        get("/api/v1/txpool") {
            call.respond(JSON.indented.stringify(TxPoolInfo.serializer(), TxPoolInfo.get()))
        }

        get("/api/v1/account/generate") {
            call.respond(JSON.indented.stringify(MnemonicInfo.serializer(), MnemonicInfo.new()))
        }

        post("/api/v1/transfer/{mnemonic}/{fee}/{amount}/{to}/{message?}/{encrypted?}") {
            val privateKey = Mnemonic.fromString(call.parameters["mnemonic"]) ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid mnemonic")
            val from = privateKey.toPublicKey()
            val seq = TxPool.getSequence(from)
            val fee = call.parameters["fee"]?.toLong() ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid fee")
            val amount = call.parameters["amount"]?.toLong() ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid amount")
            val to = Address.decode(call.parameters["to"]) ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid to")
            val message = Message.create(call.parameters["message"], call.parameters["encrypted"]?.toByte(), privateKey, to) ?: return@post call.respond(HttpStatusCode.BadRequest, "failed to create message")

            val data = Transfer(amount, to, message).serialize()
            val tx = Transaction.create(from, seq, fee, TxType.Transfer.getType(), data)
            val signed = tx.sign(privateKey)

            if (Node.broadcastTx(signed.first, signed.second, fee))
                call.respond(signed.first.toString())
            else
                call.respond("Transaction rejected")
        }

        post("/api/v1/burn/{mnemonic}/{fee}/{amount}/{message?}/") {
            val privateKey = Mnemonic.fromString(call.parameters["mnemonic"]) ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid mnemonic")
            val from = privateKey.toPublicKey()
            val seq = TxPool.getSequence(from)
            val fee = call.parameters["fee"]?.toLong() ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid fee")
            val amount = call.parameters["amount"]?.toLong() ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid amount")
            val message = SerializableByteArray.fromString(call.parameters["message"].orEmpty()) ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid message")

            val data = Burn(amount, message).serialize()
            val tx = Transaction.create(from, seq, fee, TxType.Burn.getType(), data)
            val signed = tx.sign(privateKey)

            if (Node.broadcastTx(signed.first, signed.second, fee))
                call.respond(signed.first.toString())
            else
                call.respond("Transaction rejected")
        }

        post("/api/v1/lease/{mnemonic}/{fee}/{amount}/{to}") {
            val privateKey = Mnemonic.fromString(call.parameters["mnemonic"]) ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid mnemonic")
            val from = privateKey.toPublicKey()
            val seq = TxPool.getSequence(from)
            val fee = call.parameters["fee"]?.toLong() ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid fee")
            val amount = call.parameters["amount"]?.toLong() ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid amount")
            val to = Address.decode(call.parameters["to"]) ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid to")

            val data = Lease(amount, to).serialize()
            val tx = Transaction.create(from, seq, fee, TxType.Lease.getType(), data)
            val signed = tx.sign(privateKey)

            if (Node.broadcastTx(signed.first, signed.second, fee))
                call.respond(signed.first.toString())
            else
                call.respond("Transaction rejected")
        }

        post("/api/v1/cancellease/{mnemonic}/{fee}/{amount}/{to}/{height}") {
            val privateKey = Mnemonic.fromString(call.parameters["mnemonic"]) ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid mnemonic")
            val from = privateKey.toPublicKey()
            val seq = TxPool.getSequence(from)
            val fee = call.parameters["fee"]?.toLong() ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid fee")
            val amount = call.parameters["amount"]?.toLong() ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid amount")
            val to = Address.decode(call.parameters["to"]) ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid to")
            val height = call.parameters["height"]?.toInt() ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid height")

            val data = CancelLease(amount, to, height).serialize()
            val tx = Transaction.create(from, seq, fee, TxType.CancelLease.getType(), data)
            val signed = tx.sign(privateKey)

            if (Node.broadcastTx(signed.first, signed.second, fee))
                call.respond(signed.first.toString())
            else
                call.respond("Transaction rejected")
        }

        post("/api/v1/signmessage/{mnemonic}/{message}") {
            val privateKey = Mnemonic.fromString(call.parameters["mnemonic"]) ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid mnemonic")
            val message = call.parameters["message"] ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid message")

            val signature = Message.sign(privateKey, message)

            call.respond(signature.toString())
        }

        get("/api/v1/verifymessage/{account}/{signature}/{message}") {
            val pubkey = Address.decode(call.parameters["account"]) ?: return@get call.respond(HttpStatusCode.BadRequest, "invalid account")
            val signature = Signature.fromString(call.parameters["signature"]) ?: return@get call.respond(HttpStatusCode.BadRequest, "invalid signature")
            val message = call.parameters["message"] ?: return@get call.respond(HttpStatusCode.BadRequest, "invalid message")

            val result = Message.verify(pubkey, signature, message)

            call.respond(result.toString())
        }
    }
}
