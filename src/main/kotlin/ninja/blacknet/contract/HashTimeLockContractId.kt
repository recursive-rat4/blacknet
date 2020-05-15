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
 * Represents an id of the hash time lock contract.
 */
@Serializable
class HashTimeLockContractId(val hash: Hash) {
    constructor(bytes: ByteArray) : this(Hash(bytes))

    override fun equals(other: Any?): Boolean = (other is HashTimeLockContractId) && hash == other.hash
    override fun hashCode(): Int = hashCode(serializer(), this)
    override fun toString(): String = Address.encode(Address.HTLC, hash.bytes)

    @Serializer(forClass = HashTimeLockContractId::class)
    companion object {
        /**
         * The number of bytes in a binary representation of a [HashTimeLockContractId].
         */
        const val SIZE_BYTES = Hash.SIZE_BYTES

        fun parse(string: String): HashTimeLockContractId {
            val bytes = Address.decode(Address.HTLC, string)
            return HashTimeLockContractId(bytes)
        }

        override fun deserialize(decoder: Decoder): HashTimeLockContractId {
            return when (decoder) {
                is BinaryDecoder -> HashTimeLockContractId(decoder.decodeFixedByteArray(SIZE_BYTES))
                is RequestDecoder -> HashTimeLockContractId.parse(decoder.decodeString())
                is JsonInput -> HashTimeLockContractId.parse(decoder.decodeString())
                else -> throw notSupportedDecoderError(decoder, this)
            }
        }

        override fun serialize(encoder: Encoder, value: HashTimeLockContractId) {
            when (encoder) {
                is BinaryEncoder -> encoder.encodeFixedByteArray(value.hash.bytes)
                is HashCoder -> encoder.encodeHashTimeLockContractId(value)
                is JsonOutput -> encoder.encodeString(value.toString())
                else -> throw notSupportedEncoderError(encoder, this)
            }
        }
    }
}

/**
 * Encodes a hash time lock contract id value.
 *
 * @param value the [HashTimeLockContractId] containing the data
 */
fun HashCoder.encodeHashTimeLockContractId(value: HashTimeLockContractId) {
    writer.writeByteArray(value.hash.bytes)
}
