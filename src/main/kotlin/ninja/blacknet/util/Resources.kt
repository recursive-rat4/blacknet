/*
 * Copyright (c) 2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.util

import io.ktor.utils.io.charsets.Charset
import java.io.BufferedReader
import java.io.InputStreamReader
import java.net.URL

object Resources {
    fun string(context: Any, name: String, charset: Charset = Charsets.UTF_8): String {
        return reader(context, name, charset) {
            readLine()
        }
    }

    fun lines(context: Any, name: String, charset: Charset = Charsets.UTF_8): ArrayList<String> {
        return reader(context, name, charset) {
            val result = ArrayList<String>()
            while (true) {
                val line = readLine()
                if (line != null)
                    result.add(line)
                else
                    break
            }
            result
        }
    }

    private inline fun <T> reader(context: Any, name: String, charset: Charset, implementation: BufferedReader.() -> T): T {
        val reader = BufferedReader(InputStreamReader(URL("jar:${context::class.java.protectionDomain.codeSource.location}!/$name").openStream(), charset))
        return try {
            implementation(reader)
        } finally {
            reader.close()
        }
    }
}
