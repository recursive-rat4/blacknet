/*
 * Copyright (c) 2018-2019 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.serialization

import kotlinx.serialization.Decoder
import kotlinx.serialization.Encoder
import kotlinx.serialization.KSerializer
import kotlinx.serialization.PrimitiveDescriptor
import kotlinx.serialization.PrimitiveKind
import kotlinx.serialization.SerialDescriptor
import kotlinx.serialization.json.JsonInput
import kotlinx.serialization.json.JsonOutput
import ninja.blacknet.crypto.HashCoder
import ninja.blacknet.ktor.requests.RequestDecoder

/**
 * Serializes a [Long] with a transformation to a string in a json representation.
 */
object LongSerializer : KSerializer<Long> {
    /**
     * The number of bytes in a binary representation of a [Long].
     */
    const val SIZE_BYTES = Long.SIZE_BYTES

    override val descriptor: SerialDescriptor = PrimitiveDescriptor(
        "ninja.blacknet.serialization.LongSerializer",
        PrimitiveKind.LONG  // PrimitiveKind.STRING
    )

    override fun deserialize(decoder: Decoder): Long {
        return when (decoder) {
            is BinaryDecoder,
            is RequestDecoder,
            is RequestDecoder /* XXX 1.4 */
                -> decoder.decodeLong()
            is JsonInput,
            is JsonInput /* XXX 1.4 */
                -> decoder.decodeString().toLong()
            else
                -> throw notSupportedDecoderError(decoder, this)
        }
    }

    override fun serialize(encoder: Encoder, value: Long) {
        when (encoder) {
            is BinaryEncoder,
            is HashCoder,
            is HashCoder /* XXX 1.4 */
                -> encoder.encodeLong(value)
            is JsonOutput,
            is JsonOutput /* XXX 1.4 */
                -> encoder.encodeString(value.toString())
            else
                -> throw notSupportedEncoderError(encoder, this)
        }
    }
}
