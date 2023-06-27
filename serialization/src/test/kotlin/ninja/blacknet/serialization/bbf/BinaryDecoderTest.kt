/*
 * Copyright (c) 2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.serialization.bbf

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlinx.serialization.Serializable
import kotlinx.serialization.builtins.ListSerializer
import kotlinx.serialization.builtins.serializer
import ninja.blacknet.util.byteArrayOfInts

class BinaryDecoderTest {
    @Test
    fun element() {
        assertEquals(false, BinaryDecoder(byteArrayOf(0)).decodeBoolean())
        assertEquals(0, BinaryDecoder(byteArrayOf(0)).decodeByte())
        assertEquals(0x01FF, BinaryDecoder(byteArrayOf(1, -1)).decodeShort())
        assertEquals(0x0201FFFE, BinaryDecoder(byteArrayOf(2, 1, -1, -2)).decodeInt())
        assertEquals(0x04030201FFFEFDFC, BinaryDecoder(byteArrayOf(4, 3, 2, 1, -1, -2, -3, -4)).decodeLong())
        assertEquals(ByteArray(9), BinaryDecoder(ByteArray(9)).decodeFixedByteArray(9))
        assertEquals("八", BinaryDecoder(byteArrayOfInts(0x83, 0xE5, 0x85, 0xAB)).decodeString())
        assertEquals(4, BinaryDecoder(byteArrayOfInts(0x84)).decodeCollectionSize(ListSerializer(Byte.serializer()).descriptor))
    }

    @Serializable
    data class Structure(
        val boolean: Boolean,
        val byte: Byte,
        val short: Short,
        val int: Int,
        val long: Long,
        val unit: Unit,
        val string: String,
        val inline: InlineClass,
        val list: List<Byte>,
    )

    @Serializable
    inline class InlineClass(
        val int: Int,
    )

    @Test
    fun structure() {
        assertEquals(
            Structure(
                false,
                0,
                0x01FF,
                0x0201FFFE,
                0x04030201FFFEFDFC,
                Unit,
                "八",
                InlineClass(0x0201FFFE),
                listOf(1, 2, 3, 4),
            ),
            BinaryFormat().decodeFromByteArray(
                Structure.serializer(), byteArrayOfInts(
                    0,
                    0,
                    1, -1,
                    2, 1, -1, -2,
                    4, 3, 2, 1, -1, -2, -3, -4,
                    // Unit //
                    0x83, 0xE5, 0x85, 0xAB,
                    2, 1, -1, -2,
                    0x84, 0x01, 0x02, 0x03, 0x04,
                )
            )
        )
    }
}
