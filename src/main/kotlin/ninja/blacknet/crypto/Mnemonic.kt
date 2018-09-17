/*
 * Copyright (c) 2018 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.crypto

import kotlinx.serialization.toUtf8Bytes
import ninja.blacknet.crypto.Ed25519.PrivateKey

object Mnemonic {
    fun fromString(string: String): PrivateKey? {
        val hash = Blake2b.hash(string.toUtf8Bytes())
        if (checkVersion(hash.bytes.array))
            return PrivateKey(hash.bytes.array)
        return null
    }

    private fun checkVersion(bytes: ByteArray): Boolean {
        return bytes[0].toInt() and 0xF0 == 0x10
    }
}