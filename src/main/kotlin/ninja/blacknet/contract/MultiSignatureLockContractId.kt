/*
 * Copyright (c) 2018-2019 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.contract

import kotlinx.serialization.Decoder
import kotlinx.serialization.Encoder
import kotlinx.serialization.Serializable
import kotlinx.serialization.Serializer
import kotlinx.serialization.json.JsonInput
import kotlinx.serialization.json.JsonOutput
import ninja.blacknet.crypto.Address
import ninja.blacknet.crypto.Hash
import ninja.blacknet.crypto.HashCoder
import ninja.blacknet.crypto.SipHash.hashCode
import ninja.blacknet.ktor.requests.RequestDecoder
import ninja.blacknet.serialization.BinaryDecoder
import ninja.blacknet.serialization.BinaryEncoder
import ninja.blacknet.serialization.notSupportedDecoderError
import ninja.blacknet.serialization.notSupportedEncoderError

/**
 * Represents an id of the multisignature lock contract.
 */
@Serializable
class MultiSignatureLockContractId(val hash: Hash) {
    constructor(bytes: ByteArray) : this(Hash(bytes))

    override fun equals(other: Any?): Boolean = (other is MultiSignatureLockContractId) && hash == other.hash
    override fun hashCode(): Int = hashCode(serializer(), this)
    override fun toString(): String = Address.encode(Address.MULTISIG, hash.bytes)

    @Serializer(forClass = MultiSignatureLockContractId::class)
    companion object {
        /**
         * The number of bytes in a binary representation of a [MultiSignatureLockContractId].
         */
        const val SIZE_BYTES = Hash.SIZE_BYTES

        fun parse(string: String): MultiSignatureLockContractId {
            val bytes = Address.decode(Address.MULTISIG, string)
            return MultiSignatureLockContractId(bytes)
        }

        override fun deserialize(decoder: Decoder): MultiSignatureLockContractId {
            return when (decoder) {
                is BinaryDecoder -> MultiSignatureLockContractId(decoder.decodeFixedByteArray(SIZE_BYTES))
                is RequestDecoder -> MultiSignatureLockContractId.parse(decoder.decodeString())
                is JsonInput -> MultiSignatureLockContractId.parse(decoder.decodeString())
                else -> throw notSupportedDecoderError(decoder, this)
            }
        }

        override fun serialize(encoder: Encoder, value: MultiSignatureLockContractId) {
            when (encoder) {
                is BinaryEncoder -> encoder.encodeFixedByteArray(value.hash.bytes)
                is HashCoder -> encoder.encodeMultiSignatureLockContractId(value)
                is JsonOutput -> encoder.encodeString(value.toString())
                else -> throw notSupportedEncoderError(encoder, this)
            }
        }
    }
}

/**
 * Encodes a multisignature lock contract id value.
 *
 * @param value the [MultiSignatureLockContractId] containing the data
 */
fun HashCoder.encodeMultiSignatureLockContractId(value: MultiSignatureLockContractId) {
    writer.writeByteArray(value.hash.bytes)
}
