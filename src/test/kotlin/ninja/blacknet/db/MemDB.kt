/*
 * Copyright (c) 2023 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.db

import ninja.blacknet.util.HashMap

/**
 * A simple implementation of [KeyValueStore] that keeps all its data in the main memory.
 */
class MemDB(
    private val map: HashMap<ByteArray, ByteArray> = HashMap(0)
) : KeyValueStore {
    override fun contains(key: ByteArray): Boolean {
        return map.containsKey(key)
    }

    override fun get(key: ByteArray): ByteArray? {
        return map.get(key)
    }
}
