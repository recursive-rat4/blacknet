/*
 * Copyright (c) 2018-2019 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.crypto

import ninja.blacknet.Config

/**
 * Account address encoding using [Bech32].
 *
 * SatoshiLabs Improvement Proposal 173 "Registered human-readable parts for BIP-0173"
 */
object Address {
    private val HRP_MAINNET = "blacknet".toByteArray(Charsets.US_ASCII)
    private val HRP_REGTEST = "rblacknet".toByteArray(Charsets.US_ASCII)

    private val HRP = if (Config.regTest) HRP_REGTEST else HRP_MAINNET

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
