/*
 * Copyright (c) 2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.db

import ninja.blacknet.util.plus

class DBKey(
        val prefix: Byte,
        val keyLength: Int
) {
    operator fun unaryPlus(): ByteArray {
        require(keyLength == 0)
        return byteArrayOf(prefix)
    }

    operator fun plus(key: ByteArray): ByteArray {
        require(keyLength == key.size)
        return prefix + key
    }

    operator fun rem(entry: Map.Entry<ByteArray, *>): Boolean {
        return Byte.SIZE_BYTES + keyLength == entry.key.size && entry.key[0] == prefix
    }

    operator fun minus(entry: Map.Entry<ByteArray, *>): ByteArray? {
        return if (this % entry) {
            val bytes = ByteArray(keyLength)
            System.arraycopy(entry.key, Byte.SIZE_BYTES, bytes, 0, keyLength)
            bytes
        } else {
            null
        }
    }
}
