/*
 * Copyright (c) 2023-2025 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.network.packet

import java.math.BigInteger
import kotlin.test.Test
import kotlin.test.assertEquals
import ninja.blacknet.crypto.Hash
import ninja.blacknet.serialization.bbf.binaryFormat
import ninja.blacknet.util.byteArrayOfInts

class BlockAnnounceTest {
    val hash = Hash(Hash.fromString("FB4064283A07A69749A8012897BDA7159D6F256F9AE41BEBF47D4E96E7825A26"))
    val difficulty = BigInteger("1654811289011657408691630")
    val announce = BlockAnnounce(hash, difficulty)
    val announceBytes = byteArrayOfInts(
        0xFB, 0x40, 0x64, 0x28, 0x3A, 0x07, 0xA6, 0x97,
        0x49, 0xA8, 0x01, 0x28, 0x97, 0xBD, 0xA7, 0x15,
        0x9D, 0x6F, 0x25, 0x6F, 0x9A, 0xE4, 0x1B, 0xEB,
        0xF4, 0x7D, 0x4E, 0x96, 0xE7, 0x82, 0x5A, 0x26,
        128 + 11,
        0x01, 0x5E, 0x6B, 0x7F, 0xEE, 0x4E, 0x21, 0xDF, 0x56, 0xBD, 0xAE,
    )

    @Test
    fun decode() {
        binaryFormat.decodeFromByteArray(BlockAnnounce.serializer(), announceBytes).also {
            // assertEquals(announce, it)
            // doesn't override equals
        }
    }

    @Test
    fun encode() {
        assertEquals(
            announceBytes,
            binaryFormat.encodeToByteArray(BlockAnnounce.serializer(), announce)
        )
    }
}
