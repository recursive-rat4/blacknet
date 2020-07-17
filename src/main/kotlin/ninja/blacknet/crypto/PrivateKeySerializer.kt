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
import kotlinx.serialization.KSerializer
import kotlinx.serialization.SerialDescriptor
import kotlinx.serialization.StructureKind
import ninja.blacknet.coding.HexFormatException
import ninja.blacknet.coding.fromHex
import ninja.blacknet.coding.toHex
import ninja.blacknet.crypto.SipHash.hashCode
import ninja.blacknet.crypto.encodeByteArray
import ninja.blacknet.ktor.requests.RequestDecoder
import ninja.blacknet.serialization.ConfigInput
import ninja.blacknet.serialization.notSupportedDecoderError
import ninja.blacknet.serialization.notSupportedEncoderError

/**
 * The number of bytes in a binary representation of a private key.
 */
const val PRIVATE_KEY_SIZE_BYTES = 32

/**
 * Serializes an Ed25519 private key.
 */
object PrivateKeySerializer : KSerializer<ByteArray> {
    override val descriptor: SerialDescriptor = SerialDescriptor(
        "ninja.blacknet.crypto.PrivateKeySerializer",
        StructureKind.LIST  // PrimitiveKind.STRING
    )

    fun parse(string: String): ByteArray {
        return try {
            fromHex(string, PRIVATE_KEY_SIZE_BYTES)
        } catch (e: HexFormatException) {
            try {
                Mnemonic.fromString(string)
            } catch (e: Throwable) {
                throw e
            }
        }
    }

    override fun deserialize(decoder: Decoder): ByteArray {
        return when (decoder) {
            is ConfigInput -> parse(decoder.decodeString())
            is RequestDecoder -> parse(decoder.decodeString())
            else -> throw notSupportedDecoderError(decoder, this)
        }
    }

    override fun serialize(encoder: Encoder, value: ByteArray) {
        when (encoder) {
            is HashCoder -> encoder.encodeByteArray(value)
            else -> throw notSupportedEncoderError(encoder, this)
        }
    }
}
