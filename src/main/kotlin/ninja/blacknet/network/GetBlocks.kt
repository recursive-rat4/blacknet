/*
 * Copyright (c) 2018 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.network

import kotlinx.io.core.ByteReadPacket
import kotlinx.serialization.Serializable
import kotlinx.serialization.encode
import mu.KotlinLogging
import ninja.blacknet.core.PoS
import ninja.blacknet.crypto.Hash
import ninja.blacknet.db.BlockDB
import ninja.blacknet.db.LedgerDB
import ninja.blacknet.serialization.BlacknetEncoder
import ninja.blacknet.serialization.SerializableByteArray

private val logger = KotlinLogging.logger {}

@Serializable
class GetBlocks(
        private val best: Hash,
        private val checkpoint: Hash
) : Packet {
    override fun serialize(): ByteReadPacket {
        val out = BlacknetEncoder()
        out.encode(serializer(), this)
        return out.build()
    }

    override fun getType(): Int {
        return PacketType.GetBlocks.ordinal
    }

    override suspend fun process(connection: Connection) {
        if (Node.isSynchronizing())
            return

        if (checkpoint != Hash.ZERO) {
            if (!BlockDB.contains(checkpoint)) {
                logger.info("Chain fork $best Disconnecting ${connection.remoteAddress}")
                connection.close()
                return
            }
        }

        if (best != Hash.ZERO && !BlockDB.contains(best)) {
            val response = LedgerDB.getNextBlockHashes(checkpoint, PoS.MATURITY)
            connection.sendPacket(Blocks(response, ArrayList()))
            return
        }

        val height = LedgerDB.height()
        val maxSize = Node.getMaxPacketSize()
        val response = ArrayList<SerializableByteArray>()

        var index = LedgerDB.getBlockNumber(best)!!
        var size = 8

        while (index < height) {
            index++
            val bytes = BlockDB.get(LedgerDB.getBlockHash(index)!!)!!
            if (size + bytes.size + 4 > maxSize)
                break
            size += bytes.size + 4
            response.add(SerializableByteArray(bytes))
        }

        connection.sendPacket(Blocks(ArrayList(), response))
    }
}
