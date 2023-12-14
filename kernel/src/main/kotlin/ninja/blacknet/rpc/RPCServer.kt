/*
 * Copyright (c) 2018-2020 Pavel Vasin
 * Copyright (c) 2019 Blacknet Team
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.rpc

import io.github.oshai.kotlinlogging.KotlinLogging
import io.ktor.http.HttpHeaders
import io.ktor.http.HttpStatusCode
import io.ktor.server.application.Application
import io.ktor.server.application.install
import io.ktor.server.plugins.defaultheaders.DefaultHeaders
import io.ktor.server.plugins.statuspages.StatusPages
import io.ktor.server.response.respond
import io.ktor.server.routing.Routing
import io.ktor.server.websocket.WebSockets
import io.ktor.websocket.Frame
import kotlinx.coroutines.channels.SendChannel
import kotlinx.coroutines.launch
import kotlinx.coroutines.sync.Mutex
import kotlinx.coroutines.sync.withLock
import ninja.blacknet.Runtime
import ninja.blacknet.Version
import ninja.blacknet.core.Block
import ninja.blacknet.core.ChainIndex
import ninja.blacknet.core.Transaction
import ninja.blacknet.crypto.PublicKey
import ninja.blacknet.db.WalletDB
import ninja.blacknet.logging.debug
import ninja.blacknet.logging.debugMessage
import ninja.blacknet.logging.error
import ninja.blacknet.rpc.requests.Requests
import ninja.blacknet.rpc.v1.*
import ninja.blacknet.rpc.v2.*
import ninja.blacknet.serialization.json.json
import ninja.blacknet.util.SynchronizedHashMap
import ninja.blacknet.util.SynchronizedHashSet
import ninja.blacknet.util.statusMessage

private val logger = KotlinLogging.logger {}

object RPCServer {
    internal val txMutex = Mutex()
    internal var lastIndex: Pair<ByteArray, ChainIndex>? = null
    internal val blockNotify = SynchronizedHashSet<SendChannel<Frame>>()
    internal val txPoolNotify = SynchronizedHashSet<SendChannel<Frame>>()
    internal val walletNotify = SynchronizedHashMap<PublicKey, ArrayList<SendChannel<Frame>>>()

    suspend fun blockNotify(block: Block, hash: ByteArray, height: Int, size: Int) {
        RPCServerV1.blockNotify(block, hash, height, size)

        blockNotify.mutex.withLock {
            if (blockNotify.set.isNotEmpty()) {
                val notification = WebSocketNotification(BlockNotification(block, hash, height, size))
                val message = json.encodeToString(WebSocketNotification.serializer(), notification)
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

    suspend fun txPoolNotify(tx: Transaction, hash: ByteArray, time: Long, size: Int) {
        txPoolNotify.mutex.withLock {
            if (txPoolNotify.set.isNotEmpty()) {
                val notification = WebSocketNotification(TransactionNotification(tx, hash, time, size))
                val message = json.encodeToString(WebSocketNotification.serializer(), notification)
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

    suspend fun walletNotify(tx: Transaction, hash: ByteArray, time: Long, size: Int, publicKey: PublicKey, filter: List<WalletDB.TransactionDataType>) {
        RPCServerV1.walletNotify(tx, hash, time, size, publicKey, filter)

        walletNotify.mutex.withLock {
            val subscribers = walletNotify.map.get(publicKey)
            if (subscribers != null) {
                if (subscribers.isNotEmpty()) {
                    val notification = WebSocketNotification(TransactionNotification(tx, hash, time, size, filter))
                    val message = json.encodeToString(WebSocketNotification.serializer(), notification)
                    subscribers.forEach {
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
    }
}

fun Application.RPCServer() {
    install(DefaultHeaders) {
        header(HttpHeaders.Server, "${Version.name}/${Version.version} ${Version.http_server}/${Version.http_server_version} ${Version.http_server_engine}/${Version.http_server_engine_version}")
    }
    install(StatusPages) {
        exception<Exception> { call, cause ->
            call.respond(HttpStatusCode.BadRequest, cause.statusMessage())
            logger.debug(cause)
        }
        exception<Throwable> { call, cause ->
            call.respond(HttpStatusCode.InternalServerError, cause.debugMessage())
            logger.error(cause)
        }
    }
    install(WebSockets)
    install(Requests)
    install(Routing) {
        html()

        dataBase()
        debug()
        sendTransaction()
        staking()
        node()
        wallet()
        webSocket()

        // 已被棄用
        APIV1()
    }
}
