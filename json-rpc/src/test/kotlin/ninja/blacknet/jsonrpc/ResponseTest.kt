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

class ResponseTest {
    @Test
    fun test() {
        val successJson = """{"jsonrpc":"2.0","result":19,"id":1}"""
        val errorJson = """{"jsonrpc":"2.0","error":{"code":-32600,"message":"Invalid Request"},"id":null}"""
        val badJson1 = """{"jsonrpc":"2.0","result":19,"error":{"code":-32600,"message":"Invalid Request"},"id":1}"""
        val badJson2 = """{"jsonrpc":"2.0","id":1}"""

        Json.decodeFromString(Response.serializer(), successJson)
        Json.decodeFromString(Response.serializer(), errorJson)
        assertFails { Json.decodeFromString(Response.serializer(), badJson1) }
        assertFails { Json.decodeFromString(Response.serializer(), badJson2) }
    }
}
