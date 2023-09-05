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

class ComponentsTest {
    @Test
    fun test() {
        assertEquals(byteArrayOf(1, -1), 0x01FF.toShort().toByteArray())
        assertEquals(byteArrayOf(2, 1, -1, -2), 0x0201FFFE.toByteArray())
        assertEquals(byteArrayOf(4, 3, 2, 1, -1, -2, -3, -4), 0x04030201FFFEFDFC.toByteArray())

        assertEquals(0x01FF.toShort(), Short.fromBytes(1, -1))
        assertEquals(0x0201FFFE, Int.fromBytes(2, 1, -1, -2))
        assertEquals(0x04030201FFFEFDFC, Long.fromBytes(4, 3, 2, 1, -1, -2, -3, -4))
    }
}
