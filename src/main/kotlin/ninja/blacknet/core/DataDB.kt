/*
 * Copyright (c) 2018 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.core

import kotlinx.coroutines.sync.Mutex
import kotlinx.coroutines.sync.withLock
import ninja.blacknet.crypto.Hash
import ninja.blacknet.network.Connection
import ninja.blacknet.util.SynchronizedHashSet

abstract class DataDB {
    protected val mutex = Mutex()
    protected val TX_INVALID = Pair(Status.INVALID, 0L)
    protected val TX_ALREADY_HAVE = Pair(Status.ALREADY_HAVE, 0L)
    private val rejects = SynchronizedHashSet<Hash>()

    suspend fun clearRejects() {
        rejects.clear()
    }

    suspend fun isInteresting(hash: Hash): Boolean {
        return !rejects.contains(hash) && !contains(hash)
    }

    suspend fun isRejected(hash: Hash): Boolean {
        return rejects.contains(hash)
    }

    suspend fun process(hash: Hash, bytes: ByteArray, connection: Connection? = null): Status = mutex.withLock {
        if (rejects.contains(hash))
            return Status.INVALID
        if (contains(hash))
            return Status.ALREADY_HAVE
        val status = processImpl(hash, bytes, connection)
        if (status == Status.INVALID)
            rejects.add(hash)
        return status
    }

    suspend fun processTx(hash: Hash, bytes: ByteArray, connection: Connection? = null): Pair<Status, Long> = mutex.withLock {
        if (rejects.contains(hash))
            return TX_INVALID
        if (contains(hash))
            return TX_ALREADY_HAVE
        val fee = TxPool.processImplWithFee(hash, bytes, connection)
        if (fee == TxPool.INVALID_FEE) {
            rejects.add(hash)
            return TX_INVALID
        }
        return Pair(Status.ACCEPTED, fee)
    }

    abstract suspend fun contains(hash: Hash): Boolean
    abstract suspend fun get(hash: Hash): ByteArray?
    abstract suspend fun remove(hash: Hash): ByteArray?
    protected abstract suspend fun processImpl(hash: Hash, bytes: ByteArray, connection: Connection?): Status

    enum class Status {
        ACCEPTED,
        IN_FUTURE,
        ALREADY_HAVE,
        INVALID,
        NOT_ON_THIS_CHAIN,
        ;
    }
}
