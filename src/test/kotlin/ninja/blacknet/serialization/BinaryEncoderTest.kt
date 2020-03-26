/*
 * Copyright (c) 2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.serialization

import kotlinx.serialization.Serializable
import ninja.blacknet.util.byteArrayOfInts
import org.testng.Assert.assertEquals
import org.testng.annotations.Test

class BinaryEncoderTest {
    private val encoder = BinaryEncoder()

    @Test
    fun element() {
        encoder.encodeByte(0)
        assertEquals(encoder.toBytes(), byteArrayOf(0))

        encoder.encodeShort(0x01FF)
        assertEquals(encoder.toBytes(), byteArrayOf(1, -1))

        encoder.encodeInt(0x0201FFFE)
        assertEquals(encoder.toBytes(), byteArrayOf(2, 1, -1, -2))

        encoder.encodeLong(0x04030201FFFEFDFC)
        assertEquals(encoder.toBytes(), byteArrayOf(4, 3, 2, 1, -1, -2, -3, -4))

        encoder.encodeFixedByteArray(ByteArray(9))
        assertEquals(encoder.toBytes(), ByteArray(9))

        encoder.encodeString("八")
        assertEquals(encoder.toBytes(), byteArrayOfInts(0x83, 0xE5, 0x85, 0xAB))
    }

    @Test
    fun structure() {
        @Serializable
        class Structure(
                val a: Byte,
                val b: Short,
                val c: Int,
                val d: Long,
                val e: Unit,
                val f: String
        )

        val value = Structure(
                0,
                0x01FF,
                0x0201FFFE,
                0x04030201FFFEFDFC,
                Unit,
                "八"
        )

        Structure.serializer().serialize(encoder, value)

        assertEquals(encoder.toBytes(), byteArrayOfInts(
                0,
                1, -1,
                2, 1, -1, -2,
                4, 3, 2, 1, -1, -2, -3, -4,
                // Unit //
                0x83, 0xE5, 0x85, 0xAB
        ))
    }
}
