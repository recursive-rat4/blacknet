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

abstract class DataDB {
    private val rejects = HashSet<Hash>()

    fun clearRejects() {
        rejects.clear()
    }

    fun isInteresting(hash: Hash): Boolean {
        return !rejects.contains(hash) && !contains(hash)
    }

    fun process(hash: Hash, bytes: ByteArray): Boolean {
        if (!isInteresting(hash))
            return false
        if (!processImpl(hash, bytes)) {
            rejects.add(hash)
            return false
        }
        return true
    }

    abstract fun contains(hash: Hash): Boolean
    abstract fun get(hash: Hash): ByteArray?
    abstract fun remove(hash: Hash): ByteArray?
    abstract fun processImpl(hash: Hash, bytes: ByteArray): Boolean
}