/*
 * Copyright (c) 2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet

import kotlinx.serialization.KSerializer
import kotlinx.serialization.Serializable
import kotlinx.serialization.descriptors.PrimitiveKind
import kotlinx.serialization.descriptors.PrimitiveSerialDescriptor
import kotlinx.serialization.descriptors.SerialDescriptor
import kotlinx.serialization.encoding.Decoder
import kotlinx.serialization.encoding.Encoder
import ninja.blacknet.serialization.notSupportedFormatError
import java.text.DecimalFormat
import java.util.Locale

@Serializable(Size.Companion::class)
class Size(
        val bytes: Int
) {
    fun hrp(decimal: Boolean, locale: Locale = Locale.getDefault()): String {
        val (multiplier, symbol) = if (decimal) {
            if (bytes >= 1000000)
                Pair(1000000, "MB")
            else if (bytes >= 1000)
                Pair(1000, "kB")
            else
                Pair(1, "B")
        } else {
            if (bytes >= 1048576)
                Pair(1048576, "MiB")
            else if (bytes >= 1024)
                Pair(1024, "KiB")
            else
                Pair(1, "B")
        }
        val value = bytes / multiplier.toFloat()
        val format = DecimalFormat.getInstance(locale)
        format.maximumFractionDigits = 2
        format.isGroupingUsed = false
        return "${format.format(value)} $symbol"
    }

    companion object : KSerializer<Size> {
        fun parse(string: String): Size {
            val valueString = string.takeWhile { it.isDigit() }
            val value = valueString.toInt()
            val unit = string.substring(valueString.length).trim()
            return Size(Math.multiplyExact(value, when (unit) {
                "" -> 1
                "B" -> 1
                "kB" -> 1000
                "KiB" -> 1024
                // "KB" -> 1000
                // "KB" -> 1024
                "MB" -> 1000000
                "MiB" -> 1048576
                // "GB" -> 1000000000
                // "GiB" -> 1073741824
                else -> throw ParserException("Unknown unit symbol $unit")
            }))
        }

        private class ParserException(message: String, cause: Throwable? = null)
            : RuntimeException(message, cause)

        override val descriptor: SerialDescriptor = PrimitiveSerialDescriptor(
            "ninja.blacknet.Size",
            PrimitiveKind.STRING
        )

        override fun deserialize(decoder: Decoder): Size {
            return Size.parse(decoder.decodeString())
        }

        override fun serialize(encoder: Encoder, value: Size) {
            throw notSupportedFormatError(encoder, this)
        }
    }
}
