/*
 * Copyright (c) 2020-2024 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.rpc.requests

import kotlinx.serialization.descriptors.SerialDescriptor
import kotlinx.serialization.encoding.CompositeDecoder
import kotlinx.serialization.encoding.Decoder
import kotlinx.serialization.modules.SerializersModule
import ninja.blacknet.serialization.SequentialDecoder
import ninja.blacknet.serialization.descriptor.isUnsignedNumber

class RequestDecoder(
        private val reader: RequestReader,
        override val serializersModule: SerializersModule
) : SequentialDecoder() {
    override fun decodeNotNullMark(): Boolean {
        val string = reader.readString(elementName)
        return string.toNotNullMark()
    }

    override fun decodeBoolean(): Boolean {
        val string = reader.readString(elementName)
        return string.toBoolean()
    }

    override fun decodeByte(): Byte {
        val string = reader.readString(elementName)
        return string.toByte()
    }

    override fun decodeShort(): Short {
        val string = reader.readString(elementName)
        return string.toShort()
    }

    override fun decodeInt(): Int {
        val string = reader.readString(elementName)
        return string.toInt()
    }

    override fun decodeLong(): Long {
        val string = reader.readString(elementName)
        return string.toLong()
    }

    override fun decodeFloat(): Float {
        val string = reader.readString(elementName)
        return string.toFloat()
    }

    override fun decodeDouble(): Double {
        val string = reader.readString(elementName)
        return string.toDouble()
    }

    override fun decodeString(): String {
        val string = reader.readString(elementName)
        return string.toString()
    }

    override fun decodeInline(descriptor: SerialDescriptor): Decoder {
        return if (!descriptor.isUnsignedNumber())
            this
        else
            UnsignedDecoder(serializersModule)
    }

    private var elementIndex: Int = -1
    private var elementName: String = ""

    override fun decodeSequentially(): Boolean = false

    override fun decodeElementIndex(descriptor: SerialDescriptor): Int {
        while (++elementIndex < descriptor.elementsCount) {
            elementName = descriptor.getElementName(elementIndex)
            if (reader.hasKey(elementName))
                return elementIndex
        }
        return CompositeDecoder.DECODE_DONE
    }

    private inner class UnsignedDecoder(
        override val serializersModule: SerializersModule,
    ) : SequentialDecoder() {
        override fun decodeByte(): Byte {
            val string = reader.readString(elementName)
            return string.toUByte().toByte()
        }

        override fun decodeShort(): Short {
            val string = reader.readString(elementName)
            return string.toUShort().toShort()
        }

        override fun decodeInt(): Int {
            val string = reader.readString(elementName)
            return string.toUInt().toInt()
        }

        override fun decodeLong(): Long {
            val string = reader.readString(elementName)
            return string.toULong().toLong()
        }
    }

    // 打破空值字串
    private fun String.toNotNullMark() = !equals("null", ignoreCase = true)
}
