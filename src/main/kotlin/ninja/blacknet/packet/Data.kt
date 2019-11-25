/*
 * Copyright (c) 2018 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.packet

import kotlinx.io.core.ByteReadPacket
import kotlinx.serialization.Serializable
import mu.KotlinLogging
import ninja.blacknet.core.*
import ninja.blacknet.network.Connection
import ninja.blacknet.network.TxFetcher
import ninja.blacknet.network.Node
import ninja.blacknet.serialization.BinaryEncoder
import ninja.blacknet.serialization.SerializableByteArray

private val logger = KotlinLogging.logger {}

@Serializable
internal class Data(private val list: ArrayList<Pair<DataType, SerializableByteArray>>) : Packet {
    override fun serialize(): ByteReadPacket = BinaryEncoder.toPacket(serializer(), this)

    override fun getType() = PacketType.Data

    override suspend fun process(connection: Connection) {
        if (list.size > Transactions.MAX) {
            connection.dos("invalid Data size")
            return
        }

        val inv = UnfilteredInvList()
        val time = connection.lastPacketTime

        for (i in list) {
            val type = i.first
            val bytes = i.second

            if (type != DataType.Transaction) {
                connection.dos("Unexpected $type")
                continue
            }
            val hash = Transaction.Hasher(bytes.array)

            if (!TxFetcher.fetched(hash)) {
                connection.dos("unrequested ${type.name} $hash")
                continue
            }

            val (status, fee) = TxPool.process(hash, bytes.array, time, true)

            when (status) {
                Accepted -> inv.add(Pair(hash, fee))
                is Invalid -> connection.dos("${status.reason} ${type.name} $hash")
                InFuture -> logger.debug { "in future ${type.name} $hash" }
                NotOnThisChain -> logger.debug { "not on this chain ${type.name} $hash" }
                AlreadyHave -> logger.debug { "already have  ${type.name} $hash" }
            }
        }

        if (inv.isNotEmpty()) {
            Node.broadcastInv(inv, connection)
            connection.lastTxTime = time
        }
    }
}
