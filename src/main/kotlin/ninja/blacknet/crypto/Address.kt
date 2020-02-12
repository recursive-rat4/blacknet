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
import ninja.blacknet.coding.Bech32
import ninja.blacknet.util.plus

/**
 * Address encoding using [Bech32].
 *
 * SatoshiLabs Improvement Proposal 173 "Registered human-readable parts for BIP-0173"
 */
object Address {
    private val HRP_MAINNET = "blacknet".toByteArray(Charsets.US_ASCII)
    private val HRP_REGTEST = "rblacknet".toByteArray(Charsets.US_ASCII)

    const val ACCOUNT: Byte = 0
    const val HTLC: Byte = 1
    const val MULTISIG: Byte = 2

    private val HRP = if (Config.regTest) HRP_REGTEST else HRP_MAINNET

    fun encode(publicKey: PublicKey): String {
        val bytes = publicKey.bytes
        val data = Bech32.convertBits(bytes, 8, 5, true)!!
        return Bech32.encode(HRP, data)
    }

    fun decode(string: String?): PublicKey? {
        if (string == null)
            return null
        val (hrp, data) = Bech32.decode(string) ?: return null
        if (!HRP.contentEquals(hrp))
            return null
        val bytes = Bech32.convertBits(data, 5, 8, false) ?: return null
        if (bytes.size != PublicKey.SIZE_BYTES)
            return null
        return PublicKey(bytes)
    }

    fun encodeId(version: Byte, hash: Hash): String {
        require(version == HTLC || version == MULTISIG)
        val bytes = version + hash.bytes
        val data = Bech32.convertBits(bytes, 8, 5, true)!!
        return Bech32.encode(HRP, data)
    }

    fun decodeId(version: Byte, string: String?): Hash? {
        require(version == HTLC || version == MULTISIG)
        if (string == null)
            return null
        val (hrp, data) = Bech32.decode(string) ?: return null
        if (!HRP.contentEquals(hrp))
            return null
        val bytes = Bech32.convertBits(data, 5, 8, false) ?: return null
        if (bytes.size != Byte.SIZE_BYTES + Hash.SIZE_BYTES)
            return null
        if (bytes[0] != version)
            return null
        return Hash(bytes.copyOfRange(Byte.SIZE_BYTES, Byte.SIZE_BYTES + Hash.SIZE_BYTES))
    }
}
