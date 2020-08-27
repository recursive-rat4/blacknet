/*
 * Copyright (c) 2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.util

import java.util.Collections
import kotlinx.serialization.KSerializer
import kotlinx.serialization.Serializable
import kotlinx.serialization.SerializationException
import kotlinx.serialization.descriptors.SerialDescriptor
import kotlinx.serialization.encoding.Decoder
import kotlinx.serialization.encoding.Encoder
import ninja.blacknet.crypto.SipHash.hashCode
import ninja.blacknet.serialization.ByteArraySerializer
import ninja.blacknet.serialization.descriptor.ListSerialDescriptor

@Serializable(with = HashSetSerializer::class)
open class HashSet<T>(map: HashMap<T, Boolean> = HashMap<T, Boolean>()) : MutableSet<T> by Collections.newSetFromMap(map) {
    constructor(expectedSize: Int) : this(HashMap<T, Boolean>(expectedSize = expectedSize))
}

/**
 * Serializes a [HashSet].
 */
class HashSetSerializer<K>(
        private val elementSerializer: KSerializer<K>
) : KSerializer<MutableSet<K>> {
    override val descriptor: SerialDescriptor = ListSerialDescriptor(
            "ninja.blacknet.util.HashSetSerializer",
            elementSerializer.descriptor
    )

    override fun deserialize(decoder: Decoder): MutableSet<K> {
        @Suppress("NAME_SHADOWING")
        val decoder = decoder.beginStructure(descriptor)
        val set: MutableSet<K>
        if (decoder.decodeSequentially()) {
            val size = decoder.decodeCollectionSize(descriptor)
            set = HashSet<K>(expectedSize = size)
            for (elementIndex in 0 until size) {
                val entry = decoder.decodeSerializableElement(descriptor, 0, elementSerializer)
                if (set.add(entry))
                    Unit
                else
                    throw SerializationException("Duplicate $entry in HashSet")
            }
        } else {
            set = HashSet<K>()
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

    override fun serialize(encoder: Encoder, value: MutableSet<K>) {
        @Suppress("NAME_SHADOWING")
        val encoder = encoder.beginCollection(descriptor, value.size)
        for (k in value) {
            encoder.encodeSerializableElement(descriptor, 0, elementSerializer, k)
        }
        encoder.endStructure(descriptor)
    }
}
