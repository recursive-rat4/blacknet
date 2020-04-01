/*
 * Copyright (c) 2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.contract

import com.rfksystems.blake2b.security.Blake2bProvider
import ninja.blacknet.crypto.HashCoder.Companion.buildHash
import ninja.blacknet.util.byteArrayOfInts
import org.bouncycastle.jce.provider.BouncyCastleProvider
import org.testng.Assert.assertEquals
import org.testng.Assert.assertNotEquals
import org.testng.annotations.Test
import java.security.Security

class HashTypeTest {
    init {
        Security.addProvider(Blake2bProvider())
        Security.addProvider(BouncyCastleProvider())
    }

    @Test
    fun blake256() {
        assertEquals(
                buildHash(HashTimeLock.HashType.BLAKE256.algorithm) {

                },
                byteArrayOfInts(
                        0x0E, 0x57, 0x51, 0xC0, 0x26, 0xE5, 0x43, 0xB2,
                        0xE8, 0xAB, 0x2E, 0xB0, 0x60, 0x99, 0xDA, 0xA1,
                        0xD1, 0xE5, 0xDF, 0x47, 0x77, 0x8F, 0x77, 0x87,
                        0xFA, 0xAB, 0x45, 0xCD, 0xF1, 0x2F, 0xE3, 0xA8
                )
        )
    }

    @Test
    fun sha256() {
        assertEquals(
                buildHash(HashTimeLock.HashType.SHA256.algorithm) {

                },
                byteArrayOfInts(
                        0xE3, 0xB0, 0xC4, 0x42, 0x98, 0xFC, 0x1C, 0x14,
                        0x9A, 0xFB, 0xF4, 0xC8, 0x99, 0x6F, 0xB9, 0x24,
                        0x27, 0xAE, 0x41, 0xE4, 0x64, 0x9B, 0x93, 0x4C,
                        0xA4, 0x95, 0x99, 0x1B, 0x78, 0x52, 0xB8, 0x55
                )
        )
    }

    @Test
    fun keccak256() {
        assertEquals(
                buildHash(HashTimeLock.HashType.KECCAK256.algorithm) {

                },
                byteArrayOfInts(
                        0xC5, 0xD2, 0x46, 0x01, 0x86, 0xF7, 0x23, 0x3C,
                        0x92, 0x7E, 0x7D, 0xB2, 0xDC, 0xC7, 0x03, 0xC0,
                        0xE5, 0x00, 0xB6, 0x53, 0xCA, 0x82, 0x27, 0x3B,
                        0x7B, 0xFA, 0xD8, 0x04, 0x5D, 0x85, 0xA4, 0x70
                )
        )
        assertNotEquals(
                buildHash(HashTimeLock.HashType.KECCAK256.algorithm) {

                },
                byteArrayOfInts(
                        0xA7, 0xFF, 0xC6, 0xF8, 0xBF, 0x1E, 0xD7, 0x66,
                        0x51, 0xC1, 0x47, 0x56, 0xA0, 0x61, 0xD6, 0x62,
                        0xF5, 0x80, 0xFF, 0x4D, 0xE4, 0x3B, 0x49, 0xFA,
                        0x82, 0xD8, 0x0A, 0x4B, 0x80, 0xF8, 0x43, 0x4A
                )
        )
    }

    @Test
    fun ripemd160() {
        assertEquals(
                buildHash(HashTimeLock.HashType.RIPEMD160.algorithm) {

                },
                byteArrayOfInts(
                        0x9C, 0x11, 0x85, 0xA5,
                        0xC5, 0xE9, 0xFC, 0x54,
                        0x61, 0x28, 0x08, 0x97,
                        0x7E, 0xE8, 0xF5, 0x48,
                        0xB2, 0x25, 0x8D, 0x31
                )
        )
    }
}
