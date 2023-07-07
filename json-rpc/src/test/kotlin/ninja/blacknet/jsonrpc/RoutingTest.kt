/*
 * Copyright (c) 2023 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.jsonrpc

import kotlinx.serialization.Serializable
import kotlinx.serialization.json.Json
import kotlin.test.Test
import kotlin.test.assertEquals

class RoutingTest {
    @Test
    fun test() {
        val routing = Routing()
        routing.add("subtract", Subtract.serializer())

        val request1Json = """{"jsonrpc": "2.0", "method": "subtract", "params": [42, 23], "id": 1}"""
        val request2Json = """{"jsonrpc": "2.0", "method": "subtract", "params": [23, 42], "id": 2}"""
        val request3Json = """{"jsonrpc": "2.0", "method": "subtract", "params": {"subtrahend": 23, "minuend": 42}, "id": 3}"""
        val request4Json = """{"jsonrpc": "2.0", "method": "subtract", "params": {"minuend": 42, "subtrahend": 23}, "id": 4}"""

        val request1 = Json.decodeFromString(Request.serializer(), request1Json)
        val request2 = Json.decodeFromString(Request.serializer(), request2Json)
        val request3 = Json.decodeFromString(Request.serializer(), request3Json)
        val request4 = Json.decodeFromString(Request.serializer(), request4Json)

        //FIXME positional parameters
        // assertEquals(19, routing.handle(request1))
        // assertEquals(-19, routing.handle(request2))
        assertEquals(19, routing.handle(request3))
        assertEquals(19, routing.handle(request4))
    }

    @Serializable
    private class Subtract(
        private val minuend: Int,
        private val subtrahend: Int
    ) : Handler<Int> {
        override fun handle(): Int {
            return minuend - subtrahend
        }
    }
}
