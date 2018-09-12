/*
 * Copyright (c) 2018 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.db

import ninja.blacknet.crypto.Hash
import org.mapdb.DataInput2
import org.mapdb.DataOutput2
import org.mapdb.Serializer

object HashSerializer : Serializer<Hash> {
    override fun hashCode(o: Hash, seed: Int): Int {
        return o.bytes.contentHashCode() xor seed
    }

    override fun equals(first: Hash?, second: Hash?): Boolean {
        return first == second || (first != null && second != null && first.bytes.contentEquals(second.bytes))
    }

    override fun isTrusted(): Boolean {
        return true
    }

    override fun fixedSize(): Int {
        return Hash.SIZE
    }

    override fun serialize(out: DataOutput2, value: Hash) {
        out.write(value.bytes)
    }

    override fun deserialize(input: DataInput2, available: Int): Hash {
        val b = ByteArray(Hash.SIZE)
        input.readFully(b)
        return Hash(b)
    }
}