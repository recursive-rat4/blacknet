/*
 * Copyright (c) 2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet

import java.lang.UnsupportedOperationException
import java.nio.file.Files
import java.nio.file.Path
import java.nio.file.attribute.FileAttribute
import java.nio.file.attribute.PosixFilePermission
import java.nio.file.attribute.PosixFilePermission.*
import java.nio.file.attribute.PosixFilePermissions
import java.util.EnumSet

// https://specifications.freedesktop.org/basedir-spec/basedir-spec-0.8.html

private fun XDGBaseDirectory(subdirectory: String, environmentVariable: String, defaultBase: String): Path {
    val baseDir = System.getenv(environmentVariable)?.let { Path.of(it) }
    return if (baseDir != null && baseDir.isAbsolute) {
        baseDir.resolve(subdirectory)
    } else {
        Path.of(System.getProperty("user.home"), defaultBase, subdirectory)
    }
}

public fun XDGConfigDirectory(subdirectory: String): Path = XDGBaseDirectory(subdirectory, "XDG_CONFIG_HOME", ".config")

public fun XDGDataDirectory(subdirectory: String): Path = XDGBaseDirectory(subdirectory, "XDG_DATA_HOME", ".local/share")

public fun XDGStateDirectory(subdirectory: String): Path = XDGBaseDirectory(subdirectory, "XDG_STATE_HOME", ".local/state")

public fun XDGDirectoryPermissions(path: Path): Array<FileAttribute<Set<PosixFilePermission>>> {
    var target = path
    while (Files.notExists(target))
        target = target.parent
    val fileStore = Files.getFileStore(target)
    return if (fileStore.supportsFileAttributeView("posix")) {
        arrayOf(PosixFilePermissions.asFileAttribute(EnumSet.of(OWNER_READ, OWNER_WRITE, OWNER_EXECUTE)))
    } else if (fileStore.supportsFileAttributeView("acl")) {
        emptyArray() //WINDOWS are default permissions ok?
    } else {
        val builder = StringBuilder()
        builder.append("Mount point ")
        builder.append(fileStore)
        builder.append(" supports only following permissions:")
        target.fileSystem.supportedFileAttributeViews().forEach { name ->
            if (fileStore.supportsFileAttributeView(name)) {
                builder.append(' ')
                builder.append(name)
            }
        }
        throw UnsupportedOperationException(builder.toString())
    }
}
