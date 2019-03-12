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
    override fun serialize(): ByteReadPacket = BlacknetEncoder.toPacket(serializer(), this)

    override fun getType() = PacketType.GetBlocks

    override suspend fun process(connection: Connection) {
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

        var index = LedgerDB.getBlockNumber(best)
        if (index == null) {
            logger.error("number $best null")
            connection.sendPacket(Blocks(ArrayList(), ArrayList()))
            return
        }

        val height = LedgerDB.height()
        val maxSize = LedgerDB.DEFAULT_MAX_BLOCK_SIZE // we don't know actual value, so assume minimum
        val response = ArrayList<SerializableByteArray>()

        var size = 8

        while (index < height) {
            index++
            val hash = LedgerDB.getBlockHash(index)
            if (hash == null) {
                logger.error("hash $index null")
                break
            }
            val bytes = BlockDB.get(hash)
            if (bytes == null) {
                logger.error("block $hash null")
                break
            }
            size += bytes.size + 4
            if (size > maxSize) {
                if (response.isEmpty())
                    response.add(SerializableByteArray(bytes))
                break
            }
            response.add(SerializableByteArray(bytes))
        }

        connection.sendPacket(Blocks(ArrayList(), response))
    }
}
