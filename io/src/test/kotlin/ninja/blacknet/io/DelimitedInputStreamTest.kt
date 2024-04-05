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
import kotlin.test.assertFails

class DelimitedInputStreamTest {
    @Test
    fun valid() {
        ZeroInputStream().delimited().apply {
            skip(0)
            read(ByteArray(0))
            read()
            begin(1)
            skip(0)
            read(ByteArray(0))
            read()
            end()
            skip(0)
            read(ByteArray(0))
            read()
            begin(2)
            skip(1)
            skipNBytes(1)
            end()
        }.close()
    }

    @Test
    fun trailing() {
        ZeroInputStream().delimited().apply {
            begin(2)
            read(ByteArray(0))
            read()
            assertFails { end() }
        }.close()
    }

    @Test
    fun eom() {
        ZeroInputStream().delimited().apply {
            begin(1)
            read()
            assertFails { read() }
        }.close()
        DelimitedInputStream(ZeroInputStream()).apply {
            begin(1)
            skip(1)
            assertFails { skip(1) }
        }.close()
        DelimitedInputStream(ZeroInputStream()).apply {
            begin(1)
            skipNBytes(1)
            assertFails { skipNBytes(1) }
        }.close()
    }
}
