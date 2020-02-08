/*
 * Copyright (c) 2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.coding

import org.testng.Assert.*
import org.testng.annotations.Test

class Base16Test {
    @Test
    fun test() {
        assertEquals(hex(byteArrayOf(0x01, 0x02, 0x0A, 0x0B), false), "01020A0B")
        assertEquals(hex(byteArrayOf(0x01, 0x02, 0x0A, 0x0B), true), "01020a0b")

        assertEquals(fromHex(""), ByteArray(0))
        assertEquals(fromHex("01020A0B"), byteArrayOf(0x01, 0x02, 0x0A, 0x0B))
        assertEquals(fromHex("01020a0b"), byteArrayOf(0x01, 0x02, 0x0A, 0x0B))
        assertEquals(fromHex("01020A0b"), byteArrayOf(0x01, 0x02, 0x0A, 0x0B))
        assertEquals(fromHex("01020A0B", 4), byteArrayOf(0x01, 0x02, 0x0A, 0x0B))

        assertNull(fromHex("0"))
        assertNull(fromHex("0Z"))
        assertNull(fromHex("01020A0B", 5))
        assertNull(fromHex("01020A0B", 3))
        assertNull(fromHex("01020A0B", 2))
    }
}
