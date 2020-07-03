/*
 * Copyright (c) 2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.core

import ninja.blacknet.db.Genesis
import org.testng.Assert.assertTrue
import org.testng.annotations.Test

class SystemClockTest {
    @Test
    fun test() {
        val seconds = currentTimeSeconds()
        assertTrue(seconds >= Genesis.TIME)
        assertTrue(seconds <= Long.MAX_VALUE)

        val milliseconds = currentTimeMillis()
        assertTrue(milliseconds >= Genesis.TIME * 1000L)
        assertTrue(milliseconds <= Long.MAX_VALUE)
    }
}
