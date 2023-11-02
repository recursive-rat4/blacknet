/*
 * Copyright (c) 2020-2023 Pavel Vasin
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
import kotlinx.serialization.Serializable
import ninja.blacknet.util.byteArrayOfInts

class BinaryFormatTest {
    @Serializable
    private data class Structure(
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
    private inline class InlineClass(
        val int: Int,
    )

    private val structure = Structure(
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

    private val bytes = byteArrayOfInts(
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
}
