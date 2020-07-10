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
import kotlinx.serialization.Decoder
import kotlinx.serialization.Encoder
import kotlinx.serialization.KSerializer
import kotlinx.serialization.SerialDescriptor
import kotlinx.serialization.Serializer
import kotlinx.serialization.StructureKind
import ninja.blacknet.crypto.SipHash.hashCode
import ninja.blacknet.serialization.ByteArraySerializer
import ninja.blacknet.serialization.DecoderException

// open class HashSet<T> : MutableSet<T> by Collections.newSetFromMap(HashMap<T, Boolean>())

fun <T> HashSet(): MutableSet<T> = Collections.newSetFromMap(HashMap<T, Boolean>())
fun <T> HashSet(expectedSize: Int): MutableSet<T> = Collections.newSetFromMap(HashMap<T, Boolean>(expectedSize = expectedSize))

/**
 * Serializes a hash set as [MutableSet].
 */
class HashSetSerializer<K>(
        private val keySerializer: KSerializer<K>
) : KSerializer<MutableSet<K>> {
    override val descriptor: SerialDescriptor = SerialDescriptor(
        "ninja.blacknet.util.HashSetSerializer",
        StructureKind.LIST
    )

    override fun deserialize(decoder: Decoder): MutableSet<K> {
        @Suppress("NAME_SHADOWING")
        val decoder = decoder.beginStructure(descriptor)
        val size = decoder.decodeCollectionSize(descriptor)
        val set = HashSet<K>(expectedSize = size)
        for (index in 0 until size) {
            if (set.add(
                decoder.decodeSerializableElement(descriptor, index, keySerializer)
            ) == true)
                Unit
            else
                throw DecoderException("Duplicate entry in HashSet")
        }
        decoder.endStructure(descriptor)
        return set
    }

    override fun serialize(encoder: Encoder, value: MutableSet<K>) {
        @Suppress("NAME_SHADOWING")
        val encoder = encoder.beginCollection(descriptor, value.size, keySerializer)
        var index = -1
        for (k in value) {
            encoder.encodeSerializableElement(descriptor, ++index, keySerializer, k)
        }
        encoder.endStructure(descriptor)
    }
}
