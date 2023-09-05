/*
 * Copyright (c) 2023 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.db

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFalse
import kotlin.test.assertTrue
import kotlin.test.assertNull
import kotlinx.serialization.builtins.serializer
import ninja.blacknet.serialization.bbf.BinaryFormat
import ninja.blacknet.util.hashMapOf
import ninja.blacknet.util.plus
import ninja.blacknet.util.toByteArray

class DBViewTest {
    private val prefix = 0.toByte()
    private val key0 = byteArrayOf(0, 0)
    private val key1 = byteArrayOf(0, 1)
    private val key2 = byteArrayOf(0, 2)
    private val value0 = 0
    private val value1 = 1
    private val valueBytes0 = value0.toByteArray()
    private val valueBytes1 = value1.toByteArray()

    private val view = DBView(
        MemDB(
            hashMapOf(
                prefix + key0 to valueBytes0,
                prefix + key1 to valueBytes1,
            )
        ),
        DBKey(prefix, 2),
        Int.serializer(),
        BinaryFormat()
    )

    @Test
    fun contains() {
        assertTrue(view.contains(key0))
        assertFalse(view.contains(key2))
    }

    @Test
    fun get() {
        assertEquals(value0, view.get(key0))
        assertNull(view.get(key2))
    }

    @Test
    fun getWithSize() {
        assertEquals(Pair(value0, valueBytes0.size), view.getWithSize(key0))
        assertNull(view.getWithSize(key2))
    }

    @Test
    fun getBytes() {
        assertEquals(valueBytes0, view.getBytes(key0))
        assertNull(view.getBytes(key2))
    }
}
