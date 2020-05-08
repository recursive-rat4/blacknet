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
import ninja.blacknet.serialization.AdaptorEncoder
import ninja.blacknet.serialization.EncoderError

class HashCoder(
        val writer: HashWriter,
        val charset: Charset? = Charsets.UTF_8,
        val allowFloatingPointValues: Boolean = false
) : AdaptorEncoder() {
    private val buffer = ByteArray(Long.SIZE_BYTES)
    private val memory = Memory.of(buffer)

    override fun encodeNull() {
        writer.writeByte(0)
    }

    override fun encodeNotNullMark() {
        writer.writeByte(1)
    }

    override fun encodeUnit() {
        writer.writeByteArray(buffer, 0, 0)
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
        if (allowFloatingPointValues) {
            encodeInt(value.toBits())
        } else {
            throw floatingPointValueError()
        }
    }

    override fun encodeDouble(value: Double) {
        if (allowFloatingPointValues) {
            encodeLong(value.toBits())
        } else {
            throw floatingPointValueError()
        }
    }

    override fun encodeChar(value: Char) {
        if (charset != null) {
            val bytes = value.toString().toByteArray(charset)
            writer.writeByteArray(bytes, 0, bytes.size)
        } else {
            encodeShort(value.toShort())
        }
    }

    override fun encodeString(value: String) {
        if (charset != null) {
            val bytes = value.toByteArray(charset)
            writer.writeByteArray(bytes, 0, bytes.size)
        } else {
            for (i in 0 until value.length) {
                encodeShort(value[i].toShort())
            }
        }
    }

    private fun floatingPointValueError(): Throwable {
        return EncoderError("You can enable floating point number values with 'allowFloatingPointNumbers' flag in HashCoder.")
    }

    companion object {
        /**
         * Builds a hash value with the given [input] builder.
         *
         * @param algorithm the name of the hash function
         * @param input the initialization function with the [HashCoder] receiver
         * @return the built hash value
         */
        inline fun buildHash(algorithm: String, input: HashCoder.() -> Unit): ByteArray {
            val coder = HashCoder(HashWriterJvm(algorithm))
            coder.input()
            return coder.writer.finish()
        }
    }
}
