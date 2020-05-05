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
import ninja.blacknet.coding.HexFormatException
import ninja.blacknet.coding.fromHex
import ninja.blacknet.coding.toHex
import ninja.blacknet.crypto.SipHash.hashCode
import ninja.blacknet.serialization.ConfigInput
import ninja.blacknet.serialization.notSupportedDecoderError
import ninja.blacknet.serialization.notSupportedEncoderError

/**
 * Represents an Ed25519 private key.
 */
@Serializable
class PrivateKey(val bytes: ByteArray) {
    override fun equals(other: Any?): Boolean = (other is PrivateKey) && bytes.contentEquals(other.bytes)
    override fun hashCode(): Int = hashCode(serializer(), this)
    override fun toString(): String = bytes.toHex()

    fun toPublicKey(): PublicKey {
        return Ed25519.publicKey(this)
    }

    @Serializer(forClass = PrivateKey::class)
    companion object {
        /**
         * The number of bytes in a binary representation of a [PrivateKey].
         */
        const val SIZE_BYTES = 32

        fun parse(string: String): PrivateKey {
            return try {
                val bytes = fromHex(string, SIZE_BYTES)
                PrivateKey(bytes)
            } catch (e: HexFormatException) {
                try {
                    Mnemonic.fromString(string)
                } catch (e: Throwable) {
                    throw e
                }
            }
        }

        override fun deserialize(decoder: Decoder): PrivateKey {
            return when (decoder) {
                is ConfigInput -> PrivateKey.parse(decoder.decodeString())
                else -> throw notSupportedDecoderError(decoder, this)
            }
        }

        override fun serialize(encoder: Encoder, value: PrivateKey) {
            when (encoder) {
                is HashCoder -> encoder.encodePrivateKey(value)
                else -> throw notSupportedEncoderError(encoder, this)
            }
        }
    }
}

/**
 * Encodes a private key value.
 *
 * @param value the [PrivateKey] containing the data
 */
fun HashCoder.encodePrivateKey(value: PrivateKey) {
    writer.writeByteArray(value.bytes)
}
