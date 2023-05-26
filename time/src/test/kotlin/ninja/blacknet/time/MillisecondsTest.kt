/*
 * Copyright (c) 2023 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.time

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertTrue

class MillisecondsTest {
    @Test
    fun data() {
        val answer: Long = 42
        assertEquals(answer, Milliseconds(answer).milliseconds)
        assertEquals(Milliseconds(answer), Milliseconds(answer))
        assertEquals(Milliseconds(answer).hashCode(), Milliseconds(answer).hashCode())
        assertEquals(answer.toString(), Milliseconds(answer).toString())
    }

    @Test
    fun comparison() {
        val a = Milliseconds('А'.code.toLong())
        val b = Milliseconds('Б'.code.toLong())
        assertTrue(a >= Milliseconds.MIN_VALUE && a <= Milliseconds.MAX_VALUE)
        assertTrue(b >= Milliseconds.MIN_VALUE && b <= Milliseconds.MAX_VALUE)
        assertTrue(a < b)
        assertTrue(b > a)
    }

    @Test
    fun operators() {
        val a = Milliseconds(202)
        val b = Milliseconds(2)

        assertEquals(Milliseconds(+202), +a)
        assertEquals(Milliseconds(-202), -a)

        assertEquals(Milliseconds(204), a + b)
        assertEquals(Milliseconds(200), a - b)

        assertEquals(Milliseconds(404), a * 2)
        assertEquals(101, a / b)
        assertEquals(Milliseconds(101), a / 2)

        assertEquals(Milliseconds(0), a % b)
        assertEquals(Milliseconds(1), a % 3)
    }
}
