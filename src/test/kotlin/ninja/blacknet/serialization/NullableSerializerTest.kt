/*
 * Copyright (c) 2021 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.serialization

import kotlinx.serialization.Serializable
import ninja.blacknet.serialization.bbf.binaryFormat
import ninja.blacknet.util.plus
import kotlin.test.Test
import kotlin.test.assertEquals

class NullableSerializerTest {
    private val byteArray = ByteArray(16) { it.toByte() }
    private val binaryEncoded = 1.toByte() + (144.toByte() + byteArray)

    @Test
    fun binaryNullableEncoder() {
        @Serializable
        class Structure(@Serializable(with = ByteArraySerializer::class) val maybeByteArray: ByteArray?)
        assertEquals(
                binaryEncoded,
                binaryFormat.encodeToByteArray(Structure.serializer(), Structure(byteArray))
        )
    }
}
