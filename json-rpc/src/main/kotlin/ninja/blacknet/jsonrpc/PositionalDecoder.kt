/*
 * Copyright (c) 2023 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.jsonrpc

import kotlinx.serialization.DeserializationStrategy
import kotlinx.serialization.json.*
import kotlinx.serialization.modules.SerializersModule
import ninja.blacknet.serialization.SequentialDecoder

/**
 * A sequential decoder of [JsonArray] for positional parameters in a [Request].
 */
private class PositionalDecoder(
    private val jsonArray: JsonArray,
    override val serializersModule: SerializersModule,
) : SequentialDecoder() {
    private var position: Int = -1

    override fun decodeBoolean(): Boolean {
        return jsonArray[++position].jsonPrimitive.boolean
    }

    override fun decodeByte(): Byte {
        return jsonArray[++position].jsonPrimitive.byte
    }

    override fun decodeShort(): Short {
        return jsonArray[++position].jsonPrimitive.short
    }

    override fun decodeInt(): Int {
        return jsonArray[++position].jsonPrimitive.int
    }

    override fun decodeLong(): Long {
        return jsonArray[++position].jsonPrimitive.long
    }

    override fun decodeFloat(): Float {
        return jsonArray[++position].jsonPrimitive.float
    }

    override fun decodeDouble(): Double {
        return jsonArray[++position].jsonPrimitive.double
    }

    override fun decodeChar(): Char {
        return jsonArray[++position].jsonPrimitive.char
    }

    override fun decodeString(): String {
        return jsonArray[++position].jsonPrimitive.string
    }
}

internal fun <T> Json.decodePositionalParameters(deserializer: DeserializationStrategy<T>, jsonArray: JsonArray): T {
    val decoder = PositionalDecoder(jsonArray, serializersModule)
    return deserializer.deserialize(decoder)
}

internal fun <T> Json.decodeNamedParameters(deserializer: DeserializationStrategy<T>, jsonObject: JsonObject): T {
    return decodeFromJsonElement(deserializer, jsonObject)
}
