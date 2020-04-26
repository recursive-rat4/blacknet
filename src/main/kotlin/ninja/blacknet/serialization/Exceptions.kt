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
import kotlinx.serialization.Decoder
import kotlinx.serialization.Encoder
import kotlinx.serialization.SerializationException

open class DecoderException(message: String, cause: Throwable? = null)
    : SerializationException(message, cause)

open class EncoderException(message: String, cause: Throwable? = null)
    : SerializationException(message, cause)

open class DecoderError(message: String, cause: Throwable? = null)
    : Error(message, cause)

open class EncoderError(message: String, cause: Throwable? = null)
    : Error(message, cause)

fun notSupportedDecoderError(decoder: Decoder, obj: Any): Throwable {
    return DecoderError("Decoder ${decoder::class} is not supported by ${obj::class}")
}

fun notSupportedEncoderError(encoder: Encoder, obj: Any): Throwable {
    return EncoderError("Encoder ${encoder::class} is not supported by ${obj::class}")
}
