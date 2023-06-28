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
import kotlin.test.Test
import kotlin.test.assertFails

class ErrorTest {
    @Test
    fun test() {
        val goodJson1 = """{"code":-32700,"message":"Parse error"}"""
        val goodJson2 = """{"code":-32099,"message":"Something went wrong","data":"Report this to developers"}"""
        val badJson = """{"code":0.777,"message":"Custom error"}"""

        Json.decodeFromString(Error.serializer(), goodJson1)
        Json.decodeFromString(Error.serializer(), goodJson2)
        assertFails { Json.decodeFromString(Error.serializer(), badJson) }
    }
}
