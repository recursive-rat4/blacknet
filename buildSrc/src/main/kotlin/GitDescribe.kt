/*
 * Copyright (c) 2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

import org.eclipse.jgit.api.Git
import org.eclipse.jgit.util.FS
import java.io.File
import java.io.IOException

fun dirtyDescribeGit(directory: File): String {
    @Suppress("UNUSED_VARIABLE")
    val description = "Describes the state of the Git repository"
    FS.DETECTED.setUserHome(File(directory, "buildSrc/build"))
    val git = try {
        Git.open(directory, FS.DETECTED)
    } catch (e: IOException) {
        println("Execution failed for task ':dirtyDescribeGit'.")
        println("> ${e::class.qualifiedName}: ${e.message}")
        return ""
    }
    val describe = git.describe().call()
    val status = git.status().call()
    return (if (status.hasUncommittedChanges()) {
        "$describe-dirty"
    } else {
        describe
    }).also {
        git.close().also {
            Git.shutdown()
        }
    }
}
