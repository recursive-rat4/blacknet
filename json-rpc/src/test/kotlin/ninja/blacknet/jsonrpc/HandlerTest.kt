/*
 * Copyright (c) 2023 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.jsonrpc

import kotlin.test.Test

class HandlerTest {
    @Test
    fun test() {
        val ping = object : Handler<String> {
            override fun handle(): String = "Pong"
        }
        ping.handle()
    }
}
