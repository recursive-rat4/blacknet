/*
 * Copyright (c) 2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.serialization

import kotlinx.serialization.KSerializer
import kotlinx.serialization.SerializationException
import kotlinx.serialization.encoding.Decoder
import kotlinx.serialization.encoding.Encoder

/**
 * Abstract serializer for a [Map].
 */
public abstract class MapSerializer<T : MutableMap<K, V>, K, V>(
        private val keySerializer: KSerializer<K>,
        private val valueSerializer: KSerializer<V>
) : KSerializer<T> {
    protected abstract fun factory(): T
    protected abstract fun factory(size: Int): T

    override fun deserialize(decoder: Decoder): T {
        @Suppress("NAME_SHADOWING")
        val decoder = decoder.beginStructure(descriptor)
        val map: T
        if (decoder.decodeSequentially()) {
            val size = decoder.decodeCollectionSize(descriptor)
            map = factory(size)
            for (elementIndex in 0 until size) {
                val key = decoder.decodeSerializableElement(descriptor, 0, keySerializer)
                val value = decoder.decodeSerializableElement(descriptor, 1, valueSerializer)
                if (map.put(key, value) == null)
                    Unit
                else
                    throw SerializationException("Duplicate key $key in HashMap")
            }
        } else {
            map = factory()
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

    override fun serialize(encoder: Encoder, value: T) {
        @Suppress("NAME_SHADOWING")
        val encoder = encoder.beginCollection(descriptor, value.size)
        for ((k, v) in value) {
            encoder.encodeSerializableElement(descriptor, 0, keySerializer, k)
            encoder.encodeSerializableElement(descriptor, 1, valueSerializer, v)
        }
        encoder.endStructure(descriptor)
    }
}
