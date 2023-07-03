/*
 * Copyright (c) 2023 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.jsonrpc

import kotlinx.serialization.json.JsonPrimitive
import kotlin.test.Test

class ExceptionsTest {
    @Test
    fun test() {
        ParseError()
        InvalidRequest()
        MethodNotFound()
        InvalidParams()
        InternalError()
        InternalError()
        Exception(
            -31000,
            "Replace by fee is not implemented yet",
            JsonPrimitive("See https://gitlab.com/blacknet-ninja/blacknet/-/issues/63")
        )
    }
}
