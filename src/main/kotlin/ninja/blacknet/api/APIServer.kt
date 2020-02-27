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
import io.ktor.application.ApplicationCall
import io.ktor.application.call
import io.ktor.application.install
import io.ktor.features.DefaultHeaders
import io.ktor.features.StatusPages
import io.ktor.http.ContentType
import io.ktor.http.HttpHeaders
import io.ktor.http.HttpStatusCode
import io.ktor.http.cio.websocket.Frame
import io.ktor.http.cio.websocket.readText
import io.ktor.http.content.files
import io.ktor.http.content.static
import io.ktor.response.respond
import io.ktor.response.respondRedirect
import io.ktor.response.respondText
import io.ktor.routing.get
import io.ktor.routing.routing
import io.ktor.websocket.WebSockets
import io.ktor.websocket.webSocket
import kotlinx.coroutines.channels.ClosedReceiveChannelException
import kotlinx.coroutines.channels.SendChannel
import kotlinx.coroutines.launch
import kotlinx.coroutines.sync.Mutex
import kotlinx.coroutines.sync.withLock
import kotlinx.serialization.SerializationStrategy
import ninja.blacknet.Config
import ninja.blacknet.Runtime
import ninja.blacknet.Version
import ninja.blacknet.api.v1.*
import ninja.blacknet.core.*
import ninja.blacknet.crypto.*
import ninja.blacknet.db.*
import ninja.blacknet.serialization.Json
import ninja.blacknet.util.*
import java.io.File
import kotlin.math.abs

object APIServer {
    internal val txMutex = Mutex()
    internal var lastIndex: Pair<Hash, ChainIndex>? = null
    internal val blockNotifyV0 = SynchronizedArrayList<SendChannel<Frame>>()
    internal val blockNotifyV1 = SynchronizedArrayList<SendChannel<Frame>>()
    internal val walletNotifyV1 = SynchronizedHashMap<SendChannel<Frame>, HashSet<PublicKey>>()
    internal val blockNotify = SynchronizedHashSet<SendChannel<Frame>>()
    internal val txPoolNotify = SynchronizedHashSet<SendChannel<Frame>>()
    internal val walletNotify = SynchronizedHashMap<SendChannel<Frame>, HashSet<PublicKey>>()

    suspend fun blockNotify(block: Block, hash: Hash, height: Int, size: Int) {
        blockNotifyV0.forEach {
            Runtime.launch {
                try {
                    it.send(Frame.Text(hash.toString()))
                } finally {
                }
            }
        }

        blockNotifyV1.mutex.withLock {
            if (blockNotifyV1.list.isNotEmpty()) {
                val notification = BlockNotificationV1(block, hash, height, size)
                val message = Json.stringify(BlockNotificationV1.serializer(), notification)
                blockNotifyV1.list.forEach {
                    Runtime.launch {
                        try {
                            it.send(Frame.Text(message))
                        } finally {
                        }
                    }
                }
            }
        }

        blockNotify.mutex.withLock {
            if (blockNotify.set.isNotEmpty()) {
                val notification = WebSocketNotification(BlockNotification(block, hash, height, size))
                val message = Json.stringify(WebSocketNotification.serializer(), notification)
                blockNotify.set.forEach {
                    Runtime.launch {
                        try {
                            it.send(Frame.Text(message))
                        } finally {
                        }
                    }
                }
            }
        }
    }

    suspend fun txPoolNotify(tx: Transaction, hash: Hash, time: Long, size: Int) {
        txPoolNotify.mutex.withLock {
            if (txPoolNotify.set.isNotEmpty()) {
                val notification = WebSocketNotification(TransactionNotification(tx, hash, time, size))
                val message = Json.stringify(WebSocketNotification.serializer(), notification)
                txPoolNotify.set.forEach {
                    Runtime.launch {
                        try {
                            it.send(Frame.Text(message))
                        } finally {
                        }
                    }
                }
            }
        }
    }

    suspend fun walletNotify(tx: Transaction, hash: Hash, time: Long, size: Int, publicKey: PublicKey, filter: List<WalletDB.TransactionDataType>) {
        walletNotifyV1.mutex.withLock {
            if (walletNotifyV1.map.isNotEmpty()) {
                val notification = TransactionNotificationV2(tx, hash, time, size)
                val message = Json.stringify(TransactionNotificationV2.serializer(), notification)
                walletNotifyV1.map.forEach {
                    if (it.value.contains(publicKey)) {
                        Runtime.launch {
                            try {
                                it.key.send(Frame.Text(message))
                            } finally {
                            }
                        }
                    }
                }
            }
        }

        walletNotify.mutex.withLock {
            if (walletNotify.map.isNotEmpty()) {
                val notification = WebSocketNotification(TransactionNotification(tx, hash, time, size, filter))
                val message = Json.stringify(WebSocketNotification.serializer(), notification)
                walletNotify.map.forEach {
                    if (it.value.contains(publicKey)) {
                        Runtime.launch {
                            try {
                                it.key.send(Frame.Text(message))
                            } finally {
                            }
                        }
                    }
                }
            }
        }
    }

    fun configureHeaders(config: DefaultHeaders.Configuration) {
        config.header(HttpHeaders.Server, "${Version.name}/${Version.version} ${Version.http_server}/${Version.http_server_version}")
    }
}

fun Application.APIServer() {
    install(DefaultHeaders) {
        APIServer.configureHeaders(this)
    }
    install(StatusPages) {
        exception<Throwable> { cause ->
            val status = HttpStatusCode.InternalServerError
            call.respond(status, cause.message ?: cause::class.simpleName ?: status.description)
            throw cause // 日志记录
        }
    }
    install(WebSockets)

    routing {
        get("/") {
            call.respondRedirect("static/index.html")
        }

        static("static") {
            files(Config.htmlDir)
        }

        webSocket("/api/v2/websocket") {
            try {
                while (true) {
                    val string = (incoming.receive() as Frame.Text).readText()
                    val request = Json.parseJson(string).jsonObject
                    val command = request.getPrimitive("command").content

                    if (command == "subscribe") {
                        val route = request.getPrimitive("route").content

                        if (route == "block") {
                            APIServer.blockNotify.add(outgoing)
                        } else if (route == "txpool") {
                            APIServer.txPoolNotify.add(outgoing)
                        } else if (route == "wallet") {
                            val address = request.getPrimitive("address").content
                            val publicKey = Address.decode(address)!!

                            APIServer.walletNotify.mutex.withLock {
                                val keys = APIServer.walletNotify.map.get(outgoing)
                                if (keys == null) {
                                    @Suppress("NAME_SHADOWING")
                                    val keys = HashSet<PublicKey>()
                                    keys.add(publicKey)
                                    APIServer.walletNotify.map.put(outgoing, keys)
                                } else {
                                    keys.add(publicKey)
                                }
                            }
                        }
                    } else if (command == "unsubscribe") {
                        val route = request.getPrimitive("route").content

                        if (route == "block") {
                            APIServer.blockNotify.remove(outgoing)
                        } else if (route == "txpool") {
                            APIServer.txPoolNotify.remove(outgoing)
                        } else if (route == "wallet") {
                            val address = request.getPrimitive("address").content
                            val publicKey = Address.decode(address)!!

                            APIServer.walletNotify.mutex.withLock {
                                val keys = APIServer.walletNotify.map.get(outgoing)
                                if (keys != null) {
                                    keys.remove(publicKey)
                                    if (keys.isEmpty())
                                        APIServer.walletNotify.map.remove(outgoing)
                                }
                            }
                        }
                    }
                }
            } catch (e: ClosedReceiveChannelException) {
            } finally {
                APIServer.blockNotify.remove(outgoing)
                APIServer.txPoolNotify.remove(outgoing)
                APIServer.walletNotify.remove(outgoing)
            }
        }

        get("/api/v2/peerdb") {
            call.respondJson(PeerDBInfo.serializer(), PeerDBInfo.get())
        }

        get("/api/v2/peerdb/networkstat") {
            call.respondJson(PeerDBInfo.serializer(), PeerDBInfo.get(true))
        }

        get("/api/v2/leveldb/stats") {
            call.respond(LevelDB.getProperty("leveldb.stats") ?: "Not implemented")
        }

        get("/api/v2/block/{hash}/{txdetail?}") {
            val hash = Hash.fromString(call.parameters["hash"]) ?: return@get call.respond(HttpStatusCode.BadRequest, "Invalid hash")
            val txdetail = call.parameters["txdetail"]?.toBoolean() ?: false

            val result = BlockDB.block(hash)
            if (result != null)
                call.respondJson(BlockInfo.serializer(), BlockInfo(result.first, hash, result.second, txdetail))
            else
                call.respond(HttpStatusCode.BadRequest, "Block not found")
        }

        get("/api/v2/blockhash/{height}") {
            val height = call.parameters["height"]?.toIntOrNull() ?: return@get call.respond(HttpStatusCode.BadRequest, "Invalid height")

            BlockDB.mutex.withLock {
                val state = LedgerDB.state()
                if (height < 0 || height > state.height)
                    return@get call.respond(HttpStatusCode.BadRequest, "Block not found")
                else if (height == 0)
                    return@get call.respond(Hash.ZERO.toString())
                else if (height == state.height)
                    return@get call.respond(state.blockHash.toString())

                val lastIndex = APIServer.lastIndex
                if (lastIndex != null && lastIndex.second.height == height)
                    return@get call.respond(lastIndex.first.toString())

                var hash: Hash
                var index: ChainIndex
                if (height < state.height / 2) {
                    hash = Hash.ZERO
                    index = LedgerDB.getChainIndex(hash)!!
                } else {
                    hash = state.blockHash
                    index = LedgerDB.getChainIndex(hash)!!
                }
                if (lastIndex != null && abs(height - index.height) > abs(height - lastIndex.second.height))
                    index = lastIndex.second
                while (index.height > height) {
                    hash = index.previous
                    index = LedgerDB.getChainIndex(hash)!!
                }
                while (index.height < height) {
                    hash = index.next
                    index = LedgerDB.getChainIndex(hash)!!
                }
                if (index.height < state.height - PoS.MATURITY + 1)
                    APIServer.lastIndex = Pair(hash, index)
                call.respond(hash.toString())
            }
        }

        get("/api/v2/blockindex/{hash}/") {
            val hash = Hash.fromString(call.parameters["hash"]) ?: return@get call.respond(HttpStatusCode.BadRequest, "Invalid hash")

            val result = LedgerDB.getChainIndex(hash)
            if (result != null)
                call.respondJson(ChainIndex.serializer(), result)
            else
                call.respond(HttpStatusCode.BadRequest, "Block not found")
        }

        get("/api/v2/makebootstrap") {
            val checkpoint = LedgerDB.state().rollingCheckpoint
            if (checkpoint == Hash.ZERO)
                return@get call.respond(HttpStatusCode.BadRequest, "Not synchronized")

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

        get("/api/v2/ledger") {
            call.respondJson(LedgerInfo.serializer(), LedgerInfo.get())
        }

        get("/api/v2/account/{address}/{confirmations?}") {
            val publicKey = Address.decode(call.parameters["address"]) ?: return@get call.respond(HttpStatusCode.BadRequest, "Invalid address")
            val confirmations = call.parameters["confirmations"]?.toIntOrNull() ?: PoS.DEFAULT_CONFIRMATIONS
            val result = AccountInfo.get(publicKey, confirmations)
            if (result != null)
                call.respondJson(AccountInfo.serializer(), result)
            else
                call.respond(HttpStatusCode.BadRequest, "Account not found")
        }

        get("/api/v2/ledger/check") {
            call.respondJson(LedgerDB.Check.serializer(), LedgerDB.check())
        }

        get("/api/v2/ledger/schedulesnapshot/{height}") {
            val height = call.parameters["height"]?.toIntOrNull() ?: return@get call.respond(HttpStatusCode.BadRequest, "Invalid height")

            val result = BlockDB.mutex.withLock {
                LedgerDB.scheduleSnapshotImpl(height)
            }

            call.respond(result.toString())
        }

        get("/api/v2/ledger/snapshot/{height}") {
            val height = call.parameters["height"]?.toIntOrNull() ?: return@get call.respond(HttpStatusCode.BadRequest, "Invalid height")

            val result = LedgerDB.getSnapshot(height)

            if (result != null)
                call.respondJson(LedgerDB.Snapshot.serializer(), result)
            else
                call.respond(HttpStatusCode.BadRequest, "Snapshot not found")
        }

        debug()
        sendTransaction()
        staking()
        node()
        wallet()

        // 已被弃用
        APIV1()
    }
}

/**
 * Responds to a client with an [obj]ect, using provided [serializer] and the [ContentType.Application.Json].
 *
 * @param serializer the serialization strategy
 * @param obj the object serializable to JSON
 */
suspend fun <T> ApplicationCall.respondJson(serializer: SerializationStrategy<T>, obj: T) {
    respondText(ContentType.Application.Json, HttpStatusCode.OK) {
        Json.stringify(serializer, obj)
    }
}
