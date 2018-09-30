/*
 * Copyright (c) 2018 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.db

import ninja.blacknet.util.sumByLong

class AccountState(
        var seq: Int,
        var stake: Long,
        var immature: MutableList<Input>,
        var leases: MutableList<Input>
) {
    fun balance(): Long {
        return stake + immature.sumByLong { it.amount }
    }

    fun stakingBalance(height: Int): Long {
        return stake + immature.sumByLong { it.matureBalance(height) } + leases.sumByLong { it.matureBalance(height) }
    }

    class Input(val height: Int, val amount: Long) {
        fun matureBalance(height: Int): Long {
            return if (height > this.height + 500) amount else 0
        }
    }
}