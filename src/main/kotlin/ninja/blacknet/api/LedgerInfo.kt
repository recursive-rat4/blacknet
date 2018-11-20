/*
 * Copyright (c) 2018 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.api

import kotlinx.serialization.Serializable
import ninja.blacknet.db.LedgerDB

@Serializable
class LedgerInfo(
        val height: Int,
        val blockHash: String,
        val blockTime: Long,
        val cumulativeDifficulty: String,
        val supply: Long,
        val accounts: Int,
        val maxBlockSize: Int,
        val nxtrng: String
) {
    companion object {
        fun get() = LedgerInfo(
                LedgerDB.height(),
                LedgerDB.blockHash().toString(),
                LedgerDB.blockTime(),
                LedgerDB.cumulativeDifficulty().toString(),
                LedgerDB.supply(),
                LedgerDB.accounts(),
                LedgerDB.maxBlockSize(),
                LedgerDB.nxtrng().toString())
    }
}
