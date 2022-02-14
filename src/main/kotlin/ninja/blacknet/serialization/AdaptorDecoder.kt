/*
 * Copyright (c) 2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.serialization

import kotlinx.serialization.DeserializationStrategy
import kotlinx.serialization.SerializationException
import kotlinx.serialization.descriptors.SerialDescriptor
import kotlinx.serialization.encoding.CompositeDecoder
import kotlinx.serialization.encoding.Decoder
import ninja.blacknet.util.statusMessage

abstract class AdaptorDecoder
@Suppress("RemoveEmptyPrimaryConstructor")
constructor() : Decoder, CompositeDecoder {
    private fun notImplementedError(message: String): Throwable = SerializationError("${this::class} is not implemented for $message")

    override fun decodeNotNullMark(): Boolean = throw notImplementedError("NotNullMark")
    override fun decodeNull(): Nothing? = null

    override fun decodeBoolean(): Boolean = throw notImplementedError("Boolean")
    override fun decodeByte(): Byte = throw notImplementedError("Byte")
    override fun decodeShort(): Short = throw notImplementedError("Short")
    override fun decodeInt(): Int = throw notImplementedError("Int")
    override fun decodeLong(): Long = throw notImplementedError("Long")
    override fun decodeFloat(): Float = throw notImplementedError("Float")
    override fun decodeDouble(): Double = throw notImplementedError("Double")
    override fun decodeChar(): Char = throw notImplementedError("Char")
    override fun decodeString(): String = throw notImplementedError("String")
    override fun decodeEnum(enumDescriptor: SerialDescriptor): Int = throw notImplementedError("Enum")

    override fun beginStructure(descriptor: SerialDescriptor): CompositeDecoder = this
    override fun endStructure(descriptor: SerialDescriptor): Unit = Unit

    override fun decodeSequentially(): Boolean = true
    override fun decodeElementIndex(descriptor: SerialDescriptor): Int = throw notImplementedError("non-sequential mode")

    override fun decodeBooleanElement(descriptor: SerialDescriptor, index: Int): Boolean = catcher(descriptor, index) { decodeBoolean() }
    override fun decodeByteElement(descriptor: SerialDescriptor, index: Int): Byte = catcher(descriptor, index) { decodeByte() }
    override fun decodeShortElement(descriptor: SerialDescriptor, index: Int): Short = catcher(descriptor, index) { decodeShort() }
    override fun decodeIntElement(descriptor: SerialDescriptor, index: Int): Int = catcher(descriptor, index) { decodeInt() }
    override fun decodeLongElement(descriptor: SerialDescriptor, index: Int): Long = catcher(descriptor, index) { decodeLong() }
    override fun decodeFloatElement(descriptor: SerialDescriptor, index: Int): Float = catcher(descriptor, index) { decodeFloat() }
    override fun decodeDoubleElement(descriptor: SerialDescriptor, index: Int): Double = catcher(descriptor, index) { decodeDouble() }
    override fun decodeCharElement(descriptor: SerialDescriptor, index: Int): Char = catcher(descriptor, index) { decodeChar() }
    override fun decodeStringElement(descriptor: SerialDescriptor, index: Int): String = catcher(descriptor, index) { decodeString() }

    override fun <T : Any?> decodeSerializableElement(
            descriptor: SerialDescriptor,
            index: Int,
            deserializer: DeserializationStrategy<T>,
            previousValue: T?
    ): T {
        require(previousValue == null) { notImplementedError("update mode") }
        return catcher(descriptor, index) {
            decodeSerializableValue(deserializer)
        }
    }

    override fun <T : Any> decodeNullableSerializableElement(
            descriptor: SerialDescriptor,
            index: Int,
            deserializer: DeserializationStrategy<T?>,
            previousValue: T?
    ): T? {
        require(previousValue == null) { notImplementedError("update mode") }
        return catcher(descriptor, index) {
            decodeNullableSerializableValue(deserializer)
        }
    }

    inline fun <T> catcher(descriptor: SerialDescriptor, index: Int, implementation: () -> T): T = try {
        implementation()
    } catch (e: Exception) {
        throw SerializationException("Invalid ${descriptor.getElementName(index)}: ${e.statusMessage()}", e)
    }
}
