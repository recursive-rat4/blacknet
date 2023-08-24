/*
 * Copyright (c) 2023 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.codec.base

import kotlin.test.Test
import kotlin.test.assertEquals

class Base64Test {
    private val bytes = byteArrayOf(0, 1, 2, 3)
    private val encoded = "AAECAw=="

    @Test
    fun encode() {
        assertEquals(
            encoded,
            Base64.encode(bytes)
        )
    }

    @Test
    fun decode() {
        assertEquals(
            bytes,
            Base64.decode(encoded)
        )
    }
}
