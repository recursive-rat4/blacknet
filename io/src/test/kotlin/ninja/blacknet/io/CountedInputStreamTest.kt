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

class CountedInputStreamTest {
    @Test
    fun test() {
        ZeroInputStream().counted().apply {
            skip(0)
            assertEquals(0, bytesRead)
            skipNBytes(0)
            assertEquals(0, bytesRead)
            read(ByteArray(0))
            assertEquals(0, bytesRead)
            read()
            assertEquals(1, bytesRead)
            read(ByteArray(3))
            assertEquals(4, bytesRead)
            skip(1)
            assertEquals(5, bytesRead)
            skipNBytes(1)
            assertEquals(6, bytesRead)
        }.close()
    }
}
