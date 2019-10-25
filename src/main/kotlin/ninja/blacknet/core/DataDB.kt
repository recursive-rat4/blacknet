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
import mu.KotlinLogging
import ninja.blacknet.Config
import ninja.blacknet.crypto.Hash
import ninja.blacknet.network.Connection

private val logger = KotlinLogging.logger {}

abstract class DataDB {
    internal val mutex = Mutex()
    private val rejects = HashSet<Hash>()

    internal fun clearRejectsImpl() {
        rejects.clear()
    }

    suspend fun get(hash: Hash): ByteArray? = mutex.withLock {
        return@withLock getImpl(hash)
    }

    suspend fun isInteresting(hash: Hash): Boolean = mutex.withLock {
        return !rejects.contains(hash) && !containsImpl(hash)
    }

    suspend fun isRejected(hash: Hash): Boolean = mutex.withLock {
        return rejects.contains(hash)
    }

    suspend fun process(hash: Hash, bytes: ByteArray, connection: Connection? = null): Status = mutex.withLock {
        if (rejects.contains(hash))
            return Invalid("Already rejected")
        if (containsImpl(hash))
            return AlreadyHave
        val status = processImpl(hash, bytes, connection)
        if (status is Invalid)
            rejects.add(hash)
        return status
    }

    suspend fun processTx(hash: Hash, bytes: ByteArray, connection: Connection? = null): Pair<Status, Long> = mutex.withLock {
        if (rejects.contains(hash))
            return Pair(Invalid("Already rejected"), 0)
        if (containsImpl(hash))
            return Pair(AlreadyHave, 0)
        if (TxPool.dataSizeImpl() + bytes.size > Config.txPoolSize) {
            if (connection != null)
                return Pair(InFuture, 0)
            else
                logger.warn("TxPool is full")
        }
        val result = TxPool.processImplWithFee(hash, bytes, connection)
        if (result.first is Invalid || result.first == InFuture) {
            rejects.add(hash)
        }
        return result
    }

    internal abstract suspend fun getImpl(hash: Hash): ByteArray?
    protected abstract suspend fun containsImpl(hash: Hash): Boolean
    protected abstract suspend fun processImpl(hash: Hash, bytes: ByteArray, connection: Connection?): Status
}
