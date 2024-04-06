/*
 * Copyright (c) 2019-2024 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.rpc.v2

import kotlin.concurrent.withLock
import kotlinx.serialization.Serializable
import ninja.blacknet.Kernel
import ninja.blacknet.crypto.Hash
import ninja.blacknet.db.CoinDB
import ninja.blacknet.serialization.LongSerializer

@Serializable
class SupplyInfo(
        val height: Int,
        val blockHash: Hash,
        val blockTime: Long,
        @Serializable(with = LongSerializer::class)
        val supply: Long
) {
    companion object {
        fun get(): SupplyInfo = Kernel.blockDB().reentrant.readLock().withLock {
            val state = CoinDB.state()
            return SupplyInfo(
                    state.height,
                    state.blockHash,
                    state.blockTime,
                    state.supply
            )
        }
    }
}
