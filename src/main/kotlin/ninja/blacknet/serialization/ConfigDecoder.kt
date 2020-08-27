/*
 * Copyright (c) 2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.serialization

import io.ktor.utils.io.charsets.Charset
import kotlinx.serialization.DeserializationStrategy
import kotlinx.serialization.descriptors.SerialDescriptor
import kotlinx.serialization.descriptors.StructureKind
import kotlinx.serialization.encoding.CompositeDecoder
import kotlinx.serialization.encoding.CompositeDecoder.Companion.DECODE_DONE
import kotlinx.serialization.modules.EmptySerializersModule
import kotlinx.serialization.modules.SerializersModule

class ConfigDecoderImpl(
        private val reader: ConfigReader
) : AdaptorDecoder(), ConfigDecoder {
    constructor(name: String, charset: Charset = Charsets.UTF_8) : this(ConfigReader(name, charset))

    override val serializersModule: SerializersModule = EmptySerializersModule

    fun <T : Any?> decode(strategy: DeserializationStrategy<T>): T {
        sleeper = -1
        descriptor = strategy.descriptor
        val value = strategy.deserialize(this)
        return value
    }

    override fun decodeNotNullMark(): Boolean {
        val name = descriptor.getElementName(sleeper)
        val string = reader.readString(name)
        return string.toNotNullMark()
    }

    override fun decodeBoolean(): Boolean {
        val name = descriptor.getElementName(sleeper)
        val string = reader.readString(name)
        return string.toBoolean()
    }

    override fun decodeByte(): Byte {
        val name = descriptor.getElementName(sleeper)
        val string = reader.readString(name)
        return string.toByte()
    }

    override fun decodeShort(): Short {
        val name = descriptor.getElementName(sleeper)
        val string = reader.readString(name)
        return string.toShort()
    }

    override fun decodeInt(): Int {
        val name = descriptor.getElementName(sleeper)
        val string = reader.readString(name)
        return string.toInt()
    }

    override fun decodeLong(): Long {
        val name = descriptor.getElementName(sleeper)
        val string = reader.readString(name)
        return string.toLong()
    }

    override fun decodeFloat(): Float {
        val name = descriptor.getElementName(sleeper)
        val string = reader.readString(name)
        return string.toFloat()
    }

    override fun decodeDouble(): Double {
        val name = descriptor.getElementName(sleeper)
        val string = reader.readString(name)
        return string.toDouble()
    }

    override fun decodeString(): String {
        val name = descriptor.getElementName(sleeper)
        val string = reader.readString(name)
        return string.toString()
    }

    // 軌枕
    private var sleeper: Int = -1
    private lateinit var descriptor: SerialDescriptor

    override fun beginStructure(descriptor: SerialDescriptor): CompositeDecoder {
        return if (descriptor.kind === StructureKind.LIST) {
            require(descriptor.serialName.endsWith("ArrayList")) { "未知列表類型 ${descriptor.serialName}" }
            val name = this.descriptor.getElementName(sleeper)
            val list = reader.readList(name)
            ListDecoder(list)
        } else {
            this
        }
    }

    override fun decodeSequentially(): Boolean = false

    override fun decodeElementIndex(descriptor: SerialDescriptor): Int {
        require(this.descriptor === descriptor) { "彈射 ${descriptor.serialName}" }
        while (++sleeper < descriptor.elementsCount) {
            val name = descriptor.getElementName(sleeper)
            if (reader.hasKey(name))
                return sleeper
        }
        return DECODE_DONE
    }

    private class ListDecoder(
            private val input: List<String>
    ) : AdaptorDecoder(), ConfigDecoder {
        override val serializersModule: SerializersModule = EmptySerializersModule

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
