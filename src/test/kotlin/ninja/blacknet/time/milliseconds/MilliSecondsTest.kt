/*
 * Copyright (c) 2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

@file:Suppress("WrapUnaryOperator")

package ninja.blacknet.time.milliseconds

import org.testng.Assert.assertEquals
import org.testng.annotations.Test

class MilliSecondsTest {
    @Test
    fun test() {
        assertEquals(4.days.days, 4)
        assertEquals(4.days.hours, 96)
        assertEquals(4.days.minutes, 5760)
        assertEquals(4.days.seconds, 345600)
        assertEquals(4.days.milliseconds, 345600000)

        assertEquals(4.days, 96.hours)
        assertEquals(4.hours, 240.minutes)
        assertEquals(4.minutes, 240.seconds)
        assertEquals(4.seconds, 4000.milliseconds)

        assertEquals( 4.days, ( 4).days)
        assertEquals(+4.days, (+4).days)
        assertEquals(-4.days, (-4).days)

        assertEquals(0.days + 4.days, +4.days)
        assertEquals(0.days - 4.days, -4.days)

        assertEquals(4.days / 2.days, 2)
        assertEquals(4.days % 3.days, 1.days)

        assertEquals(4.days + 2.hours, 98.hours)
        assertEquals(4.days - 2.hours, 94.hours)

        assertEquals(2.days * 2, 4.days)
        assertEquals(4.days / 2, 2.days)
        assertEquals(4.days % 3, 0.days)

        assertEquals(2 * 2.days, 4.days)
        assertEquals(4 % 3.milliseconds, 1)
    }
}
