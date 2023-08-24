/*
 * Copyright (c) 2019-2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.codec.base

import java.util.Base64

//XXX inline?

object Base64 {
    private val encoder = Base64.getEncoder()
    private val decoder = Base64.getDecoder()

    fun encode(bytes: ByteArray): String {
        return encoder.encodeToString(bytes)
    }

    fun decode(string: String): ByteArray {
        return decoder.decode(string)
    }
}
