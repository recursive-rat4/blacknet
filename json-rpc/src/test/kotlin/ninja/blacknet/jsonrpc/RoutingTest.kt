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
import kotlin.test.assertIs

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

        val result1 = routing.handle<Int>(request1)
        val result2 = routing.handle<Int>(request2)
        val result3 = routing.handle<Int>(request3)
        val result4 = routing.handle<Int>(request4)

        assertIs<Right<Int>>(result1)
        assertIs<Right<Int>>(result2)
        assertIs<Right<Int>>(result3)
        assertIs<Right<Int>>(result4)

        assertEquals(19, result1.right)
        assertEquals(-19, result2.right)
        assertEquals(19, result3.right)
        assertEquals(19, result4.right)
    }

    @Serializable
    private class Subtract(
        private val minuend: Int,
        private val subtrahend: Int
    ) : Handler<Int> {
        override fun handle() = Right(minuend - subtrahend)
    }
}
