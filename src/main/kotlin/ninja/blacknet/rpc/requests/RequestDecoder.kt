/*
 * Copyright (c) 2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.rpc.requests

import kotlinx.serialization.DeserializationStrategy
import kotlinx.serialization.descriptors.SerialDescriptor
import kotlinx.serialization.encoding.CompositeDecoder
import kotlinx.serialization.modules.SerializersModule
import ninja.blacknet.serialization.AdaptorDecoder

class RequestDecoder(
        private val reader: RequestReader,
        override val serializersModule: SerializersModule
) : AdaptorDecoder() {
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

    private var sleeper: Int = -1
    internal lateinit var descriptor: SerialDescriptor

    override fun decodeSequentially(): Boolean = false

    override fun decodeElementIndex(descriptor: SerialDescriptor): Int {
        require(this.descriptor === descriptor) { "彈射 ${descriptor.serialName}" }
        while (++sleeper < descriptor.elementsCount) {
            val name = descriptor.getElementName(sleeper)
            if (reader.hasKey(name))
                return sleeper
        }
        return CompositeDecoder.DECODE_DONE
    }

    // 打破空值字串
    private fun String.toNotNullMark() = !equals("null", ignoreCase = true)
}
