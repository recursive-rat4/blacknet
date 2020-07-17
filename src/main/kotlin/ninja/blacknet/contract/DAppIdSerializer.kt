/*
 * Copyright (c) 2020 Pavel Vasin
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
import ninja.blacknet.crypto.HashCoder
import ninja.blacknet.crypto.SipHash.hashCode
import ninja.blacknet.crypto.encodeByteArray
import ninja.blacknet.ktor.requests.RequestDecoder
import ninja.blacknet.serialization.BinaryDecoder
import ninja.blacknet.serialization.BinaryEncoder
import ninja.blacknet.serialization.notSupportedDecoderError
import ninja.blacknet.serialization.notSupportedEncoderError

/**
 * The number of bytes in a binary representation of an id of a dapp.
 */
const val DAPP_ID_SIZE_BYTES = 4

/**
 * Serializes an id of a dapp.
 */
object DAppIdSerializer : KSerializer<ByteArray> {
    override val descriptor: SerialDescriptor = SerialDescriptor(
        "ninja.blacknet.contract.DAppIdSerializer",
        StructureKind.LIST  // PrimitiveKind.STRING
    )

    fun parse(string: String): ByteArray {
        return Address.decode(Address.DAPP, string)
    }

    fun stringify(id: ByteArray): String {
        return Address.encode(Address.DAPP, id)
    }

    override fun deserialize(decoder: Decoder): ByteArray {
        return when (decoder) {
            is BinaryDecoder -> decoder.decodeFixedByteArray(DAPP_ID_SIZE_BYTES)
            is RequestDecoder,
            is JsonInput -> parse(decoder.decodeString())
            else -> throw notSupportedDecoderError(decoder, this)
        }
    }

    override fun serialize(encoder: Encoder, value: ByteArray) {
        when (encoder) {
            is BinaryEncoder -> encoder.encodeFixedByteArray(value)
            is HashCoder -> encoder.encodeByteArray(value)
            is JsonOutput -> encoder.encodeString(stringify(value))
            else -> throw notSupportedEncoderError(encoder, this)
        }
    }
}
