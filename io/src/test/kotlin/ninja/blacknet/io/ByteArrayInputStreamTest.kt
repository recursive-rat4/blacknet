/*
 * Copyright (c) 2023-2024 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

@file:Suppress("KotlinConstantConditions")

package ninja.blacknet.io

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFails

class ByteArrayInputStreamTest {
    @Test
    fun test() {
        byteArrayOf(
            5,
            4,
            (-253).toByte(),
            1,
            0,
        ).inputStream().apply {
            skipNBytes(0)
            skipNBytes(1)
            assertEquals(1, skip(1))
            assertEquals(3, read())
            assertEquals(2, available())
            assertEquals(1, read())
            assertEquals(0, read())
            assertEquals(0, skip(0))
        }
    }

    @Test
    fun eof() {
        ByteArray(0).inputStream().apply {
            assertEquals(0, skip(1))
            assertFails { skipNBytes(1) }
        }
    }
}
