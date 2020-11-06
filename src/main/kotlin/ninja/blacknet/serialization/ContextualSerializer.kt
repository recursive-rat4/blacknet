/*
 * Copyright (c) 2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.serialization

import kotlinx.serialization.KSerializer
import kotlinx.serialization.descriptors.SerialDescriptor
import kotlinx.serialization.encoding.Decoder
import kotlinx.serialization.encoding.Encoder
import ninja.blacknet.serialization.descriptor.ContextualSerialDescriptor

/**
 * Abstract contextual serializer with a runtime module.
 */
public abstract class ContextualSerializer<T> : KSerializer<T> {
    override val descriptor: SerialDescriptor = ContextualSerialDescriptor("ninja.blacknet.serialization.ContextualSerializer")

    override fun deserialize(decoder: Decoder): T {
        @Suppress("UNCHECKED_CAST")
        return ((decoder.serializersModule.getContextual(this::class) ?: throw notSupportedFormatError(decoder, this)) as KSerializer<T>).deserialize(decoder)
    }

    override fun serialize(encoder: Encoder, value: T) {
        @Suppress("UNCHECKED_CAST")
        ((encoder.serializersModule.getContextual(this::class) ?: throw notSupportedFormatError(encoder, this)) as KSerializer<T>).serialize(encoder, value)
    }
}
