/*
 * Copyright (c) 2019-2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.serialization

import kotlinx.serialization.DeserializationStrategy
import kotlinx.serialization.SerializationStrategy
import kotlinx.serialization.encoding.Decoder
import kotlinx.serialization.encoding.Encoder

public open class SerializationError(
        message: String,
        cause: Throwable? = null
) : Error(message, cause)

public fun notSupportedFormatError(
        decoder: Decoder,
        deserializer: DeserializationStrategy<*>
): Throwable = SerializationError(
        "Decoder ${decoder::class} is not supported by deserializer ${deserializer::class}"
)

public fun notSupportedFormatError(
        encoder: Encoder,
        serializer: SerializationStrategy<*>
): Throwable = SerializationError(
        "Encoder ${encoder::class} is not supported by serializer ${serializer::class}"
)
