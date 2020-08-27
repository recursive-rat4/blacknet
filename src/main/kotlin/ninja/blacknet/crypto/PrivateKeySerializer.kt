/*
 * Copyright (c) 2018-2019 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.crypto

import kotlinx.serialization.KSerializer
import kotlinx.serialization.builtins.serializer
import kotlinx.serialization.descriptors.SerialDescriptor
import kotlinx.serialization.encoding.Decoder
import kotlinx.serialization.encoding.Encoder
import ninja.blacknet.codec.base.Base16
import ninja.blacknet.codec.base.HexCodecException
import ninja.blacknet.rpc.requests.RequestDecoder
import ninja.blacknet.serialization.ConfigDecoder
import ninja.blacknet.serialization.notSupportedFormatError
import ninja.blacknet.serialization.descriptor.ListSerialDescriptor

/**
 * Serializes an Ed25519 private key.
 */
object PrivateKeySerializer : KSerializer<ByteArray> {
    /**
     * The number of bytes in a binary representation of the private key.
     */
    const val SIZE_BYTES = 32

    override val descriptor: SerialDescriptor = ListSerialDescriptor(
            "ninja.blacknet.crypto.PrivateKeySerializer",
            Byte.serializer().descriptor  // PrimitiveKind.STRING
    )

    fun decode(string: String): ByteArray {
        return try {
            decodeHex(string, SIZE_BYTES * 2)
        } catch (e: HexCodecException) {
            Mnemonic.fromString(string)
        }
    }

    override fun deserialize(decoder: Decoder): ByteArray {
        return when (decoder) {
            is ConfigDecoder,
            is RequestDecoder -> decode(decoder.decodeString())
            else -> throw notSupportedFormatError(decoder, this)
        }
    }

    override fun serialize(encoder: Encoder, value: ByteArray) {
        when (encoder) {
            is HashCoder -> encoder.encodeByteArray(value)
            else -> throw notSupportedFormatError(encoder, this)
        }
    }
}
