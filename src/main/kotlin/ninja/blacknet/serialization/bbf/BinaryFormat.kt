/*
 * Copyright (c) 2018-2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.serialization.bbf

import io.ktor.utils.io.core.BytePacketBuilder
import io.ktor.utils.io.core.ByteReadPacket
import io.ktor.utils.io.core.readBytes
import kotlinx.serialization.BinaryFormat
import kotlinx.serialization.DeserializationStrategy
import kotlinx.serialization.SerializationException
import kotlinx.serialization.SerializationStrategy
import kotlinx.serialization.modules.EmptySerializersModule
import kotlinx.serialization.modules.SerializersModule

class BinaryFormat(
        override val serializersModule: SerializersModule = EmptySerializersModule
) : BinaryFormat {
    override fun <T> decodeFromByteArray(deserializer: DeserializationStrategy<T>, bytes: ByteArray): T {
        return decodeFromPacket(deserializer, ByteReadPacket(bytes))
    }

    fun <T : Any?> decodeFromPacket(strategy: DeserializationStrategy<T>, packet: ByteReadPacket): T {
        val decoder = BinaryDecoder(packet, serializersModule)
        val value = strategy.deserialize(decoder)
        val remaining = decoder.input.remaining
        return if (remaining == 0L) {
            value
        } else {
            decoder.input.release()
            throw SerializationException("$remaining trailing bytes")
        }
    }

    override fun <T> encodeToByteArray(serializer: SerializationStrategy<T>, value: T): ByteArray {
        return encodeToPacket(serializer, value).readBytes()
    }

    fun <T : Any?> encodeToPacket(strategy: SerializationStrategy<T>, value: T): ByteReadPacket {
        val encoder = BinaryEncoder(BytePacketBuilder(), serializersModule)
        strategy.serialize(encoder, value)
        return encoder.output.build()
    }
}

val binaryFormat = BinaryFormat()
