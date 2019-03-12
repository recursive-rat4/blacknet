/*
 * Copyright (c) 2018 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.db

import ninja.blacknet.core.AccountState
import org.mapdb.DataInput2
import org.mapdb.DataOutput2
import org.mapdb.Serializer

object AccountStateSerializer : Serializer<AccountState> {
    override fun hashCode(o: AccountState, seed: Int): Int {
        return o.hashCode() xor seed
    }

    override fun equals(first: AccountState?, second: AccountState?): Boolean {
        return first === second || first == second
    }

    override fun isTrusted(): Boolean {
        return true
    }

    override fun serialize(out: DataOutput2, state: AccountState) {
        out.packInt(state.seq)
        out.packLong(state.stake)
        out.packInt(state.immature.size)
        for (i in state.immature) {
            out.packInt(i.height)
            out.packLong(i.amount)
        }
        out.packInt(state.leases.size)
        for (i in state.leases) {
            PublicKeySerializer.serialize(out, i.from)
            out.packInt(i.height)
            out.packLong(i.amount)
        }
    }

    override fun deserialize(input: DataInput2, available: Int): AccountState {
        val seq = input.unpackInt()
        val stake = input.unpackLong()
        val immatureSize = input.unpackInt()
        val immature = ArrayList<AccountState.Input>(immatureSize)
        if (immatureSize > 0)
            for (i in 1..immatureSize)
                immature.add(AccountState.Input(input.unpackInt(), input.unpackLong()))
        val leasesSize = input.unpackInt()
        val leases = ArrayList<AccountState.LeaseInput>(leasesSize)
        if (leasesSize > 0)
            for (i in 1..leasesSize)
                leases.add(AccountState.LeaseInput(PublicKeySerializer.deserialize(input, 0), input.unpackInt(), input.unpackLong()))
        return AccountState(seq, stake, immature, leases)
    }
}