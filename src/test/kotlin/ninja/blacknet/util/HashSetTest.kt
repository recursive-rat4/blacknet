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

class HashSetTest {
    @Test
    fun int() {
        val set = HashSet<Int>()
        assertEquals(0, set.size)
        set.add(32)
        assertEquals(1, set.size)
        set.add(32)
        assertEquals(1, set.size)
    }

    @Test
    fun bytearray() {
        val set = HashSet<ByteArray>()
        assertEquals(0, set.size)
        set.add(ByteArray(32))
        assertEquals(1, set.size)
        set.add(ByteArray(32))
        assertEquals(1, set.size)
    }
}
