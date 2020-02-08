/*
 * Copyright (c) 2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.time.milliseconds

import org.testng.Assert.assertTrue
import org.testng.annotations.Test
import kotlin.random.Random

class RandomTest {
    @Test
    fun test() {
        val from = 2.minutes
        val until = 16.minutes

        repeat(10) {
            val random = Random.nextTime()
            assertTrue(random >= MilliSeconds.MIN_VALUE)
            assertTrue(random <= MilliSeconds.MAX_VALUE)
        }
        repeat(10) {
            val random = Random.nextTime(until)
            assertTrue(random >= MilliSeconds.ZERO)
            assertTrue(random < until)
        }
        repeat(10) {
            val random = Random.nextTime(from, until)
            assertTrue(random >= from)
            assertTrue(random < until)
        }
    }
}
