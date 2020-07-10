/*
 * Copyright (c) 2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.util

import org.testng.Assert.assertEquals
import org.testng.annotations.Test

class LinkedHashMapTest {
    @Test
    fun int() {
        val map = LinkedHashMap<Int, Int>()
        assertEquals(map.size, 0)
        map.put(4, 16)
        assertEquals(map.size, 1)
        map.put(4, 16)
        assertEquals(map.size, 1)
    }

    @Test
    fun bytearray() {
        val map = LinkedHashMap<ByteArray, ByteArray>()
        assertEquals(map.size, 0)
        map.put(ByteArray(4), ByteArray(16))
        assertEquals(map.size, 1)
        map.put(ByteArray(4), ByteArray(16))
        assertEquals(map.size, 1)
    }
}
