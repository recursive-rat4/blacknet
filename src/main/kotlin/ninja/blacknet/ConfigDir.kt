/*
 * Copyright (c) 2018-2019 Pavel Vasin
 * Copyright (c) 2019 Blacknet Team
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet

import java.io.File

val configDir: File get() = File(gittyPath("config"))

private fun gittyPath(path: String): String {
    if (Runtime.windowsOS) {
        val file = File(path)
        if (file.isFile()) {
            // git symlink
            return file.readText()
        }
    }
    return path
}
