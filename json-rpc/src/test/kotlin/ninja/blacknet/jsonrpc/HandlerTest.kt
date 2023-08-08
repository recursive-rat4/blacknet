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
import kotlin.test.assertEquals
import kotlin.test.assertFailsWith
import kotlin.test.assertIs

class HandlerTest {
    @Test
    fun right() {
        val string = "Pong"
        val ping = object : Handler<String> {
            override fun handle() = Right(string)
        }
        val pong = ping.handle()
        assertIs<Right<String>>(pong)
        assertEquals(string, pong.right)
    }

    @Test
    fun left() {
        val ping = object : Handler<String> {
            override fun handle() = Left(Error.of(0, "Error success"))
        }
        val pong = ping.handle()
        assertIs<Left<Error>>(pong)
    }

    @Test
    fun exception() {
        class CustomException : RuntimeException()
        val erroneous = object : Handler<String> {
            override fun handle() = throw CustomException()
        }
        assertFailsWith<CustomException> { erroneous.handle() }
    }
}
