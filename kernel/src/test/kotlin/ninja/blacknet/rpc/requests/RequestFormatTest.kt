/*
 * Copyright (c) 2024 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.rpc.requests

import io.ktor.http.parametersOf
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFails
import kotlinx.serialization.Serializable

class RequestFormatTest {
    private val format = RequestFormat()

    @Serializable
    data class Request(
        val byte: Byte,
        val ubyte: UByte,
    )

    @Test
    fun decode() {
        assertEquals(
            Request(
                -1,
                200u,
            ),
            format.decodeFromParameters(Request.serializer(), parametersOf(
                "byte" to listOf("-1"),
                "ubyte" to listOf("200"),
            ))
        )
        assertFails {
            format.decodeFromParameters(Request.serializer(), parametersOf(
                "byte" to listOf("200"),
                "ubyte" to listOf("200"),
            ))
        }
        assertFails {
            format.decodeFromParameters(Request.serializer(), parametersOf(
                "byte" to listOf("-1"),
                "ubyte" to listOf("-1"),
            ))
        }
    }
}
