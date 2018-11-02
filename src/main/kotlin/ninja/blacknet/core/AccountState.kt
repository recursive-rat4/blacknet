/*
 * Copyright (c) 2018 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.core

import mu.KotlinLogging
import ninja.blacknet.util.sumByLong

private val logger = KotlinLogging.logger {}

data class AccountState(
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

    fun credit(amount: Long): Boolean {
        if (amount < 0) {
            logger.info("negative amount")
            return false
        }

        if (amount <= stake) {
            stake -= amount
            return true
        }

        if (balance() < amount) {
            logger.info("insufficient funds")
            return false
        }
        return false //TODO
    }

    fun debit(height: Int, amount: Long) {
        immature.add(Input(height, amount))
    }

    fun prune(height: Int) {
        if (height < 0) return

        val mature = immature.sumByLong { it.matureBalance(height) }
        if (mature == 0L) return

        stake += mature
        immature = immature.asSequence().filter { !it.isMature(height) }.toMutableList()
    }

    class Input(val height: Int, val amount: Long) {
        fun isMature(height: Int): Boolean = height > this.height + PoS.MATURITY
        fun matureBalance(height: Int): Long = if (isMature(height)) amount else 0
    }

    companion object {
        fun create(stake: Long = 0): AccountState {
            return AccountState(0, stake, ArrayList(), ArrayList())
        }
    }
}