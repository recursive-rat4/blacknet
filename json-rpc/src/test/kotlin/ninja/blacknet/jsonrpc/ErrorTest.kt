/*
 * Copyright (c) 2023 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.jsonrpc

import kotlinx.serialization.json.Json
import kotlinx.serialization.json.JsonPrimitive
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertFails

class ErrorTest {
    @Test
    fun serialization() {
        val parseError = Error.parseError()
        val applicationError = Error.of(-31000, "Something went wrong", JsonPrimitive("Report this to developers"))
        val parseErrorJson = """{"code":-32700,"message":"Parse error"}"""
        val applicationErrorJson = """{"code":-31000,"message":"Something went wrong","data":"Report this to developers"}"""
        val badJson = """{"code":0.777,"message":"Custom error"}"""

        assertEquals(parseErrorJson, Json.encodeToString(Error.serializer(), parseError))
        assertEquals(applicationErrorJson, Json.encodeToString(Error.serializer(), applicationError))
        Json.decodeFromString(Error.serializer(), parseErrorJson)
        Json.decodeFromString(Error.serializer(), applicationErrorJson)
        assertFails { Json.decodeFromString(Error.serializer(), badJson) }
    }

    @Test
    fun code() {
        Error.parseError()
        Error.invalidRequest()
        Error.methodNotFound()
        Error.invalidParams()
        Error.internalError()

        assertFails { Error.of(-32099, "Server error") }
    }
}
