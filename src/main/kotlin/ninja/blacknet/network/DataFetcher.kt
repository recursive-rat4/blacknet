/*
 * Copyright (c) 2018 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.network

import kotlinx.coroutines.experimental.channels.LinkedListChannel
import kotlinx.coroutines.experimental.launch
import ninja.blacknet.core.delay
import ninja.blacknet.crypto.Hash

object DataFetcher {
    private val inventoryChannel = LinkedListChannel<Pair<Connection, InvList>>()
    private val requested = HashMap<Hash, Long>()

    init {
        launch { fetcher() }
        launch { watchdog() }
    }

    fun offer(from: Connection, list: InvList) {
        inventoryChannel.offer(Pair(from, list))
    }

    fun fetched(hash: Hash) {
        requested.remove(hash)
    }

    private suspend fun fetcher() {
        for (inventory in inventoryChannel) {
            val connection = inventory.first
            val list = inventory.second

            val request = InvList()
            val currTime = Node.time()

            for (i in list) {
                val type = i.first
                val hash = i.second

                if (requested.containsKey(hash))
                    continue

                if (type.db.isInteresting(hash)) {
                    requested[hash] = currTime
                    request.add(Pair(type, hash))
                }

                if (request.size == DataType.MAX_DATA) {
                    val getData = GetData(request)
                    connection.sendPacket(getData)
                    request.clear()
                }
            }

            if (request.size == 0)
                return

            val getData = GetData(request)
            connection.sendPacket(getData)
        }
    }

    private suspend fun watchdog() {
        while (true) {
            delay(Node.NETWORK_TIMEOUT)

            val currTime = Node.time()
            val timeouted = requested.filterValues { currTime > it + Node.NETWORK_TIMEOUT }
            timeouted.forEach { requested.remove(it.key) }
        }
    }
}