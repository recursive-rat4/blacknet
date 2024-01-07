/*
 * Copyright (c) 2023 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.db

import java.util.HashMap.newHashMap

/**
 * A simple implementation of [KeyValueStore] that keeps all its data in the main memory.
 */
class MemDB private constructor(
    private val map: HashMap<Wrapper, Wrapper> = HashMap(0)
) : KeyValueStore {
    override fun contains(key: ByteArray): Boolean {
        return map.containsKey(Wrapper(key))
    }

    override fun get(key: ByteArray): ByteArray? {
        return map.get(Wrapper(key))?.bytes
    }

    companion object {
        fun memDBOf(vararg content: Pair<ByteArray, ByteArray>) = MemDB(
            newHashMap(content.size)
        ).apply {
            content.forEach { (k, v) ->
                map.put(Wrapper(k), Wrapper(v))
            }
        }
    }

    private class Wrapper(
        internal val bytes: ByteArray
    ) {
        override fun equals(other: Any?): Boolean {
            return other is Wrapper && bytes.contentEquals(other.bytes)
        }

        override fun hashCode(): Int {
            return bytes.contentHashCode()
        }
    }
}
