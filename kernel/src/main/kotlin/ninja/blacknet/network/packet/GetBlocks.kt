/*
 * Copyright (c) 2018-2024 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.network.packet

import kotlinx.serialization.Serializable
import ninja.blacknet.Kernel
import ninja.blacknet.crypto.Hash
import ninja.blacknet.db.LedgerDB
import ninja.blacknet.network.Connection
import ninja.blacknet.network.Node

@Serializable
class GetBlocks(
    private val best: Hash,
    private val checkpoint: Hash
) : Packet {
    override suspend fun process(connection: Connection) {
        val cachedBlock = Kernel.blockDB().cachedBlock
        if (cachedBlock != null) {
            val (previousHash, bytes) = cachedBlock
            if (previousHash == best) {
                connection.sendPacket(PacketType.Blocks, Blocks(emptyList(), listOf(bytes)))
                return
            }
        }

        var chainIndex = LedgerDB.chainIndexes.get(best.bytes)

        if (chainIndex == null) {
            val nextBlockHashes = LedgerDB.getNextBlockHashes(checkpoint, Blocks.MAX_HASHES)
            if (nextBlockHashes != null) {
                connection.sendPacket(PacketType.Blocks, Blocks(nextBlockHashes, emptyList()))
                return
            } else {
                connection.sendPacket(PacketType.ChainFork, ChainFork())
                connection.dos("Chain fork")
                return
            }
        }

        var size = PACKET_HEADER_SIZE_BYTES + 2 + 1
        val maxSize = Node.getMinPacketSize() // we don't know actual value, so assume minimum
        val response = ArrayList<ByteArray>()

        while (true) {
            if (chainIndex == null)
                break
            val hash = chainIndex.next
            if (hash == Hash.ZERO)
                break
            size += chainIndex.nextSize + 4 //TODO VarInt.size()
            if (response.isNotEmpty() && size >= maxSize)
                break
            val bytes = Kernel.blockDB().blocks.getBytes(hash.bytes)
            if (bytes == null)
                break
            response.add(bytes)
            if (response.size == Blocks.MAX_BLOCKS)
                break
            chainIndex = LedgerDB.chainIndexes.get(hash.bytes)
        }

        connection.sendPacket(PacketType.Blocks, Blocks(emptyList(), response))
    }
}
