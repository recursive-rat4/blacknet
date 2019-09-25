/*
 * Copyright (c) 2018 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.crypto

/**
 * Blacknet address
 */
object Address {
    private val HRP = "blacknet".toByteArray(Charsets.US_ASCII)

    fun encode(publicKey: PublicKey): String {
        return Bech32.encode(HRP, publicKey.bytes)
    }

    fun decode(string: String?): PublicKey? {
        if (string == null)
            return null
        val (hrp, data) = Bech32.decode(string) ?: return null
        if (!HRP.contentEquals(hrp))
            return null
        if (data.size != PublicKey.SIZE)
            return null
        return PublicKey(data)
    }
}
