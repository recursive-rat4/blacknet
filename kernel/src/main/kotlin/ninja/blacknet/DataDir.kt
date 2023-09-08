/*
 * Copyright (c) 2018-2020 Pavel Vasin
 * Copyright (c) 2019 Blacknet Team
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet

import java.io.File
import java.nio.file.Files

val dataDir: File = run {
    val custom = System.getProperty("ninja.blacknet.dataDir")
    var dir = if (custom != null) {
        File(custom)
    } else if (Runtime.macOS) {
        File(System.getProperty("user.home"), "Library/Application Support/$XDG_SUBDIRECTORY")
    } else if (Runtime.windowsOS) {
        File(System.getProperty("user.home"), "AppData\\Roaming\\$XDG_SUBDIRECTORY")
    } else {
        val new = XDGDataDirectory(XDG_SUBDIRECTORY).toFile()
        val old = File(System.getProperty("user.home"), ".blacknet")
        if (old.exists()) {
            if (new.exists()) throw RuntimeException("Both $old and $new exist")
            new.parentFile.mkdirs()
            if (!old.renameTo(new)) throw RuntimeException("Rename $old to $new not succeeded")
        }
        new
    }
    if (regtest) {
        dir = File(dir, "regtest")
    }
    dir.toPath().let {
        Files.createDirectories(it, *XDGDirectoryPermissions(it))
    }
    dir
}
