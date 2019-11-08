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
class Transactions(
        private val list: ArrayList<SerializableByteArray>
) : Packet {
    override fun serialize(): ByteReadPacket = BinaryEncoder.toPacket(serializer(), this)

    override fun getType() = PacketType.Transactions

    override suspend fun process(connection: Connection) {
        if (list.size > DataType.MAX_DATA) {
            connection.dos("invalid Transactions size")
            return
        }

        val inv = UnfilteredInvList()
        val time = connection.lastPacketTime

        for (bytes in list) {
            val hash = Transaction.Hasher(bytes.array)

            if (!TxFetcher.fetched(hash)) {
                connection.dos("unrequested $hash")
                continue
            }

            val (status, fee) = TxPool.processTx(hash, bytes.array, time, true)

            when (status) {
                Accepted -> inv.add(Pair(hash, fee))
                is Invalid -> connection.dos("${status.reason} $hash")
                InFuture -> logger.debug { "In future $hash" }
                NotOnThisChain -> logger.debug { "Not on this chain $hash" }
                AlreadyHave -> logger.debug { "Already have $hash" }
            }
        }

        if (inv.isNotEmpty()) {
            Node.broadcastInv(inv, connection)
            connection.lastTxTime = time
        }
    }

    companion object {
        const val MIN_VERSION = 10
    }
}
