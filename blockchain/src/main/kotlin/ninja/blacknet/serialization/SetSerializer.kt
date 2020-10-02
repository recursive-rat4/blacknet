/*
 * Copyright (c) 2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.serialization

import java.util.Collections
import kotlinx.serialization.KSerializer
import kotlinx.serialization.Serializable
import kotlinx.serialization.SerializationException
import kotlinx.serialization.descriptors.SerialDescriptor
import kotlinx.serialization.encoding.Decoder
import kotlinx.serialization.encoding.Encoder
import ninja.blacknet.serialization.descriptor.ListSerialDescriptor

/**
 * Abstract serializer for a [Set].
 */
public abstract class SetSerializer<T : MutableSet<E>, E>(
        private val elementSerializer: KSerializer<E>
) : KSerializer<T> {
    protected abstract fun factory(): T
    protected abstract fun factory(size: Int): T

    override fun deserialize(decoder: Decoder): T {
        @Suppress("NAME_SHADOWING")
        val decoder = decoder.beginStructure(descriptor)
        val set: T
        if (decoder.decodeSequentially()) {
            val size = decoder.decodeCollectionSize(descriptor)
            set = factory(size)
            for (elementIndex in 0 until size) {
                val entry = decoder.decodeSerializableElement(descriptor, 0, elementSerializer)
                if (set.add(entry))
                    Unit
                else
                    throw SerializationException("Duplicate $entry in HashSet")
            }
        } else {
            set = factory()
            while (decoder.decodeElementIndex(descriptor) >= 0) {
                val entry = decoder.decodeSerializableElement(descriptor, 0, elementSerializer)
                if (set.add(entry))
                    Unit
                else
                    throw SerializationException("Duplicate $entry in HashSet")
            }
        }
        decoder.endStructure(descriptor)
        return set
    }

    override fun serialize(encoder: Encoder, value: T) {
        @Suppress("NAME_SHADOWING")
        val encoder = encoder.beginCollection(descriptor, value.size)
        for (e in value) {
            encoder.encodeSerializableElement(descriptor, 0, elementSerializer, e)
        }
        encoder.endStructure(descriptor)
    }
}
