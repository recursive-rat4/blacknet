/*
 * Copyright (c) 2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.time

import kotlin.test.Test
import kotlin.test.assertTrue

class SystemClockTest {
    @Test
    fun test() {
        val seconds = currentTimeSeconds()
        assertTrue(seconds >= 0)
        assertTrue(seconds <= Long.MAX_VALUE)

        val milliseconds = currentTimeMillis()
        assertTrue(milliseconds >= 0)
        assertTrue(milliseconds <= Long.MAX_VALUE)
    }
}
