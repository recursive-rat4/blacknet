/*
 * Copyright (c) 2018-2019 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.core

import kotlinx.serialization.builtins.serializer
import ninja.blacknet.contract.HashTimeLock
import ninja.blacknet.crypto.PublicKey
import ninja.blacknet.serialization.SerializableByteArray

private annotation class Serializable

@Serializable
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
            val value = deserialize(bytes, PublicKey.serializer(), Long.serializer())
            return HTLC(value.height, value.time, value.lot, value.from, value.to, value.timeLockType, value.timeLock, value.hashType, value.hashLock)
        }
    }
}
