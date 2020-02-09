/*
 * Copyright (c) 2018-2019 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.contract

import kotlinx.serialization.KSerializer
import kotlinx.serialization.Serializable
import ninja.blacknet.serialization.BinaryDecoder
import ninja.blacknet.serialization.BinaryEncoder
import ninja.blacknet.serialization.SerializableByteArray

@Serializable
open class HashTimeLock<K, L>(
        val height: Int,
        val time: Long,
        val lot: L,
        val from: K,
        val to: K,
        val timeLockType: Byte,
        val timeLock: Long,
        val hashType: Byte,
        val hashLock: SerializableByteArray
) {
    fun serialize(kSerializer: KSerializer<K>, lSerializer: KSerializer<L>): ByteArray {
        return BinaryEncoder.toBytes(serializer(kSerializer, lSerializer), this)
    }

    fun verifyTimeLock(processor: Processor): Boolean {
        val type = TimeLockType.get(timeLockType)
                ?: return false
        return type.verify(this, processor)
    }

    fun verifyHashLock(preimage: SerializableByteArray): Boolean {
        val type = HashType.get(hashType) ?: return false
        return type.verify(this, preimage)
    }

    enum class HashType(
            val hash: (ByteArray) -> ByteArray,
            val hashSize: Int
    ) {
        BLAKE256(ninja.blacknet.crypto.Blake2b, ninja.blacknet.crypto.Blake2b.DIGEST_SIZE_BYTES),
        SHA256(ninja.blacknet.crypto.SHA256, ninja.blacknet.crypto.SHA256.DIGEST_SIZE_BYTES),
        KECCAK256(ninja.blacknet.crypto.Keccak256, ninja.blacknet.crypto.Keccak256.DIGEST_SIZE_BYTES),
        RIPEMD160(ninja.blacknet.crypto.RIPEMD160, ninja.blacknet.crypto.RIPEMD160.DIGEST_SIZE_BYTES),
        ;

        fun verify(htlc: HashTimeLock<*, *>, preimage: SerializableByteArray): Boolean {
            return hash(preimage.array).contentEquals(htlc.hashLock.array)
        }

        companion object {
            fun get(type: Byte): HashType? = when (type) {
                BLAKE256.ordinal.toByte() -> BLAKE256
                SHA256.ordinal.toByte() -> SHA256
                KECCAK256.ordinal.toByte() -> KECCAK256
                RIPEMD160.ordinal.toByte() -> RIPEMD160
                else -> null
            }
        }
    }

    enum class TimeLockType(
            val verify: (HashTimeLock<*, *>, Processor) -> Boolean
    ) {
        TIME({ htlc, processor -> htlc.timeLock < processor.HashTimeLockGetBlockTime() }),
        HEIGHT({ htlc, processor -> htlc.timeLock < processor.HashTimeLockGetHeight() }),
        RELATIVE_TIME({ htlc, processor -> htlc.time + htlc.timeLock < processor.HashTimeLockGetBlockTime() }),
        RELATIVE_HEIGHT({ htlc, processor -> htlc.height + htlc.timeLock < processor.HashTimeLockGetHeight() }),
        ;

        companion object {
            fun get(type: Byte): TimeLockType? = when (type) {
                TIME.ordinal.toByte() -> TIME
                HEIGHT.ordinal.toByte() -> HEIGHT
                RELATIVE_TIME.ordinal.toByte() -> RELATIVE_TIME
                RELATIVE_HEIGHT.ordinal.toByte() -> RELATIVE_HEIGHT
                else -> null
            }
        }
    }

    interface Processor {
        fun HashTimeLockGetBlockTime(): Long
        fun HashTimeLockGetHeight(): Int
        //TODO 骂人用语
    }

    companion object {
        fun <K, L> deserialize(bytes: ByteArray, kSerializer: KSerializer<K>, lSerializer: KSerializer<L>): HashTimeLock<K, L> {
            return BinaryDecoder(bytes).decode(serializer(kSerializer, lSerializer))
        }

        fun isValidTimeLockType(type: Byte): Boolean = TimeLockType.get(type) != null
        fun isValidHashType(type: Byte): Boolean = HashType.get(type) != null

        fun isValidHashLock(hashType: Byte, hashLock: SerializableByteArray): Boolean {
            val type = HashType.get(hashType)
            return if (type != null)
                hashLock.array.size == type.hashSize
            else
                false
        }
    }
}
