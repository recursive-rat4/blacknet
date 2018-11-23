/*
 * Copyright (c) 2018 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.db

import ninja.blacknet.core.HTLC
import ninja.blacknet.core.UndoBlock
import ninja.blacknet.core.UndoHTLCList
import ninja.blacknet.core.UndoList
import org.mapdb.DataInput2
import org.mapdb.DataOutput2
import org.mapdb.Serializer

object UndoSerializer : Serializer<UndoBlock> {
    override fun hashCode(o: UndoBlock, seed: Int): Int {
        return o.hashCode() xor seed
    }

    override fun equals(first: UndoBlock?, second: UndoBlock?): Boolean {
        return first === second || first == second
    }

    override fun isTrusted(): Boolean {
        return true
    }

    override fun deserialize(input: DataInput2, available: Int): UndoBlock {
        val blockTime = input.unpackLong()
        val supply = input.unpackLong()
        val nxtrng = HashSerializer.deserialize(input, 0)
        val accountsSize = input.unpackInt()
        val accounts = UndoList(accountsSize)
        if (accountsSize > 0)
            for (i in 1..accountsSize) {
                val key = PublicKeySerializer.deserialize(input, 0)
                val state = AccountStateSerializer.deserialize(input, 0)
                accounts.add(Pair(key, state))
            }
        val htlcsSize = input.unpackInt()
        val htlcs = UndoHTLCList(htlcsSize)
        if (htlcsSize > 0)
            for (i in 1..htlcsSize) {
                val id = HashSerializer.deserialize(input, 0)
                var htlc: HTLC? = null
                if (input.readBoolean())
                    htlc = HTLCSerializer.deserialize(input, 0)
                htlcs.add(Pair(id, htlc))
            }
        return UndoBlock(blockTime, supply, nxtrng, accounts, htlcs)
    }

    override fun serialize(out: DataOutput2, value: UndoBlock) {
        out.packLong(value.blockTime)
        out.packLong(value.supply)
        HashSerializer.serialize(out, value.nxtrng)
        out.packInt(value.accounts.size)
        for (i in value.accounts) {
            PublicKeySerializer.serialize(out, i.first)
            AccountStateSerializer.serialize(out, i.second)
        }
        out.packInt(value.htlcs.size)
        for (i in value.htlcs) {
            HashSerializer.serialize(out, i.first)
            val htlc = i.second
            if (htlc != null) {
                out.writeBoolean(true)
                HTLCSerializer.serialize(out, htlc)
            } else {
                out.writeBoolean(false)
            }
        }
    }
}
