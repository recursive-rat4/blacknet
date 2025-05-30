/*
 * Copyright (c) 2018-2024 Pavel Vasin
 * Copyright (c) 2019 Blacknet Team
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

@file:Suppress("DEPRECATION")

package ninja.blacknet.rpc.v1

import io.ktor.http.HttpStatusCode
import io.ktor.server.application.call
import io.ktor.server.response.respond
import io.ktor.server.routing.Route
import io.ktor.server.routing.get
import io.ktor.server.routing.post
import io.ktor.server.websocket.webSocket
import io.ktor.websocket.CloseReason
import io.ktor.websocket.Frame
import io.ktor.websocket.close
import io.ktor.websocket.readText
import kotlin.math.abs
import kotlin.concurrent.withLock
import kotlinx.coroutines.channels.ClosedReceiveChannelException
import kotlinx.coroutines.runBlocking
import kotlinx.coroutines.sync.withLock
import kotlinx.serialization.builtins.*
import ninja.blacknet.Kernel
import ninja.blacknet.rpc.*
import ninja.blacknet.core.*
import ninja.blacknet.crypto.*
import ninja.blacknet.db.*
import ninja.blacknet.mode
import ninja.blacknet.network.Network
import ninja.blacknet.network.Node
import ninja.blacknet.serialization.bbf.*
import ninja.blacknet.transaction.*
import ninja.blacknet.util.startInterruptible

fun Route.APIV1() {
    webSocket("/api/v1/notify/block") {
        try {
            RPCServerV1.blockNotifyV0.add(outgoing)
            while (true) {
                incoming.receive()
            }
        } catch (e: ClosedReceiveChannelException) {
        } finally {
            RPCServerV1.blockNotifyV0.remove(outgoing)
        }
    }

    webSocket("/api/v2/notify/block") {
        try {
            RPCServerV1.blockNotifyV1.add(outgoing)
            while (true) {
                incoming.receive()
            }
        } catch (e: ClosedReceiveChannelException) {
        } finally {
            RPCServerV1.blockNotifyV1.remove(outgoing)
        }
    }

    webSocket("/api/v1/notify/transaction") {
        try {
            while (true) {
                val string = (incoming.receive() as Frame.Text).readText()
                @Suppress("USELESS_ELVIS")
                val publicKey = PublicKey(Address.decode(string)) ?: return@webSocket this.close(CloseReason(CloseReason.Codes.PROTOCOL_ERROR, "invalid account"))

                RPCServerV1.walletNotifyV1.mutex.withLock<Unit> {
                    val keys = RPCServerV1.walletNotifyV1.map.get(outgoing)
                    if (keys == null) {
                        @Suppress("NAME_SHADOWING")
                        val keys = HashSet<PublicKey>()
                        keys.add(publicKey)
                        RPCServerV1.walletNotifyV1.map.put(outgoing, keys)
                    } else {
                        keys.add(publicKey)
                    }
                }
            }
        } catch (e: ClosedReceiveChannelException) {
        } finally {
            RPCServerV1.walletNotifyV1.remove(outgoing)
        }
    }

    get("/api/v1/peerinfo") {
        call.respond(Json.stringify(ListSerializer(PeerInfoV1.serializer()), PeerInfoV1.getAll()))
    }

    get("/api/v1/nodeinfo") {
        call.respond(Json.stringify(NodeInfo.serializer(), NodeInfo.get()))
    }

    get("/api/v1/peerdb") {
        call.respond(Json.stringify(PeerDBInfo.serializer(), PeerDBInfo.get()))
    }

    get("/api/v1/peerdb/networkstat") {
        call.respond(Json.stringify(PeerDBInfo.serializer(), PeerDBInfo.get(true)))
    }

    get("/api/v1/leveldb/stats") {
        call.respond(LevelDB.getProperty("leveldb.stats") ?: "Not implemented")
    }

    get("/api/v1/blockdb/get/{hash}/{txdetail?}") {
        val hash = call.parameters["hash"]?.let { Hash(Hash.fromString(it)) } ?: return@get call.respond(HttpStatusCode.BadRequest, "invalid hash")
        val txdetail = call.parameters["txdetail"]?.toBoolean() ?: false

        val result = BlockInfoV1.get(hash, txdetail)
        if (result != null)
            call.respond(Json.stringify(BlockInfoV1.serializer(), result))
        else
            call.respond(HttpStatusCode.NotFound, "block not found")
    }

    get("/api/v2/blockdb/get/{hash}/{txdetail?}") {
        val hash = call.parameters["hash"]?.let { Hash(Hash.fromString(it)) } ?: return@get call.respond(HttpStatusCode.BadRequest, "invalid hash")
        val txdetail = call.parameters["txdetail"]?.toBoolean() ?: false

        val result = Kernel.blockDB().blocks.getWithSize(hash.bytes)
        if (result != null)
            call.respond(Json.stringify(BlockInfoV2.serializer(), BlockInfoV2(result.first, hash, result.second, txdetail)))
        else
            call.respond(HttpStatusCode.NotFound, "block not found")
    }

    get("/api/v1/blockdb/getblockhash/{height}") {
        val height = call.parameters["height"]?.toIntOrNull() ?: return@get call.respond(HttpStatusCode.BadRequest, "invalid height")

        Kernel.blockDB().reentrant.readLock().withLock { runBlocking {
            val state = CoinDB.state()
            if (height < 0 || height > state.height)
                return@runBlocking call.respond(HttpStatusCode.NotFound, "block not found")
            else if (height == 0)
                return@runBlocking call.respond(Genesis.BLOCK_HASH.toString())
            else if (height == state.height)
                return@runBlocking call.respond(state.blockHash.toString())

            if (RPCServer.lastIndex != null && RPCServer.lastIndex!!.second.height == height)
                return@runBlocking call.respond(RPCServer.lastIndex!!.first.toString())

            var hash: Hash
            var index: BlockIndex
            if (height < state.height / 2) {
                hash = Genesis.BLOCK_HASH
                index = CoinDB.blockIndexes.getOrThrow(hash.bytes)
            } else {
                hash = state.blockHash
                index = CoinDB.blockIndexes.getOrThrow(hash.bytes)
            }
            if (RPCServer.lastIndex != null && abs(height - index.height) > abs(height - RPCServer.lastIndex!!.second.height))
                index = RPCServer.lastIndex!!.second
            while (index.height > height) {
                hash = index.previous
                index = CoinDB.blockIndexes.getOrThrow(hash.bytes)
            }
            while (index.height < height) {
                hash = index.next
                index = CoinDB.blockIndexes.getOrThrow(hash.bytes)
            }
            if (index.height < state.height - PoS.ROLLBACK_LIMIT + 1)
                RPCServer.lastIndex = Pair(hash, index)
            call.respond(hash.toString())
        }}
    }

    get("/api/v1/blockdb/getblockindex/{hash}/") {
        val hash = call.parameters["hash"]?.let { Hash(Hash.fromString(it)) } ?: return@get call.respond(HttpStatusCode.BadRequest, "invalid hash")

        val result = CoinDB.blockIndexes.get(hash.bytes)
        if (result != null)
            call.respond(Json.stringify(BlockIndex.serializer(), result))
        else
            call.respond(HttpStatusCode.NotFound, "block not found")
    }

    get("/api/v1/blockdb/makebootstrap") {
        val file = Bootstrap.export()
        if (file != null)
            call.respond(file.toAbsolutePath().toString())
        else
            call.respond(HttpStatusCode.BadRequest, "not synchronized")
    }

    get("/api/v1/ledger") {
        call.respond(Json.stringify(LedgerInfo.serializer(), LedgerInfo.get()))
    }

    get("/api/v1/ledger/get/{account}/{confirmations?}") {
        val publicKey = call.parameters["account"]?.let { PublicKey(Address.decode(it)) } ?: return@get call.respond(HttpStatusCode.BadRequest, "invalid account")
        val confirmations = call.parameters["confirmations"]?.toIntOrNull() ?: PoS.DEFAULT_CONFIRMATIONS
        val result = AccountInfoV1.get(publicKey, confirmations)
        if (result != null)
            call.respond(Json.stringify(AccountInfoV1.serializer(), result))
        else
            call.respond(HttpStatusCode.NotFound, "account not found")
    }

    get("/api/v1/ledger/check") {
        call.respond(Json.stringify(CoinDB.Check.serializer(), CoinDB.check()))
    }

    get("/api/v1/txpool") {
        call.respond(Json.stringify(TxPoolInfo.serializer(), TxPoolInfo.get()))
    }

    get("/api/v1/account/generate") {
        val wordlist = Wordlists.get("english")

        call.respond(Json.stringify(NewMnemonicInfo.serializer(), NewMnemonicInfo.new(wordlist)))
    }

    get("/api/v1/address/info/{address}") {
        val info = call.parameters["address"]?.let { AddressInfo.fromString(it) } ?: return@get call.respond(HttpStatusCode.BadRequest, "invalid address")

        call.respond(Json.stringify(AddressInfo.serializer(), info))
    }

    post("/api/v1/mnemonic/info/{mnemonic}") {
        val info = call.parameters["mnemonic"]?.let { NewMnemonicInfo.fromString(it) } ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid mnemonic")

        call.respond(Json.stringify(NewMnemonicInfo.serializer(), info))
    }

    post("/api/v1/transfer/{mnemonic}/{fee}/{amount}/{to}/{message?}/{encrypted?}/{blockHash?}") {
        val privateKey = call.parameters["mnemonic"]?.let { Mnemonic.fromString(it) } ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid mnemonic")
        val from = Ed25519.toPublicKey(privateKey)
        val fee = call.parameters["fee"]?.toLongOrNull() ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid fee")
        val amount = call.parameters["amount"]?.toLongOrNull() ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid amount")
        val to = call.parameters["to"]?.let { PublicKey(Address.decode(it)) } ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid to")
        val encrypted = call.parameters["encrypted"]?.let { it.toByteOrNull() ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid encrypted") }
        val message = PaymentId.create(call.parameters["message"], encrypted, privateKey, to) ?: return@post call.respond(HttpStatusCode.BadRequest, "failed to create message")
        val blockHash = call.parameters["blockHash"]?.let @Suppress("USELESS_ELVIS") { Hash(Hash.fromString(it)) ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid blockHash") }

        WalletDB.txLock.withLock {
            @Suppress("USELESS_ELVIS")
            val seq = WalletDB.getSequence(from) ?: return@post runBlocking { call.respond(HttpStatusCode.BadRequest, "wallet reached sequence threshold") }
            val data = binaryFormat.encodeToByteArray(Transfer.serializer(), Transfer(amount, to, message))
            val tx = Transaction.create(from, seq, blockHash
                    ?: WalletDB.anchor(), fee, TxType.Transfer.type, data)
            val signed = tx.sign(privateKey)

            if (Node.broadcastTx(signed.first, signed.second) == Accepted)
                runBlocking { call.respond(signed.first.bytes.toHex()) }
            else
                runBlocking { call.respond("Transaction rejected") }
        }
    }

    post("/api/v1/burn/{mnemonic}/{fee}/{amount}/{message?}/{blockHash?}") {
        val privateKey = call.parameters["mnemonic"]?.let { Mnemonic.fromString(it) } ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid mnemonic")
        val from = Ed25519.toPublicKey(privateKey)
        val fee = call.parameters["fee"]?.toLongOrNull() ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid fee")
        val amount = call.parameters["amount"]?.toLongOrNull() ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid amount")
        val message = call.parameters["message"]?.let { fromHex(it) } ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid message")
        val blockHash = call.parameters["blockHash"]?.let @Suppress("USELESS_ELVIS") { Hash(Hash.fromString(it)) ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid blockHash") }

        WalletDB.txLock.withLock {
            @Suppress("USELESS_ELVIS")
            val seq = WalletDB.getSequence(from) ?: return@post runBlocking { call.respond(HttpStatusCode.BadRequest, "wallet reached sequence threshold") }
            val data = binaryFormat.encodeToByteArray(Burn.serializer(), Burn(amount, message))
            val tx = Transaction.create(from, seq, blockHash ?: WalletDB.anchor(), fee, TxType.Burn.type, data)
            val signed = tx.sign(privateKey)

            if (Node.broadcastTx(signed.first, signed.second) == Accepted)
                runBlocking { call.respond(signed.first.bytes.toHex()) }
            else
                runBlocking { call.respond("Transaction rejected") }
        }
    }

    post("/api/v1/lease/{mnemonic}/{fee}/{amount}/{to}/{blockHash?}") {
        val privateKey = call.parameters["mnemonic"]?.let { Mnemonic.fromString(it) } ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid mnemonic")
        val from = Ed25519.toPublicKey(privateKey)
        val fee = call.parameters["fee"]?.toLongOrNull() ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid fee")
        val amount = call.parameters["amount"]?.toLongOrNull() ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid amount")
        val to = call.parameters["to"]?.let { PublicKey(Address.decode(it)) } ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid to")
        val blockHash = call.parameters["blockHash"]?.let @Suppress("USELESS_ELVIS") { Hash(Hash.fromString(it)) ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid blockHash") }

        WalletDB.txLock.withLock {
            @Suppress("USELESS_ELVIS")
            val seq = WalletDB.getSequence(from) ?: return@post runBlocking { call.respond(HttpStatusCode.BadRequest, "wallet reached sequence threshold") }
            val data = binaryFormat.encodeToByteArray(Lease.serializer(), Lease(amount, to))
            val tx = Transaction.create(from, seq, blockHash ?: WalletDB.anchor(), fee, TxType.Lease.type, data)
            val signed = tx.sign(privateKey)

            if (Node.broadcastTx(signed.first, signed.second) == Accepted)
                runBlocking { call.respond(signed.first.bytes.toHex()) }
            else
                runBlocking { call.respond("Transaction rejected") }
        }
    }

    post("/api/v1/cancellease/{mnemonic}/{fee}/{amount}/{to}/{height}/{blockHash?}") {
        val privateKey = call.parameters["mnemonic"]?.let { Mnemonic.fromString(it) } ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid mnemonic")
        val from = Ed25519.toPublicKey(privateKey)
        val fee = call.parameters["fee"]?.toLongOrNull() ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid fee")
        val amount = call.parameters["amount"]?.toLongOrNull() ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid amount")
        val to = call.parameters["to"]?.let { PublicKey(Address.decode(it)) } ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid to")
        val height = call.parameters["height"]?.toIntOrNull() ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid height")
        val blockHash = call.parameters["blockHash"]?.let @Suppress("USELESS_ELVIS") { Hash(Hash.fromString(it)) ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid blockHash") }

        WalletDB.txLock.withLock {
            @Suppress("USELESS_ELVIS")
            val seq = WalletDB.getSequence(from) ?: return@post runBlocking { call.respond(HttpStatusCode.BadRequest, "wallet reached sequence threshold") }
            val data = binaryFormat.encodeToByteArray(CancelLease.serializer(), CancelLease(amount, to, height))
            val tx = Transaction.create(from, seq, blockHash
                    ?: WalletDB.anchor(), fee, TxType.CancelLease.type, data)
            val signed = tx.sign(privateKey)

            if (Node.broadcastTx(signed.first, signed.second) == Accepted)
                runBlocking { call.respond(signed.first.bytes.toHex()) }
            else
                runBlocking { call.respond("Transaction rejected") }
        }
    }

    get("/api/v1/transaction/raw/send/{serialized}") {
        val serialized = call.parameters["serialized"]?.let { fromHex(it) } ?: return@get call.respond(HttpStatusCode.BadRequest, "invalid serialized")
        val hash = Transaction.hash(serialized)

        WalletDB.txLock.withLock {
            if (Node.broadcastTx(hash, serialized) == Accepted)
                runBlocking { call.respond(hash.bytes.toHex()) }
            else
                runBlocking { call.respond("Transaction rejected") }
        }
    }

    post("/api/v1/decryptmessage/{mnemonic}/{from}/{message}") {
        val privateKey = call.parameters["mnemonic"]?.let { Mnemonic.fromString(it) } ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid mnemonic")
        val publicKey = call.parameters["from"]?.let { PublicKey(Address.decode(it)) } ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid from")
        val message = call.parameters["message"] ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid message")

        val decrypted = PaymentId.decrypt(privateKey, publicKey, message)
        if (decrypted != null)
            call.respond(decrypted)
        else
            call.respond(HttpStatusCode.NotFound, "Decryption failed")
    }

    post("/api/v1/signmessage/{mnemonic}/{message}") {
        val privateKey = call.parameters["mnemonic"]?.let { Mnemonic.fromString(it) } ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid mnemonic")
        val message = call.parameters["message"] ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid message")

        val signature = Message.sign(privateKey, message)

        call.respond(signature.toHex())
    }

    get("/api/v1/verifymessage/{account}/{signature}/{message}") {
        val publicKey = call.parameters["account"]?.let { PublicKey(Address.decode(it)) } ?: return@get call.respond(HttpStatusCode.BadRequest, "invalid account")
        val signature = call.parameters["signature"]?.let { SignatureSerializer.decode(it) } ?: return@get call.respond(HttpStatusCode.BadRequest, "invalid signature")
        val message = call.parameters["message"] ?: return@get call.respond(HttpStatusCode.BadRequest, "invalid message")

        val result = Message.verify(publicKey, signature, message)

        call.respond(result.toString())
    }

    get("/api/v1/addpeer/{address}/{port?}/{force?}") {
        val port = call.parameters["port"]?.let { Network.parsePort(it) ?: return@get call.respond(HttpStatusCode.BadRequest, "invalid port") } ?: mode.defaultP2PPort
        val address = Network.parse(call.parameters["address"], port) ?: return@get call.respond(HttpStatusCode.BadRequest, "invalid address")
        @Suppress("UNUSED_VARIABLE")
        val force = call.parameters["force"]?.toBoolean() ?: false

        try {
            if (PeerDB.tryContact(address)) {
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
                call.respond("Connected")
            } else if (address.isLocal() || address.isPrivate()) {
                Node.connectTo(address, v2 = false)
                call.respond("Connected")
            } else {
                call.respond("Already in contact")
            }
        } catch (e: Throwable) {
            call.respond(e.message ?: "unknown error")
        }
    }

    get("/api/v1/disconnectpeer/{address}/{port?}/{force?}") {
        val port = call.parameters["port"]?.let { Network.parsePort(it) ?: return@get call.respond(HttpStatusCode.BadRequest, "invalid port") } ?: mode.defaultP2PPort
        val address = Network.parse(call.parameters["address"], port) ?: return@get call.respond(HttpStatusCode.BadRequest, "invalid address")
        @Suppress("UNUSED_VARIABLE")
        val force = call.parameters["force"]?.toBoolean() ?: false

        val connection = Node.connections.find { it.remoteAddress == address }
        if (connection != null) {
            connection.close()
            call.respond("Disconnected")
        } else {
            call.respond("Not connected to ${address}")
        }
    }

    get("/api/v1/disconnectpeerbyid/{id}/{force?}") {
        val id = call.parameters["id"]?.toLongOrNull() ?: return@get call.respond(HttpStatusCode.BadRequest, "invalid id")
        @Suppress("UNUSED_VARIABLE")
        val force = call.parameters["force"]?.toBoolean() ?: false

        val connection = Node.connections.find { it.peerId == id }
        if (connection != null) {
            connection.close()
            call.respond("Disconnected")
        } else {
            call.respond("Not connected")
        }
    }

    post("/api/v1/staker/start/{mnemonic}") {
        val privateKey = call.parameters["mnemonic"]?.let { Mnemonic.fromString(it) } ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid mnemonic")

        call.respond(Staker.startStaking(privateKey).toString())
    }

    post("/api/v1/staker/stop/{mnemonic}") {
        val privateKey = call.parameters["mnemonic"]?.let { Mnemonic.fromString(it) } ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid mnemonic")

        call.respond(Staker.stopStaking(privateKey).toString())
    }

    post("/api/v1/startStaking/{mnemonic}") {
        val privateKey = call.parameters["mnemonic"]?.let { Mnemonic.fromString(it) } ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid mnemonic")

        call.respond(Staker.startStaking(privateKey).toString())
    }

    post("/api/v1/stopStaking/{mnemonic}") {
        val privateKey = call.parameters["mnemonic"]?.let { Mnemonic.fromString(it) } ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid mnemonic")

        call.respond(Staker.stopStaking(privateKey).toString())
    }

    post("/api/v1/isStaking/{mnemonic}") {
        val privateKey = call.parameters["mnemonic"]?.let { Mnemonic.fromString(it) } ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid mnemonic")

        call.respond(Staker.isStaking(privateKey).toString())
    }

    get("/api/v1/walletdb/getwallet/{address}") {
        val publicKey = call.parameters["address"]?.let { PublicKey(Address.decode(it)) } ?: return@get call.respond(HttpStatusCode.BadRequest, "invalid address")

        WalletDB.reentrant.readLock().withLock {
            runBlocking {
                call.respond(Json.stringify(WalletV1.serializer(), WalletV1(WalletDB.getWalletImpl(publicKey))))
            }
        }
    }

    get("/api/v1/walletdb/getoutleases/{address}") {
        val publicKey = call.parameters["address"]?.let { PublicKey(Address.decode(it)) } ?: return@get call.respond(HttpStatusCode.BadRequest, "invalid address")

        WalletDB.reentrant.readLock().withLock {
            val wallet = WalletDB.getWalletImpl(publicKey)
            runBlocking {
                call.respond(Json.stringify(ListSerializer(AccountState.Lease.serializer()), wallet.outLeases))
            }
        }
    }

    get("/api/v1/walletdb/getsequence/{address}") {
        val publicKey = call.parameters["address"]?.let { PublicKey(Address.decode(it)) } ?: return@get call.respond(HttpStatusCode.BadRequest, "invalid address")

        call.respond(WalletDB.getSequence(publicKey).toString())
    }

    get("/api/v1/walletdb/gettransaction/{hash}/{raw?}") {
        val hash = call.parameters["hash"]?.let { Hash(Hash.fromString(it)) } ?: return@get call.respond(HttpStatusCode.BadRequest, "invalid hash")
        val raw = call.parameters["raw"]?.toBoolean() ?: false

        val result = WalletDB.reentrant.readLock().withLock {
            WalletDB.getTransactionImpl(hash)
        }
        if (result != null) {
            if (raw)
                return@get call.respond(result.toHex())

            val tx = binaryFormat.decodeFromByteArray(Transaction.serializer(), result)
            call.respond(Json.stringify(TransactionInfoV2.serializer(), TransactionInfoV2(tx, hash, result.size)))
        } else {
            call.respond(HttpStatusCode.NotFound, "transaction not found")
        }
    }

    get("/api/v1/walletdb/getconfirmations/{hash}") {
        val hash = call.parameters["hash"]?.let { Hash(Hash.fromString(it)) } ?: return@get call.respond(HttpStatusCode.BadRequest, "invalid hash")

        val result = WalletDB.getConfirmations(hash)
        if (result != null)
            call.respond(result.toString())
        else
            call.respond(HttpStatusCode.NotFound, "transaction not found")
    }
}
