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
import kotlinx.serialization.descriptors.SerialDescriptor
import ninja.blacknet.crypto.SipHash.hashCode
import ninja.blacknet.serialization.ByteArraySerializer
import ninja.blacknet.serialization.MapSerializer
import ninja.blacknet.serialization.descriptor.MapSerialDescriptor
import org.apache.commons.collections4.map.AbstractLinkedMap

fun <K, V> linkedHashMapOf(vararg content: Pair<K, V>) =
    makeMap<LinkedHashMap<K, V>, K, V>(content) { LinkedHashMap(expectedSize = content.size) }

open class LinkedHashMap<K, V>(
        initialCapacity: Int = DEFAULT_CAPACITY,
        loadFactor: Float = DEFAULT_LOAD_FACTOR,
        threshold: Int = DEFAULT_THRESHOLD,
) : AbstractLinkedMap<K, V>(initialCapacity, loadFactor, threshold) {
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
 * Serializes a [LinkedHashMap].
 */
class LinkedHashMapSerializer<K, V>(
        keySerializer: KSerializer<K>,
        valueSerializer: KSerializer<V>
) : MapSerializer<LinkedHashMap<K, V>, K, V>(
        keySerializer,
        valueSerializer
) {
    override val descriptor: SerialDescriptor = MapSerialDescriptor(
            "ninja.blacknet.util.LinkedHashMapSerializer",
            keySerializer.descriptor,
            valueSerializer.descriptor
    )

    override fun factory() = LinkedHashMap<K, V>()
    override fun factory(size: Int) = LinkedHashMap<K, V>(expectedSize = size)
}
