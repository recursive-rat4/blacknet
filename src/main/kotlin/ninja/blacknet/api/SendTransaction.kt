/*
 * Copyright (c) 2018-2019 Pavel Vasin
 * Copyright (c) 2019 Blacknet Team
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.api

import io.ktor.application.call
import io.ktor.http.HttpStatusCode
import io.ktor.request.receiveParameters
import io.ktor.response.respond
import io.ktor.routing.Route
import io.ktor.routing.get
import io.ktor.routing.post
import kotlinx.coroutines.sync.withLock
import ninja.blacknet.coding.fromHex
import ninja.blacknet.core.Accepted
import ninja.blacknet.core.Transaction
import ninja.blacknet.crypto.Address
import ninja.blacknet.crypto.Hash
import ninja.blacknet.crypto.Message
import ninja.blacknet.crypto.Mnemonic
import ninja.blacknet.db.WalletDB
import ninja.blacknet.network.Node
import ninja.blacknet.serialization.SerializableByteArray
import ninja.blacknet.transaction.*

fun Route.sendTransaction() {
    post("/api/v2/transfer") {
        val parameters = call.receiveParameters()
        val privateKey = Mnemonic.fromString(parameters["mnemonic"]) ?: return@post call.respond(HttpStatusCode.BadRequest, "Invalid mnemonic")
        val from = privateKey.toPublicKey()
        val fee = parameters["fee"]?.toLongOrNull() ?: return@post call.respond(HttpStatusCode.BadRequest, "Invalid fee")
        val amount = parameters["amount"]?.toLongOrNull() ?: return@post call.respond(HttpStatusCode.BadRequest, "Invalid amount")
        val to = Address.decode(parameters["to"]) ?: return@post call.respond(HttpStatusCode.BadRequest, "Invalid to")
        val encrypted = parameters["encrypted"]?.let { it.toByteOrNull() ?: return@post call.respond(HttpStatusCode.BadRequest, "Invalid encrypted") }
        val message = Message.create(parameters["message"], encrypted, privateKey, to) ?: return@post call.respond(HttpStatusCode.BadRequest, "Failed to create message")
        val referenceChain = parameters["referenceChain"]?.let { Hash.fromString(it) ?: return@post call.respond(HttpStatusCode.BadRequest, "Invalid reference chain") }

        APIServer.txMutex.withLock {
            val seq = WalletDB.getSequence(from) ?: return@post call.respond(HttpStatusCode.BadRequest, "Wallet reached sequence threshold")
            val data = Transfer(amount, to, message).serialize()
            val tx = Transaction.create(from, seq, referenceChain ?: WalletDB.referenceChain(), fee, TxType.Transfer.type, data)
            val (hash, bytes) = tx.sign(privateKey)

            val status = Node.broadcastTx(hash, bytes)
            if (status == Accepted)
                call.respond(hash.toString())
            else
                call.respond(HttpStatusCode.BadRequest, "Transaction rejected: $status")
        }
    }

    post("/api/v2/burn") {
        val parameters = call.receiveParameters()
        val privateKey = Mnemonic.fromString(parameters["mnemonic"]) ?: return@post call.respond(HttpStatusCode.BadRequest, "Invalid mnemonic")
        val from = privateKey.toPublicKey()
        val fee = parameters["fee"]?.toLongOrNull() ?: return@post call.respond(HttpStatusCode.BadRequest, "Invalid fee")
        val amount = parameters["amount"]?.toLongOrNull() ?: return@post call.respond(HttpStatusCode.BadRequest, "Invalid amount")
        val message = SerializableByteArray.fromString(parameters["message"].orEmpty()) ?: return@post call.respond(HttpStatusCode.BadRequest, "Invalid message")
        val referenceChain = parameters["referenceChain"]?.let { Hash.fromString(it) ?: return@post call.respond(HttpStatusCode.BadRequest, "Invalid reference chain") }

        APIServer.txMutex.withLock {
            val seq = WalletDB.getSequence(from) ?: return@post call.respond(HttpStatusCode.BadRequest, "Wallet reached sequence threshold")
            val data = Burn(amount, message).serialize()
            val tx = Transaction.create(from, seq, referenceChain ?: WalletDB.referenceChain(), fee, TxType.Burn.type, data)
            val (hash, bytes) = tx.sign(privateKey)

            val status = Node.broadcastTx(hash, bytes)
            if (status == Accepted)
                call.respond(hash.toString())
            else
                call.respond(HttpStatusCode.BadRequest, "Transaction rejected: $status")
        }
    }

    post("/api/v2/lease") {
        val parameters = call.receiveParameters()
        val privateKey = Mnemonic.fromString(parameters["mnemonic"]) ?: return@post call.respond(HttpStatusCode.BadRequest, "Invalid mnemonic")
        val from = privateKey.toPublicKey()
        val fee = parameters["fee"]?.toLongOrNull() ?: return@post call.respond(HttpStatusCode.BadRequest, "Invalid fee")
        val amount = parameters["amount"]?.toLongOrNull() ?: return@post call.respond(HttpStatusCode.BadRequest, "Invalid amount")
        val to = Address.decode(parameters["to"]) ?: return@post call.respond(HttpStatusCode.BadRequest, "Invalid to")
        val referenceChain = parameters["referenceChain"]?.let { Hash.fromString(it) ?: return@post call.respond(HttpStatusCode.BadRequest, "Invalid reference chain") }

        APIServer.txMutex.withLock {
            val seq = WalletDB.getSequence(from) ?: return@post call.respond(HttpStatusCode.BadRequest, "Wallet reached sequence threshold")
            val data = Lease(amount, to).serialize()
            val tx = Transaction.create(from, seq, referenceChain ?: WalletDB.referenceChain(), fee, TxType.Lease.type, data)
            val (hash, bytes) = tx.sign(privateKey)

            val status = Node.broadcastTx(hash, bytes)
            if (status == Accepted)
                call.respond(hash.toString())
            else
                call.respond(HttpStatusCode.BadRequest, "Transaction rejected: $status")
        }
    }

    post("/api/v2/cancellease") {
        val parameters = call.receiveParameters()
        val privateKey = Mnemonic.fromString(parameters["mnemonic"]) ?: return@post call.respond(HttpStatusCode.BadRequest, "Invalid mnemonic")
        val from = privateKey.toPublicKey()
        val fee = parameters["fee"]?.toLongOrNull() ?: return@post call.respond(HttpStatusCode.BadRequest, "Invalid fee")
        val amount = parameters["amount"]?.toLongOrNull() ?: return@post call.respond(HttpStatusCode.BadRequest, "Invalid amount")
        val to = Address.decode(parameters["to"]) ?: return@post call.respond(HttpStatusCode.BadRequest, "Invalid to")
        val height = parameters["height"]?.toIntOrNull() ?: return@post call.respond(HttpStatusCode.BadRequest, "Invalid height")
        val referenceChain = parameters["referenceChain"]?.let { Hash.fromString(it) ?: return@post call.respond(HttpStatusCode.BadRequest, "Invalid reference chain") }

        APIServer.txMutex.withLock {
            val seq = WalletDB.getSequence(from) ?: return@post call.respond(HttpStatusCode.BadRequest, "Wallet reached sequence threshold")
            val data = CancelLease(amount, to, height).serialize()
            val tx = Transaction.create(from, seq, referenceChain ?: WalletDB.referenceChain(), fee, TxType.CancelLease.type, data)
            val (hash, bytes) = tx.sign(privateKey)

            val status = Node.broadcastTx(hash, bytes)
            if (status == Accepted)
                call.respond(hash.toString())
            else
                call.respond(HttpStatusCode.BadRequest, "Transaction rejected: $status")
        }
    }

    post("/api/v2/withdrawfromlease") {
        val parameters = call.receiveParameters()
        val privateKey = Mnemonic.fromString(parameters["mnemonic"]) ?: return@post call.respond(HttpStatusCode.BadRequest, "Invalid mnemonic")
        val from = privateKey.toPublicKey()
        val fee = parameters["fee"]?.toLongOrNull() ?: return@post call.respond(HttpStatusCode.BadRequest, "Invalid fee")
        val withdraw = parameters["withdraw"]?.toLongOrNull() ?: return@post call.respond(HttpStatusCode.BadRequest, "Invalid withdraw")
        val amount = parameters["amount"]?.toLongOrNull() ?: return@post call.respond(HttpStatusCode.BadRequest, "Invalid amount")
        val to = Address.decode(parameters["to"]) ?: return@post call.respond(HttpStatusCode.BadRequest, "Invalid to")
        val height = parameters["height"]?.toIntOrNull() ?: return@post call.respond(HttpStatusCode.BadRequest, "Invalid height")
        val referenceChain = parameters["referenceChain"]?.let { Hash.fromString(it) ?: return@post call.respond(HttpStatusCode.BadRequest, "Invalid reference chain") }

        APIServer.txMutex.withLock {
            val seq = WalletDB.getSequence(from) ?: return@post call.respond(HttpStatusCode.BadRequest, "Wallet reached sequence threshold")
            val data = WithdrawFromLease(withdraw, amount, to, height).serialize()
            val tx = Transaction.create(from, seq, referenceChain ?: WalletDB.referenceChain(), fee, TxType.WithdrawFromLease.type, data)
            val (hash, bytes) = tx.sign(privateKey)

            val status = Node.broadcastTx(hash, bytes)
            if (status == Accepted)
                call.respond(hash.toString())
            else
                call.respond(HttpStatusCode.BadRequest, "Transaction rejected: $status")
        }
    }

    get("/api/v2/sendrawtransaction/{hex}/") {
        val bytes = call.parameters["hex"]?.let { fromHex(it) } ?: return@get call.respond(HttpStatusCode.BadRequest, "Invalid hex")

        APIServer.txMutex.withLock {
            val hash = Transaction.hash(bytes)
            val status = Node.broadcastTx(hash, bytes)
            if (status == Accepted)
                call.respond(hash.toString())
            else
                call.respond(HttpStatusCode.BadRequest, "Transaction rejected: $status")
        }
    }
}
