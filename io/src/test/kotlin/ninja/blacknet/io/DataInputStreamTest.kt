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

class DataInputStreamTest {
    @Test
    fun test() {
        (ByteArray(1 + 2 + 4 + 8) { (-1).toByte() }
        ).inputStream().data().apply {
            assertEquals(UByte.MAX_VALUE, readUByte())
            assertEquals(UShort.MAX_VALUE, readUShort())
            assertEquals(UInt.MAX_VALUE, readUInt())
            assertEquals(ULong.MAX_VALUE, readULong())
        }.close()
    }
}
