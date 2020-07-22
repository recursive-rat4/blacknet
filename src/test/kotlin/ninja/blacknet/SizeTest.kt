/*
 * Copyright (c) 2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet

import java.util.Locale
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFails

class SizeTest {
    @Test
    fun parse() {
        for ((string, bytes) in arrayOf(
                Pair("400", 400),
                Pair("4kB", 4000),
                Pair("4MB", 4000000),
                Pair("400B", 400),
                Pair("4KiB", 4096),
                Pair("4MiB", 4194304),
                Pair("1Apple", null),
                Pair("1iRobot", null)
        )) {
            if (bytes != null)
                assertEquals(Size.parse(string).bytes, bytes)
            else
                assertFails { Size.parse(string) }
        }
    }

    @Test
    fun hrp() {
        for ((bytes, hrp, decimal) in arrayOf(
                Triple(5737956, "5.74 MB", true),
                Triple(2494885, "2.49 MB", true),
                Triple(1025, "1.02 kB", true),
                Triple(33554432, "32 MiB", false),
                Triple(97516, "95.23 KiB", false),
                Triple(1024, "1 KiB", false),
                Triple(1023, "1023 B", false)
        )) {
            assertEquals(Size(bytes).hrp(decimal, Locale.CHINESE), hrp)
        }
    }
}
