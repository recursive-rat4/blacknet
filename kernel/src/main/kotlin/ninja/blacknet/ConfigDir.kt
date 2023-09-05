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
import java.io.FileOutputStream
import java.util.jar.JarFile
import ninja.blacknet.util.Resources

//FIXME permission 0700

val configDir: File = run {
    val custom = System.getProperty("ninja.blacknet.configDir")
    val dir = if (System.getProperty("org.gradle.test.worker") != null) {
        File("build/resources/main/config").also {
            println("Using config directory ${it.absolutePath}")
        }
    } else if (custom != null) {
        File(custom)
    } else if (Runtime.macOS) {
        File(System.getProperty("user.home"), "Library/Application Support/$XDG_SUBDIRECTORY")
    } else if (Runtime.windowsOS) {
        File(System.getProperty("user.home"), "AppData\\Roaming\\$XDG_SUBDIRECTORY")
    } else {
        XDGConfigDirectory(XDG_SUBDIRECTORY)
    }
    dir.mkdirs()
    dir
}

private fun configFiles() = arrayOf(
        "blacknet.conf",
        "logging.properties",
        "rpc.conf",
        "rpcregtest.conf"
)

fun populateConfigDir(): Int {
    var createdFiles = 0

    var jar: JarFile? = null
    for (name in configFiles()) {
        val file = File(configDir, name)
        if (file.exists())
            continue
        if (jar == null)
            jar = Resources.jar(Version::class.java)
        val input = jar.getInputStream(jar.getJarEntry("config/$name"))
        val output = FileOutputStream(file)
        input.transferTo(output)
        input.close()
        output.close()
        ++createdFiles
    }
    jar?.close()

    return createdFiles
}
