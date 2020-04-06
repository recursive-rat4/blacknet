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

val dataDir: File = {
    var dir = if (Config.instance.portable) {
        File("db")
    } else if (Config.instance.datadir == null) {
        if (Runtime.macOS) {
            File(System.getProperty("user.home"), "Library/Application Support/Blacknet")
        } else if (Runtime.windowsOS) {
            File(System.getProperty("user.home"), "AppData\\Roaming\\Blacknet")
        } else {
            // https://specifications.freedesktop.org/basedir-spec/basedir-spec-0.7.html
            val xdgDataHome = System.getenv("XDG_DATA_HOME")?.let { File(it) }
            val new = if (xdgDataHome != null && xdgDataHome.isAbsolute) {
                File(xdgDataHome, "Blacknet")
            } else {
                File(System.getProperty("user.home"), ".local/share/Blacknet")
            }
            val old = File(System.getProperty("user.home"), ".blacknet")
            if (old.exists()) {
                if (new.exists()) throw RuntimeException("Both $old and $new exist")
                new.parentFile.mkdirs()
                if (!old.renameTo(new)) throw RuntimeException("Rename $old to $new not succeeded")
            }
            new
        }
    } else {
        File(Config.instance.datadir)
    }
    if (Config.instance.regtest) {
        dir = File(dir, "regtest")
    }
    dir.mkdirs()
    dir
}()
