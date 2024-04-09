/*
 * Copyright (c) 2018-2024 Pavel Vasin
 * Copyright (c) 2019 Blacknet Team
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.rpc.v2

import io.ktor.server.routing.Route
import kotlinx.atomicfu.locks.withLock
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import ninja.blacknet.contract.BAppId
import ninja.blacknet.contract.HashLock
import ninja.blacknet.contract.HashTimeLockContractId
import ninja.blacknet.contract.TimeLock
import ninja.blacknet.core.Accepted
import ninja.blacknet.core.Transaction
import ninja.blacknet.crypto.Ed25519
import ninja.blacknet.crypto.Hash
import ninja.blacknet.crypto.PaymentId
import ninja.blacknet.crypto.PrivateKeySerializer
import ninja.blacknet.crypto.PublicKey
import ninja.blacknet.db.WalletDB
import ninja.blacknet.network.Node
import ninja.blacknet.rpc.requests.*
import ninja.blacknet.serialization.bbf.binaryFormat
import ninja.blacknet.serialization.ByteArraySerializer
import ninja.blacknet.transaction.*

@Serializable
class TransferRequest(
    @SerialName("mnemonic")
    @Serializable(with = PrivateKeySerializer::class)
    val privateKey: ByteArray,
    val fee: Long,
    val amount: Long,
    val to: PublicKey,
    val encrypted: Byte? = null,
    val message: String? = null,
    @SerialName("referenceChain")
    val anchor: Hash? = null,
) : Request {
    override fun handle(): TextContent = WalletDB.txLock.withLock {
        val message = PaymentId.create(message, encrypted, privateKey, to) ?: return respondError("Failed to create payment id")
        val from = Ed25519.toPublicKey(privateKey)
        val seq = WalletDB.getSequence(from)
        val data = binaryFormat.encodeToByteArray(Transfer.serializer(), Transfer(amount, to, message))
        val tx = Transaction.create(from, seq, anchor ?: WalletDB.anchor(), fee, TxType.Transfer.type, data)
        val (hash, bytes) = tx.sign(privateKey)

        val status = Node.broadcastTx(hash, bytes)
        return if (status == Accepted)
            respondText(hash.toString())
        else
            respondError("Transaction rejected: $status")
    }
}

@Serializable
class BurnRequest(
    @SerialName("mnemonic")
    @Serializable(with = PrivateKeySerializer::class)
    val privateKey: ByteArray,
    val fee: Long,
    val amount: Long,
    @Serializable(with = ByteArraySerializer::class)
    val message: ByteArray,
    @SerialName("referenceChain")
    val anchor: Hash? = null,
) : Request {
    override fun handle(): TextContent = WalletDB.txLock.withLock {
        val from = Ed25519.toPublicKey(privateKey)
        val seq = WalletDB.getSequence(from)
        val data = binaryFormat.encodeToByteArray(Burn.serializer(), Burn(amount, message))
        val tx = Transaction.create(from, seq, anchor ?: WalletDB.anchor(), fee, TxType.Burn.type, data)
        val (hash, bytes) = tx.sign(privateKey)

        val status = Node.broadcastTx(hash, bytes)
        return if (status == Accepted)
            respondText(hash.toString())
        else
            respondError("Transaction rejected: $status")
    }
}

@Serializable
class LeaseRequest(
    @SerialName("mnemonic")
    @Serializable(with = PrivateKeySerializer::class)
    val privateKey: ByteArray,
    val fee: Long,
    val amount: Long,
    val to: PublicKey,
    @SerialName("referenceChain")
    val anchor: Hash? = null,
) : Request {
    override fun handle(): TextContent = WalletDB.txLock.withLock {
        val from = Ed25519.toPublicKey(privateKey)
        val seq = WalletDB.getSequence(from)
        val data = binaryFormat.encodeToByteArray(Lease.serializer(), Lease(amount, to))
        val tx = Transaction.create(from, seq, anchor ?: WalletDB.anchor(), fee, TxType.Lease.type, data)
        val (hash, bytes) = tx.sign(privateKey)

        val status = Node.broadcastTx(hash, bytes)
        return if (status == Accepted)
            respondText(hash.toString())
        else
            respondError("Transaction rejected: $status")
    }
}

@Serializable
class CancelLeaseRequest(
    @SerialName("mnemonic")
    @Serializable(with = PrivateKeySerializer::class)
    val privateKey: ByteArray,
    val fee: Long,
    val amount: Long,
    val to: PublicKey,
    val height: Int,
    @SerialName("referenceChain")
    val anchor: Hash? = null,
) : Request {
    override fun handle(): TextContent = WalletDB.txLock.withLock {
        val from = Ed25519.toPublicKey(privateKey)
        val seq = WalletDB.getSequence(from)
        val data = binaryFormat.encodeToByteArray(CancelLease.serializer(), CancelLease(amount, to, height))
        val tx = Transaction.create(from, seq, anchor ?: WalletDB.anchor(), fee, TxType.CancelLease.type, data)
        val (hash, bytes) = tx.sign(privateKey)

        val status = Node.broadcastTx(hash, bytes)
        return if (status == Accepted)
            respondText(hash.toString())
        else
            respondError("Transaction rejected: $status")
    }
}

@Serializable
class WithdrawFromLeaseRequest(
    @SerialName("mnemonic")
    @Serializable(with = PrivateKeySerializer::class)
    val privateKey: ByteArray,
    val fee: Long,
    val withdraw: Long,
    val amount: Long,
    val to: PublicKey,
    val height: Int,
    @SerialName("referenceChain")
    val anchor: Hash? = null,
) : Request {
    override fun handle(): TextContent = WalletDB.txLock.withLock {
        val from = Ed25519.toPublicKey(privateKey)
        val seq = WalletDB.getSequence(from)
        val data = binaryFormat.encodeToByteArray(WithdrawFromLease.serializer(), WithdrawFromLease(withdraw, amount, to, height))
        val tx = Transaction.create(from, seq, anchor ?: WalletDB.anchor(), fee, TxType.WithdrawFromLease.type, data)
        val (hash, bytes) = tx.sign(privateKey)

        val status = Node.broadcastTx(hash, bytes)
        return if (status == Accepted)
            respondText(hash.toString())
        else
            respondError("Transaction rejected: $status")
    }
}

@Serializable
class BundleRequest(
    @SerialName("mnemonic")
    @Serializable(with = PrivateKeySerializer::class)
    val privateKey: ByteArray,
    val fee: Long,
    val id: BAppId,
    @Serializable(with = ByteArraySerializer::class)
    val data: ByteArray,
    @SerialName("referenceChain")
    val anchor: Hash? = null,
) : Request {
    override fun handle(): TextContent = WalletDB.txLock.withLock {
        val from = Ed25519.toPublicKey(privateKey)
        val seq = WalletDB.getSequence(from)
        val data = binaryFormat.encodeToByteArray(BApp.serializer(), BApp(id, data))
        val tx = Transaction.create(from, seq, anchor ?: WalletDB.anchor(), fee, TxType.BApp.type, data)
        val (hash, bytes) = tx.sign(privateKey)

        val status = Node.broadcastTx(hash, bytes)
        return if (status == Accepted)
            respondText(hash.toString())
        else
            respondError("Transaction rejected: $status")
    }
}

@Serializable
class CreateSwapRequest(
    @SerialName("mnemonic")
    @Serializable(with = PrivateKeySerializer::class)
    val privateKey: ByteArray,
    val fee: Long,
    val amount: Long,
    val to: PublicKey,
    val timeLockType: Byte,
    val timeLockData: Long,
    val hashLockType: Byte,
    @Serializable(with = ByteArraySerializer::class)
    val hashLockData: ByteArray,
    @SerialName("referenceChain")
    val anchor: Hash? = null,
) : Request {
    override fun handle(): TextContent = WalletDB.txLock.withLock {
        val timeLock = TimeLock(timeLockType, timeLockData).also { it.validate() }
        val hashLock = HashLock(hashLockType, hashLockData).also { it.validate() }
        val from = Ed25519.toPublicKey(privateKey)
        val seq = WalletDB.getSequence(from)
        val data = binaryFormat.encodeToByteArray(CreateHTLC.serializer(), CreateHTLC(amount, to, timeLock, hashLock))
        val tx = Transaction.create(from, seq, anchor ?: WalletDB.anchor(), fee, TxType.CreateHTLC.type, data)
        val (hash, bytes) = tx.sign(privateKey)

        val status = Node.broadcastTx(hash, bytes)
        return if (status == Accepted)
            respondText(hash.toString())
        else
            respondError("Transaction rejected: $status")
    }
}

@Serializable
class ClaimSwapRequest(
    @SerialName("mnemonic")
    @Serializable(with = PrivateKeySerializer::class)
    val privateKey: ByteArray,
    val fee: Long,
    val id: HashTimeLockContractId,
    @Serializable(with = ByteArraySerializer::class)
    val preimage: ByteArray,
    @SerialName("referenceChain")
    val anchor: Hash? = null,
) : Request {
    override fun handle(): TextContent = WalletDB.txLock.withLock {
        val from = Ed25519.toPublicKey(privateKey)
        val seq = WalletDB.getSequence(from)
        val data = binaryFormat.encodeToByteArray(ClaimHTLC.serializer(), ClaimHTLC(id, preimage))
        val tx = Transaction.create(from, seq, anchor ?: WalletDB.anchor(), fee, TxType.ClaimHTLC.type, data)
        val (hash, bytes) = tx.sign(privateKey)

        val status = Node.broadcastTx(hash, bytes)
        return if (status == Accepted)
            respondText(hash.toString())
        else
            respondError("Transaction rejected: $status")
    }
}

@Serializable
class RefundSwapRequest(
    @SerialName("mnemonic")
    @Serializable(with = PrivateKeySerializer::class)
    val privateKey: ByteArray,
    val fee: Long,
    val id: HashTimeLockContractId,
    @SerialName("referenceChain")
    val anchor: Hash? = null,
) : Request {
    override fun handle(): TextContent = WalletDB.txLock.withLock {
        val from = Ed25519.toPublicKey(privateKey)
        val seq = WalletDB.getSequence(from)
        val data = binaryFormat.encodeToByteArray(RefundHTLC.serializer(), RefundHTLC(id))
        val tx = Transaction.create(from, seq, anchor ?: WalletDB.anchor(), fee, TxType.RefundHTLC.type, data)
        val (hash, bytes) = tx.sign(privateKey)

        val status = Node.broadcastTx(hash, bytes)
        return if (status == Accepted)
            respondText(hash.toString())
        else
            respondError("Transaction rejected: $status")
    }
}

@Serializable
class SendRawTransaction(
    @SerialName("hex")
    @Serializable(with = ByteArraySerializer::class)
    val bytes: ByteArray
) : Request {
    override fun handle(): TextContent = WalletDB.txLock.withLock {
        val hash = Transaction.hash(bytes)

        val status = Node.broadcastTx(hash, bytes)
        return if (status == Accepted)
            respondText(hash.toString())
        else
            respondError("Transaction rejected: $status")
    }
}

fun Route.sendTransaction() {
    post(TransferRequest.serializer(), "/api/v2/transfer")

    post(BurnRequest.serializer(), "/api/v2/burn")

    post(LeaseRequest.serializer(), "/api/v2/lease")

    post(CancelLeaseRequest.serializer(), "/api/v2/cancellease")

    post(WithdrawFromLeaseRequest.serializer(), "/api/v2/withdrawfromlease")

    post(BundleRequest.serializer(), "/api/v2/bundle")

    post(CreateSwapRequest.serializer(), "/api/v2/createswap")

    post(ClaimSwapRequest.serializer(), "/api/v2/claimswap")

    post(RefundSwapRequest.serializer(), "/api/v2/refundswap")

    get(SendRawTransaction.serializer(), "/api/v2/sendrawtransaction/{hex}")
}
