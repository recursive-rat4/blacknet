/*
 * Copyright (c) 2018 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.crypto

import ninja.blacknet.util.fromHex
import ninja.blacknet.util.toHex

class PrivateKey(val bytes: ByteArray) {
    override fun toString(): String = bytes.toHex()

    fun toPublicKey(): PublicKey {
        return Ed25519.publicKey(this)
    }

    companion object {
        const val SIZE = 32

        fun fromString(hex: String?): PrivateKey? {
            if (hex == null || hex.length != SIZE * 2)
                return null
            val bytes = fromHex(hex) ?: return null
            return PrivateKey(bytes)
        }
    }
}
