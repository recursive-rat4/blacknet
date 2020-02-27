/*
 * Copyright (c) 2018-2019 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.api

import io.ktor.http.cio.websocket.Frame
import io.ktor.http.cio.websocket.readText
import io.ktor.routing.Route
import io.ktor.websocket.webSocket
import kotlinx.coroutines.channels.ClosedReceiveChannelException
import kotlinx.coroutines.sync.withLock
import ninja.blacknet.crypto.Address
import ninja.blacknet.crypto.PublicKey
import ninja.blacknet.serialization.Json

fun Route.webSocket() {
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
}
