/*
 * Copyright (c) 2018 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.crypto

object Address {
    private val HRP = "blacknet".toByteArray(Charsets.US_ASCII)

    fun encode(publicKey: PublicKey): String {
        val converted = Bech32.convertBits(publicKey.bytes.array, 8, 5, true)
        return Bech32.encode(Bech32.Data(HRP, converted!!))
    }

    fun decode(string: String?): PublicKey? {
        if (string == null)
            return null
        val data = Bech32.decode(string)
        if (data == null || !HRP.contentEquals(data.hrp))
            return null
        val converted = Bech32.convertBits(data.data, 5, 8, false)
        if (converted == null || converted.size != PublicKey.SIZE)
            return null
        return PublicKey(converted)
    }
}