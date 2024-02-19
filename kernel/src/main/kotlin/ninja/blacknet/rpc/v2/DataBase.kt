/*
 * Copyright (c) 2018-2024 Pavel Vasin
 * Copyright (c) 2019 Blacknet Team
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.rpc.v2

import io.ktor.server.routing.Route
import kotlin.concurrent.withLock
import kotlin.math.abs
import kotlinx.serialization.Serializable
import ninja.blacknet.Kernel
import ninja.blacknet.core.ChainIndex
import ninja.blacknet.crypto.Hash
import ninja.blacknet.crypto.PoS
import ninja.blacknet.crypto.PublicKey
import ninja.blacknet.db.*
import ninja.blacknet.rpc.RPCServer
import ninja.blacknet.rpc.requests.*
import ninja.blacknet.rpc.v1.LedgerInfo
import ninja.blacknet.rpc.v1.PeerDBInfo

@Serializable
class PeerDB : Request {
    override suspend fun handle(): TextContent {
        return respondJson(PeerDBInfo.serializer(), PeerDBInfo.get())
    }
}

@Serializable
class NetworkStat : Request {
    override suspend fun handle(): TextContent {
        return respondJson(PeerDBInfo.serializer(), PeerDBInfo.get(true))
    }
}

@Serializable
class LevelDBStats : Request {
    override suspend fun handle(): TextContent {
        return respondText(LevelDB.getProperty("leveldb.stats") ?: "Not implemented")
    }
}

@Serializable
class Block(
    val hash: Hash,
    val txdetail: Boolean = false
) : Request {
    override suspend fun handle(): TextContent {
        val (block, size) = Kernel.blockDB().blocks.getWithSize(hash.bytes) ?: return respondError("Block not found")
        return respondJson(BlockInfo.serializer(), BlockInfo(block, hash, size, txdetail))
    }
}

@Serializable
class BlockDBCheck : Request {
    override suspend fun handle(): TextContent {
        return respondJson(BlockDB.Check.serializer(), Kernel.blockDB().check())
    }
}

@Serializable
class BlockHash(
    val height: Int
) : Request {
    override suspend fun handle(): TextContent = Kernel.blockDB().reentrant.readLock().withLock {
        val state = LedgerDB.state()
        if (height < 0 || height > state.height)
            return respondError("Block not found")
        else if (height == 0)
            return respondText(Genesis.BLOCK_HASH.toString())
        else if (height == state.height)
            return respondText(state.blockHash.toString())

        val lastIndex = RPCServer.lastIndex
        if (lastIndex != null && lastIndex.second.height == height)
            return respondText(lastIndex.first.toString())

        var hash: Hash
        var index: ChainIndex
        if (height < state.height / 2) {
            hash = Genesis.BLOCK_HASH
            index = LedgerDB.chainIndexes.getOrThrow(hash.bytes)
        } else {
            hash = state.blockHash
            index = LedgerDB.chainIndexes.getOrThrow(hash.bytes)
        }
        if (lastIndex != null && abs(height - index.height) > abs(height - lastIndex.second.height))
            index = lastIndex.second
        while (index.height > height) {
            hash = index.previous
            index = LedgerDB.chainIndexes.getOrThrow(hash.bytes)
        }
        while (index.height < height) {
            hash = index.next
            index = LedgerDB.chainIndexes.getOrThrow(hash.bytes)
        }
        if (index.height < state.height - PoS.ROLLBACK_LIMIT + 1)
            RPCServer.lastIndex = Pair(hash, index)

        return respondText(hash.toString())
    }
}

@Serializable
class BlockIndex(
    val hash: Hash
) : Request {
    override suspend fun handle(): TextContent {
        val index = LedgerDB.chainIndexes.get(hash.bytes)
        return if (index != null)
            respondJson(ChainIndex.serializer(), index)
        else
            respondError("Block not found")
    }
}

@Serializable
class MakeBootstrap : Request {
    override suspend fun handle(): TextContent {
        val file = Bootstrap.export()
        return if (file != null)
            respondText(file.toAbsolutePath().toString())
        else
            respondError("Not synchronized")
    }
}

@Serializable
class Ledger : Request {
    override suspend fun handle(): TextContent {
        return respondJson(LedgerInfo.serializer(), LedgerInfo.get())
    }
}

@Serializable
class Account(
    val address: PublicKey,
    val confirmations: Int = PoS.DEFAULT_CONFIRMATIONS
) : Request {
    override suspend fun handle(): TextContent {
        val info = AccountInfo.get(address, confirmations)
        return if (info != null)
            respondJson(AccountInfo.serializer(), info)
        else
            respondError("Account not found")
    }
}

@Serializable
class LedgerDBCheck : Request {
    override suspend fun handle(): TextContent {
        return respondJson(LedgerDB.Check.serializer(), LedgerDB.check())
    }
}

@Serializable
class ScheduleSnapshot(
    val height: Int
) : Request {
    override suspend fun handle(): TextContent = Kernel.blockDB().reentrant.writeLock().withLock {
        val scheduled = LedgerDB.scheduleSnapshotImpl(height)
        return respondText(scheduled.toString())
    }
}

@Serializable
class Snapshot(
    val height: Int
) : Request {
    override suspend fun handle(): TextContent {
        val snapshot = LedgerDB.getSnapshot(height)
        return if (snapshot != null)
            respondJson(LedgerDB.Snapshot.serializer(), snapshot)
        else
            respondError("Snapshot not found")
    }
}

fun Route.dataBase() {
    get(PeerDB.serializer(), "/api/v2/peerdb")

    get(NetworkStat.serializer(), "/api/v2/peerdb/networkstat")

    get(LevelDBStats.serializer(), "/api/v2/leveldb/stats")

    get(Block.serializer(), "/api/v2/block")
    get(Block.serializer(), "/api/v2/block/{hash}/{txdetail?}")

    get(BlockDBCheck.serializer(), "/api/v2/blockdb/check")

    get(BlockHash.serializer(), "/api/v2/blockhash")
    get(BlockHash.serializer(), "/api/v2/blockhash/{height}")

    get(BlockIndex.serializer(), "/api/v2/blockindex")
    get(BlockIndex.serializer(), "/api/v2/blockindex/{hash}/")

    get(BlockIndex.serializer(), "/api/v2/blockindex/{hash}")

    get(MakeBootstrap.serializer(), "/api/v2/makebootstrap")

    get(Ledger.serializer(), "/api/v2/ledger")

    get(Account.serializer(), "/api/v2/account")
    get(Account.serializer(), "/api/v2/account/{address}/{confirmations?}")

    get(LedgerDBCheck.serializer(), "/api/v2/ledger/check")

    get(ScheduleSnapshot.serializer(), "/api/v2/ledger/schedulesnapshot")
    get(ScheduleSnapshot.serializer(), "/api/v2/ledger/schedulesnapshot/{height}")

    get(Snapshot.serializer(), "/api/v2/ledger/snapshot")
    get(Snapshot.serializer(), "/api/v2/ledger/snapshot/{height}")
}
