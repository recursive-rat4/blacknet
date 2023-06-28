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

class IdTest {
    @Test
    fun test() {
        val string = "a"
        val stringJson = "\"$string\""
        val number = 1
        val numberJson = "$number"
        val nil = null
        val nullJson = "$nil"
        val badJson = "true"

        assertEquals(stringJson, Json.encodeToString(Id.serializer(), Id(string)))
        assertEquals(Id(string), Json.decodeFromString(Id.serializer(), stringJson))
        assertEquals(numberJson, Json.encodeToString(Id.serializer(), Id(number)))
        assertEquals(Id(number), Json.decodeFromString(Id.serializer(), numberJson))
        assertEquals(nullJson, Json.encodeToString(Id.serializer(), Id()))
        assertEquals(Id(), Json.decodeFromString(Id.serializer(), nullJson))
        assertFails { Json.decodeFromString(Id.serializer(), badJson) }
    }
}
