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
import ninja.blacknet.crypto.Hash
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

    fun isEmpty(): Boolean {
        return hashes.isEmpty() && blocks.isEmpty()
    }

    override suspend fun process(connection: Connection) {
        ChainFetcher.blocks(connection, this)
    }
}
