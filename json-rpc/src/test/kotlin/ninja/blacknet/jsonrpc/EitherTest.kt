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
import kotlin.test.assertIs
import kotlin.test.assertIsNot

class EitherTest {
    @Test
    fun test() {
        val l = Left(21)
        val r = Right(42)
        assertIs<E>(l)
        assertIs<E>(r)
        assertIsNot<R>(l)
        assertIsNot<L>(r)
    }
}

typealias E = Either<Int, Int>
typealias L = Left<Int>
typealias R = Right<Int>
