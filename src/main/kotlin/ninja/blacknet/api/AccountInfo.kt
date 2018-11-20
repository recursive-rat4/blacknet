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
import ninja.blacknet.crypto.PublicKey
import ninja.blacknet.db.LedgerDB

@Serializable
class AccountInfo(
        val seq: Int,
        val balance: Long,
        val stakingBalance: Long
) {
    companion object {
        suspend fun get(publicKey: PublicKey): AccountInfo? {
            val state = LedgerDB.get(publicKey) ?: return null
            return AccountInfo(state.seq, state.balance(), state.stakingBalance(LedgerDB.height()))
        }
    }
}
