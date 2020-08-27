/*
 * Copyright (c) 2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.util

import kotlin.test.Test
import kotlin.test.assertEquals

class HashMapTest {
    @Test
    fun int() {
        val map = HashMap<Int, Int>()
        assertEquals(0, map.size)
        map.put(4, 16)
        assertEquals(1, map.size)
        map.put(4, 16)
        assertEquals(1, map.size)
    }

    @Test
    fun bytearray() {
        val map = HashMap<ByteArray, ByteArray>()
        assertEquals(0, map.size)
        map.put(ByteArray(4), ByteArray(16))
        assertEquals(1, map.size)
        map.put(ByteArray(4), ByteArray(16))
        assertEquals(1, map.size)
    }
}
