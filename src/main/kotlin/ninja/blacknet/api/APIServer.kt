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
import io.ktor.http.content.files
import io.ktor.http.content.static
import io.ktor.response.respond
import io.ktor.response.respondRedirect
import io.ktor.response.respondText
import io.ktor.routing.get
import io.ktor.routing.routing
import io.ktor.websocket.WebSockets
import kotlinx.coroutines.channels.SendChannel
import kotlinx.coroutines.launch
import kotlinx.coroutines.sync.Mutex
import kotlinx.coroutines.sync.withLock
import kotlinx.serialization.SerializationStrategy
import ninja.blacknet.Runtime
import ninja.blacknet.Version
import ninja.blacknet.api.v1.APIV1
import ninja.blacknet.api.v1.BlockNotificationV1
import ninja.blacknet.api.v1.TransactionNotificationV2
import ninja.blacknet.core.Block
import ninja.blacknet.core.ChainIndex
import ninja.blacknet.core.Transaction
import ninja.blacknet.crypto.Hash
import ninja.blacknet.crypto.PublicKey
import ninja.blacknet.db.WalletDB
import ninja.blacknet.htmlDir
import ninja.blacknet.serialization.Json
import ninja.blacknet.util.SynchronizedArrayList
import ninja.blacknet.util.SynchronizedHashMap
import ninja.blacknet.util.SynchronizedHashSet

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
            files(htmlDir)
        }

        dataBase()
        debug()
        sendTransaction()
        staking()
        node()
        wallet()
        webSocket()

        // 已被弃用
        APIV1()
    }
}

/**
 * Responds to a client with a [value], using provided [serializer] and the [ContentType.Application.Json].
 *
 * @param serializer the serialization strategy
 * @param value the object serializable to JSON
 */
suspend fun <T> ApplicationCall.respondJson(serializer: SerializationStrategy<T>, value: T) {
    respondText(ContentType.Application.Json, HttpStatusCode.OK) {
        Json.stringify(serializer, value)
    }
}
