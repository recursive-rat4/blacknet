/*
 * Copyright (c) 2018-2024 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.rpc.v1

import kotlin.concurrent.withLock
import kotlinx.serialization.Serializable
import ninja.blacknet.Kernel
import ninja.blacknet.crypto.PublicKey
import ninja.blacknet.db.CoinDB

@Serializable
class AccountInfoV1(
        val seq: Int,
        val balance: Long,
        val confirmedBalance: Long,
        val stakingBalance: Long
) {
    companion object {
        fun get(publicKey: PublicKey, confirmations: Int): AccountInfoV1? = Kernel.blockDB().reentrant.readLock().withLock {
            val account = CoinDB.get(publicKey) ?: return null
            val state = CoinDB.state()
            return AccountInfoV1(
                    account.seq,
                    account.balance(),
                    account.confirmedBalance(state.height, confirmations),
                    account.stakingBalance(state.height))
        }
    }
}
