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
import kotlin.test.assertFalse
import kotlin.test.assertTrue

class RequestTest {
    @Test
    fun test() {
        val positionalParametersJson = """{"jsonrpc":"2.0","method":"subtract","params":[42,23],"id":1}"""
        val namedParametersJson = """{"jsonrpc":"2.0","method":"subtract","params":{"subtrahend":23,"minuend":42},"id":3}"""
        val notificationJson = """{"jsonrpc":"2.0","method":"update","params":[1,2,3,4,5]}"""
        val badJson1 = """{"jsonrpc":"2.0","method":"foobar,"params":"bar","baz]"""
        val badJson2 = """{"jsonrpc":"2.0","method":1,"params":"bar"}"""

        assertFalse(Json.decodeFromString(Request.serializer(), positionalParametersJson).isNotification())
        assertFalse(Json.decodeFromString(Request.serializer(), namedParametersJson).isNotification())
        assertTrue(Json.decodeFromString(Request.serializer(), notificationJson).isNotification())
        assertFails { Json.decodeFromString(Request.serializer(), badJson1) }
        assertFails { Json.decodeFromString(Request.serializer(), badJson2) }
    }
}
