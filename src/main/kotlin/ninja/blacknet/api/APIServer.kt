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
import io.ktor.routing.post
import io.ktor.routing.routing
import kotlinx.serialization.json.JSON
import kotlinx.serialization.list
import ninja.blacknet.core.*
import ninja.blacknet.crypto.*
import ninja.blacknet.db.BlockDB
import ninja.blacknet.db.LedgerDB
import ninja.blacknet.db.PeerDB
import ninja.blacknet.network.Node
import ninja.blacknet.serialization.SerializableByteArray

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
            call.respond(JSON.indented.stringify(NodeInfo.serializer(), ret))
        }

        get("/peerdb") {
            val peers = PeerDB.getAll().map { it.toString() }
            val ret = PeerDBInfo(peers.size, peers)
            call.respond(JSON.indented.stringify(PeerDBInfo.serializer(), ret))
        }

        get("/blockdb") {
            val ret = BlockDBInfo(BlockDB.size())
            call.respond(JSON.indented.stringify(BlockDBInfo.serializer(), ret))
        }

        get("/blockdb/get/{hash}") {
            val hash = Hash.fromString(call.parameters["hash"]) ?: return@get call.respond(HttpStatusCode.BadRequest, "invalid hash")
            val bytes = BlockDB.get(hash)
            if (bytes != null) {
                val block = Block.deserialize(bytes)
                val ret = BlockInfo(block!!, bytes.size)
                call.respond(JSON.indented.stringify(BlockInfo.serializer(), ret))
            } else {
                call.respond(HttpStatusCode.NotFound, "block not found")
            }
        }

        get("/ledger") {
            val ret = LedgerInfo(LedgerDB.height(), LedgerDB.blockHash().toString(), LedgerDB.supply(), LedgerDB.accounts(), LedgerDB.getMaxBlockSize())
            call.respond(JSON.indented.stringify(LedgerInfo.serializer(), ret))
        }

        get("/ledger/get/{account}") {
            val pubkey = Address.decode(call.parameters["account"]) ?: return@get call.respond(HttpStatusCode.BadRequest, "invalid account")
            val state = LedgerDB.get(pubkey)
            if (state != null) {
                val ret = AccountInfo(state.seq, state.balance(), state.stakingBalance(LedgerDB.height()))
                call.respond(JSON.indented.stringify(AccountInfo.serializer(), ret))
            } else {
                call.respond(HttpStatusCode.NotFound, "account not found")
            }
        }

        get("/txpool") {
            val tx = TxPool.mapHashes { it.toString() }
            val ret = TxPoolInfo(TxPool.size(), TxPool.dataSize(), tx)
            call.respond(JSON.indented.stringify(TxPoolInfo.serializer(), ret))
        }

        get("/account/generate") {
            val pair = Mnemonic.generate()
            val publicKey = pair.second.toPublicKey()
            val ret = MnemonicInfo(pair.first, Address.encode(publicKey), publicKey.toString())
            call.respond(JSON.indented.stringify(MnemonicInfo.serializer(), ret))
        }

        post("/transfer/{mnemonic}/{fee}/{amount}/{to}/{message?}/{encrypted?}") {
            val privateKey = Mnemonic.fromString(call.parameters["mnemonic"]) ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid mnemonic")
            val from = privateKey.toPublicKey()
            val seq = TxPool.getSequence(from)
            val fee = call.parameters["fee"]?.toLong() ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid fee")
            val amount = call.parameters["amount"]?.toLong() ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid amount")
            val to = Address.decode(call.parameters["to"]) ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid to")
            val message = Message.create(call.parameters["message"], call.parameters["encrypted"]?.toByte())

            val data = Transfer(amount, to, message).serialize()
            val tx = Transaction.create(from, seq, fee, TxType.Transfer.getType(), data)
            val signed = tx.sign(privateKey)

            if (Node.broadcastTx(signed.first, signed.second, fee))
                call.respond(signed.first.toString())
            else
                call.respond("Transaction rejected")
        }

        post("/burn/{mnemonic}/{fee}/{amount}/{message?}/") {
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

        post("/lease/{mnemonic}/{fee}/{amount}/{to}") {
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

        post("/cancellease/{mnemonic}/{fee}/{amount}/{to}/{height}") {
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

        post("/signmessage/{mnemonic}/{message}") {
            val privateKey = Mnemonic.fromString(call.parameters["mnemonic"]) ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid mnemonic")
            val message = call.parameters["message"] ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid message")

            val signature = Message.sign(privateKey, message)

            call.respond(signature.toString())
        }

        get("/verifymessage/{account}/{signature}/{message}") {
            val pubkey = Address.decode(call.parameters["account"]) ?: return@get call.respond(HttpStatusCode.BadRequest, "invalid account")
            val signature = Signature.fromString(call.parameters["signature"]) ?: return@get call.respond(HttpStatusCode.BadRequest, "invalid signature")
            val message = call.parameters["message"] ?: return@get call.respond(HttpStatusCode.BadRequest, "invalid message")

            val result = Message.verify(pubkey, signature, message)

            call.respond(result.toString())
        }
    }
}
