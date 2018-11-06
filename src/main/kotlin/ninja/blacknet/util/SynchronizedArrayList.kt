/*
 * Copyright (c) 2018 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.util

import kotlinx.coroutines.sync.Mutex
import kotlinx.coroutines.sync.withLock

class SynchronizedArrayList<T>(private val list: ArrayList<T>) {
    constructor() : this(ArrayList())

    private val mutex = Mutex()

    suspend fun size() = mutex.withLock { list.size }

    suspend fun add(element: T) = mutex.withLock { list.add(element) }

    suspend fun remove(element: T) = mutex.withLock { list.remove(element) }

    suspend fun removeIf(filter: (T) -> Boolean) = mutex.withLock { list.removeIf(filter) }

    suspend fun forEach(action: (T) -> Unit) = mutex.withLock { list.forEach(action) }

    suspend fun sumBy(selector: (T) -> Int) = mutex.withLock { list.sumBy(selector) }

    suspend fun filter(predicate: (T) -> Boolean) = mutex.withLock { list.filter(predicate) }

    suspend fun <R> map(transform: (T) -> R): ArrayList<R> = mutex.withLock {
        val ret = ArrayList<R>(list.size)
        list.forEach { ret.add(transform(it)) }
        ret
    }
}