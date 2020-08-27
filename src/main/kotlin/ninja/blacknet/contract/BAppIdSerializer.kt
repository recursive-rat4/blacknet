/*
 * Copyright (c) 2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.contract

import kotlinx.serialization.KSerializer
import kotlinx.serialization.builtins.serializer
import kotlinx.serialization.descriptors.SerialDescriptor
import kotlinx.serialization.encoding.Decoder
import kotlinx.serialization.encoding.Encoder
import kotlinx.serialization.json.JsonDecoder
import kotlinx.serialization.json.JsonEncoder
import ninja.blacknet.crypto.Address
import ninja.blacknet.crypto.HashCoder
import ninja.blacknet.crypto.encodeByteArray
import ninja.blacknet.rpc.requests.RequestDecoder
import ninja.blacknet.serialization.BinaryDecoder
import ninja.blacknet.serialization.BinaryEncoder
import ninja.blacknet.serialization.notSupportedFormatError
import ninja.blacknet.serialization.descriptor.ListSerialDescriptor

/**
 * Serializes an id of a Blacknet blockchain application.
 */
object BAppIdSerializer : KSerializer<ByteArray> {
    /**
     * The number of bytes in a binary representation of the Blacknet blockchain application id.
     */
    const val SIZE_BYTES = 4

    override val descriptor: SerialDescriptor = ListSerialDescriptor(
            "ninja.blacknet.contract.BAppIdSerializer",
            Byte.serializer().descriptor  // PrimitiveKind.STRING
    )

    fun decode(string: String): ByteArray {
        return Address.decode(Address.BAPP, string)
    }

    fun encode(id: ByteArray): String {
        return Address.encode(Address.BAPP, id)
    }

    override fun deserialize(decoder: Decoder): ByteArray {
        return when (decoder) {
            is BinaryDecoder -> decoder.decodeFixedByteArray(SIZE_BYTES)
            is RequestDecoder,
            is JsonDecoder -> decode(decoder.decodeString())
            else -> throw notSupportedFormatError(decoder, this)
        }
    }

    override fun serialize(encoder: Encoder, value: ByteArray) {
        when (encoder) {
            is BinaryEncoder -> encoder.encodeFixedByteArray(value)
            is HashCoder -> encoder.encodeByteArray(value)
            is JsonEncoder -> encoder.encodeString(encode(value))
            else -> throw notSupportedFormatError(encoder, this)
        }
    }
}
