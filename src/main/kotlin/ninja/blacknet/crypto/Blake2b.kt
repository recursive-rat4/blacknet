/*
 * Copyright (c) 2018 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.crypto

import com.rfksystems.blake2b.Blake2b
import com.rfksystems.blake2b.security.Blake2bProvider
import java.security.Security

object Blake2b {
    const val BLAKE2_B_512 = Blake2b.BLAKE2_B_512

    init {
        Security.addProvider(Blake2bProvider())
    }

    class Hash(val bytes: ByteArray)

    fun hash(message: ByteArray): Hash {
        val bytes = ByteArray(32)
        val b = Blake2b(256)
        b.update(message, 0, message.size)
        b.digest(bytes, 0)
        return Hash(bytes)
    }
}