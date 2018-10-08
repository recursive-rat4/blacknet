/*
 * Copyright (c) 2018 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.util

import kotlinx.coroutines.experimental.sync.Mutex
import kotlinx.coroutines.experimental.sync.withLock

class SynchronizedHashSet<T>(private val set: HashSet<T>) {
    constructor() : this(HashSet())

    private val mutex = Mutex()

    suspend fun add(element: T) = mutex.withLock { set.add(element) }

    suspend fun filter(predicate: (T) -> Boolean) = mutex.withLock { set.filter(predicate) }

    suspend fun toList() = mutex.withLock { set.toList() }

    suspend fun <R> map(transform: (T) -> R): ArrayList<R> = mutex.withLock {
        val ret = ArrayList<R>(set.size)
        set.forEach { ret.add(transform(it)) }
        ret
    }
}