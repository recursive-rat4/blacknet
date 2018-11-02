/*
 * Copyright (c) 2018 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.db

import ninja.blacknet.crypto.PublicKey
import org.mapdb.DataInput2
import org.mapdb.DataOutput2
import org.mapdb.Serializer

object PublicKeySerializer : Serializer<PublicKey> {
    override fun hashCode(o: PublicKey, seed: Int): Int {
        return o.hashCode() xor seed
    }

    override fun equals(first: PublicKey?, second: PublicKey?): Boolean {
        return first === second || first == second
    }

    override fun isTrusted(): Boolean {
        return true
    }

    override fun fixedSize(): Int {
        return PublicKey.SIZE
    }

    override fun serialize(out: DataOutput2, value: PublicKey) {
        out.write(value.bytes.array)
    }

    override fun deserialize(input: DataInput2, available: Int): PublicKey {
        val b = ByteArray(PublicKey.SIZE)
        input.readFully(b)
        return PublicKey(b)
    }
}