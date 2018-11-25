/*
 * Copyright (c) 2018 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.db

import ninja.blacknet.core.Multisig
import ninja.blacknet.crypto.PublicKey
import org.mapdb.DataInput2
import org.mapdb.DataOutput2
import org.mapdb.Serializer

object MultisigSerializer : Serializer<Multisig> {
    override fun hashCode(o: Multisig, seed: Int): Int {
        return o.hashCode() xor seed
    }

    override fun equals(first: Multisig?, second: Multisig?): Boolean {
        return first === second || first == second
    }

    override fun isTrusted(): Boolean {
        return true
    }

    override fun deserialize(input: DataInput2, available: Int): Multisig {
        val amount = input.unpackLong()
        val n = input.readByte()
        val keysSize = input.unpackInt()
        val keys = ArrayList<PublicKey>(keysSize)
        if (keysSize > 0)
            for (i in 1..keysSize) {
                val key = PublicKeySerializer.deserialize(input, 0)
                keys.add(key)
            }
        return Multisig(amount, n, keys)
    }

    override fun serialize(out: DataOutput2, value: Multisig) {
        out.packLong(value.amount)
        out.writeByte(value.n)
        out.packInt(value.keys.size)
        for (key in value.keys)
            PublicKeySerializer.serialize(out, key)
    }
}

private fun DataOutput2.writeByte(byte: Byte) {
    writeByte(byte.toInt())
}
