/*
 * Copyright (c) 2023 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.db

import kotlinx.serialization.BinaryFormat
import kotlinx.serialization.DeserializationStrategy
import ninja.blacknet.codec.base.Base16
import ninja.blacknet.codec.base.encode

/**
 * A view of [KeyValueStore] using the given [DBKey], [DeserializationStrategy], [BinaryFormat].
 */
class DBView<T>(
    private val store: KeyValueStore,
    private val dbKey: DBKey,
    private val deserializer: DeserializationStrategy<T>,
    private val format: BinaryFormat,
) {
    /**
     * @return `true` if an object is associated with the given key or `false` otherwise.
     */
    fun contains(key: ByteArray): Boolean {
        return store.contains(dbKey + key)
    }

    /**
     * @return the object associated with the given key or `null` if there is none.
     */
    fun get(key: ByteArray): T? {
        return getBytes(key)?.let { bytes ->
            decode(bytes)
        }
    }

    /**
     * @return the object that ought to be associated with the given key.
     * @throws Error if the database is inconsistent.
     */
    fun getOrThrow(key: ByteArray): T {
        return get(key) ?: throw notInDB(key)
    }

    /**
     * @return the object and its serialized size or `null` if there is none.
     */
    fun getWithSize(key: ByteArray): Pair<T, Int>? {
        return getBytes(key)?.let { bytes ->
            Pair(
                decode(bytes),
                bytes.size,
            )
        }
    }

    /**
     * @return the object that ought to be associated with the given key and its serialized size.
     * @throws Error if the database is inconsistent.
     */
    fun getWithSizeOrThrow(key: ByteArray): Pair<T, Int> {
        return getWithSize(key) ?: throw notInDB(key)
    }

    /**
     * @return the [ByteArray] associated with the given key or `null` if there is none.
     */
    fun getBytes(key: ByteArray): ByteArray? {
        return store.get(dbKey + key)
    }

    /**
     * @return the [ByteArray] that ought to be associated with the given key.
     * @throws Error if the database is inconsistent.
     */
    fun getBytesOrThrow(key: ByteArray): ByteArray {
        return getBytes(key) ?: throw notInDB(key)
    }

    private fun decode(bytes: ByteArray): T {
        return format.decodeFromByteArray(deserializer, bytes)
    }

    private fun notInDB(key: ByteArray): Throwable {
        return Error("Not in DB ${Base16.encode(key)}")
    }
}
