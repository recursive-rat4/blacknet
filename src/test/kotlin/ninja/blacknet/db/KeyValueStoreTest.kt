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
import ninja.blacknet.util.hashMapOf

class KeyValueStoreTest {
    @Test
    fun test() {
        val key0 = byteArrayOf(0)
        val key1 = byteArrayOf(1)
        val key2 = byteArrayOf(2)
        val key3 = byteArrayOf(3)

        val value0 = byteArrayOf(0)
        val value1 = byteArrayOf(0)
        val value2 = byteArrayOf(2)

        val store = MemDB(
            hashMapOf(
                key0 to value0,
                key1 to value1,
                key2 to value2,
            )
        )

        assertTrue(store.contains(key0))
        assertTrue(store.contains(key1))
        assertTrue(store.contains(key2))
        assertFalse(store.contains(key3))

        assertEquals(value0, store.get(key0))
        assertEquals(value1, store.get(key1))
        assertEquals(value2, store.get(key2))
        assertNull(store.get(key3))
    }
}
