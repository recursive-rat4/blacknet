/*
 * Copyright (c) 2018 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.crypto

import kotlinx.serialization.Decoder
import kotlinx.serialization.Encoder
import kotlinx.serialization.Serializable
import kotlinx.serialization.Serializer
import ninja.blacknet.serialization.BlacknetDecoder
import ninja.blacknet.serialization.BlacknetEncoder
import ninja.blacknet.serialization.SerializableByteArray
import ninja.blacknet.util.fromHex
import java.math.BigInteger

@Serializable
class BigInt(private val int: BigInteger) {
    constructor(bytes: ByteArray) : this(BigInteger(bytes))
    constructor(bytes: SerializableByteArray) : this(bytes.array)

    operator fun compareTo(other: BigInt): Int = int.compareTo(other.int)
    operator fun plus(other: BigInt): BigInt = BigInt(int.add(other.int))
    operator fun minus(other: BigInt): BigInt = BigInt(int.subtract(other.int))
    operator fun times(other: BigInt): BigInt = BigInt(int.multiply(other.int))
    operator fun div(other: BigInt): BigInt = BigInt(int.divide(other.int))
    operator fun rem(other: BigInt): BigInt = BigInt(int.remainder(other.int))
    operator fun unaryMinus(): BigInt = BigInt(int.negate())

    override fun equals(other: Any?): Boolean = (other is BigInt) && int == other.int
    override fun hashCode(): Int = int.hashCode()
    override fun toString(): String = int.toString()

    fun toByteArray(): ByteArray = int.toByteArray()

    @Serializer(forClass = BigInt::class)
    companion object {
        val ZERO = BigInt(BigInteger.ZERO)

        fun fromString(hex: String?): BigInt? {
            if (hex == null) return null
            val bytes = fromHex(hex) ?: return null
            return BigInt(bytes)
        }

        override fun deserialize(input: Decoder): BigInt {
            return BigInt((input as BlacknetDecoder).decodeSerializableByteArrayValue())
        }

        override fun serialize(output: Encoder, obj: BigInt) {
            val bytes = SerializableByteArray(obj.toByteArray())
            (output as BlacknetEncoder).encodeSerializableByteArrayValue(bytes)
        }
    }
}
