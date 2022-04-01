/*
 * Copyright (c) 2022 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.util

import kotlin.test.Test
import kotlin.test.assertEquals

class ByteArrayTest {
    @Test
    fun empty() {
        assertEquals(emptyByteArray(), ByteArray(0))
    }

    @Test
    fun ints() {
        assertEquals(byteArrayOfInts(), ByteArray(0))
        assertEquals(byteArrayOfInts(1, 2, 3, 4), ByteArray(4) { (it + 1).toByte() } )
    }

    @Test
    fun plus() {
        val a = ByteArray(4) { (it + 0).toByte() }
        val b = ByteArray(4) { (it + 4).toByte() }
        assertEquals(a + b, ByteArray(8) { it.toByte() })
    }
}
