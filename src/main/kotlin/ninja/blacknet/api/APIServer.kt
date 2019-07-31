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

import io.ktor.application.Application
import io.ktor.application.call
import io.ktor.application.install
import io.ktor.features.DefaultHeaders
import io.ktor.http.HttpStatusCode
import io.ktor.http.cio.websocket.CloseReason
import io.ktor.http.cio.websocket.Frame
import io.ktor.http.cio.websocket.close
import io.ktor.http.cio.websocket.readText
import io.ktor.http.content.*
import io.ktor.request.isMultipart
import io.ktor.request.receiveMultipart
import io.ktor.response.respond
import io.ktor.response.respondRedirect
import io.ktor.routing.get
import io.ktor.routing.post
import io.ktor.routing.routing
import io.ktor.websocket.WebSockets
import io.ktor.websocket.webSocket
import kotlinx.coroutines.channels.ClosedReceiveChannelException
import kotlinx.coroutines.channels.SendChannel
import kotlinx.coroutines.debug.DebugProbes
import kotlinx.coroutines.launch
import kotlinx.coroutines.sync.Mutex
import kotlinx.coroutines.sync.withLock
import kotlinx.serialization.list
import ninja.blacknet.Config
import ninja.blacknet.api.v1.BlockInfoV1
import ninja.blacknet.core.*
import ninja.blacknet.crypto.*
import ninja.blacknet.db.*
import ninja.blacknet.network.Network
import ninja.blacknet.network.Node
import ninja.blacknet.network.Runtime
import ninja.blacknet.serialization.Json
import ninja.blacknet.serialization.SerializableByteArray
import ninja.blacknet.serialization.toHex
import ninja.blacknet.transaction.*
import ninja.blacknet.util.SynchronizedArrayList
import ninja.blacknet.util.SynchronizedHashMap
import ninja.blacknet.util.buffered
import ninja.blacknet.util.data
import java.io.File
import java.io.PrintStream
import kotlin.math.abs

object APIServer {
    internal val txMutex = Mutex()
    internal var lastIndex: Pair<Hash, ChainIndex>? = null
    internal val blockNotifyV1 = SynchronizedArrayList<SendChannel<Frame>>()
    internal val blockNotify = SynchronizedArrayList<SendChannel<Frame>>()
    internal val transactionNotify = SynchronizedHashMap<SendChannel<Frame>, HashSet<PublicKey>>()

    suspend fun blockNotify(block: Block, hash: Hash, height: Int, size: Int) = blockNotify.mutex.withLock {
        blockNotifyV1.forEach {
            Runtime.launch {
                try {
                    it.send(Frame.Text(hash.toString()))
                } finally {
                }
            }
        }

        if (blockNotify.list.isEmpty())
            return@withLock
        val notification = BlockNotification(block, hash, height, size)
        val message = Json.stringify(BlockNotification.serializer(), notification)
        blockNotify.list.forEach {
            Runtime.launch {
                try {
                    it.send(Frame.Text(message))
                } finally {
                }
            }
        }
    }

    suspend fun transactionNotify(tx: Transaction, hash: Hash, time: Long, size: Int, publicKey: PublicKey) = transactionNotify.mutex.withLock {
        if (transactionNotify.map.isEmpty())
            return@withLock
        val notification = TransactionNotification(tx, hash, time, size)
        val message = Json.stringify(TransactionNotification.serializer(), notification)
        transactionNotify.map.forEach {
            Runtime.launch {
                try {
                    if (it.value.contains(publicKey))
                        it.key.send(Frame.Text(message))
                } finally {
                }
            }
        }
    }
}

fun Application.APIServer() {
    install(DefaultHeaders)
    install(WebSockets)

    routing {
        get("/") {
            call.respondRedirect("static/index.html")
        }

        static("static") {
            files(Config.htmlDir)
        }

        webSocket("/api/v1/notify/block") {
            try {
                APIServer.blockNotifyV1.add(outgoing)
                while (true) {
                    incoming.receive()
                }
            } catch (e: ClosedReceiveChannelException) {
            } finally {
                APIServer.blockNotifyV1.remove(outgoing)
            }
        }

        webSocket("/api/v2/notify/block") {
            try {
                APIServer.blockNotify.add(outgoing)
                while (true) {
                    incoming.receive()
                }
            } catch (e: ClosedReceiveChannelException) {
            } finally {
                APIServer.blockNotify.remove(outgoing)
            }
        }

        webSocket("/api/v1/notify/transaction") {
            try {
                while (true) {
                    val string = (incoming.receive() as Frame.Text).readText()
                    val publicKey = Address.decode(string) ?: return@webSocket this.close(CloseReason(CloseReason.Codes.PROTOCOL_ERROR, "invalid account"))

                    APIServer.transactionNotify.mutex.withLock {
                        val keys = APIServer.transactionNotify.map.get(outgoing)
                        if (keys == null) {
                            @Suppress("NAME_SHADOWING")
                            val keys = HashSet<PublicKey>()
                            keys.add(publicKey)
                            APIServer.transactionNotify.map.put(outgoing, keys)
                        } else {
                            keys.add(publicKey)
                        }
                    }
                }
            } catch (e: ClosedReceiveChannelException) {
            } finally {
                APIServer.transactionNotify.remove(outgoing)
            }
        }

        get("/api/v1/peerinfo") {
            call.respond(Json.stringify(PeerInfo.serializer().list, PeerInfo.getAll()))
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
            val hash = Hash.fromString(call.parameters["hash"]) ?: return@get call.respond(HttpStatusCode.BadRequest, "invalid hash")
            val txdetail = call.parameters["txdetail"]?.toBoolean() ?: false

            val result = BlockInfoV1.get(hash, txdetail)
            if (result != null)
                call.respond(Json.stringify(BlockInfoV1.serializer(), result))
            else
                call.respond(HttpStatusCode.NotFound, "block not found")
        }

        get("/api/v2/blockdb/get/{hash}/{txdetail?}") {
            val hash = Hash.fromString(call.parameters["hash"]) ?: return@get call.respond(HttpStatusCode.BadRequest, "invalid hash")
            val txdetail = call.parameters["txdetail"]?.toBoolean() ?: false

            val result = BlockDB.block(hash)
            if (result != null)
                call.respond(Json.stringify(Block.Info.serializer(), Block.Info(result.first, hash, result.second, txdetail)))
            else
                call.respond(HttpStatusCode.NotFound, "block not found")
        }

        get("/api/v1/blockdb/getblockhash/{height}") {
            val height = call.parameters["height"]?.toIntOrNull() ?: return@get call.respond(HttpStatusCode.BadRequest, "invalid height")

            BlockDB.mutex.withLock {
                if (height < 0 || height > LedgerDB.height())
                    return@get call.respond(HttpStatusCode.NotFound, "block not found")
                else if (height == 0)
                    return@get call.respond(Hash.ZERO.toString())
                else if (height == LedgerDB.height())
                    return@get call.respond(LedgerDB.blockHash().toString())

                if (APIServer.lastIndex != null && APIServer.lastIndex!!.second.height == height)
                    return@get call.respond(APIServer.lastIndex!!.first.toString())

                var hash: Hash
                var index: ChainIndex
                if (height < LedgerDB.height() / 2) {
                    hash = Hash.ZERO
                    index = LedgerDB.getChainIndex(hash)!!
                } else {
                    hash = LedgerDB.blockHash()
                    index = LedgerDB.getChainIndex(hash)!!
                }
                if (APIServer.lastIndex != null && abs(height - index.height) > abs(height - APIServer.lastIndex!!.second.height))
                    index = APIServer.lastIndex!!.second
                while (index.height > height) {
                    hash = index.previous
                    index = LedgerDB.getChainIndex(hash)!!
                }
                while (index.height < height) {
                    hash = index.next
                    index = LedgerDB.getChainIndex(hash)!!
                }
                if (index.height < LedgerDB.height() - PoS.MATURITY - 1)
                    APIServer.lastIndex = Pair(hash, index)
                call.respond(hash.toString())
            }
        }

        get("/api/v1/blockdb/getblockindex/{hash}/") {
            val hash = Hash.fromString(call.parameters["hash"]) ?: return@get call.respond(HttpStatusCode.BadRequest, "invalid hash")

            val result = LedgerDB.getChainIndex(hash)
            if (result != null)
                call.respond(Json.stringify(ChainIndex.serializer(), result))
            else
                call.respond(HttpStatusCode.NotFound, "block not found")
        }

        get("/api/v1/blockdb/makebootstrap") {
            val checkpoint = LedgerDB.rollingCheckpoint()
            if (checkpoint == Hash.ZERO)
                return@get call.respond(HttpStatusCode.BadRequest, "not synchronized")

            val file = File(Config.dataDir, "bootstrap.dat.new")
            val stream = file.outputStream().buffered().data()

            var hash = Hash.ZERO
            var index = LedgerDB.getChainIndex(hash)!!
            do {
                hash = index.next
                index = LedgerDB.getChainIndex(hash)!!
                val bytes = BlockDB.getImpl(hash)!!
                stream.writeInt(bytes.size)
                stream.write(bytes, 0, bytes.size)
            } while (hash != checkpoint)

            stream.close()

            call.respond(file.absolutePath)
        }

        get("/api/v1/ledger") {
            call.respond(Json.stringify(LedgerInfo.serializer(), LedgerInfo.get()))
        }

        get("/api/v1/ledger/get/{account}/{confirmations?}") {
            val publicKey = Address.decode(call.parameters["account"]) ?: return@get call.respond(HttpStatusCode.BadRequest, "invalid account")
            val confirmations = call.parameters["confirmations"]?.toIntOrNull() ?: PoS.DEFAULT_CONFIRMATIONS
            val result = AccountInfo.get(publicKey, confirmations)
            if (result != null)
                call.respond(Json.stringify(AccountInfo.serializer(), result))
            else
                call.respond(HttpStatusCode.NotFound, "account not found")
        }

        get("/api/v1/ledger/check") {
            call.respond(Json.stringify(LedgerDB.Check.serializer(), LedgerDB.check()))
        }

        get("/api/v1/txpool") {
            call.respond(Json.stringify(TxPoolInfo.serializer(), TxPoolInfo.get()))
        }

        get("/api/v1/account/generate") {
            call.respond(Json.stringify(MnemonicInfo.serializer(), MnemonicInfo.new()))
        }

        get("/api/v1/address/info/{address}") {
            val info = AddressInfo.fromString(call.parameters["address"]) ?: return@get call.respond(HttpStatusCode.BadRequest, "invalid address")

            call.respond(Json.stringify(AddressInfo.serializer(), info))
        }

        post("/api/v1/mnemonic/info/{mnemonic}") {
            val info = MnemonicInfo.fromString(call.parameters["mnemonic"]) ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid mnemonic")

            call.respond(Json.stringify(MnemonicInfo.serializer(), info))
        }

        // check if call.request is multipart first
        // TODO here still have an issue https://github.com/ktorio/ktor/issues/482
        suspend fun readMultipart(multipart : MultiPartData) : Map<String, String> {
            val result = hashMapOf<String, String>()
            multipart.forEachPart {
                when (it) {
                    is PartData.FormItem -> {
                        result[it.name.toString()] = it.value
                    }
                }
            }
            return result
        }

        post("/api/v2/mnemonic/info") {
            val multipart = call.receiveMultipart()
            if (!call.request.isMultipart()) {
                return@post call.respond(HttpStatusCode.BadRequest, "not a multipart request")
            }
            val args = readMultipart(multipart)
            val info = MnemonicInfo.fromString(args.getOrDefault("mnemonic", "")) ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid mnemonic")
            call.respond(Json.stringify(MnemonicInfo.serializer(), info))
        }

        post("/api/v1/transfer/{mnemonic}/{fee}/{amount}/{to}/{message?}/{encrypted?}/{blockHash?}/") {
            val privateKey = Mnemonic.fromString(call.parameters["mnemonic"]) ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid mnemonic")
            val from = privateKey.toPublicKey()
            val fee = call.parameters["fee"]?.toLongOrNull() ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid fee")
            val amount = call.parameters["amount"]?.toLongOrNull() ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid amount")
            val to = Address.decode(call.parameters["to"]) ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid to")
            val encrypted = call.parameters["encrypted"]?.let { it.toByteOrNull() ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid encrypted") }
            val message = Message.create(call.parameters["message"], encrypted, privateKey, to) ?: return@post call.respond(HttpStatusCode.BadRequest, "failed to create message")
            val blockHash = call.parameters["blockHash"]?.let { Hash.fromString(it) ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid blockHash") }

            APIServer.txMutex.withLock {
                val seq = WalletDB.getSequence(from)
                val data = Transfer(amount, to, message).serialize()
                val tx = Transaction.create(from, seq, blockHash, fee, TxType.Transfer.type, data)
                val signed = tx.sign(privateKey)

                if (Node.broadcastTx(signed.first, signed.second))
                    call.respond(signed.first.toString())
                else
                    call.respond("Transaction rejected")
            }
        }

        post("/api/v2/transfer") {
            val multipart = call.receiveMultipart()
            if (!call.request.isMultipart()) {
                return@post call.respond(HttpStatusCode.BadRequest, "not a multipart request")
            }
            val args = readMultipart(multipart)

            val privateKey = Mnemonic.fromString(args.getOrDefault("mnemonic", "")) ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid mnemonic")
            val from = privateKey.toPublicKey()
            val fee = args.getOrDefault("fee", null)?.toLongOrNull() ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid fee")
            val amount = args.getOrDefault("amount", null)?.toLongOrNull() ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid amount")
            val to = Address.decode(args.getOrDefault("to", "")) ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid to")
            val encrypted = args.getOrDefault("encrypted", null)?.let { it.toByteOrNull() ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid encrypted") }
            val message = Message.create(args.getOrDefault("message", ""), encrypted, privateKey, to) ?: return@post call.respond(HttpStatusCode.BadRequest, "failed to create message")
            val blockHash = args.getOrDefault("blockHash", null)?.let { Hash.fromString(it) ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid blockHash") }

            APIServer.txMutex.withLock {
                val seq = WalletDB.getSequence(from)
                val data = Transfer(amount, to, message).serialize()
                val tx = Transaction.create(from, seq, blockHash, fee, TxType.Transfer.type, data)
                val signed = tx.sign(privateKey)

                if (Node.broadcastTx(signed.first, signed.second))
                    call.respond(signed.first.toString())
                else
                    call.respond("Transaction rejected")
            }
        }

        post("/api/v1/burn/{mnemonic}/{fee}/{amount}/{message?}/{blockHash?}/") {
            val privateKey = Mnemonic.fromString(call.parameters["mnemonic"]) ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid mnemonic")
            val from = privateKey.toPublicKey()
            val fee = call.parameters["fee"]?.toLongOrNull() ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid fee")
            val amount = call.parameters["amount"]?.toLongOrNull() ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid amount")
            val message = SerializableByteArray.fromString(call.parameters["message"].orEmpty()) ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid message")
            val blockHash = call.parameters["blockHash"]?.let { Hash.fromString(it) ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid blockHash") }

            APIServer.txMutex.withLock {
                val seq = WalletDB.getSequence(from)
                val data = Burn(amount, message).serialize()
                val tx = Transaction.create(from, seq, blockHash, fee, TxType.Burn.type, data)
                val signed = tx.sign(privateKey)

                if (Node.broadcastTx(signed.first, signed.second))
                    call.respond(signed.first.toString())
                else
                    call.respond("Transaction rejected")
            }
        }

        post("/api/v2/burn") {
            val multipart = call.receiveMultipart()
            if (!call.request.isMultipart()) {
                return@post call.respond(HttpStatusCode.BadRequest, "not a multipart request")
            }
            val args = readMultipart(multipart)
            val privateKey = Mnemonic.fromString(args.getOrDefault("mnemonic", ""))
                    ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid mnemonic")
            val from = privateKey.toPublicKey()
            val fee = args.getOrDefault("fee", null)?.toLongOrNull()
                    ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid fee")
            val amount = args.getOrDefault("amount", null)?.toLongOrNull()
                    ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid amount")
            val message = SerializableByteArray.fromString(args.getOrDefault("message", null).orEmpty())
                    ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid message")
            val blockHash = args.getOrDefault("blockHash", null)?.let {
                Hash.fromString(it)
                        ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid blockHash")
            }

            APIServer.txMutex.withLock {
                val seq = WalletDB.getSequence(from)
                val data = Burn(amount, message).serialize()
                val tx = Transaction.create(from, seq, blockHash, fee, TxType.Burn.type, data)
                val signed = tx.sign(privateKey)

                if (Node.broadcastTx(signed.first, signed.second))
                    call.respond(signed.first.toString())
                else
                    call.respond("Transaction rejected")
            }
        }

        post("/api/v1/lease/{mnemonic}/{fee}/{amount}/{to}/{blockHash?}/") {
            val privateKey = Mnemonic.fromString(call.parameters["mnemonic"]) ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid mnemonic")
            val from = privateKey.toPublicKey()
            val fee = call.parameters["fee"]?.toLongOrNull() ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid fee")
            val amount = call.parameters["amount"]?.toLongOrNull() ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid amount")
            val to = Address.decode(call.parameters["to"]) ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid to")
            val blockHash = call.parameters["blockHash"]?.let { Hash.fromString(it) ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid blockHash") }

            APIServer.txMutex.withLock {
                val seq = WalletDB.getSequence(from)
                val data = Lease(amount, to).serialize()
                val tx = Transaction.create(from, seq, blockHash, fee, TxType.Lease.type, data)
                val signed = tx.sign(privateKey)

                if (Node.broadcastTx(signed.first, signed.second))
                    call.respond(signed.first.toString())
                else
                    call.respond("Transaction rejected")
            }
        }

        post("/api/v2/lease") {
            val multipart = call.receiveMultipart()
            if (!call.request.isMultipart()) {
                return@post call.respond(HttpStatusCode.BadRequest, "not a multipart request")
            }
            val args = readMultipart(multipart)
            val privateKey = Mnemonic.fromString(args.getOrDefault("mnemonic", "")) ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid mnemonic")
            val from = privateKey.toPublicKey()
            val fee = args.getOrDefault("fee", null)?.toLongOrNull() ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid fee")
            val amount = args.getOrDefault("amount", null)?.toLongOrNull() ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid amount")
            val to = Address.decode(args.getOrDefault("to", "")) ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid to")
            val blockHash = args.getOrDefault("blockHash", null)?.let { Hash.fromString(it) ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid blockHash") }

            APIServer.txMutex.withLock {
                val seq = WalletDB.getSequence(from)
                val data = Lease(amount, to).serialize()
                val tx = Transaction.create(from, seq, blockHash, fee, TxType.Lease.type, data)
                val signed = tx.sign(privateKey)

                if (Node.broadcastTx(signed.first, signed.second))
                    call.respond(signed.first.toString())
                else
                    call.respond("Transaction rejected")
            }
        }

        post("/api/v1/cancellease/{mnemonic}/{fee}/{amount}/{to}/{height}/{blockHash?}/") {
            val privateKey = Mnemonic.fromString(call.parameters["mnemonic"]) ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid mnemonic")
            val from = privateKey.toPublicKey()
            val fee = call.parameters["fee"]?.toLongOrNull() ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid fee")
            val amount = call.parameters["amount"]?.toLongOrNull() ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid amount")
            val to = Address.decode(call.parameters["to"]) ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid to")
            val height = call.parameters["height"]?.toIntOrNull() ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid height")
            val blockHash = call.parameters["blockHash"]?.let { Hash.fromString(it) ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid blockHash") }

            APIServer.txMutex.withLock {
                val seq = WalletDB.getSequence(from)
                val data = CancelLease(amount, to, height).serialize()
                val tx = Transaction.create(from, seq, blockHash, fee, TxType.CancelLease.type, data)
                val signed = tx.sign(privateKey)

                if (Node.broadcastTx(signed.first, signed.second))
                    call.respond(signed.first.toString())
                else
                    call.respond("Transaction rejected")
            }
        }

        post("/api/v2/cancellease") {
            val multipart = call.receiveMultipart()
            if (!call.request.isMultipart()) {
                return@post call.respond(HttpStatusCode.BadRequest, "not a multipart request")
            }
            val args = readMultipart(multipart)
            val privateKey = Mnemonic.fromString(args.getOrDefault("mnemonic", "")) ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid mnemonic")
            val from = privateKey.toPublicKey()
            val fee = args.getOrDefault("fee", null)?.toLongOrNull() ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid fee")
            val amount = args.getOrDefault("amount", null)?.toLongOrNull() ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid amount")
            val to = Address.decode(args.getOrDefault("to", "")) ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid to")
            val height = args.getOrDefault("height", null)?.toIntOrNull() ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid height")
            val blockHash = args.getOrDefault("blockHash", null)?.let { Hash.fromString(it) ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid blockHash") }

            APIServer.txMutex.withLock {
                val seq = WalletDB.getSequence(from)
                val data = CancelLease(amount, to, height).serialize()
                val tx = Transaction.create(from, seq, blockHash, fee, TxType.CancelLease.type, data)
                val signed = tx.sign(privateKey)

                if (Node.broadcastTx(signed.first, signed.second))
                    call.respond(signed.first.toString())
                else
                    call.respond("Transaction rejected")
            }
        }

        get("/api/v1/transaction/raw/send/{serialized}/") {
            val serialized = SerializableByteArray.fromString(call.parameters["serialized"]) ?: return@get call.respond(HttpStatusCode.BadRequest, "invalid serialized")
            val hash = Transaction.Hasher(serialized.array)

            APIServer.txMutex.withLock {
                if (Node.broadcastTx(hash, serialized.array))
                    call.respond(hash.toString())
                else
                    call.respond("Transaction rejected")
            }
        }

        post("/api/v1/decryptmessage/{mnemonic}/{from}/{message}") {
            val privateKey = Mnemonic.fromString(call.parameters["mnemonic"]) ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid mnemonic")
            val publicKey = Address.decode(call.parameters["from"]) ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid from")
            val message = call.parameters["message"] ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid message")

            val decrypted = Message.decrypt(privateKey, publicKey, message)
            if (decrypted != null)
                call.respond(decrypted)
            else
                call.respond(HttpStatusCode.NotFound, "Decryption failed")
        }

        post("/api/v2/decryptmessage") {
            val multipart = call.receiveMultipart()
            if (!call.request.isMultipart()) {
                return@post call.respond(HttpStatusCode.BadRequest, "not a multipart request")
            }
            val args = readMultipart(multipart)
            val privateKey = Mnemonic.fromString(args.getOrDefault("mnemonic", "")) ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid mnemonic")
            val publicKey = Address.decode(args.getOrDefault("from", "")) ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid from")
            val message = args.getOrDefault("message", null) ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid message")

            val decrypted = Message.decrypt(privateKey, publicKey, message)
            if (decrypted != null)
                call.respond(decrypted)
            else
                call.respond(HttpStatusCode.NotFound, "Decryption failed")
        }

        post("/api/v1/signmessage/{mnemonic}/{message}") {
            val privateKey = Mnemonic.fromString(call.parameters["mnemonic"]) ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid mnemonic")
            val message = call.parameters["message"] ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid message")

            val signature = Message.sign(privateKey, message)

            call.respond(signature.toString())
        }

        post("/api/v2/signmessage") {
            val multipart = call.receiveMultipart()
            if (!call.request.isMultipart()) {
                return@post call.respond(HttpStatusCode.BadRequest, "not a multipart request")
            }
            val args = readMultipart(multipart)
            val privateKey = Mnemonic.fromString(args.getOrDefault("mnemonic", "")) ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid mnemonic")
            val message = args.getOrDefault("message", null) ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid message")

            val signature = Message.sign(privateKey, message)

            call.respond(signature.toString())
        }

        get("/api/v1/verifymessage/{account}/{signature}/{message}") {
            val publicKey = Address.decode(call.parameters["account"]) ?: return@get call.respond(HttpStatusCode.BadRequest, "invalid account")
            val signature = Signature.fromString(call.parameters["signature"]) ?: return@get call.respond(HttpStatusCode.BadRequest, "invalid signature")
            val message = call.parameters["message"] ?: return@get call.respond(HttpStatusCode.BadRequest, "invalid message")

            val result = Message.verify(publicKey, signature, message)

            call.respond(result.toString())
        }

        get("/api/v1/addpeer/{address}/{port?}/{force?}") {
            val port = call.parameters["port"]?.toIntOrNull() ?: Node.DEFAULT_P2P_PORT
            val address = Network.parse(call.parameters["address"], port) ?: return@get call.respond(HttpStatusCode.BadRequest, "invalid address")
            val force = call.parameters["force"]?.toBoolean() ?: false

            try {
                val connection = Node.connections.find { it.remoteAddress == address }
                if (force || connection == null) {
                    Node.connectTo(address)
                    call.respond("Connected")
                } else {
                    call.respond("Already connected on ${connection.localAddress}")
                }
            } catch (e: Throwable) {
                call.respond(e.message ?: "unknown error")
            }
        }

        get("/api/v1/disconnectpeer/{address}/{port?}/{force?}") {
            val port = call.parameters["port"]?.toIntOrNull() ?: Node.DEFAULT_P2P_PORT
            val address = Network.parse(call.parameters["address"], port) ?: return@get call.respond(HttpStatusCode.BadRequest, "invalid address")
            val force = call.parameters["force"]?.toBoolean() ?: false

            val connection = Node.connections.find { it.remoteAddress == address }
            if (connection != null) {
                connection.close(force)
                call.respond("Disconnected")
            } else {
                call.respond("Not connected to ${address}")
            }
        }

        get("/api/v1/disconnectpeerbyid/{id}/{force?}") {
            val id = call.parameters["id"]?.toLongOrNull() ?: return@get call.respond(HttpStatusCode.BadRequest, "invalid id")
            val force = call.parameters["force"]?.toBoolean() ?: false

            val connection = Node.connections.find { it.peerId == id }
            if (connection != null) {
                connection.close(force)
                call.respond("Disconnected")
            } else {
                call.respond("Not connected")
            }
        }

        post("/api/v1/staker/start/{mnemonic}") {
            val privateKey = Mnemonic.fromString(call.parameters["mnemonic"]) ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid mnemonic")

            call.respond(PoS.startStaking(privateKey).toString())
        }

        post("/api/v2/staker/start") {
            val multipart = call.receiveMultipart()
            if (!call.request.isMultipart()) {
                return@post call.respond(HttpStatusCode.BadRequest, "not a multipart request")
            }
            val args = readMultipart(multipart)
            val privateKey = Mnemonic.fromString(args.getOrDefault("mnemonic","")) ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid mnemonic")

            call.respond(PoS.startStaking(privateKey).toString())
        }

        post("/api/v1/staker/stop/{mnemonic}") {
            val privateKey = Mnemonic.fromString(call.parameters["mnemonic"]) ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid mnemonic")

            call.respond(PoS.stopStaking(privateKey).toString())
        }

        post("/api/v2/staker/stop") {
            val multipart = call.receiveMultipart()
            if (!call.request.isMultipart()) {
                return@post call.respond(HttpStatusCode.BadRequest, "not a multipart request")
            }
            val args = readMultipart(multipart)
            val privateKey = Mnemonic.fromString(args.getOrDefault("mnemonic","")) ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid mnemonic")

            call.respond(PoS.stopStaking(privateKey).toString())
        }

        post("/api/v1/startStaking/{mnemonic}") {
            val privateKey = Mnemonic.fromString(call.parameters["mnemonic"]) ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid mnemonic")

            call.respond(PoS.startStaking(privateKey).toString())
        }

        post("/api/v2/startStaking") {
            val multipart = call.receiveMultipart()
            if (!call.request.isMultipart()) {
                return@post call.respond(HttpStatusCode.BadRequest, "not a multipart request")
            }
            val args = readMultipart(multipart)
            val privateKey = Mnemonic.fromString(args.getOrDefault("mnemonic","")) ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid mnemonic")

            call.respond(PoS.startStaking(privateKey).toString())
        }

        post("/api/v1/stopStaking/{mnemonic}") {
            val privateKey = Mnemonic.fromString(call.parameters["mnemonic"]) ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid mnemonic")

            call.respond(PoS.stopStaking(privateKey).toString())
        }

        post("/api/v2/stopStaking") {
            val multipart = call.receiveMultipart()
            if (!call.request.isMultipart()) {
                return@post call.respond(HttpStatusCode.BadRequest, "not a multipart request")
            }
            val args = readMultipart(multipart)
            val privateKey = Mnemonic.fromString(args.getOrDefault("mnemonic","")) ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid mnemonic")

            call.respond(PoS.stopStaking(privateKey).toString())
        }

        post("/api/v1/isStaking/{mnemonic}") {
            val privateKey = Mnemonic.fromString(call.parameters["mnemonic"]) ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid mnemonic")

            call.respond(PoS.isStaking(privateKey).toString())
        }

        post("/api/v2/isStaking") {
            val multipart = call.receiveMultipart()
            if (!call.request.isMultipart()) {
                return@post call.respond(HttpStatusCode.BadRequest, "not a multipart request")
            }
            val args = readMultipart(multipart)
            val privateKey = Mnemonic.fromString(args.getOrDefault("mnemonic","")) ?: return@post call.respond(HttpStatusCode.BadRequest, "invalid mnemonic")

            call.respond(PoS.isStaking(privateKey).toString())
        }

        get("/api/v1/walletdb/getwallet/{address}") {
            val publicKey = Address.decode(call.parameters["address"]) ?: return@get call.respond(HttpStatusCode.BadRequest, "invalid address")

            BlockDB.mutex.withLock {
                call.respond(Json.stringify(WalletDB.Wallet.serializer(), WalletDB.getWalletImpl(publicKey)))
            }
        }

        get("/api/v1/walletdb/getoutleases/{address}") {
            val publicKey = Address.decode(call.parameters["address"]) ?: return@get call.respond(HttpStatusCode.BadRequest, "invalid address")

            call.respond(Json.stringify(AccountState.Lease.serializer().list, WalletDB.getOutLeases(publicKey)))
        }

        get("/api/v1/walletdb/getsequence/{address}") {
            val publicKey = Address.decode(call.parameters["address"]) ?: return@get call.respond(HttpStatusCode.BadRequest, "invalid address")

            call.respond(WalletDB.getSequence(publicKey).toString())
        }

        get("/api/v1/walletdb/gettransaction/{hash}/{raw?}") {
            val hash = Hash.fromString(call.parameters["hash"]) ?: return@get call.respond(HttpStatusCode.BadRequest, "invalid hash")
            val raw = call.parameters["raw"]?.toBoolean() ?: false

            val result = WalletDB.getTransaction(hash)
            if (result != null) {
                if (raw)
                    return@get call.respond(result.toHex())

                val tx = Transaction.deserialize(result)!!
                call.respond(Json.stringify(Transaction.Info.serializer(), Transaction.Info(tx, hash, result.size)))
            } else {
                call.respond(HttpStatusCode.NotFound, "transaction not found")
            }
        }

        get("/api/v1/walletdb/getconfirmations/{hash}") {
            val hash = Hash.fromString(call.parameters["hash"]) ?: return@get call.respond(HttpStatusCode.BadRequest, "invalid hash")

            val result = WalletDB.getConfirmations(hash)
            if (result != null)
                call.respond(result.toString())
            else
                call.respond(HttpStatusCode.NotFound, "transaction not found")
        }

        get("/api/dumpcoroutines") {
            if (Config.debugCoroutines) {
                val stream = PrintStream(File(Config.dataDir, "coroutines_${Runtime.time()}.log"))
                DebugProbes.dumpCoroutines(stream)
                stream.close()
                call.respond(true.toString())
            } else {
                call.respond(false.toString())
            }
        }
    }
}
