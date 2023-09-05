/*
 * Copyright (c) 2018-2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.crypto

import io.ktor.utils.io.bits.*
import io.ktor.utils.io.charsets.Charset
import kotlinx.serialization.modules.SerializersModule
import ninja.blacknet.serialization.SequentialEncoder
import ninja.blacknet.serialization.binaryModule

class HashEncoder(
        val writer: HashWriter,
        val charset: Charset? = Charsets.UTF_8,
        override val serializersModule: SerializersModule = binaryModule
) : SequentialEncoder() {
    private val buffer = ByteArray(Long.SIZE_BYTES)
    private val memory = Memory.of(buffer)

    override fun encodeNull() {
        writer.writeByte(0)
    }

    override fun encodeNotNullMark() {
        writer.writeByte(1)
    }

    override fun encodeBoolean(value: Boolean) {
        writer.writeByte(if (value) 1 else 0)
    }

    override fun encodeByte(value: Byte) {
        writer.writeByte(value)
    }

    override fun encodeShort(value: Short) {
        memory.storeShortAt(0, value)
        writer.writeByteArray(buffer, 0, Short.SIZE_BYTES)
    }

    override fun encodeInt(value: Int) {
        memory.storeIntAt(0, value)
        writer.writeByteArray(buffer, 0, Int.SIZE_BYTES)
    }

    override fun encodeLong(value: Long) {
        memory.storeLongAt(0, value)
        writer.writeByteArray(buffer, 0, Long.SIZE_BYTES)
    }

    override fun encodeFloat(value: Float) {
        encodeInt(value.toBits())
    }

    override fun encodeDouble(value: Double) {
        encodeLong(value.toBits())
    }

    override fun encodeChar(value: Char) {
        if (charset != null) {
            val bytes = value.toString().toByteArray(charset)
            writer.writeByteArray(bytes, 0, bytes.size)
        } else {
            encodeShort(value.code.toShort())
        }
    }

    override fun encodeString(value: String) {
        if (charset != null) {
            val bytes = value.toByteArray(charset)
            writer.writeByteArray(bytes, 0, bytes.size)
        } else {
            for (i in 0 until value.length) {
                encodeShort(value[i].code.toShort())
            }
        }
    }

    companion object {
        /**
         * Builds a hash value with the given [input] builder.
         *
         * @param algorithm the name of the hash function
         * @param input the initialization function with the [HashEncoder] receiver
         * @return the built hash value
         */
        inline fun buildHash(algorithm: String, input: HashEncoder.() -> Unit): ByteArray {
            val coder = HashEncoder(HashWriterJvm(algorithm))
            coder.input()
            return coder.writer.finish()
        }
    }
}
