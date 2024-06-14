/*
 * Copyright (c) 2018-2024 Pavel Vasin
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
import ninja.blacknet.core.AccountState
import ninja.blacknet.crypto.PublicKey
import ninja.blacknet.db.CoinDB

@Serializable
class AccountInfo(
        val seq: Int,
        val balance: String,
        val confirmedBalance: String,
        val stakingBalance: String,
        val inLeases: List<AccountState.Lease>,
) {
    companion object {
        fun get(publicKey: PublicKey, confirmations: Int): AccountInfo? = Kernel.blockDB().reentrant.readLock().withLock {
            val account = CoinDB.get(publicKey) ?: return null
            val state = CoinDB.state()
            return AccountInfo(
                    account.seq,
                    account.balance().toString(),
                    account.confirmedBalance(state.height, confirmations).toString(),
                    account.stakingBalance(state.height).toString(),
                    account.leases,
            )
        }
    }
}
