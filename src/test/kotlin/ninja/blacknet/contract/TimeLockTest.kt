/*
 * Copyright (c) 2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.contract

import org.testng.Assert.assertFalse
import org.testng.Assert.assertTrue
import org.testng.annotations.Test

class TimeLockTest {
    @Test
    fun absoluteTime() {
        assertFalse(TimeLock(TIME, 1000000).verify(0, 0, 0, 1000000 - 1))
        assertFalse(TimeLock(TIME, 1000000).verify(0, 0, 0, 1000000))
        assertTrue(TimeLock(TIME, 1000000).verify(0, 0, 0, 1000001))
        assertTrue(TimeLock(TIME, 1000000).verify(0, 0, 0, 1000001 + 1))
    }

    @Test
    fun absoluteHeight() {
        assertFalse(TimeLock(HEIGHT, 1000000).verify(0, 0, 1000000 - 1, 0))
        assertFalse(TimeLock(HEIGHT, 1000000).verify(0, 0, 1000000, 0))
        assertTrue(TimeLock(HEIGHT, 1000000).verify(0, 0, 1000001, 0))
        assertTrue(TimeLock(HEIGHT, 1000000).verify(0, 0, 1000001 + 1, 0))
    }

    @Test
    fun relativeTime() {
        assertFalse(TimeLock(RELATIVE_TIME, 10000).verify(0, 990000, 0, 1000000 - 1))
        assertFalse(TimeLock(RELATIVE_TIME, 10000).verify(0, 990000, 0, 1000000))
        assertTrue(TimeLock(RELATIVE_TIME, 10000).verify(0, 990000, 0, 1000001))
        assertTrue(TimeLock(RELATIVE_TIME, 10000).verify(0, 990000, 0, 1000001 + 1))
    }

    @Test
    fun relativeHeight() {
        assertFalse(TimeLock(RELATIVE_HEIGHT, 10000).verify(990000, 0, 1000000 - 1, 0))
        assertFalse(TimeLock(RELATIVE_HEIGHT, 10000).verify(990000, 0, 1000000, 0))
        assertTrue(TimeLock(RELATIVE_HEIGHT, 10000).verify(990000, 0, 1000001, 0))
        assertTrue(TimeLock(RELATIVE_HEIGHT, 10000).verify(990000, 0, 1000001 + 1, 0))
    }
}
