/*
 * Copyright (c) 2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.serialization.config

import io.ktor.utils.io.charsets.Charset
import kotlinx.serialization.DeserializationStrategy
import kotlinx.serialization.SerialFormat
import kotlinx.serialization.modules.EmptySerializersModule
import kotlinx.serialization.modules.SerializersModule
import java.io.File
import java.util.Properties

class ConfigFormat(
        override val serializersModule: SerializersModule = EmptySerializersModule
) : SerialFormat {
    fun <T : Any?> decodeFromFile(strategy: DeserializationStrategy<T>, file: File, charset: Charset = Charsets.UTF_8): T {
        val decoder = ConfigDecoder(ConfigReader(file, charset), serializersModule)
        decoder.descriptor = strategy.descriptor
        return strategy.deserialize(decoder)
    }
}

typealias ConfigInput = Properties

typealias ConfigOutput = Properties
