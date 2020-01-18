/*
 * Copyright (c) 2018-2019 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.core

import kotlinx.serialization.serializer
import ninja.blacknet.contract.HashTimeLock
import ninja.blacknet.crypto.PublicKey
import ninja.blacknet.serialization.SerializableByteArray

class HTLC(
        height: Int,
        time: Long,
        lot: Long,
        from: PublicKey,
        to: PublicKey,
        timeLockType: Byte,
        timeLock: Long,
        hashType: Byte,
        hashLock: SerializableByteArray
) : HashTimeLock<PublicKey, Long>(
        height,
        time,
        lot,
        from,
        to,
        timeLockType,
        timeLock,
        hashType,
        hashLock
) {
    fun serialize(): ByteArray {
        return serialize(PublicKey.serializer(), Long.serializer())
    }

    companion object {
        fun deserialize(bytes: ByteArray): HTLC {
            val obj = deserialize(bytes, PublicKey.serializer(), Long.serializer())
            return HTLC(obj.height, obj.time, obj.lot, obj.from, obj.to, obj.timeLockType, obj.timeLock, obj.hashType, obj.hashLock)
        }
    }
}
