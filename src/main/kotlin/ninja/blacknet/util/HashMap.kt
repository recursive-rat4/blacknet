/*
 * Copyright (c) 2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.util

import kotlinx.serialization.KSerializer
import kotlinx.serialization.Serializable
import kotlinx.serialization.SerializationException
import kotlinx.serialization.descriptors.SerialDescriptor
import kotlinx.serialization.encoding.Decoder
import kotlinx.serialization.encoding.Encoder
import ninja.blacknet.crypto.SipHash.hashCode
import ninja.blacknet.serialization.ByteArraySerializer
import ninja.blacknet.serialization.MapSerializer
import ninja.blacknet.serialization.descriptor.MapSerialDescriptor
import org.apache.commons.collections4.map.AbstractHashedMap

inline fun <T, K, V> makeMap(content: Array<out Pair<K, V>>, factory: () -> T): T
    where T : MutableMap<K, V> = factory().apply { putAll(content) }

fun <K, V> hashMapOf(vararg content: Pair<K, V>) =
    makeMap<HashMap<K, V>, K, V>(content) { HashMap(expectedSize = content.size) }

@Serializable(with = HashMapSerializer::class)
open class HashMap<K, V>(
        initialCapacity: Int = DEFAULT_CAPACITY,
        loadFactor: Float = DEFAULT_LOAD_FACTOR,
        threshold: Int = DEFAULT_THRESHOLD,
) : AbstractHashedMap<K, V>(initialCapacity, loadFactor, threshold) {
    constructor(expectedSize: Int) : this(initialCapacity = (expectedSize / DEFAULT_LOAD_FACTOR + 1.0f).toInt())

    override fun hash(key: Any): Int = if (key is ByteArray)
                                           hashCode(ByteArraySerializer, key)
                                       else
                                           key.hashCode()

    override fun isEqualKey(key1: Any, key2: Any) = if (key1 is ByteArray)
                                                        key1.contentEquals(key2 as ByteArray)
                                                    else
                                                        key1.equals(key2)

    override fun isEqualValue(value1: Any, value2: Any) = if (value1 is ByteArray)
                                                              value1.contentEquals(value2 as ByteArray)
                                                          else
                                                              value1.equals(value2)
}

/**
 * Serializes a [HashMap].
 */
class HashMapSerializer<K, V>(
        keySerializer: KSerializer<K>,
        valueSerializer: KSerializer<V>
) : MapSerializer<HashMap<K, V>, K, V>(
        keySerializer,
        valueSerializer
) {
    override val descriptor: SerialDescriptor = MapSerialDescriptor(
            "ninja.blacknet.util.HashMapSerializer",
            keySerializer.descriptor,
            valueSerializer.descriptor
    )

    override fun factory() = HashMap<K, V>()
    override fun factory(size: Int) = HashMap<K, V>(expectedSize = size)
}
