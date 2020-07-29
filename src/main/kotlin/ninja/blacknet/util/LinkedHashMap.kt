/*
 * Copyright (c) 2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.util

import kotlinx.serialization.Decoder
import kotlinx.serialization.Encoder
import kotlinx.serialization.KSerializer
import kotlinx.serialization.SerialDescriptor
import kotlinx.serialization.Serializer
import kotlinx.serialization.StructureKind
import kotlinx.serialization.mapDescriptor
import ninja.blacknet.crypto.SipHash.hashCode
import ninja.blacknet.serialization.ByteArraySerializer
import ninja.blacknet.serialization.SerializationException
import org.apache.commons.collections4.map.AbstractLinkedMap

open class LinkedHashMap<K, V>(
        initialCapacity: Int = DEFAULT_CAPACITY,
        loadFactor: Float = DEFAULT_LOAD_FACTOR,
        threshold: Int = DEFAULT_THRESHOLD,
        @Suppress("UNUSED_PARAMETER") unit: Unit = Unit // XXX 1.4
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
        private val keySerializer: KSerializer<K>,
        private val valueSerializer: KSerializer<V>
) : KSerializer<LinkedHashMap<K, V>> {
    override val descriptor: SerialDescriptor = SerialDescriptor(
        "ninja.blacknet.util.LinkedHashMapSerializer",
        StructureKind.MAP
    ) {
        //mapDescriptor(keySerializer.descriptor, valueSerializer.descriptor)
        element("key", keySerializer.descriptor)
        element("value", valueSerializer.descriptor)
    }

    override fun deserialize(decoder: Decoder): LinkedHashMap<K, V> {
        @Suppress("NAME_SHADOWING")
        val decoder = decoder.beginStructure(descriptor)
        val size = decoder.decodeCollectionSize(descriptor)
        val map = LinkedHashMap<K, V>(expectedSize = size)
        var index = -1
        for (i in 0 until size) {
            if (map.put(
                decoder.decodeSerializableElement(descriptor, ++index, keySerializer),
                decoder.decodeSerializableElement(descriptor, ++index, valueSerializer)
            ) == null)
                Unit
            else
                throw SerializationException("Duplicate entry in LinkedHashMap")
        }
        decoder.endStructure(descriptor)
        return map
    }

    override fun serialize(encoder: Encoder, value: LinkedHashMap<K, V>) {
        @Suppress("NAME_SHADOWING")
        val encoder = encoder.beginCollection(descriptor, value.size, keySerializer, valueSerializer)
        var index = -1
        for ((k, v) in value) {
            encoder.encodeSerializableElement(descriptor, ++index, keySerializer, k)
            encoder.encodeSerializableElement(descriptor, ++index, valueSerializer, v)
        }
        encoder.endStructure(descriptor)
    }
}
