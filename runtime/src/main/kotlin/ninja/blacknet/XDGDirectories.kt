/*
 * Copyright (c) 2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet

import java.io.File

// https://specifications.freedesktop.org/basedir-spec/basedir-spec-0.8.html

private const val subdirectory = "Blacknet"

private fun XDGBaseDirectory(environmentVariable: String, defaultBase: String): File {
    val baseDir = System.getenv(environmentVariable)?.let { File(it) }
    return if (baseDir != null && baseDir.isAbsolute) {
        File(baseDir, subdirectory)
    } else {
        File("${System.getProperty("user.home")}/$defaultBase/$subdirectory")
    }
}

public fun XDGConfigDirectory(): File = XDGBaseDirectory("XDG_CONFIG_HOME", ".config")

public fun XDGDataDirectory(): File = XDGBaseDirectory("XDG_DATA_HOME", ".local/share")

public fun XDGStateDirectory(): File = XDGBaseDirectory("XDG_STATE_HOME", ".local/state")
