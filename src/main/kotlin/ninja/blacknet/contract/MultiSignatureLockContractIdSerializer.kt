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
import kotlinx.serialization.KSerializer
import kotlinx.serialization.SerialDescriptor
import kotlinx.serialization.StructureKind
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
 * The number of bytes in a binary representation of a multisignature lock contract id.
 */
const val MULTI_SIGNATURE_LOCK_CONTRACT_ID_SIZE_BYTES = Hash.SIZE_BYTES

/**
 * Serializes an id of the multisignature lock contract.
 */
object MultiSignatureLockContractIdSerializer : KSerializer<ByteArray> {
    override val descriptor: SerialDescriptor = SerialDescriptor(
        "ninja.blacknet.contract.MultiSignatureLockContractIdSerializer",
        StructureKind.LIST  // PrimitiveKind.STRING
    )

    fun parse(string: String): ByteArray {
        return Address.decode(Address.MULTISIG, string)
    }

    fun stringify(id: ByteArray): String {
        return Address.encode(Address.MULTISIG, id)
    }

    override fun deserialize(decoder: Decoder): ByteArray {
        return when (decoder) {
            is BinaryDecoder -> decoder.decodeFixedByteArray(MULTI_SIGNATURE_LOCK_CONTRACT_ID_SIZE_BYTES)
            is RequestDecoder,
            is JsonInput -> parse(decoder.decodeString())
            else -> throw notSupportedDecoderError(decoder, this)
        }
    }

    override fun serialize(encoder: Encoder, value: ByteArray) {
        when (encoder) {
            is BinaryEncoder -> encoder.encodeFixedByteArray(value)
            is HashCoder -> encoder.encodeMultiSignatureLockContractId(value)
            is JsonOutput -> encoder.encodeString(stringify(value))
            else -> throw notSupportedEncoderError(encoder, this)
        }
    }
}

/**
 * Encodes a multisignature lock contract id value.
 *
 * @param value the [ByteArray] containing the data
 */
fun HashCoder.encodeMultiSignatureLockContractId(value: ByteArray) {
    writer.writeByteArray(value)
}
