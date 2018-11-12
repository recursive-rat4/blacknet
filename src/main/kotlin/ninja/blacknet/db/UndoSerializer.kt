/*
 * Copyright (c) 2018 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.db

import ninja.blacknet.core.UndoBlock
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
        val supply = input.unpackLong()
        val size = input.unpackInt()
        val accounts = UndoList(size)
        if (size > 0)
            for (i in 1..size) {
                val key = PublicKeySerializer.deserialize(input, 0)
                val state = AccountStateSerializer.deserialize(input, 0)
                accounts.add(Pair(key, state))
            }
        return UndoBlock(supply, accounts)
    }

    override fun serialize(out: DataOutput2, value: UndoBlock) {
        out.packLong(value.supply)
        out.packInt(value.accounts.size)
        for (i in value.accounts) {
            PublicKeySerializer.serialize(out, i.first)
            AccountStateSerializer.serialize(out, i.second)
        }
    }
}
