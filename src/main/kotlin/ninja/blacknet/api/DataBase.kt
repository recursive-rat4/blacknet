/*
 * Copyright (c) 2018-2019 Pavel Vasin
 * Copyright (c) 2019 Blacknet Team
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.api

import io.ktor.application.ApplicationCall
import io.ktor.application.call
import io.ktor.http.HttpStatusCode
import io.ktor.response.respond
import io.ktor.routing.Route
import io.ktor.routing.get
import kotlinx.coroutines.sync.withLock
import kotlinx.serialization.Serializable
import ninja.blacknet.core.ChainIndex
import ninja.blacknet.crypto.Hash
import ninja.blacknet.crypto.PoS
import ninja.blacknet.crypto.PublicKey
import ninja.blacknet.dataDir
import ninja.blacknet.db.BlockDB
import ninja.blacknet.db.LedgerDB
import ninja.blacknet.db.LevelDB
import ninja.blacknet.ktor.requests.Request
import ninja.blacknet.ktor.requests.get
import ninja.blacknet.util.buffered
import ninja.blacknet.util.data
import java.io.File
import kotlin.math.abs

fun Route.dataBase() {
    get("/api/v2/peerdb") {
        call.respondJson(PeerDBInfo.serializer(), PeerDBInfo.get())
    }

    get("/api/v2/peerdb/networkstat") {
        call.respondJson(PeerDBInfo.serializer(), PeerDBInfo.get(true))
    }

    get("/api/v2/leveldb/stats") {
        call.respond(LevelDB.getProperty("leveldb.stats") ?: "Not implemented")
    }

    @Serializable
    class Block(
            val hash: Hash,
            val txdetail: Boolean = false
    ) : Request {
        override suspend fun handle(call: ApplicationCall): Unit {
            val (block, size) = BlockDB.block(hash)
                    ?: return call.respond(HttpStatusCode.BadRequest, "Block not found")
            return call.respondJson(BlockInfo.serializer(), BlockInfo(block, hash, size, txdetail))
        }
    }

    get(Block.serializer(), "/api/v2/block")
    get(Block.serializer(), "/api/v2/block/{hash}/{txdetail?}")

    @Serializable
    class BlockHash(
            val height: Int
    ) : Request {
        override suspend fun handle(call: ApplicationCall): Unit = BlockDB.mutex.withLock {
            val state = LedgerDB.state()
            if (height < 0 || height > state.height)
                return call.respond(HttpStatusCode.BadRequest, "Block not found")
            else if (height == 0)
                return call.respond(Hash.ZERO.toString())
            else if (height == state.height)
                return call.respond(state.blockHash.toString())

            val lastIndex = APIServer.lastIndex
            if (lastIndex != null && lastIndex.second.height.int == height)
                return call.respond(lastIndex.first.toString())

            var hash: Hash
            var index: ChainIndex
            if (height < state.height / 2) {
                hash = Hash.ZERO
                index = LedgerDB.getChainIndex(hash)!!
            } else {
                hash = state.blockHash
                index = LedgerDB.getChainIndex(hash)!!
            }
            if (lastIndex != null && abs(height - index.height.int) > abs(height - lastIndex.second.height.int))
                index = lastIndex.second
            while (index.height.int > height) {
                hash = index.previous
                index = LedgerDB.getChainIndex(hash)!!
            }
            while (index.height.int < height) {
                hash = index.next
                index = LedgerDB.getChainIndex(hash)!!
            }
            if (index.height.int < state.height - PoS.MATURITY + 1)
                APIServer.lastIndex = Pair(hash, index)

            return call.respond(hash.toString())
        }
    }

    get(BlockHash.serializer(), "/api/v2/blockhash")
    get(BlockHash.serializer(), "/api/v2/blockhash/{height}")

    @Serializable
    class BlockIndex(
            val hash: Hash
    ) : Request {
        override suspend fun handle(call: ApplicationCall): Unit {
            val index = LedgerDB.getChainIndex(hash)
            return if (index != null)
                call.respondJson(ChainIndex.serializer(), index)
            else
                call.respond(HttpStatusCode.BadRequest, "Block not found")
        }
    }

    get(BlockIndex.serializer(), "/api/v2/blockindex")
    get(BlockIndex.serializer(), "/api/v2/blockindex/{hash}/")

    get("/api/v2/makebootstrap") {
        val checkpoint = LedgerDB.state().rollingCheckpoint
        if (checkpoint == Hash.ZERO)
            return@get call.respond(HttpStatusCode.BadRequest, "Not synchronized")

        val file = File(dataDir, "bootstrap.dat.new")
        val stream = file.outputStream().buffered().data()

        var hash = Hash.ZERO
        var index = LedgerDB.getChainIndex(hash)!!
        do {
            hash = index.next
            index = LedgerDB.getChainIndex(hash)!!
            val bytes = BlockDB.getImpl(hash)!!
            stream.writeInt(bytes.size)
            stream.write(bytes, 0, bytes.size)
        } while (hash != checkpoint)

        stream.close()

        call.respond(file.absolutePath)
    }

    get("/api/v2/ledger") {
        call.respondJson(LedgerInfo.serializer(), LedgerInfo.get())
    }

    @Serializable
    class Account(
            val address: PublicKey,
            val confirmations: Int = PoS.DEFAULT_CONFIRMATIONS
    ) : Request {
        override suspend fun handle(call: ApplicationCall): Unit {
            val info = AccountInfo.get(address, confirmations)
            return if (info != null)
                call.respondJson(AccountInfo.serializer(), info)
            else
                call.respond(HttpStatusCode.BadRequest, "Account not found")
        }
    }

    get(Account.serializer(), "/api/v2/account")
    get(Account.serializer(), "/api/v2/account/{address}/{confirmations?}")

    get("/api/v2/ledger/check") {
        call.respondJson(LedgerDB.Check.serializer(), LedgerDB.check())
    }

    @Serializable
    class ScheduleSnapshot(
            val height: Int
    ) : Request {
        override suspend fun handle(call: ApplicationCall): Unit = BlockDB.mutex.withLock {
            val scheduled = LedgerDB.scheduleSnapshotImpl(height)
            return call.respond(scheduled.toString())
        }
    }

    get(ScheduleSnapshot.serializer(), "/api/v2/ledger/schedulesnapshot")
    get(ScheduleSnapshot.serializer(), "/api/v2/ledger/schedulesnapshot/{height}")

    @Serializable
    class Snapshot(
            val height: Int
    ) : Request {
        override suspend fun handle(call: ApplicationCall): Unit {
            val snapshot = LedgerDB.getSnapshot(height)
            return if (snapshot != null)
                call.respondJson(LedgerDB.Snapshot.serializer(), snapshot)
            else
                call.respond(HttpStatusCode.BadRequest, "Snapshot not found")
        }
    }

    get(Snapshot.serializer(), "/api/v2/ledger/snapshot")
    get(Snapshot.serializer(), "/api/v2/ledger/snapshot/{height}")
}
