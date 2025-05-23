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

class VersionTest {
    @Test
    fun test() {
        val good = """"2.0""""
        val bad = """"1.0""""

        assertEquals(good, Json.encodeToString(Version.serializer(), Version.DEFAULT))
        assertEquals(Version.DEFAULT, Json.decodeFromString(Version.serializer(), good))
        assertFails { Json.decodeFromString(Version.serializer(), bad) }
    }
}
