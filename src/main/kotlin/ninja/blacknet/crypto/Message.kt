/*
 * Copyright (c) 2018-2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.crypto

import ninja.blacknet.MESSAGE_SIGN_NAME
import ninja.blacknet.crypto.Blake2b.buildHash

object Message {
    private const val SIGN_MAGIC = "$MESSAGE_SIGN_NAME Signed Message:\n"

    fun sign(privateKey: ByteArray, message: String): ByteArray {
        return Ed25519.sign(hash(SIGN_MAGIC, message), privateKey)
    }

    fun verify(publicKey: ByteArray, signature: ByteArray, message: String): Boolean {
        return verifyWithMagic(publicKey, signature, message, SIGN_MAGIC)
    }

    internal fun verifyWithMagic(publicKey: ByteArray, signature: ByteArray, message: String, magic: String): Boolean {
        return Ed25519.verify(signature, hash(magic, message), publicKey)
    }

    private fun hash(magic: String, message: String): ByteArray {
        return buildHash {
            encodeString(magic)
            encodeString(message)
        }
    }
}
