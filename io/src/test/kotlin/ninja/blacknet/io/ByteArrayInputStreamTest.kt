/*
 * Copyright (c) 2023-2024 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.io

import kotlin.test.Test
import kotlin.test.assertEquals

class ByteArrayInputStreamTest {
    @Test
    fun test() {
        byteArrayOf(
            (-253).toByte(),
            1,
            0,
        ).inputStream().apply {
            assertEquals(3, read())
            assertEquals(2, available())
            assertEquals(1, read())
            assertEquals(0, read())
        }
    }
}
