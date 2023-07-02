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
import kotlin.test.assertEquals
import kotlin.test.assertFails

class MethodTest {
    @Test
    fun test() {
        val goodName = "foo"
        val goodJson = "\"$goodName\""
        val badName = "rpc.foo"
        val badJson = "\"$badName\""

        /*
        assertEquals(goodJson, Json.encodeToString(Method.serializer(), Method(goodName)))
        assertEquals(Method(goodName), Json.decodeFromString(Method.serializer(), goodJson))
        assertFails { Json.encodeToString(Method.serializer(), Method(badName)) }
        */

        Json.decodeFromString(Method.serializer(), goodJson)
        assertFails { Json.decodeFromString(Method.serializer(), badJson) }
    }
}
