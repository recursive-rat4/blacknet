/*
 * Copyright (c) 2023 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.util

import kotlin.test.Test
import kotlin.test.assertFailsWith

class ValidationsTest {
    @Test
    fun test() {
        class CustomException(iCanHaz: Boolean) : Throwable() {
            init {
                if (iCanHaz)
                    Unit
                else
                    throw Error("Constructor was called when it must not")
            }
        }

        check(true) { CustomException(false) }
        assertFailsWith(CustomException::class) { check(false) { CustomException(true) } }
    }
}
