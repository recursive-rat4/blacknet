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

class ParamsTest {
    @Test
    fun test() {
        val arrayJson = "[1,2,3]"
        val objectJson = """{"foo":"bar"}"""
        val badJson = "\"baz\""

        Json.decodeFromString(Params.serializer(), arrayJson)
        Json.decodeFromString(Params.serializer(), objectJson)
        assertFails { Json.decodeFromString(Params.serializer(), badJson) }
    }
}
