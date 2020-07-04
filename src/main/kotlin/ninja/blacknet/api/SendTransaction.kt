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
import ninja.blacknet.crypto.PaymentId
import ninja.blacknet.crypto.PrivateKey
import ninja.blacknet.crypto.PublicKey
import ninja.blacknet.db.WalletDB
import ninja.blacknet.ktor.requests.Request
import ninja.blacknet.ktor.requests.get
import ninja.blacknet.ktor.requests.post
import ninja.blacknet.network.Node
import ninja.blacknet.serialization.BinaryEncoder
import ninja.blacknet.serialization.ByteArraySerializer
import ninja.blacknet.transaction.*

fun Route.sendTransaction() {
    @Serializable
    class TransferRequest(
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
            val message = PaymentId.create(message, encrypted, privateKey, to) ?: return call.respond(HttpStatusCode.BadRequest, "Failed to create payment id")
            val from = privateKey.toPublicKey()
            val seq = WalletDB.getSequence(from)
            val data = BinaryEncoder.toBytes(Transfer.serializer(), Transfer(amount, to, message))
            val tx = Transaction.create(from, seq, referenceChain ?: WalletDB.referenceChain(), fee, TxType.Transfer.type, data)
            val (hash, bytes) = tx.sign(privateKey)

            val status = Node.broadcastTx(hash, bytes)
            return if (status == Accepted)
                call.respond(hash.toString())
            else
                call.respond(HttpStatusCode.BadRequest, "Transaction rejected: $status")
        }
    }

    post(TransferRequest.serializer(), "/api/v2/transfer")

    @Serializable
    class BurnRequest(
            val mnemonic: PrivateKey,
            val fee: Long,
            val amount: Long,
            @Serializable(with = ByteArraySerializer::class)
            val message: ByteArray,
            val referenceChain: Hash? = null
    ) : Request {
        override suspend fun handle(call: ApplicationCall): Unit = APIServer.txMutex.withLock {
            val privateKey = mnemonic
            val from = privateKey.toPublicKey()
            val seq = WalletDB.getSequence(from)
            val data = BinaryEncoder.toBytes(Burn.serializer(), Burn(amount, message))
            val tx = Transaction.create(from, seq, referenceChain ?: WalletDB.referenceChain(), fee, TxType.Burn.type, data)
            val (hash, bytes) = tx.sign(privateKey)

            val status = Node.broadcastTx(hash, bytes)
            return if (status == Accepted)
                call.respond(hash.toString())
            else
                call.respond(HttpStatusCode.BadRequest, "Transaction rejected: $status")
        }
    }

    post(BurnRequest.serializer(), "/api/v2/burn")

    @Serializable
    class LeaseRequest(
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
            val data = BinaryEncoder.toBytes(Lease.serializer(), Lease(amount, to))
            val tx = Transaction.create(from, seq, referenceChain ?: WalletDB.referenceChain(), fee, TxType.Lease.type, data)
            val (hash, bytes) = tx.sign(privateKey)

            val status = Node.broadcastTx(hash, bytes)
            return if (status == Accepted)
                call.respond(hash.toString())
            else
                call.respond(HttpStatusCode.BadRequest, "Transaction rejected: $status")
        }
    }

    post(LeaseRequest.serializer(), "/api/v2/lease")

    @Serializable
    class CancelLeaseRequest(
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
            val data = BinaryEncoder.toBytes(CancelLease.serializer(), CancelLease(amount, to, height))
            val tx = Transaction.create(from, seq, referenceChain ?: WalletDB.referenceChain(), fee, TxType.CancelLease.type, data)
            val (hash, bytes) = tx.sign(privateKey)

            val status = Node.broadcastTx(hash, bytes)
            return if (status == Accepted)
                call.respond(hash.toString())
            else
                call.respond(HttpStatusCode.BadRequest, "Transaction rejected: $status")
        }
    }

    post(CancelLeaseRequest.serializer(), "/api/v2/cancellease")

    @Serializable
    class WithdrawFromLeaseRequest(
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
            val data = BinaryEncoder.toBytes(WithdrawFromLease.serializer(), WithdrawFromLease(withdraw, amount, to, height))
            val tx = Transaction.create(from, seq, referenceChain ?: WalletDB.referenceChain(), fee, TxType.WithdrawFromLease.type, data)
            val (hash, bytes) = tx.sign(privateKey)

            val status = Node.broadcastTx(hash, bytes)
            return if (status == Accepted)
                call.respond(hash.toString())
            else
                call.respond(HttpStatusCode.BadRequest, "Transaction rejected: $status")
        }
    }

    post(WithdrawFromLeaseRequest.serializer(), "/api/v2/withdrawfromlease")

    @Serializable
    class SendRawTransaction(
            @Serializable(with = ByteArraySerializer::class)
            val hex: ByteArray
    ) : Request {
        override suspend fun handle(call: ApplicationCall): Unit = APIServer.txMutex.withLock {
            val bytes = hex
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
