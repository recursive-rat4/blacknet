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

class DataOutputStreamTest {
    @Test
    fun test() {
        val size = 1 + 2 + 1 + 2 + 4 + 8
        assertEquals(
            ByteArray(size) { (-1).toByte() },
            ByteArray(size).apply {
                outputStream().data().apply {
                    writeByte((-1).toByte())
                    writeShort((-1).toShort())
                    writeUByte(UByte.MAX_VALUE)
                    writeUShort(UShort.MAX_VALUE)
                    writeUInt(UInt.MAX_VALUE)
                    writeULong(ULong.MAX_VALUE)
                }.close()
            }
        )
    }
}
