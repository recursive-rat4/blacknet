/*
 * Copyright (c) 2018-2019 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.core

import kotlinx.serialization.Serializable
import ninja.blacknet.crypto.PoS
import ninja.blacknet.crypto.PublicKey
import ninja.blacknet.crypto.SipHash.hashCode
import ninja.blacknet.serialization.BinaryDecoder
import ninja.blacknet.serialization.BinaryEncoder
import ninja.blacknet.serialization.VarInt
import ninja.blacknet.serialization.VarLong
import ninja.blacknet.util.sumByLong

@Serializable
class AccountState(
        var seq: VarInt = VarInt.ZERO,
        var stake: VarLong = VarLong.ZERO,
        var immature: MutableList<Input> = ArrayList(),
        var leases: MutableList<Lease> = ArrayList()
) {
    override fun equals(other: Any?): Boolean {
        return (other is AccountState) && seq == other.seq && stake == other.stake && immature == other.immature && leases == other.leases
    }

    override fun hashCode(): Int {
        return hashCode(serializer(), this)
    }

    fun balance(): Long {
        return stake.long + immature.sumByLong { it.amount.long }
    }

    fun confirmedBalance(height: Int, confirmations: Int): Long {
        return stake.long + immature.sumByLong { it.confirmedBalance(height, confirmations) }
    }

    fun stakingBalance(height: Int): Long {
        return stake.long + immature.sumByLong { it.matureBalance(height) } + leases.sumByLong { it.matureBalance(height) }
    }

    fun totalBalance(): Long {
        return stake.long + immature.sumByLong { it.amount.long } + leases.sumByLong { it.amount.long }
    }

    fun credit(amount: Long): Status {
        if (amount < 0) {
            return Invalid("Negative amount")
        }

        if (amount <= stake.long) {
            stake = VarLong(stake.long - amount)
            return Accepted
        }

        if (balance() < amount) {
            return Invalid("Insufficient funds")
        }

        var r = amount - stake.long
        stake = VarLong.ZERO
        while (r > 0) {
            if (r < immature[0].amount.long) {
                immature[0].amount = VarLong(immature[0].amount.long - r)
                break
            } else {
                r -= immature[0].amount.long
                immature.removeAt(0)
            }
        }

        return Accepted
    }

    fun debit(height: Int, amount: Long) {
        if (amount != 0L)
            immature.add(Input(VarInt(height), VarLong(amount)))
    }

    fun prune(height: Int): Boolean {
        val mature = immature.sumByLong { it.matureBalance(height) }
        return if (mature == 0L) {
            false
        } else {
            stake = VarLong(stake.long + mature)
            immature = immature.asSequence().filter { !it.isMature(height) }.toMutableList()
            true
        }
    }

    @Serializable
    class Input(val height: VarInt, var amount: VarLong) {
        override fun equals(other: Any?): Boolean = (other is Input) && height == other.height && amount == other.amount
        override fun hashCode(): Int = hashCode(serializer(), this)
        fun copy(): Input = Input(height, amount)
        fun isConfirmed(height: Int, confirmations: Int): Boolean = height > this.height.int + confirmations
        fun isMature(height: Int): Boolean = height > this.height.int + PoS.MATURITY
        fun confirmedBalance(height: Int, confirmations: Int): Long = if (isConfirmed(height, confirmations)) amount.long else 0
        fun matureBalance(height: Int): Long = if (isMature(height)) amount.long else 0
    }

    @Serializable
    class Lease(val publicKey: PublicKey, val height: VarInt, var amount: VarLong) {
        override fun equals(other: Any?): Boolean = (other is Lease) && publicKey == other.publicKey && height == other.height && amount == other.amount
        override fun hashCode(): Int = hashCode(serializer(), this)
        fun copy(): Lease = Lease(publicKey, height, amount)
        fun isMature(height: Int): Boolean = height > this.height.int + PoS.MATURITY
        fun matureBalance(height: Int): Long = if (isMature(height)) amount.long else 0
    }

    fun copy(): AccountState {
        val copyImmature = ArrayList<Input>(immature.size)
        for (i in 0 until immature.size)
            copyImmature.add(immature[i].copy())
        val copyLeases = ArrayList<Lease>(leases.size)
        for (i in 0 until leases.size)
            copyLeases.add(leases[i].copy())
        return AccountState(seq, stake, copyImmature, copyLeases)
    }
}
