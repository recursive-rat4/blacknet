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
import kotlin.test.assertFails
import kotlin.test.assertFalse
import kotlin.test.assertNull
import kotlin.test.assertTrue
import ninja.blacknet.util.plus

class DBKeyTest {
    val prefix0 = 4.toByte()
    val key0 = DBKey(prefix0, 0)
    val bytes0 = ByteArray(0)
    val entry0 = object : Map.Entry<ByteArray, Any> {
        override val key = prefix0 + bytes0
        override val value = Any()
    }
    val prefix4 = 0.toByte()
    val key4 = DBKey(prefix4, 4)
    val bytes4 = ByteArray(4)
    val entry4 = object : Map.Entry<ByteArray, Any> {
        override val key = prefix4 + bytes4
        override val value = Any()
    }

    @Test
    fun unaryPlus() {
        assertEquals(byteArrayOf(prefix0), +key0)
        assertFails { +key4 }
    }

    @Test
    fun plus() {
        assertEquals(byteArrayOf(prefix0), key0 + bytes0)
        assertEquals(prefix4 + bytes4, key4 + bytes4)
        assertFails { key0 + bytes4 }
        assertFails { key4 + bytes0 }
    }

    @Test
    fun rem() {
        assertTrue(key0 % entry0)
        assertTrue(key4 % entry4)
        assertFalse(key0 % entry4)
        assertFalse(key4 % entry0)
    }

    @Test
    fun minus() {
        assertEquals(bytes0, key0 - entry0)
        assertEquals(bytes4, key4 - entry4)
        assertNull(key0 - entry4)
        assertNull(key4 - entry0)
    }
}
