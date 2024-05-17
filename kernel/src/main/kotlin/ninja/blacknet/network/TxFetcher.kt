/*
 * Copyright (c) 2018-2024 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.network

import java.lang.Thread.sleep
import java.util.concurrent.ConcurrentHashMap
import java.util.concurrent.LinkedBlockingQueue
import ninja.blacknet.Kernel
import ninja.blacknet.crypto.Hash
import ninja.blacknet.network.packet.GetTransactions
import ninja.blacknet.network.packet.PacketType
import ninja.blacknet.network.packet.Transactions
import ninja.blacknet.time.currentTimeMillis
import ninja.blacknet.util.rotate

/**
 * 交易獲取器
 */
object TxFetcher {
    private val inventoryQueue = LinkedBlockingQueue<Pair<Connection, List<Hash>>>()
    //TODO review capacity
    private val requested = ConcurrentHashMap<Hash, Request>()

    init {
        rotate("TxFetcher::implementation", ::implementation)
        rotate("TxFetcher::watchdog", ::watchdog)
    }

    fun offer(connection: Connection, list: List<Hash>) {
        inventoryQueue.offer(Pair(connection, list))
    }

    fun fetched(connection: Connection, hash: Hash): Boolean {
        val request = requested.get(hash)
        if (request != null && request.connection == connection) {
            return requested.remove(hash) != null
        }
        return false
    }

    private fun implementation() {
        val (connection, inventory) = inventoryQueue.take()

        var request = ArrayList<Hash>(inventory.size)
        val currTime = currentTimeMillis()

        for (hash in inventory) {
            if (requested.containsKey(hash)) {
                continue
            }

            if (Kernel.txPool().isInteresting(hash)) {
                requested.put(hash, Request(currTime, connection))
                request.add(hash)
            }

            if (request.size == Transactions.MAX) {
                sendRequest(connection, request)
                request = ArrayList(inventory.size)
            }
        }

        if (request.size != 0) {
            sendRequest(connection, request)
        }
    }

    private fun sendRequest(connection: Connection, request: ArrayList<Hash>) {
        connection.sendPacket(PacketType.GetTransactions, GetTransactions(request))
    }

    /**
     * 看門狗計時器
     */
    private fun watchdog() {
        sleep(Node.NETWORK_TIMEOUT)

        val currTime = currentTimeMillis()
        requested.forEach { (hash, request) ->
            if (currTime > request.time + Node.NETWORK_TIMEOUT)
                requested.remove(hash)
        }
    }

    private class Request(
        val time: Long,
        val connection: Connection,
    )
}
