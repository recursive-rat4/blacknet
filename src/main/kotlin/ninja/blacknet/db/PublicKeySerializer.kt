/*
 * Copyright (c) 2018 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.db

import ninja.blacknet.crypto.Ed25519
import org.mapdb.DataInput2
import org.mapdb.DataOutput2
import org.mapdb.Serializer

object PublicKeySerializer : Serializer<Ed25519.PublicKey> {
    override fun hashCode(o: Ed25519.PublicKey, seed: Int): Int {
        return o.bytes.contentHashCode() xor seed
    }

    override fun equals(first: Ed25519.PublicKey?, second: Ed25519.PublicKey?): Boolean {
        return first == second || (first != null && second != null && first.bytes.contentEquals(second.bytes))
    }

    override fun isTrusted(): Boolean {
        return true
    }

    override fun fixedSize(): Int {
        return Ed25519.PublicKey.SIZE
    }

    override fun serialize(out: DataOutput2, value: Ed25519.PublicKey) {
        out.write(value.bytes)
    }

    override fun deserialize(input: DataInput2, available: Int): Ed25519.PublicKey {
        val b = ByteArray(Ed25519.PublicKey.SIZE)
        input.readFully(b)
        return Ed25519.PublicKey(b)
    }
}