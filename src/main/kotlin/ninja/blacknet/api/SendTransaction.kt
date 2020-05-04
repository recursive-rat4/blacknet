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

import io.ktor.application.ApplicationCall
import io.ktor.http.HttpStatusCode
import io.ktor.response.respond
import io.ktor.routing.Route
import kotlinx.coroutines.sync.withLock
import kotlinx.serialization.Serializable
import ninja.blacknet.core.Accepted
import ninja.blacknet.core.Transaction
import ninja.blacknet.crypto.Hash
import ninja.blacknet.crypto.Message
import ninja.blacknet.crypto.PrivateKey
import ninja.blacknet.crypto.PublicKey
import ninja.blacknet.db.WalletDB
import ninja.blacknet.ktor.requests.Request
import ninja.blacknet.ktor.requests.get
import ninja.blacknet.ktor.requests.post
import ninja.blacknet.network.Node
import ninja.blacknet.serialization.SerializableByteArray
import ninja.blacknet.transaction.*

fun Route.sendTransaction() {
    @Serializable
    class Transfer(
            val mnemonic: PrivateKey,
            val fee: Long,
            val amount: Long,
            val to: PublicKey,
            val encrypted: Byte? = null,
            val message: String? = null,
            val referenceChain: Hash? = null
    ) : Request {
        override suspend fun handle(call: ApplicationCall): Unit = APIServer.txMutex.withLock {
            val privateKey = mnemonic
            val message = Message.create(message, encrypted, privateKey, to) ?: return call.respond(HttpStatusCode.BadRequest, "Failed to create message")
            val from = privateKey.toPublicKey()
            val seq = WalletDB.getSequence(from)
            val data = Transfer(amount, to, message).serialize()
            val tx = Transaction.create(from, seq, referenceChain ?: WalletDB.referenceChain(), fee, TxType.Transfer.type, data)
            val (hash, bytes) = tx.sign(privateKey)

            val status = Node.broadcastTx(hash, bytes)
            return if (status == Accepted)
                call.respond(hash.toString())
            else
                call.respond(HttpStatusCode.BadRequest, "Transaction rejected: $status")
        }
    }

    post(Transfer.serializer(), "/api/v2/transfer")

    @Serializable
    class Burn(
            val mnemonic: PrivateKey,
            val fee: Long,
            val amount: Long,
            val message: SerializableByteArray,
            val referenceChain: Hash? = null
    ) : Request {
        override suspend fun handle(call: ApplicationCall): Unit = APIServer.txMutex.withLock {
            val privateKey = mnemonic
            val from = privateKey.toPublicKey()
            val seq = WalletDB.getSequence(from)
            val data = Burn(amount, message).serialize()
            val tx = Transaction.create(from, seq, referenceChain ?: WalletDB.referenceChain(), fee, TxType.Burn.type, data)
            val (hash, bytes) = tx.sign(privateKey)

            val status = Node.broadcastTx(hash, bytes)
            return if (status == Accepted)
                call.respond(hash.toString())
            else
                call.respond(HttpStatusCode.BadRequest, "Transaction rejected: $status")
        }
    }

    post(Burn.serializer(), "/api/v2/burn")

    @Serializable
    class Lease(
            val mnemonic: PrivateKey,
            val fee: Long,
            val amount: Long,
            val to: PublicKey,
            val referenceChain: Hash? = null
    ) : Request {
        override suspend fun handle(call: ApplicationCall): Unit = APIServer.txMutex.withLock {
            val privateKey = mnemonic
            val from = privateKey.toPublicKey()
            val seq = WalletDB.getSequence(from)
            val data = Lease(amount, to).serialize()
            val tx = Transaction.create(from, seq, referenceChain ?: WalletDB.referenceChain(), fee, TxType.Lease.type, data)
            val (hash, bytes) = tx.sign(privateKey)

            val status = Node.broadcastTx(hash, bytes)
            return if (status == Accepted)
                call.respond(hash.toString())
            else
                call.respond(HttpStatusCode.BadRequest, "Transaction rejected: $status")
        }
    }

    post(Lease.serializer(), "/api/v2/lease")

    @Serializable
    class CancelLease(
            val mnemonic: PrivateKey,
            val fee: Long,
            val amount: Long,
            val to: PublicKey,
            val height: Int,
            val referenceChain: Hash? = null
    ) : Request {
        override suspend fun handle(call: ApplicationCall): Unit = APIServer.txMutex.withLock {
            val privateKey = mnemonic
            val from = privateKey.toPublicKey()
            val seq = WalletDB.getSequence(from)
            val data = CancelLease(amount, to, height).serialize()
            val tx = Transaction.create(from, seq, referenceChain ?: WalletDB.referenceChain(), fee, TxType.CancelLease.type, data)
            val (hash, bytes) = tx.sign(privateKey)

            val status = Node.broadcastTx(hash, bytes)
            return if (status == Accepted)
                call.respond(hash.toString())
            else
                call.respond(HttpStatusCode.BadRequest, "Transaction rejected: $status")
        }
    }

    post(CancelLease.serializer(), "/api/v2/cancellease")

    @Serializable
    class WithdrawFromLease(
            val mnemonic: PrivateKey,
            val fee: Long,
            val withdraw: Long,
            val amount: Long,
            val to: PublicKey,
            val height: Int,
            val referenceChain: Hash? = null
    ) : Request {
        override suspend fun handle(call: ApplicationCall): Unit = APIServer.txMutex.withLock {
            val privateKey = mnemonic
            val from = privateKey.toPublicKey()
            val seq = WalletDB.getSequence(from)
            val data = WithdrawFromLease(withdraw, amount, to, height).serialize()
            val tx = Transaction.create(from, seq, referenceChain ?: WalletDB.referenceChain(), fee, TxType.WithdrawFromLease.type, data)
            val (hash, bytes) = tx.sign(privateKey)

            val status = Node.broadcastTx(hash, bytes)
            return if (status == Accepted)
                call.respond(hash.toString())
            else
                call.respond(HttpStatusCode.BadRequest, "Transaction rejected: $status")
        }
    }

    post(WithdrawFromLease.serializer(), "/api/v2/withdrawfromlease")

    @Serializable
    class SendRawTransaction(
            val hex: SerializableByteArray
    ) : Request {
        override suspend fun handle(call: ApplicationCall): Unit = APIServer.txMutex.withLock {
            val bytes = hex.array
            val hash = Transaction.hash(bytes)

            val status = Node.broadcastTx(hash, bytes)
            return if (status == Accepted)
                call.respond(hash.toString())
            else
                call.respond(HttpStatusCode.BadRequest, "Transaction rejected: $status")
        }
    }

    get(SendRawTransaction.serializer(), "/api/v2/sendrawtransaction/{hex}/")
}
