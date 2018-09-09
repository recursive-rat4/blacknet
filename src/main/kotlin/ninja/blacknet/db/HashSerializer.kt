/*
 * Copyright (c) 2018 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.db

import ninja.blacknet.crypto.Blake2b
import org.mapdb.DataInput2
import org.mapdb.DataOutput2
import org.mapdb.Serializer

object HashSerializer : Serializer<Blake2b.Hash> {
    override fun hashCode(o: Blake2b.Hash, seed: Int): Int {
        return o.bytes.contentHashCode() xor seed
    }

    override fun equals(first: Blake2b.Hash?, second: Blake2b.Hash?): Boolean {
        return first == second || (first != null && second != null && first.bytes.contentEquals(second.bytes))
    }

    override fun isTrusted(): Boolean {
        return true
    }

    override fun fixedSize(): Int {
        return Blake2b.Hash.SIZE
    }

    override fun serialize(out: DataOutput2, value: Blake2b.Hash) {
        out.write(value.bytes)
    }

    override fun deserialize(input: DataInput2, available: Int): Blake2b.Hash {
        val b = ByteArray(Blake2b.Hash.SIZE)
        input.readFully(b)
        return Blake2b.Hash(b)
    }
}