/*
 * Copyright (c) 2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.contract

import kotlin.test.Test
import kotlin.test.assertFalse
import kotlin.test.assertTrue

class TimeLockTest {
    @Test
    fun absoluteTime() {
        assertFalse(TimeLock(TIME, 1000000L).verify(0, 0L, 0, 1000000L - 1))
        assertFalse(TimeLock(TIME, 1000000L).verify(0, 0L, 0, 1000000L))
        assertTrue(TimeLock(TIME, 1000000L).verify(0, 0L, 0, 1000001L))
        assertTrue(TimeLock(TIME, 1000000L).verify(0, 0L, 0, 1000001L + 1))
    }

    @Test
    fun absoluteHeight() {
        assertFalse(TimeLock(HEIGHT, 1000000L).verify(0, 0L, 1000000 - 1, 0L))
        assertFalse(TimeLock(HEIGHT, 1000000L).verify(0, 0L, 1000000, 0L))
        assertTrue(TimeLock(HEIGHT, 1000000L).verify(0, 0L, 1000001, 0L))
        assertTrue(TimeLock(HEIGHT, 1000000L).verify(0, 0L, 1000001 + 1, 0L))
    }

    @Test
    fun relativeTime() {
        assertFalse(TimeLock(RELATIVE_TIME, 10000L).verify(0, 990000L, 0, 1000000L - 1))
        assertFalse(TimeLock(RELATIVE_TIME, 10000L).verify(0, 990000L, 0, 1000000L))
        assertTrue(TimeLock(RELATIVE_TIME, 10000L).verify(0, 990000L, 0, 1000001L))
        assertTrue(TimeLock(RELATIVE_TIME, 10000L).verify(0, 990000L, 0, 1000001L + 1))
    }

    @Test
    fun relativeHeight() {
        assertFalse(TimeLock(RELATIVE_HEIGHT, 10000L).verify(990000, 0L, 1000000 - 1, 0L))
        assertFalse(TimeLock(RELATIVE_HEIGHT, 10000L).verify(990000, 0L, 1000000, 0L))
        assertTrue(TimeLock(RELATIVE_HEIGHT, 10000L).verify(990000, 0L, 1000001, 0L))
        assertTrue(TimeLock(RELATIVE_HEIGHT, 10000L).verify(990000, 0L, 1000001 + 1, 0L))
    }
}
