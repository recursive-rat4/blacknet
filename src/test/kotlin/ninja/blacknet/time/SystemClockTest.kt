/*
 * Copyright (c) 2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.time

import ninja.blacknet.db.Genesis
import ninja.blacknet.time.milliseconds.MilliSeconds
import ninja.blacknet.time.milliseconds.seconds
import org.testng.Assert.assertTrue
import org.testng.annotations.Test

class SystemClockTest {
    @Test
    fun test() {
        val seconds = SystemClock.seconds
        assertTrue(seconds >= Genesis.TIME)
        assertTrue(seconds <= Long.MAX_VALUE)

        val milliseconds = SystemClock.milliseconds
        assertTrue(milliseconds >= Genesis.TIME.seconds)
        assertTrue(milliseconds <= MilliSeconds.MAX_VALUE)
    }
}
