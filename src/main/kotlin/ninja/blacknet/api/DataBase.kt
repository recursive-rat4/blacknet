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

import io.ktor.application.call
import io.ktor.http.HttpStatusCode
import io.ktor.response.respond
import io.ktor.routing.Route
import io.ktor.routing.get
import kotlinx.coroutines.sync.withLock
import ninja.blacknet.core.ChainIndex
import ninja.blacknet.crypto.Address
import ninja.blacknet.crypto.Hash
import ninja.blacknet.crypto.PoS
import ninja.blacknet.dataDir
import ninja.blacknet.db.BlockDB
import ninja.blacknet.db.LedgerDB
import ninja.blacknet.db.LevelDB
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

    get("/api/v2/block/{hash}/{txdetail?}") {
        val hash = call.parameters["hash"]?.let { Hash.fromString(it) } ?: return@get call.respond(HttpStatusCode.BadRequest, "Invalid hash")
        val txdetail = call.parameters["txdetail"]?.toBoolean() ?: false

        val result = BlockDB.block(hash)
        if (result != null)
            call.respondJson(BlockInfo.serializer(), BlockInfo(result.first, hash, result.second, txdetail))
        else
            call.respond(HttpStatusCode.BadRequest, "Block not found")
    }

    get("/api/v2/blockhash/{height}") {
        val height = call.parameters["height"]?.toIntOrNull() ?: return@get call.respond(HttpStatusCode.BadRequest, "Invalid height")

        BlockDB.mutex.withLock {
            val state = LedgerDB.state()
            if (height < 0 || height > state.height)
                return@get call.respond(HttpStatusCode.BadRequest, "Block not found")
            else if (height == 0)
                return@get call.respond(Hash.ZERO.toString())
            else if (height == state.height)
                return@get call.respond(state.blockHash.toString())

            val lastIndex = APIServer.lastIndex
            if (lastIndex != null && lastIndex.second.height == height)
                return@get call.respond(lastIndex.first.toString())

            var hash: Hash
            var index: ChainIndex
            if (height < state.height / 2) {
                hash = Hash.ZERO
                index = LedgerDB.getChainIndex(hash)!!
            } else {
                hash = state.blockHash
                index = LedgerDB.getChainIndex(hash)!!
            }
            if (lastIndex != null && abs(height - index.height) > abs(height - lastIndex.second.height))
                index = lastIndex.second
            while (index.height > height) {
                hash = index.previous
                index = LedgerDB.getChainIndex(hash)!!
            }
            while (index.height < height) {
                hash = index.next
                index = LedgerDB.getChainIndex(hash)!!
            }
            if (index.height < state.height - PoS.MATURITY + 1)
                APIServer.lastIndex = Pair(hash, index)
            call.respond(hash.toString())
        }
    }

    get("/api/v2/blockindex/{hash}/") {
        val hash = call.parameters["hash"]?.let { Hash.fromString(it) } ?: return@get call.respond(HttpStatusCode.BadRequest, "Invalid hash")

        val result = LedgerDB.getChainIndex(hash)
        if (result != null)
            call.respondJson(ChainIndex.serializer(), result)
        else
            call.respond(HttpStatusCode.BadRequest, "Block not found")
    }

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

    get("/api/v2/account/{address}/{confirmations?}") {
        val publicKey = call.parameters["address"]?.let { Address.decode(it) } ?: return@get call.respond(HttpStatusCode.BadRequest, "Invalid address")
        val confirmations = call.parameters["confirmations"]?.toIntOrNull() ?: PoS.DEFAULT_CONFIRMATIONS
        val result = AccountInfo.get(publicKey, confirmations)
        if (result != null)
            call.respondJson(AccountInfo.serializer(), result)
        else
            call.respond(HttpStatusCode.BadRequest, "Account not found")
    }

    get("/api/v2/ledger/check") {
        call.respondJson(LedgerDB.Check.serializer(), LedgerDB.check())
    }

    get("/api/v2/ledger/schedulesnapshot/{height}") {
        val height = call.parameters["height"]?.toIntOrNull() ?: return@get call.respond(HttpStatusCode.BadRequest, "Invalid height")

        val result = BlockDB.mutex.withLock {
            LedgerDB.scheduleSnapshotImpl(height)
        }

        call.respond(result.toString())
    }

    get("/api/v2/ledger/snapshot/{height}") {
        val height = call.parameters["height"]?.toIntOrNull() ?: return@get call.respond(HttpStatusCode.BadRequest, "Invalid height")

        val result = LedgerDB.getSnapshot(height)

        if (result != null)
            call.respondJson(LedgerDB.Snapshot.serializer(), result)
        else
            call.respond(HttpStatusCode.BadRequest, "Snapshot not found")
    }
}
