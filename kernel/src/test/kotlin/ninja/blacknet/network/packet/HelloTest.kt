/*
 * Copyright (c) 2023 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.network.packet

import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertNull
import ninja.blacknet.serialization.bbf.binaryFormat
import ninja.blacknet.util.byteArrayOfInts

class HelloTest {
    val empty = Hello()
    val emptyBytes = byteArrayOfInts(128)
    val hello = Hello().apply {
        magic = 1
        version = 2
        nonce = 3
        agent = "4"
        feeFilter = 5
    }
    val helloBytes = byteArrayOfInts(
        128 + 5,
        128, 128 + 4, 0, 0, 0, 1,
        129, 128 + 4, 0, 0, 0, 2,
        130, 128 + 8, 0, 0, 0, 0, 0, 0, 0, 3,
        131, 128 + 2, 128 + 1, '4'.code,
        132, 128 + 8, 0, 0, 0, 0, 0, 0, 0, 5,
    )

    @Test
    fun decode() {
        binaryFormat.decodeFromByteArray(Hello.serializer(), emptyBytes).also {
            assertNull(it.magic)
            assertNull(it.version)
            assertNull(it.nonce)
            assertNull(it.agent)
            assertNull(it.feeFilter)
        }
        binaryFormat.decodeFromByteArray(Hello.serializer(), helloBytes).also {
            assertEquals(1, it.magic)
            assertEquals(2, it.version)
            assertEquals(3, it.nonce)
            assertEquals("4", it.agent)
            assertEquals(5, it.feeFilter)
        }
    }

    @Test
    fun encode() {
        assertEquals(
            emptyBytes,
            binaryFormat.encodeToByteArray(Hello.serializer(), empty)
        )
        // field order isn't fixed
        assertEquals(
            helloBytes.size,
            binaryFormat.encodeToByteArray(Hello.serializer(), hello).size
        )
    }
}
