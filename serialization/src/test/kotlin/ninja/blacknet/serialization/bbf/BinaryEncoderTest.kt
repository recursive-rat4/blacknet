/*
 * Copyright (c) 2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

@file:Suppress("INLINE_CLASS_DEPRECATED")

package ninja.blacknet.serialization.bbf

import io.ktor.utils.io.core.readBytes
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlinx.serialization.Serializable
import kotlinx.serialization.builtins.ListSerializer
import kotlinx.serialization.builtins.serializer
import ninja.blacknet.util.byteArrayOfInts

class BinaryEncoderTest {
    private val encoder = BinaryEncoder()

    @Test
    fun element() {
        encoder.encodeBoolean(false)
        assertEquals(byteArrayOf(0), encoder.toBytes())

        encoder.encodeByte(0)
        assertEquals(byteArrayOf(0), encoder.toBytes())

        encoder.encodeShort(0x01FF)
        assertEquals(byteArrayOf(1, -1), encoder.toBytes())

        encoder.encodeInt(0x0201FFFE)
        assertEquals(byteArrayOf(2, 1, -1, -2), encoder.toBytes())

        encoder.encodeLong(0x04030201FFFEFDFC)
        assertEquals(byteArrayOf(4, 3, 2, 1, -1, -2, -3, -4), encoder.toBytes())

        encoder.encodeFixedByteArray(ByteArray(9))
        assertEquals(ByteArray(9), encoder.toBytes())

        encoder.encodeString("八")
        assertEquals(byteArrayOfInts(0x83, 0xE5, 0x85, 0xAB), encoder.toBytes())

        encoder.beginCollection(ListSerializer(Byte.serializer()).descriptor, 4)
        assertEquals(byteArrayOfInts(0x84), encoder.toBytes())
    }

    @Serializable
    class Structure(
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
        val value = Structure(
            false,
            0,
            0x01FF,
            0x0201FFFE,
            0x04030201FFFEFDFC,
            Unit,
            "八",
            InlineClass(0x0201FFFE),
            listOf(1, 2, 3, 4),
        )

        Structure.serializer().serialize(encoder, value)

        assertEquals(
            byteArrayOfInts(
                0,
                0,
                1, -1,
                2, 1, -1, -2,
                4, 3, 2, 1, -1, -2, -3, -4,
                // Unit //
                0x83, 0xE5, 0x85, 0xAB,
                2, 1, -1, -2,
                0x84, 0x01, 0x02, 0x03, 0x04,
            ),
            encoder.toBytes()
        )
    }

    private fun BinaryEncoder.toBytes(): ByteArray {
        return output.build().readBytes()
    }
}
