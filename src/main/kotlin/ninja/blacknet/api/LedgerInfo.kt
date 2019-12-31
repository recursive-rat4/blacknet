/*
 * Copyright (c) 2018-2019 Pavel Vasin
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
        val rollingCheckpoint: String,
        val difficulty: String,
        val cumulativeDifficulty: String,
        val supply: String,
        val maxBlockSize: Int,
        val nxtrng: String
) {
    companion object {
        fun get(): LedgerInfo {
            val state = LedgerDB.state()
            return LedgerInfo(
                    state.height,
                    state.blockHash.toString(),
                    state.blockTime,
                    state.rollingCheckpoint.toString(),
                    state.difficulty.toString(),
                    state.cumulativeDifficulty.toString(),
                    state.supply.toString(),
                    state.maxBlockSize,
                    state.nxtrng.toString()
            )
        }
    }
}
