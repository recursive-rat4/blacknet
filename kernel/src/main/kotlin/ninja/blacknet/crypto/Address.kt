/*
 * Copyright (c) 2018-2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.crypto

import ninja.blacknet.BECH32_HRP
import ninja.blacknet.codec.base.Bech32
import ninja.blacknet.contract.BAppId
import ninja.blacknet.contract.HashTimeLockContractId
import ninja.blacknet.contract.MultiSignatureLockContractId
import ninja.blacknet.mode
import ninja.blacknet.util.plus

/**
 * Address encoding using [Bech32].
 *
 * SatoshiLabs Improvement Proposal 173 "Registered human-readable parts for BIP-0173"
 */
object Address {
    val ACCOUNT: Byte? = null
    const val STAKER: Byte = 0
    const val HTLC: Byte = 1
    const val MULTISIG: Byte = 2
    const val BAPP: Byte = 3

    private val HRP = when (val prefix = mode.addressPrefix) {
        null -> BECH32_HRP
        else -> prefix + BECH32_HRP
    }.toByteArray(Charsets.US_ASCII)

    fun encode(publicKey: ByteArray): String {
        val data = Bech32.convertBits(publicKey, 8, 5, true)
        return Bech32.encode(HRP, data)
    }

    fun decode(string: String): ByteArray {
        val (hrp, data) = Bech32.decode(string)
        if (!HRP.contentEquals(hrp))
            throw Exception("Expected HRP ${String(HRP, Charsets.US_ASCII)} actual ${String(hrp, Charsets.US_ASCII)}")
        val bytes = Bech32.convertBits(data, 5, 8, false)
        if (PublicKey.SIZE_BYTES != bytes.size)
            throw Exception("Expected size ${PublicKey.SIZE_BYTES} actual ${bytes.size}")
        return bytes
    }

    fun encode(version: Byte, bytes: ByteArray): String {
        @Suppress("NAME_SHADOWING")
        val bytes = version + bytes
        if (expectedSize(version) != bytes.size)
            throw Exception("Expected size ${expectedSize(version)} actual ${bytes.size}")
        val data = Bech32.convertBits(bytes, 8, 5, true)
        return Bech32.encode(HRP, data)
    }

    fun decode(version: Byte, string: String): ByteArray {
        val (hrp, data) = Bech32.decode(string)
        if (!HRP.contentEquals(hrp))
            throw Exception("Expected HRP ${String(HRP, Charsets.US_ASCII)} actual ${String(hrp, Charsets.US_ASCII)}")
        val bytes = Bech32.convertBits(data, 5, 8, false)
        if (expectedSize(version) != bytes.size)
            throw Exception("Expected size ${expectedSize(version)} actual ${bytes.size}")
        if (bytes[0] != version)
            throw Exception("Expected version ${bytes[0].toUByte()} actual ${version.toUByte()}")
        return bytes.copyOfRange(Byte.SIZE_BYTES, bytes.size)
    }

    private fun expectedSize(version: Byte): Int = Byte.SIZE_BYTES + when (version) {
        STAKER -> throw Error("保留地址版本字節")
        HTLC -> HashTimeLockContractId.SIZE_BYTES
        MULTISIG -> MultiSignatureLockContractId.SIZE_BYTES
        BAPP -> BAppId.SIZE_BYTES
        else -> throw Exception("Unknown address version $version")
    }

    private class Error     constructor(message: String) : kotlin.Error    (message)
    private class Exception constructor(message: String) : kotlin.Exception(message)
}
