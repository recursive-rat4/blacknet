/*
 * Copyright (c) 2018 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.crypto

import kotlinx.serialization.*
import kotlinx.serialization.json.JsonInput
import kotlinx.serialization.json.JsonOutput
import ninja.blacknet.crypto.Ed25519.x25519
import ninja.blacknet.serialization.BlacknetDecoder
import ninja.blacknet.serialization.BlacknetEncoder
import ninja.blacknet.serialization.SerializableByteArray
import ninja.blacknet.util.fromHex
import ninja.blacknet.util.toHex

@Serializable
class Message(
        val type: Byte,
        val message: SerializableByteArray
) {
    constructor(type: Byte, bytes: ByteArray) : this(type, SerializableByteArray(bytes))

    fun isEmpty(): Boolean {
        return type == PLAIN && message.array.isEmpty()
    }

    fun decrypt(privateKey: PrivateKey, publicKey: PublicKey): String? {
        val sharedKey = x25519(privateKey, publicKey)
        return ChaCha20.decryptUtf8(sharedKey, message.array)
    }

    override fun toString() = when (type) {
        PLAIN -> String(message.array)
        ENCRYPTED -> "ENCRYPTED:$message"
        else -> "UNKNOWN TYPE:$type DATA:$message"
    }

    @Serializer(forClass = Message::class)
    companion object {
        private val SIGN_MAGIC = "Blacknet Signed Message:\n".toUtf8Bytes()
        const val PLAIN: Byte = 0
        const val ENCRYPTED: Byte = 1
        val EMPTY = Message(PLAIN, SerializableByteArray.EMPTY)

        fun create(string: String?, type: Byte?, privateKey: PrivateKey?, publicKey: PublicKey?): Message? {
            if (string == null)
                return EMPTY

            if (type == null || type == PLAIN)
                return plain(string)

            if (type != ENCRYPTED || privateKey == null || publicKey == null)
                return null

            return encrypted(string, privateKey, publicKey)
        }

        fun plain(string: String): Message {
            return Message(PLAIN, string.toUtf8Bytes())
        }

        fun encrypted(string: String, privateKey: PrivateKey, publicKey: PublicKey): Message {
            val sharedKey = x25519(privateKey, publicKey)
            return Message(ENCRYPTED, ChaCha20.encryptUtf8(sharedKey, string))
        }

        fun decrypt(privateKey: PrivateKey, publicKey: PublicKey, hex: String): String? {
            val sharedKey = x25519(privateKey, publicKey)
            val bytes = fromHex(hex) ?: return null
            return ChaCha20.decryptUtf8(sharedKey, bytes)
        }

        fun sign(privateKey: PrivateKey, message: String): Signature {
            return Ed25519.sign(hash(message), privateKey)
        }

        fun verify(publicKey: PublicKey, signature: Signature, message: String): Boolean {
            return Ed25519.verify(signature, hash(message), publicKey)
        }

        private fun hash(message: String): Hash {
            return (Blake2b.Hasher() + SIGN_MAGIC + message).hash()
        }

        override fun deserialize(decoder: Decoder): Message {
            return when (decoder) {
                is BlacknetDecoder -> Message(decoder.decodeByte(), decoder.decodeSerializableByteArrayValue())
                is JsonInput -> throw NotImplementedError("not implemented for JsonInput")
                else -> throw RuntimeException("unsupported decoder")
            }
        }

        override fun serialize(encoder: Encoder, obj: Message) {
            when (encoder) {
                is BlacknetEncoder -> {
                    encoder.encodeByte(obj.type)
                    encoder.encodeSerializableByteArrayValue(obj.message)
                }
                is JsonOutput -> {
                    val data = if (obj.type == PLAIN) String(obj.message.array) else obj.message.array.toHex()
                    encoder.beginStructure(Message.descriptor)
                    encoder.encodeByteElement(Message.descriptor, 0, obj.type)
                    encoder.encodeStringElement(Message.descriptor, 1, data)
                    encoder.endStructure(Message.descriptor)
                }
                else -> throw RuntimeException("unsupported encoder")
            }
        }
    }
}
