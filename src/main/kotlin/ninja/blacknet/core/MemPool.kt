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

    fun clearImpl() {
        return map.clear()
    }

    fun copyImpl(): HashMap<Hash, ByteArray> {
        return HashMap(map)
    }

    fun sizeImpl(): Int {
        return map.size
    }

    fun dataSizeImpl(): Int {
        return map.values.sumBy { it.size }
    }

    fun <T> mapHashesToListImpl(transform: (Hash) -> T): MutableList<T> {
        return map.keys.mapTo(ArrayList(map.size), transform)
    }

    protected fun addImpl(hash: Hash, bytes: ByteArray) {
        map.put(hash, bytes)
    }

    override suspend fun containsImpl(hash: Hash): Boolean {
        return map.containsKey(hash)
    }

    override suspend fun getImpl(hash: Hash): ByteArray? {
        return map.get(hash)
    }

    fun removeImpl(hash: Hash) {
        map.remove(hash)
    }
}
