/*
 * Copyright (c) 2018-2019 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.crypto

import ninja.blacknet.coding.fromHex
import ninja.blacknet.coding.toHex

/**
 * Ed25519 private key
 */
class PrivateKey(val bytes: ByteArray) {
    override fun equals(other: Any?): Boolean = (other is PrivateKey) && bytes.contentEquals(other.bytes)
    override fun hashCode(): Int = Salt.hashCode { x(bytes) }
    override fun toString(): String = bytes.toHex()

    fun toPublicKey(): PublicKey {
        return Ed25519.publicKey(this)
    }

    companion object {
        /**
         * The number of bytes in a binary representation of a [PrivateKey].
         */
        const val SIZE = 32

        fun fromString(hex: String?): PrivateKey? {
            if (hex == null) return null
            val bytes = fromHex(hex, SIZE) ?: return null
            return PrivateKey(bytes)
        }
    }
}
