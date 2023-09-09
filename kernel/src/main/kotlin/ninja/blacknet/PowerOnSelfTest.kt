/*
 * Copyright (c) 2023 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet

import java.io.IOException
import java.nio.file.Files
import java.nio.file.Path
import java.nio.file.StandardCopyOption.ATOMIC_MOVE

/**
 * Atomic file move depends on Java implementation and filesystem of [dir].
 * @param dir directory where atomic moves will occur.
 * @throws IOException if not supported.
 */
fun testAtomicFileMove(dir: Path) {
    val source = Files.createTempFile(dir, "PowerOnSelfTest-", null)
    val destination = Files.createTempFile(dir, "PowerOnSelfTest-", null)
    try {
        Files.move(source, destination, ATOMIC_MOVE)
    } finally {
        Files.deleteIfExists(source)
        Files.deleteIfExists(destination)
    }
}
