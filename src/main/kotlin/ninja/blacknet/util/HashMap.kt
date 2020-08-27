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
import ninja.blacknet.serialization.descriptor.MapSerialDescriptor
import org.apache.commons.collections4.map.AbstractHashedMap

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
        private val keySerializer: KSerializer<K>,
        private val valueSerializer: KSerializer<V>
) : KSerializer<HashMap<K, V>> {
    override val descriptor: SerialDescriptor = MapSerialDescriptor(
            "ninja.blacknet.util.HashMapSerializer",
            keySerializer.descriptor,
            valueSerializer.descriptor
    )

    override fun deserialize(decoder: Decoder): HashMap<K, V> {
        @Suppress("NAME_SHADOWING")
        val decoder = decoder.beginStructure(descriptor)
        val map: HashMap<K, V>
        if (decoder.decodeSequentially()) {
            val size = decoder.decodeCollectionSize(descriptor)
            map = HashMap<K, V>(expectedSize = size)
            for (elementIndex in 0 until size) {
                val key = decoder.decodeSerializableElement(descriptor, 0, keySerializer)
                val value = decoder.decodeSerializableElement(descriptor, 1, valueSerializer)
                if (map.put(key, value) == null)
                    Unit
                else
                    throw SerializationException("Duplicate key $key in HashMap")
            }
        } else {
            map = HashMap<K, V>()
            while (decoder.decodeElementIndex(descriptor) >= 0) {
                val key = decoder.decodeSerializableElement(descriptor, 0, keySerializer)
                require(decoder.decodeElementIndex(descriptor) > 0)
                val value = decoder.decodeSerializableElement(descriptor, 1, valueSerializer)
                if (map.put(key, value) == null)
                    Unit
                else
                    throw SerializationException("Duplicate key $key in HashMap")
            }
        }
        decoder.endStructure(descriptor)
        return map
    }

    override fun serialize(encoder: Encoder, value: HashMap<K, V>) {
        @Suppress("NAME_SHADOWING")
        val encoder = encoder.beginCollection(descriptor, value.size)
        for ((k, v) in value) {
            encoder.encodeSerializableElement(descriptor, 0, keySerializer, k)
            encoder.encodeSerializableElement(descriptor, 1, valueSerializer, v)
        }
        encoder.endStructure(descriptor)
    }
}
