/*
 * Copyright (c) 2018-2024 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.rpc.v1

import kotlinx.serialization.Serializable
import ninja.blacknet.crypto.Hash
import ninja.blacknet.db.CoinDB
import ninja.blacknet.network.packet.BlockAnnounce
import ninja.blacknet.network.Connection
import ninja.blacknet.network.Node

@Serializable
class PeerInfoV1(
        val peerId: Long,
        val remoteAddress: String,
        val localAddress: String,
        val timeOffset: Long,
        val ping: Long,
        val protocolVersion: Int,
        val agent: String,
        val state: String,
        val dosScore: Int,
        val feeFilter: String,
        val connectedAt: Long,
        val lastChain: ChainInfo,
        val totalBytesRead: Long,
        val totalBytesWritten: Long
) {
    @Serializable
    class ChainInfo(
            val chain: String,
            val cumulativeDifficulty: String,
            val fork: Boolean
    ) {
        constructor(chain: BlockAnnounce, fork: Boolean) : this(
                chain.hash.toString(),
                chain.cumulativeDifficulty.toString(),
                fork
        )

        companion object {
            fun get(chain: BlockAnnounce, forkCache: HashMap<Hash, Boolean>): ChainInfo {
                val cached = forkCache.get(chain.hash)
                return if (cached != null) {
                    ChainInfo(chain, cached)
                } else {
                    val fork = !CoinDB.blockIndexes.contains(chain.hash.bytes)
                    forkCache.put(chain.hash, fork)
                    ChainInfo(chain, fork)
                }
            }
        }
    }

    companion object {
        fun get(connection: Connection, forkCache: HashMap<Hash, Boolean>): PeerInfoV1 {
            return PeerInfoV1(
                    connection.peerId,
                    connection.remoteAddress.toString(),
                    connection.localAddress.toString(),
                    connection.timeOffset,
                    connection.ping,
                    connection.version,
                    connection.agent,
                    connection.state.name,
                    connection.dosScore(),
                    connection.feeFilter.toString(),
                    connection.connectedAt,
                    ChainInfo.get(connection.lastBlock, forkCache),
                    connection.getTotalBytesRead(),
                    connection.getTotalBytesWritten(),
            )
        }

        fun getAll(): List<PeerInfoV1> {
            val forkCache = HashMap<Hash, Boolean>()
            return Node.connections.map { PeerInfoV1.get(it, forkCache) }
        }
    }
}
