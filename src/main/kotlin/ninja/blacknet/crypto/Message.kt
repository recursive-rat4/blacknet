/*
 * Copyright (c) 2018 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.crypto

import kotlinx.serialization.Serializable
import kotlinx.serialization.toUtf8Bytes
import ninja.blacknet.crypto.Ed25519.x25519
import ninja.blacknet.serialization.SerializableByteArray

@Serializable
class Message(
        val type: Byte,
        val data: SerializableByteArray
) {
    constructor(type: Byte, bytes: ByteArray) : this(type, SerializableByteArray(bytes))

    fun decrypt(privateKey: PrivateKey, publicKey: PublicKey): String? {
        val sharedKey = x25519(privateKey, publicKey)
        return ChaCha20.decryptUtf8(sharedKey, data.array)
    }

    companion object {
        private val SIGN_MAGIC = "Blacknet Signed Message:\n".toUtf8Bytes()
        const val PLAIN: Byte = 0
        const val ENCRYPTED: Byte = 1

        fun create(string: String?, type: Byte?, privateKey: PrivateKey?, publicKey: PublicKey?): Message? {
            if (string == null)
                return empty()

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

        fun empty() = Message(PLAIN, SerializableByteArray.EMPTY)

        fun sign(privateKey: PrivateKey, message: String): Signature {
            return Ed25519.sign(hash(message), privateKey)
        }

        fun verify(publicKey: PublicKey, signature: Signature, message: String): Boolean {
            return Ed25519.verify(signature, hash(message), publicKey)
        }

        private fun hash(message: String): Hash {
            return (Blake2b.Hasher() + SIGN_MAGIC + message).hash()
        }
    }
}
