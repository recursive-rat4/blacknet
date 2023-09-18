/*
 * Copyright (c) 2023 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet

import java.nio.file.Files
import java.nio.file.Path

val stateDir: Path = run {
    val custom = System.getProperty("ninja.blacknet.stateDir")
    var dir = if (custom != null) {
        Path.of(custom)
    } else if (Runtime.macOS) {
        Path.of(System.getProperty("user.home"), "Library/Application Support/$XDG_SUBDIRECTORY")
    } else if (Runtime.windowsOS) {
        Path.of(System.getProperty("user.home"), "AppData\\Roaming\\$XDG_SUBDIRECTORY")
    } else {
        XDGStateDirectory(XDG_SUBDIRECTORY)
    }
    mode.subdirectory?.let {
        dir = dir.resolve(it)
    }
    Files.createDirectories(dir, *XDGDirectoryPermissions(dir))
    dir
}
