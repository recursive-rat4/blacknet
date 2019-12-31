/*
 * Copyright (c) 2018-2019 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.packet

import kotlinx.io.core.ByteReadPacket
import kotlinx.serialization.Serializable
import ninja.blacknet.crypto.Hash
import ninja.blacknet.crypto.PoS
import ninja.blacknet.network.ChainFetcher
import ninja.blacknet.network.Connection
import ninja.blacknet.serialization.BinaryEncoder
import ninja.blacknet.serialization.SerializableByteArray

@Serializable
class Blocks(
        val hashes: List<Hash>,
        val blocks: List<SerializableByteArray>
) : Packet {
    override fun serialize(): ByteReadPacket = BinaryEncoder.toPacket(serializer(), this)

    override fun getType() = PacketType.Blocks

    override suspend fun process(connection: Connection) {
        if (hashes.size > MAX_HASHES) {
            connection.dos("Invalid hashes size ${hashes.size}")
            return
        }
        if (blocks.size > MAX_BLOCKS) {
            connection.dos("Invalid blocks size ${blocks.size}")
            return
        }
        if (hashes.isNotEmpty() && blocks.isNotEmpty()) {
            connection.dos("Invalid blocks size ${blocks.size} hashes size ${hashes.size}")
            return
        }

        if (!connection.requestedBlocks) {
            connection.dos("Unexpected Blocks")
            return
        }

        connection.requestedBlocks = false
        ChainFetcher.blocks(connection, this)
    }

    companion object {
        const val MAX_BLOCKS = 1000
        const val MAX_HASHES = PoS.MATURITY
    }
}
