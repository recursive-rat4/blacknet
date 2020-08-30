/*
 * Copyright (c) 2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.rpc.requests

import io.ktor.http.Parameters

class RequestReader(
        private val input: Parameters
) {
    fun hasKey(key: String): Boolean {
        return input.contains(key)
    }

    fun readString(key: String): String {
        return input[key]!!.trim()
    }
}
