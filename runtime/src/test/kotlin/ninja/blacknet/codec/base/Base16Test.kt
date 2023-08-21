/*
 * Copyright (c) 2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.codec.base

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFailsWith
import kotlin.test.assertTrue

class Base16Test {
    @Test
    fun decode() {
        assertEquals(byteArrayOf(0x01, 0x02, 0x0A, 0x0B), Base16.decode("01020A0B"))
        assertEquals(byteArrayOf(0x01, 0x02, 0x0A, 0x0B), Base16.decode("01020a0b"))
        assertEquals(byteArrayOf(0x01, 0x02, 0x0A, 0x0B), Base16.decode("01020A0b"))
        assertEquals(byteArrayOf(0x01, 0x02, 0x0A, 0x0B), Base16.decode("01020a0B"))

        assertFailsWith(IllegalArgumentException::class) { Base16.decode("0") }
        assertFailsWith(IllegalArgumentException::class) { Base16.decode("0Z") }
        assertFailsWith(IllegalArgumentException::class) { Base16.decode("000") }
        assertFailsWith(IllegalArgumentException::class) { Base16.decode("000Z") }
    }

    @Test
    fun encode() {
        assertTrue(Base16.encode(byteArrayOf(0x01, 0x02, 0x0A, 0x0B)).let {
            it == "01020A0B"
         || it == "01020a0b"
         // it == "01020A0b"
         // it == "01020a0B"
        })
    }
}
