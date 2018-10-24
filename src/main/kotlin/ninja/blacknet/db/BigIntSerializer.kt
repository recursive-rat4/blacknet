/*
 * Copyright (c) 2018 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.db

import ninja.blacknet.crypto.BigInt
import org.mapdb.DataInput2
import org.mapdb.DataOutput2
import org.mapdb.Serializer

object BigIntSerializer : Serializer<BigInt> {
    override fun hashCode(o: BigInt, seed: Int): Int {
        return o.hashCode() xor seed
    }

    override fun equals(first: BigInt?, second: BigInt?): Boolean {
        return first === second || first == second
    }

    override fun isTrusted(): Boolean {
        return true
    }

    override fun serialize(out: DataOutput2, value: BigInt) {
        val bytes = value.toByteArray()
        out.packInt(bytes.size)
        out.write(bytes)
    }

    override fun deserialize(input: DataInput2, available: Int): BigInt {
        val size = input.unpackInt()
        val bytes = ByteArray(size)
        input.readFully(bytes)
        return BigInt(bytes)
    }
}
