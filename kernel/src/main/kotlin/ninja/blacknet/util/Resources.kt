/*
 * Copyright (c) 2020-2024 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.util

import java.io.BufferedReader
import java.io.File
import java.io.InputStreamReader
import java.nio.charset.Charset
import java.util.jar.JarFile

object Resources {
    fun file(context: Class<*>): File {
        return File(context.protectionDomain.codeSource.location.path)
    }

    fun jar(context: Class<*>): JarFile {
        return JarFile(file(context))
    }

    fun string(context: Class<*>, name: String, charset: Charset = Charsets.UTF_8): String {
        return reader(context, name, charset) {
            readLine()
        }
    }

    fun lines(context: Class<*>, name: String, charset: Charset = Charsets.UTF_8): ArrayList<String> {
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

    private inline fun <T> reader(context: Class<*>, name: String, charset: Charset, implementation: BufferedReader.() -> T): T {
        return jar(context).use {
            implementation(BufferedReader(InputStreamReader(it.getInputStream(it.getJarEntry(name)), charset)))
        }
    }
}
