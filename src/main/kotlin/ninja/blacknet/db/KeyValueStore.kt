/*
 * Copyright (c) 2023 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.db

/**
 * It represents an associative array with [ByteArray] keys and values.
 */
interface KeyValueStore {
    /**
     * @return `true` if a value is associated with the given key or `false` otherwise.
     */
    fun contains(key: ByteArray): Boolean

    /**
     * @return the value associated with the given key or `null` if there is none.
     */
    fun get(key: ByteArray): ByteArray?
}
