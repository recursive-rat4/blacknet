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

class SynchronizedHashMap<K, V>(private val map: HashMap<K, V>) {
    constructor() : this(HashMap())

    private val mutex = Mutex()

    suspend fun size() = mutex.withLock { map.size }

    suspend fun get(key: K): V? = mutex.withLock { map.get(key) }

    suspend fun set(key: K, value: V) = mutex.withLock { map.set(key, value) }

    suspend fun remove(key: K) = mutex.withLock { map.remove(key) }

    suspend fun containsKey(key: K) = mutex.withLock { map.containsKey(key) }

    suspend fun sumValuesBy(selector: (V) -> Int) = mutex.withLock { map.values.sumBy(selector) }

    suspend fun <R> mapKeys(transform: (K) -> R): ArrayList<R> {
        mutex.withLock {
            val ret = ArrayList<R>(map.size)
            map.keys.forEach { ret.add(transform(it)) }
            return ret
        }
    }

    suspend fun filterValues(predicate: (V) -> Boolean): Map<K, V> {
        mutex.withLock {
            val result = HashMap<K, V>()
            for (entry in map) {
                if (predicate(entry.value)) {
                    result.put(entry.key, entry.value)
                }
            }
            return result
        }
    }
}
