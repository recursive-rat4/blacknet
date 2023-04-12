/*
 * Copyright (c) 2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.serialization.config

import kotlinx.serialization.descriptors.SerialDescriptor
import kotlinx.serialization.descriptors.StructureKind
import kotlinx.serialization.encoding.CompositeDecoder
import kotlinx.serialization.encoding.CompositeDecoder.Companion.DECODE_DONE
import kotlinx.serialization.modules.SerializersModule
import ninja.blacknet.serialization.SequentialDecoder

class ConfigDecoder(
        private val reader: ConfigReader,
        override val serializersModule: SerializersModule
) : SequentialDecoder() {
    override fun decodeNotNullMark(): Boolean {
        val name = descriptor.getElementName(position)
        val string = reader.readString(name)
        return string.toNotNullMark()
    }

    override fun decodeBoolean(): Boolean {
        val name = descriptor.getElementName(position)
        val string = reader.readString(name)
        return string.toBoolean()
    }

    override fun decodeByte(): Byte {
        val name = descriptor.getElementName(position)
        val string = reader.readString(name)
        return string.toByte()
    }

    override fun decodeShort(): Short {
        val name = descriptor.getElementName(position)
        val string = reader.readString(name)
        return string.toShort()
    }

    override fun decodeInt(): Int {
        val name = descriptor.getElementName(position)
        val string = reader.readString(name)
        return string.toInt()
    }

    override fun decodeLong(): Long {
        val name = descriptor.getElementName(position)
        val string = reader.readString(name)
        return string.toLong()
    }

    override fun decodeFloat(): Float {
        val name = descriptor.getElementName(position)
        val string = reader.readString(name)
        return string.toFloat()
    }

    override fun decodeDouble(): Double {
        val name = descriptor.getElementName(position)
        val string = reader.readString(name)
        return string.toDouble()
    }

    override fun decodeString(): String {
        val name = descriptor.getElementName(position)
        val string = reader.readString(name)
        return string.toString()
    }

    private var position: Int = -1
    internal lateinit var descriptor: SerialDescriptor

    override fun beginStructure(descriptor: SerialDescriptor): CompositeDecoder {
        return if (descriptor.kind === StructureKind.LIST) {
            require(descriptor.serialName.endsWith("ArrayList")) { "未知列表類型 ${descriptor.serialName}" }
            val name = this.descriptor.getElementName(position)
            val list = reader.readList(name)
            ListDecoder(list, serializersModule)
        } else {
            this
        }
    }

    override fun decodeSequentially(): Boolean = false

    override fun decodeElementIndex(descriptor: SerialDescriptor): Int {
        require(this.descriptor === descriptor) { "彈射 ${descriptor.serialName}" }
        while (++position < descriptor.elementsCount) {
            val name = descriptor.getElementName(position)
            if (reader.hasKey(name))
                return position
        }
        return DECODE_DONE
    }

    private class ListDecoder(
            private val input: List<String>,
            override val serializersModule: SerializersModule
    ) : SequentialDecoder() {
        override fun decodeBoolean(): Boolean = input[++position].toBoolean()
        override fun decodeByte(): Byte = input[++position].toByte()
        override fun decodeShort(): Short = input[++position].toShort()
        override fun decodeInt(): Int = input[++position].toInt()
        override fun decodeLong(): Long = input[++position].toLong()
        override fun decodeFloat(): Float = input[++position].toFloat()
        override fun decodeDouble(): Double = input[++position].toDouble()
        // 坐鎮左邊此處
        override fun decodeString(): String = input[++position].toString()

        private var position: Int = -1

        override fun beginStructure(descriptor: SerialDescriptor): CompositeDecoder {
            require(descriptor.kind !== StructureKind.LIST) { "彈射一個遞歸列表" }
            return this
        }

        override fun decodeCollectionSize(descriptor: SerialDescriptor): Int = input.size
    }

    // 打破空值字串
    private fun String.toNotNullMark() = !equals("null", ignoreCase = true)
}
