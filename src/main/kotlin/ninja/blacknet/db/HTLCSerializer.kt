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
import ninja.blacknet.serialization.SerializableByteArray
import org.mapdb.DataInput2
import org.mapdb.DataOutput2
import org.mapdb.Serializer

object HTLCSerializer : Serializer<HTLC> {
    override fun hashCode(o: HTLC, seed: Int): Int {
        return o.hashCode() xor seed
    }

    override fun equals(first: HTLC?, second: HTLC?): Boolean {
        return first === second || first == second
    }

    override fun isTrusted(): Boolean {
        return true
    }

    override fun deserialize(input: DataInput2, available: Int): HTLC {
        val height = input.unpackInt()
        val time = input.unpackLong()
        val amount = input.unpackLong()
        val from = PublicKeySerializer.deserialize(input, 0)
        val to = PublicKeySerializer.deserialize(input, 0)
        val timeLockType = input.readByte()
        val timeLock = input.unpackLong()
        val hashType = input.readByte()
        val hashSize = input.unpackInt()
        val hashLock = ByteArray(hashSize)
        input.readFully(hashLock)
        return HTLC(height, time, amount, from, to, timeLockType, timeLock, hashType, SerializableByteArray(hashLock))
    }

    override fun serialize(out: DataOutput2, value: HTLC) {
        out.packInt(value.height)
        out.packLong(value.time)
        out.packLong(value.amount)
        PublicKeySerializer.serialize(out, value.from)
        PublicKeySerializer.serialize(out, value.to)
        out.writeByte(value.timeLockType)
        out.packLong(value.timeLock)
        out.writeByte(value.hashType)
        out.packInt(value.hashLock.array.size)
        out.write(value.hashLock.array)
    }
}

private fun DataOutput2.writeByte(byte: Byte) {
    writeByte(byte.toInt())
}
