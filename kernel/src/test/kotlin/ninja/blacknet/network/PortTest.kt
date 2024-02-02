/*
 * Copyright (c) 2024 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.network

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFails
import kotlin.test.assertNotEquals
import kotlin.test.assertTrue

class PortTest {
    @Test
    fun test() {
        assertEquals(Port(443u), Port(443))
        assertNotEquals(Port(80u), Port(443u))
        assertTrue(Port(0u).compareTo(Port(0u)) == 0)
        assertTrue(Port(10000u) < Port(50000u))
        assertEquals(4, Port(4u).toJava())
        assertFails { Port(66666) }
        assertEquals("10", Port(10u).toString())
    }
}
