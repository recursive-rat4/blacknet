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
import ninja.blacknet.network.Connection
import ninja.blacknet.util.SynchronizedHashSet

abstract class DataDB {
    private val rejects = SynchronizedHashSet<Hash>()

    suspend fun clearRejects() {
        rejects.clear()
    }

    suspend fun isInteresting(hash: Hash): Boolean {
        return !rejects.contains(hash) && !contains(hash)
    }

    suspend fun process(hash: Hash, bytes: ByteArray, connection: Connection? = null): Boolean {
        if (!processImpl(hash, bytes, connection)) {
            rejects.add(hash)
            return false
        }
        return true
    }

    abstract suspend fun contains(hash: Hash): Boolean
    abstract suspend fun get(hash: Hash): ByteArray?
    abstract suspend fun remove(hash: Hash): ByteArray?
    abstract suspend fun processImpl(hash: Hash, bytes: ByteArray, connection: Connection?): Boolean
}
