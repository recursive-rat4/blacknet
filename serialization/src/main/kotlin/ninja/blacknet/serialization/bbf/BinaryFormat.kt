/*
 * Copyright (c) 2018-2024 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.serialization.bbf

import io.ktor.utils.io.core.BytePacketBuilder
import io.ktor.utils.io.core.ByteReadPacket
import java.io.DataInputStream
import java.io.DataOutputStream
import kotlinx.serialization.BinaryFormat
import kotlinx.serialization.DeserializationStrategy
import kotlinx.serialization.SerializationException
import kotlinx.serialization.SerializationStrategy
import kotlinx.serialization.modules.EmptySerializersModule
import kotlinx.serialization.modules.SerializersModule

/**
 * The Blacknet Binary Format
 */
public class BinaryFormat(
    override val serializersModule: SerializersModule = EmptySerializersModule()
) : BinaryFormat {
    override fun <T> decodeFromByteArray(deserializer: DeserializationStrategy<T>, bytes: ByteArray): T {
        val stream = DataInputStream(ByteArrayInputStream(bytes))
        val value = decodeFromStream(deserializer, stream)
        return value
    }

    public fun <T : Any?> decodeFromPacket(strategy: DeserializationStrategy<T>, packet: ByteReadPacket): T {
        val decoder = BinaryDecoder(PacketReader(packet), serializersModule)
        val value = strategy.deserialize(decoder)
        val remaining = packet.remaining
        return if (remaining == 0L) {
            value
        } else {
            packet.release()
            throw SerializationException("$remaining trailing bytes")
        }
    }

    public fun <T> decodeFromStream(deserializer: DeserializationStrategy<T>, stream: DataInputStream): T {
        val decoder = BinaryDecoder(StreamReader(stream), serializersModule)
        val value = deserializer.deserialize(decoder)
        val remaining = stream.available()
        return if (remaining == 0) {
            value
        } else {
            throw SerializationException("$remaining trailing bytes")
        }
    }

    override fun <T> encodeToByteArray(serializer: SerializationStrategy<T>, value: T): ByteArray {
        val size = computeSize(serializer, value)
        val bytes = ByteArray(size)
        val stream = ByteArrayOutputStream(bytes)
        encodeToStream(serializer, value, DataOutputStream(stream))
        return bytes
    }

    public fun <T : Any?> encodeToPacket(strategy: SerializationStrategy<T>, value: T): ByteReadPacket {
        val output = BytePacketBuilder()
        val encoder = BinaryEncoder(PacketWriter(output), serializersModule)
        strategy.serialize(encoder, value)
        return output.build()
    }

    public fun <T> encodeToStream(serializer: SerializationStrategy<T>, value: T, stream: DataOutputStream) {
        val encoder = BinaryEncoder(StreamWriter(stream), serializersModule)
        serializer.serialize(encoder, value)
    }

    public fun <T> computeSize(serializer: SerializationStrategy<T>, value: T): Int {
        val writer = SizeWriter()
        val encoder = BinaryEncoder(writer, serializersModule)
        serializer.serialize(encoder, value)
        return writer.output()
    }
}
