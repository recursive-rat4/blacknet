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
import ninja.blacknet.crypto.PublicKey
import ninja.blacknet.util.sumByLong

private val logger = KotlinLogging.logger {}

data class AccountState(
        var seq: Int,
        var stake: Long,
        var immature: MutableList<Input>,
        var leases: MutableList<LeaseInput>
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

        var r = amount - stake
        stake = 0
        while (r > 0) {
            if (r < immature[0].amount) {
                immature[0].amount -= r
                break
            } else {
                r -= immature[0].amount
                immature.removeAt(0)
            }
        }

        return true
    }

    fun debit(height: Int, amount: Long) {
        if (amount != 0L)
            immature.add(Input(height, amount))
    }

    fun prune(height: Int) {
        if (height < 0) return

        val mature = immature.sumByLong { it.matureBalance(height) }
        if (mature == 0L) return

        stake += mature
        immature = immature.asSequence().filter { !it.isMature(height) }.toMutableList()
    }

    data class Input(val height: Int, var amount: Long) {
        fun isMature(height: Int): Boolean = height > this.height + PoS.MATURITY
        fun matureBalance(height: Int): Long = if (isMature(height)) amount else 0
    }

    data class LeaseInput(val from: PublicKey, val height: Int, val amount: Long) {
        fun isMature(height: Int): Boolean = height > this.height + PoS.MATURITY
        fun matureBalance(height: Int): Long = if (isMature(height)) amount else 0
    }

    fun copy() = AccountState(seq, stake, immature.toMutableList(), leases.toMutableList())

    fun isEmpty() = seq == 0 && stake == 0L && immature.isEmpty() && leases.isEmpty()

    companion object {
        fun create(stake: Long = 0): AccountState {
            return AccountState(0, stake, ArrayList(), ArrayList())
        }
    }
}