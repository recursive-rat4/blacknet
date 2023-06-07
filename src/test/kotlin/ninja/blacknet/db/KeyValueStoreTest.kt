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
import kotlin.test.assertFalse
import kotlin.test.assertNull

class KeyValueStoreTest {
    @Test
    fun test() {
        val store = MemDB()

        val key0 = byteArrayOf(0)
        val key1 = byteArrayOf(1)
        val key2 = byteArrayOf(2)

        assertFalse(store.contains(key0))
        assertFalse(store.contains(key1))
        assertFalse(store.contains(key2))

        assertNull(store.get(key0))
        assertNull(store.get(key1))
        assertNull(store.get(key2))
    }
}
