/*
 * Copyright (c) 2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.serialization

import kotlinx.serialization.CompositeDecoder
import kotlinx.serialization.CompositeDecoder.Companion.READ_DONE
import kotlinx.serialization.DeserializationStrategy
import kotlinx.serialization.KSerializer
import kotlinx.serialization.SerialDescriptor
import kotlinx.serialization.StructureKind

class ConfigDecoder(
        private val reader: ConfigReader
) : AdaptorDecoder() {
    fun <T : Any?> decode(strategy: DeserializationStrategy<T>): T {
        counter = -1
        descriptor = strategy.descriptor
        val value = strategy.deserialize(this)
        return value
    }

    override fun decodeNotNullMark(): Boolean {
        val name = descriptor.getConfigElementName(counter)
        val string = reader.readString(name)
        return string.toNotNullMark()
    }

    override fun decodeBoolean(): Boolean {
        val name = descriptor.getConfigElementName(counter)
        val string = reader.readString(name)
        return string.toBoolean()
    }

    override fun decodeByte(): Byte {
        val name = descriptor.getConfigElementName(counter)
        val string = reader.readString(name)
        return string.toByte()
    }

    override fun decodeShort(): Short {
        val name = descriptor.getConfigElementName(counter)
        val string = reader.readString(name)
        return string.toShort()
    }

    override fun decodeInt(): Int {
        val name = descriptor.getConfigElementName(counter)
        val string = reader.readString(name)
        return string.toInt()
    }

    override fun decodeLong(): Long {
        val name = descriptor.getConfigElementName(counter)
        val string = reader.readString(name)
        return string.toLong()
    }

    override fun decodeFloat(): Float {
        val name = descriptor.getConfigElementName(counter)
        val string = reader.readString(name)
        return string.toFloat()
    }

    override fun decodeDouble(): Double {
        val name = descriptor.getConfigElementName(counter)
        val string = reader.readString(name)
        return string.toDouble()
    }

    override fun decodeString(): String {
        val name = descriptor.getConfigElementName(counter)
        val string = reader.readString(name)
        return string.toString()
    }

    private var counter: Int = -1
    private lateinit var descriptor: SerialDescriptor

    override fun beginStructure(descriptor: SerialDescriptor, vararg typeParams: KSerializer<*>): CompositeDecoder {
        return if (descriptor.kind === StructureKind.LIST) {
            require(descriptor.serialName.endsWith("ArrayList")) { "未知列表類型 ${descriptor.serialName}" }
            val name = this.descriptor.getConfigElementName(counter)
            val list = reader.readList(name)
            ListDecoder(list)
        } else {
            this
        }
    }

    override fun decodeSequentially(): Boolean = false

    override fun decodeElementIndex(descriptor: SerialDescriptor): Int {
        require(this.descriptor === descriptor) { "彈射 ${descriptor.serialName}" }
        while (++counter < descriptor.elementsCount) {
            val name = descriptor.getConfigElementName(counter)
            if (reader.hasKey(name))
                return counter
        }
        return READ_DONE
    }

    private fun SerialDescriptor.getConfigElementName(counter: Int): String {
        return getElementName(counter).replace('_', '.')
    }

    private class ListDecoder(
            private val input: List<String>
    ) : AdaptorDecoder() {
        override fun decodeBoolean(): Boolean = input[++counter].toBoolean()
        override fun decodeByte(): Byte = input[++counter].toByte()
        override fun decodeShort(): Short = input[++counter].toShort()
        override fun decodeInt(): Int = input[++counter].toInt()
        override fun decodeLong(): Long = input[++counter].toLong()
        override fun decodeFloat(): Float = input[++counter].toFloat()
        override fun decodeDouble(): Double = input[++counter].toDouble()
        // 坐鎮左邊此處
        override fun decodeString(): String = input[++counter].toString()

        private var counter: Int = -1

        override fun beginStructure(descriptor: SerialDescriptor, vararg typeParams: KSerializer<*>): CompositeDecoder {
            require(descriptor.kind !== StructureKind.LIST) { "彈射一個遞歸列表" }
            return this
        }

        override fun decodeCollectionSize(descriptor: SerialDescriptor): Int = input.size
    }

    // 打破空值字串
    private fun String.toNotNullMark() = !equals("null", ignoreCase = true)
}
