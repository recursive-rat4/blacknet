/*
 * Copyright (c) 2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.serialization

import ninja.blacknet.util.byteArrayOfInts
import org.testng.Assert.assertEquals
import org.testng.annotations.Test

class BinaryDecoderTest {
    @Test
    fun test() {
        assertEquals(BinaryDecoder(byteArrayOf(0)).decodeByte(), 0)
        assertEquals(BinaryDecoder(byteArrayOf(1, -1)).decodeShort(), 0x01FF)
        assertEquals(BinaryDecoder(byteArrayOf(2, 1, -1, -2)).decodeInt(), 0x0201FFFE)
        assertEquals(BinaryDecoder(byteArrayOf(4, 3, 2, 1, -1, -2, -3, -4)).decodeLong(), 0x04030201FFFEFDFC)
        assertEquals(BinaryDecoder(ByteArray(9)).decodeFixedByteArray(9), ByteArray(9))
        assertEquals(BinaryDecoder(byteArrayOfInts(0x83, 0xE5, 0x85, 0xAB)).decodeString(), "八")
    }
}
