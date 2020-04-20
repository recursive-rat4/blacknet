/*
 * Copyright (c) 2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.serialization

import kotlinx.serialization.*
import kotlinx.serialization.builtins.UnitSerializer
import kotlinx.serialization.modules.EmptyModule
import kotlinx.serialization.modules.SerialModule

abstract class AdaptorEncoder
@Suppress("RemoveEmptyPrimaryConstructor")
constructor() : Encoder, CompositeEncoder {
    private fun notImplementedException(four: String): Throwable = EncoderException("${this::class} is not implemented for $four")

    override val context: SerialModule = EmptyModule
    open val updateMode: UpdateMode = UpdateMode.BANNED

    override fun encodeNotNullMark(): Unit = throw notImplementedException("NotNullMark")
    override fun encodeNull(): Unit = throw notImplementedException("Null")
    override fun encodeUnit(): Unit = UnitSerializer().serialize(this, Unit)

    override fun encodeBoolean(value: Boolean): Unit = throw notImplementedException("Boolean")
    override fun encodeByte(value: Byte): Unit = throw notImplementedException("Byte")
    override fun encodeShort(value: Short): Unit = throw notImplementedException("Short")
    override fun encodeInt(value: Int): Unit = throw notImplementedException("Int")
    override fun encodeLong(value: Long): Unit = throw notImplementedException("Long")
    override fun encodeFloat(value: Float): Unit = throw notImplementedException("Float")
    override fun encodeDouble(value: Double): Unit = throw notImplementedException("Double")
    override fun encodeChar(value: Char): Unit = throw notImplementedException("Char")
    override fun encodeString(value: String): Unit = throw notImplementedException("String")
    override fun encodeEnum(enumDescriptor: SerialDescriptor, index: Int): Unit = throw notImplementedException("Enum")

    override fun beginStructure(descriptor: SerialDescriptor, vararg typeSerializers: KSerializer<*>): CompositeEncoder = this
    override fun endStructure(descriptor: SerialDescriptor): Unit = Unit

    open fun encodeSequentially(): Boolean = true
    open fun encodeElementIndex(descriptor: SerialDescriptor, index: Int): Unit = throw notImplementedException("non-sequential mode")

    override fun encodeUnitElement(descriptor: SerialDescriptor, index: Int): Unit = encodeUnit()
    override fun encodeBooleanElement(descriptor: SerialDescriptor, index: Int, value: Boolean): Unit = encodeBoolean(value)
    override fun encodeByteElement(descriptor: SerialDescriptor, index: Int, value: Byte): Unit = encodeByte(value)
    override fun encodeShortElement(descriptor: SerialDescriptor, index: Int, value: Short): Unit = encodeShort(value)
    override fun encodeIntElement(descriptor: SerialDescriptor, index: Int, value: Int): Unit = encodeInt(value)
    override fun encodeLongElement(descriptor: SerialDescriptor, index: Int, value: Long): Unit = encodeLong(value)
    override fun encodeFloatElement(descriptor: SerialDescriptor, index: Int, value: Float): Unit = encodeFloat(value)
    override fun encodeDoubleElement(descriptor: SerialDescriptor, index: Int, value: Double): Unit = encodeDouble(value)
    override fun encodeCharElement(descriptor: SerialDescriptor, index: Int, value: Char): Unit = encodeChar(value)
    override fun encodeStringElement(descriptor: SerialDescriptor, index: Int, value: String): Unit = encodeString(value)

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
