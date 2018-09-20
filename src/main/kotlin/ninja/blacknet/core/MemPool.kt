/*
 * Copyright (c) 2018 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.core

import ninja.blacknet.crypto.Hash

abstract class MemPool : DataDB() {
    private val map = HashMap<Hash, ByteArray>()

    protected fun add(hash: Hash, bytes: ByteArray) {
        map[hash] = bytes
    }

    override fun contains(hash: Hash): Boolean {
        return map.containsKey(hash)
    }

    override fun get(hash: Hash): ByteArray? {
        return map[hash]
    }

    override fun remove(hash: Hash): ByteArray? {
        return map.remove(hash)
    }
}