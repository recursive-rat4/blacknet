/*
 * Copyright (c) 2018-2024 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.rpc.v2

import java.math.BigInteger
import kotlinx.serialization.Serializable
import ninja.blacknet.crypto.BigIntegerSerializer
import ninja.blacknet.crypto.Hash
import ninja.blacknet.db.CoinDB
import ninja.blacknet.db.Genesis
import ninja.blacknet.network.Connection
import ninja.blacknet.network.Node
import ninja.blacknet.network.packet.BlockAnnounce
import ninja.blacknet.serialization.LongSerializer

@Serializable
class PeerInfo(
        val peerId: Long,
        val remoteAddress: String,
        val localAddress: String,
        val timeOffset: Long,
        val ping: Long,
        val protocolVersion: Int,
        val agent: String,
        val outgoing: Boolean,
        val banScore: Int,
        @Serializable(with = LongSerializer::class)
        val feeFilter: Long,
        val connectedAt: Long,
        val lastChain: ChainInfo,
        val requestedBlocks: Boolean,
        val totalBytesRead: Long,
        val totalBytesWritten: Long,
) {
    @Serializable
    class ChainInfo(
            val chain: Hash,
            @Serializable(with = BigIntegerSerializer::class)
            val cumulativeDifficulty: BigInteger,
            val fork: Boolean
    ) {
        constructor(chain: BlockAnnounce, fork: Boolean) : this(
                chain.hash,
                chain.cumulativeDifficulty,
                fork
        )

        companion object {
            fun get(chain: BlockAnnounce, forkCache: HashMap<Hash, Boolean>): ChainInfo {
                val fork = forkCache.computeIfAbsent(chain.hash) { !CoinDB.blockIndexes.contains(it.bytes) }
                return ChainInfo(chain, fork)
            }
        }
    }

    companion object {
        fun get(connection: Connection, forkCache: HashMap<Hash, Boolean>): PeerInfo {
            return PeerInfo(
                    connection.peerId,
                    connection.remoteAddress.toString(),
                    connection.localAddress.toString(),
                    connection.timeOffset,
                    connection.ping,
                    connection.version,
                    connection.agent,
                    connection.state.isOutgoing(),
                    connection.dosScore(),
                    connection.feeFilter,
                    connection.connectedAt,
                    ChainInfo.get(connection.lastBlock, forkCache),
                    connection.requestedBlocks,
                    connection.getTotalBytesRead(),
                    connection.getTotalBytesWritten(),
            )
        }

        fun getAll(): List<PeerInfo> {
            val forkCache = HashMap<Hash, Boolean>()
            forkCache.put(Genesis.BLOCK_HASH, false)
            return Node.connections.map { PeerInfo.get(it, forkCache) }
        }
    }
}
