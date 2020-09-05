/*
 * Copyright (c) 2019-2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.serialization

import kotlin.Error
import kotlin.Throwable
import kotlinx.serialization.Decoder
import kotlinx.serialization.DeserializationStrategy
import kotlinx.serialization.Encoder
import kotlinx.serialization.SerializationException
import kotlinx.serialization.SerializationStrategy

open class SerializationException(message: String, cause: Throwable? = null)
    : SerializationException(message, cause)

open class SerializationError(message: String, cause: Throwable? = null)
    : Error(message, cause)

fun notSupportedFormatError(decoder: Decoder, obj: DeserializationStrategy<*>): Throwable {
    return SerializationError("Decoder ${decoder::class} is not supported by deserializer ${obj::class}")
}

fun notSupportedFormatError(encoder: Encoder, obj: SerializationStrategy<*>): Throwable {
    return SerializationError("Encoder ${encoder::class} is not supported by serializer ${obj::class}")
}
