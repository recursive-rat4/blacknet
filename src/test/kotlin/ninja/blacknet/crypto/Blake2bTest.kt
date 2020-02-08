/*
 * Copyright (c) 2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.crypto

import ninja.blacknet.byte.byteArrayOfInts
import org.testng.Assert.assertEquals
import org.testng.annotations.Test

class Blake2bTest {
    @Test
    fun test() {
        assertEquals(
                Blake2b.hasher {
                    x(0x01.toByte())
                    x(0x0203.toShort())
                    x(0x04050607)
                    x(0x08090A0B0C0D0E0F)
                    x("八")
                },
                Blake2b.hasher {
                    x(byteArrayOfInts(
                            1,
                            2, 3,
                            4, 5, 6, 7,
                            8, 9, 10, 11, 12, 13, 14, 15,
                            0xE5, 0x85, 0xAB
                    ))
                }
        )
    }
}
