/*
 * Copyright (c) 2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.serialization.config

import java.nio.file.Path
import kotlinx.serialization.DeserializationStrategy
import kotlinx.serialization.SerialFormat
import kotlinx.serialization.modules.EmptySerializersModule
import kotlinx.serialization.modules.SerializersModule

public class ConfigFormat(
        override val serializersModule: SerializersModule = EmptySerializersModule()
) : SerialFormat {
    public fun <T : Any?> decodeFromFile(strategy: DeserializationStrategy<T>, file: Path): T {
        val decoder = ConfigDecoder(ConfigReader(file), serializersModule)
        return strategy.deserialize(decoder)
    }
}
