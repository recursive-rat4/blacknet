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

import java.nio.file.Files
import java.nio.file.Path
import java.nio.file.StandardOpenOption.CREATE_NEW
import java.util.jar.JarFile
import ninja.blacknet.util.Resources

val configDir: Path = run {
    val custom = System.getProperty("ninja.blacknet.configDir")
    val dir = if (System.getProperty("org.gradle.test.worker") != null) {
        Path.of("build/resources/main/config").also {
            println("Using config directory ${it.toAbsolutePath()}")
        }
    } else if (custom != null) {
        Path.of(custom)
    } else if (Runtime.macOS) {
        Path.of(System.getProperty("user.home"), "Library/Application Support/$XDG_SUBDIRECTORY")
    } else if (Runtime.windowsOS) {
        Path.of(System.getProperty("user.home"), "AppData\\Roaming\\$XDG_SUBDIRECTORY")
    } else {
        XDGConfigDirectory(XDG_SUBDIRECTORY)
    }
    Files.createDirectories(dir, *XDGDirectoryPermissions(dir))
    dir
}

private fun configFiles() = arrayOf(
        "blacknet.conf",
        "logging.properties",
        "rpc.conf",
        "rpcregtest.conf"
)

//XXX atomicity
fun populateConfigDir(): Int {
    var createdFiles = 0

    var jar: JarFile? = null
    for (name in configFiles()) {
        val file = configDir.resolve(name)
        if (Files.exists(file))
            continue
        if (jar == null)
            jar = Resources.jar(Kernel::class.java)
        val input = jar.getInputStream(jar.getJarEntry("config/$name"))
        val output = Files.newOutputStream(file, CREATE_NEW)
        input.transferTo(output)
        input.close()
        output.close()
        ++createdFiles
    }
    jar?.close()

    return createdFiles
}
