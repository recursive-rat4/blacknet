/*
 * Copyright (c) 2020-2024 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

@file:Suppress("INLINE_CLASS_DEPRECATED")

package ninja.blacknet.serialization.bbf

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFails
import kotlinx.serialization.Serializable
import kotlinx.serialization.builtins.DoubleArraySerializer
import kotlinx.serialization.builtins.FloatArraySerializer
import kotlinx.serialization.builtins.nullable
import kotlinx.serialization.builtins.serializer
import ninja.blacknet.util.byteArrayOfInts

class BinaryFormatTest {
    @Serializable
    private data class Structure(
        val boolean1: Boolean,
        val boolean2: Boolean,
        val byte: Byte,
        val short: Short,
        val int: Int,
        val long: Long,
        val nullable1: Unit?,
        val nullable2: Unit?,
        val string: String,
        val inline: InlineClass,
        val list: List<Byte>,
    )

    @Serializable
    private inline class InlineClass(
        val int: Int,
    )

    private val structure = Structure(
        false,
        true,
        0,
        0x01FF,
        0x0201FFFE,
        0x04030201FFFEFDFC,
        null,
        Unit,
        "八",
        InlineClass(0x0201FFFE),
        listOf(1, 2, 3, 4),
    )

    private val bytes = byteArrayOfInts(
        0,
        1,
        0,
        1, -1,
        2, 1, -1, -2,
        4, 3, 2, 1, -1, -2, -3, -4,
        0,
        1, // Unit //
        0x83, 0xE5, 0x85, 0xAB,
        2, 1, -1, -2,
        0x84, 0x01, 0x02, 0x03, 0x04,
    )

    private val format = BinaryFormat()

    @Test
    fun decode() {
        assertEquals(
            structure,
            format.decodeFromByteArray(Structure.serializer(), bytes)
        )
    }

    @Test
    fun encode() {
        assertEquals(
            bytes,
            format.encodeToByteArray(Structure.serializer(), structure)
        )
    }

    @Test
    fun computeSize() {
        assertEquals(
            bytes.size,
            format.computeSize(Structure.serializer(), structure)
        )
    }

    @Test
    fun invalidMark() {
        val bytes = ByteArray(1) { 2 }
        assertFails { format.decodeFromByteArray(Boolean.serializer(), bytes) }
        assertFails { format.decodeFromByteArray(Unit.serializer().nullable, bytes) }
    }

    @Test
    fun invalidSize() {
        assertFails { format.decodeFromByteArray(Structure.serializer(), bytes.copyOf(bytes.size - 1)) }
        assertFails { format.decodeFromByteArray(Structure.serializer(), bytes.plus(1)) }
    }

    @Test
    fun float() {
        val bytes = byteArrayOfInts(
            128 + 7,
            0x40, 0xA0, 0x00, 0x00,
            0xC0, 0xA0, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x80, 0x00, 0x00, 0x00,
            0x7F, 0x80, 0x00, 0x00,
            0xFF, 0x80, 0x00, 0x00,
            0x7F, 0xC0, 0x00, 0x00,
        )
        val floats = floatArrayOf(
            +5.0f,
            -5.0f,
            +0.0f,
            -0.0f,
            Float.POSITIVE_INFINITY,
            Float.NEGATIVE_INFINITY,
            Float.NaN,
        )
        assertEquals(
            floats,
            format.decodeFromByteArray(FloatArraySerializer(), bytes)
        )
        assertEquals(
            bytes,
            format.encodeToByteArray(FloatArraySerializer(), floats)
        )
        assertEquals(
            bytes.size,
            format.computeSize(FloatArraySerializer(), floats)
        )
    }

    @Test
    fun double() {
        val bytes = byteArrayOfInts(
            128 + 7,
            0x40, 0x14, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0xC0, 0x14, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x7F, 0xF0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0xFF, 0xF0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x7F, 0xF8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        )
        val doubles = doubleArrayOf(
            +5.0,
            -5.0,
            +0.0,
            -0.0,
            Double.POSITIVE_INFINITY,
            Double.NEGATIVE_INFINITY,
            Double.NaN,
        )
        assertEquals(
            doubles,
            format.decodeFromByteArray(DoubleArraySerializer(), bytes)
        )
        assertEquals(
            bytes,
            format.encodeToByteArray(DoubleArraySerializer(), doubles)
        )
        assertEquals(
            bytes.size,
            format.computeSize(DoubleArraySerializer(), doubles)
        )
    }
}
