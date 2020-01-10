/*
 * Copyright (c) 2018-2019 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.network

import kotlinx.coroutines.channels.Channel
import ninja.blacknet.Runtime
import ninja.blacknet.core.TxPool
import ninja.blacknet.crypto.Hash
import ninja.blacknet.packet.GetTransactions
import ninja.blacknet.packet.Transactions
import ninja.blacknet.time.SystemClock
import ninja.blacknet.time.delay
import ninja.blacknet.time.milliseconds.MilliSeconds
import ninja.blacknet.util.SynchronizedHashMap

object TxFetcher {
    private val inventoryChannel: Channel<Pair<Connection, List<Hash>>> = Channel(Channel.UNLIMITED)
    private val requested = SynchronizedHashMap<Hash, MilliSeconds>()

    init {
        Runtime.rotate(::implementation)
        Runtime.rotate(::watchdog)
    }

    fun offer(connection: Connection, list: List<Hash>) {
        inventoryChannel.offer(Pair(connection, list))
    }

    suspend fun fetched(hash: Hash): Boolean {
        return requested.remove(hash) != null
    }

    private suspend fun implementation() {
        val (connection, inventory) = inventoryChannel.receive()

        val request = ArrayList<Hash>(inventory.size)
        val currTime = SystemClock.milliseconds

        for (hash in inventory) {
            if (requested.containsKey(hash)) {
                continue
            }

            if (TxPool.isInteresting(hash)) {
                requested.put(hash, currTime)
                request.add(hash)
            }

            if (request.size == Transactions.MAX) {
                sendRequest(connection, request)
                request.clear()
            }
        }

        if (request.size != 0) {
            sendRequest(connection, request)
        }
    }

    private fun sendRequest(connection: Connection, request: ArrayList<Hash>) {
        connection.sendPacket(GetTransactions(request))
    }

    private suspend fun watchdog() {
        delay(Node.NETWORK_TIMEOUT)

        val currTime = SystemClock.milliseconds
        val timeouted = requested.filterToKeyList { _, time -> currTime > time + Node.NETWORK_TIMEOUT }
        requested.removeAll(timeouted)
    }
}
