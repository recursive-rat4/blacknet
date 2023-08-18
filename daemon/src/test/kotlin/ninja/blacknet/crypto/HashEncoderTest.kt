/*
 * Copyright (c) 2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.crypto

import kotlin.test.Test
import kotlin.test.assertEquals
import ninja.blacknet.crypto.HashEncoder.Companion.buildHash
import ninja.blacknet.util.byteArrayOfInts

class HashEncoderTest {
    @Test
    fun test() {
        assertEquals(
                buildHash("MD5") {
                    encodeByteArray(byteArrayOfInts(
                            1,
                            2, 3,
                            4, 5, 6, 7,
                            8, 9, 10, 11, 12, 13, 14, 15,
                            0xE5, 0x85, 0xAB,
                            0x10, 0x11, 0x12, 0x13,
                            /* 0x14, 0x15, */ 0x16, 0x17
                    ))
                },
                buildHash("MD5") {
                    encodeByte(0x01)
                    encodeShort(0x0203)
                    encodeInt(0x04050607)
                    encodeLong(0x08090A0B0C0D0E0F)
                    encodeString("八")
                    encodeByteArray(byteArrayOfInts(0x10, 0x11, 0x12, 0x13))
                    encodeByteArray(byteArrayOfInts(0x14, 0x15, 0x16, 0x17), 2, 2)
                }
        )
    }
}
