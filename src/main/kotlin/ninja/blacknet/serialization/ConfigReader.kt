/*
 * Copyright (c) 2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.serialization

import io.ktor.utils.io.charsets.Charset
import java.io.File

class ConfigReader(
        private val input: ConfigInput
) {
    constructor(file: File, charset: Charset = Charsets.UTF_8) : this(ConfigInput()) {
        val reader = file.reader(charset)
        try {
            input.load(reader)
        } finally {
            reader.close()
        }
    }

    fun hasKey(key: String): Boolean {
        return input.containsKey(key)
    }

    fun readString(key: String): String {
        return input.getProperty(key)!!.trim()
    }

    fun readList(key: String): List<String> {
        return input.getProperty(key)!!.split(',').map { it.trim() }
    }
}
