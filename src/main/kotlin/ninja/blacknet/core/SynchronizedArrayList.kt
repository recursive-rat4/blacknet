/*
 * Copyright (c) 2018 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.core

import kotlinx.coroutines.experimental.sync.Mutex
import kotlinx.coroutines.experimental.sync.withLock

class SynchronizedArrayList<T>(private val list: ArrayList<T>) {
    constructor() : this(ArrayList())

    private val mutex = Mutex()

    suspend fun getSize(): Int = mutex.withLock { list.size }

    suspend fun add(element: T) = mutex.withLock { list.add(element) }

    suspend fun remove(element: T) = mutex.withLock { list.remove(element) }

    suspend fun forEach(action: (T) -> Unit) = mutex.withLock { list.forEach(action) }
}