/*
 * Copyright (c) 2019 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.core

import kotlinx.serialization.*
import kotlinx.serialization.json.JsonOutput
import ninja.blacknet.crypto.Hash
import ninja.blacknet.serialization.BinaryDecoder
import ninja.blacknet.serialization.BinaryEncoder

@Serializable
class ChainIndex(
        val previous: Hash,
        var next: Hash,
        var nextSize: Int,
        val height: Int,
        val generated: Long
) {
    fun serialize(): ByteArray = BinaryEncoder.toBytes(serializer(), this)

    @Serializer(forClass = ChainIndex::class)
    companion object {
        fun deserialize(bytes: ByteArray): ChainIndex = BinaryDecoder(bytes).decode(serializer())

        override fun deserialize(decoder: Decoder): ChainIndex {
            return when (decoder) {
                is BinaryDecoder -> ChainIndex(
                        Hash(decoder.decodeFixedByteArray(Hash.SIZE_BYTES)),
                        Hash(decoder.decodeFixedByteArray(Hash.SIZE_BYTES)),
                        decoder.decodeVarInt(),
                        decoder.decodeVarInt(),
                        decoder.decodeVarLong())
                else -> throw RuntimeException("Unsupported decoder")
            }
        }

        override fun serialize(encoder: Encoder, obj: ChainIndex) {
            when (encoder) {
                is BinaryEncoder -> {
                    encoder.encodeFixedByteArray(obj.previous.bytes)
                    encoder.encodeFixedByteArray(obj.next.bytes)
                    encoder.encodeVarInt(obj.nextSize)
                    encoder.encodeVarInt(obj.height)
                    encoder.encodeVarLong(obj.generated)
                }
                is JsonOutput -> {
                    @Suppress("NAME_SHADOWING")
                    val encoder = encoder.beginStructure(descriptor)
                    encoder.encodeSerializableElement(descriptor, 0, Hash.serializer(), obj.previous)
                    encoder.encodeSerializableElement(descriptor, 1, Hash.serializer(), obj.next)
                    encoder.encodeSerializableElement(descriptor, 2, Int.serializer(), obj.nextSize)
                    encoder.encodeSerializableElement(descriptor, 3, Int.serializer(), obj.height)
                    encoder.encodeSerializableElement(descriptor, 4, String.serializer(), obj.generated.toString())
                    encoder.endStructure(descriptor)
                }
                else -> throw RuntimeException("Unsupported encoder")
            }
        }
    }
}
