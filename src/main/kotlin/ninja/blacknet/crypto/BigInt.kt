/*
 * Copyright (c) 2018-2019 Pavel Vasin
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
import kotlinx.serialization.json.JsonOutput
import ninja.blacknet.coding.fromHex
import ninja.blacknet.coding.toHex
import ninja.blacknet.crypto.SipHash.hashCode
import ninja.blacknet.serialization.BinaryDecoder
import ninja.blacknet.serialization.BinaryEncoder
import ninja.blacknet.serialization.notSupportedDecoderException
import ninja.blacknet.serialization.notSupportedEncoderException
import java.math.BigInteger

@Serializable
class BigInt(private val int: BigInteger) : Comparable<BigInt> {
    constructor(bytes: ByteArray) : this(BigInteger(bytes))
    constructor(n: Long) : this(BigInteger.valueOf(n))
    constructor(hash: Hash) : this(BigInteger(1, hash.bytes))

    override operator fun compareTo(other: BigInt): Int = int.compareTo(other.int)
    operator fun plus(other: BigInt): BigInt = BigInt(int.add(other.int))
    operator fun minus(other: BigInt): BigInt = BigInt(int.subtract(other.int))
    operator fun times(other: BigInt): BigInt = BigInt(int.multiply(other.int))
    operator fun div(other: BigInt): BigInt = BigInt(int.divide(other.int))
    operator fun rem(other: BigInt): BigInt = BigInt(int.remainder(other.int))
    operator fun unaryMinus(): BigInt = BigInt(int.negate())

    operator fun plus(long: Long): BigInt = this.plus(BigInt(long))
    operator fun minus(long: Long): BigInt = this.minus(BigInt(long))
    operator fun times(long: Long): BigInt = this.times(BigInt(long))
    operator fun div(long: Long): BigInt = this.div(BigInt(long))

    infix fun shl(n: Int): BigInt = BigInt(int.shiftLeft(n))
    infix fun shr(n: Int): BigInt = BigInt(int.shiftRight(n))

    override fun equals(other: Any?): Boolean = (other is BigInt) && int == other.int
    override fun hashCode(): Int = hashCode(serializer(), this)
    override fun toString(): String = int.toString()

    fun toByteArray(): ByteArray = int.toByteArray()
    fun toHex(): String = toByteArray().toHex()
    fun toLong(): Long = int.toLong()

    @Serializer(forClass = BigInt::class)
    companion object {
        val ZERO = BigInt(BigInteger.ZERO)
        val ONE = BigInt(BigInteger.ONE)

        fun fromString(hex: String?): BigInt? {
            if (hex == null) return null
            val bytes = fromHex(hex) ?: return null
            return BigInt(bytes)
        }

        override fun deserialize(decoder: Decoder): BigInt {
            return when (decoder) {
                is BinaryDecoder -> BigInt(decoder.decodeByteArray())
                else -> throw notSupportedDecoderException(decoder, this)
            }
        }

        override fun serialize(encoder: Encoder, value: BigInt) {
            when (encoder) {
                is BinaryEncoder -> encoder.encodeByteArray(value.toByteArray())
                is HashCoder -> encoder.encodeByteArray(value.toByteArray())
                is JsonOutput -> encoder.encodeString(value.int.toString())
                else -> throw notSupportedEncoderException(encoder, this)
            }
        }
    }
}
