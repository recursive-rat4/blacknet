/*
 * Copyright (c) 2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.rpc.requests

import io.ktor.http.Parameters
import kotlinx.serialization.DeserializationStrategy
import kotlinx.serialization.SerialFormat
import kotlinx.serialization.modules.EmptySerializersModule
import kotlinx.serialization.modules.SerializersModule

class RequestFormat(
        override val serializersModule: SerializersModule = EmptySerializersModule()
) : SerialFormat {
    fun <T : Any?> decodeFromParameters(strategy: DeserializationStrategy<T>, parameters: Parameters): T {
        val decoder = RequestDecoder(RequestReader(parameters), serializersModule)
        return strategy.deserialize(decoder)
    }
}
