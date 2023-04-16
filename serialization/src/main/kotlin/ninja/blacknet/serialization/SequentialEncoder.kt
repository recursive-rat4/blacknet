/*
 * Copyright (c) 2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.serialization

import kotlinx.serialization.SerializationStrategy
import kotlinx.serialization.descriptors.SerialDescriptor
import kotlinx.serialization.encoding.CompositeEncoder
import kotlinx.serialization.encoding.Encoder

public abstract class SequentialEncoder
@Suppress("RemoveEmptyPrimaryConstructor")
constructor() : Encoder, CompositeEncoder {
    private fun notImplementedError(message: String): Throwable = SerializationError("${this::class} is not implemented for $message")

    override fun encodeNotNullMark(): Unit = throw notImplementedError("NotNullMark")
    override fun encodeNull(): Unit = throw notImplementedError("Null")

    override fun encodeBoolean(value: Boolean): Unit = throw notImplementedError("Boolean")
    override fun encodeByte(value: Byte): Unit = throw notImplementedError("Byte")
    override fun encodeShort(value: Short): Unit = throw notImplementedError("Short")
    override fun encodeInt(value: Int): Unit = throw notImplementedError("Int")
    override fun encodeLong(value: Long): Unit = throw notImplementedError("Long")
    override fun encodeFloat(value: Float): Unit = throw notImplementedError("Float")
    override fun encodeDouble(value: Double): Unit = throw notImplementedError("Double")
    override fun encodeChar(value: Char): Unit = throw notImplementedError("Char")
    override fun encodeString(value: String): Unit = throw notImplementedError("String")
    override fun encodeEnum(enumDescriptor: SerialDescriptor, index: Int): Unit = throw notImplementedError("Enum")

    override fun encodeInline(descriptor: SerialDescriptor): Encoder = this

    override fun beginStructure(descriptor: SerialDescriptor): CompositeEncoder = this
    override fun endStructure(descriptor: SerialDescriptor): Unit = Unit

    override fun encodeBooleanElement(descriptor: SerialDescriptor, index: Int, value: Boolean): Unit = encodeBoolean(value)
    override fun encodeByteElement(descriptor: SerialDescriptor, index: Int, value: Byte): Unit = encodeByte(value)
    override fun encodeShortElement(descriptor: SerialDescriptor, index: Int, value: Short): Unit = encodeShort(value)
    override fun encodeIntElement(descriptor: SerialDescriptor, index: Int, value: Int): Unit = encodeInt(value)
    override fun encodeLongElement(descriptor: SerialDescriptor, index: Int, value: Long): Unit = encodeLong(value)
    override fun encodeFloatElement(descriptor: SerialDescriptor, index: Int, value: Float): Unit = encodeFloat(value)
    override fun encodeDoubleElement(descriptor: SerialDescriptor, index: Int, value: Double): Unit = encodeDouble(value)
    override fun encodeCharElement(descriptor: SerialDescriptor, index: Int, value: Char): Unit = encodeChar(value)
    override fun encodeStringElement(descriptor: SerialDescriptor, index: Int, value: String): Unit = encodeString(value)

    override fun encodeInlineElement(
            descriptor: SerialDescriptor,
            index: Int
    ): Encoder =
            encodeInline(descriptor.getElementDescriptor(index))

    override fun <T : Any?> encodeSerializableElement(
            descriptor: SerialDescriptor,
            index: Int,
            serializer: SerializationStrategy<T>,
            value: T
    ): Unit = encodeSerializableValue(serializer, value)
    override fun <T : Any> encodeNullableSerializableElement(
            descriptor: SerialDescriptor,
            index: Int,
            serializer: SerializationStrategy<T>,
            value: T?
    ): Unit = encodeNullableSerializableValue(serializer, value)
}
