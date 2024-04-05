/*
 * Copyright (c) 2020-2024 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.io

import kotlin.test.Test
import kotlin.test.assertEquals

class ByteArrayOutputStreamTest {
    @Test
    fun test() {
        assertEquals(
            byteArrayOf(
                2,
                1,
                0,
            ),
            ByteArray(3).apply {
                outputStream().apply {
                    write(-254)
                    write(1)
                    write(0)
                }
            }
        )
    }
}
