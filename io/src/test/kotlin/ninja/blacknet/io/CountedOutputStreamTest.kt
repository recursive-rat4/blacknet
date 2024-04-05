/*
 * Copyright (c) 2020-2024 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.io

import java.io.OutputStream.nullOutputStream
import kotlin.test.Test
import kotlin.test.assertEquals

class CountedOutputStreamTest {
    @Test
    fun test() {
        nullOutputStream().counted().apply {
            write(ByteArray(0))
            assertEquals(0, bytesWritten)
            write(0)
            assertEquals(1, bytesWritten)
            write(ByteArray(3))
            assertEquals(4, bytesWritten)
        }.close()
    }
}
