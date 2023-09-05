/*
 * Copyright (c) 2023 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet

import java.io.File
import java.io.IOException
import ninja.blacknet.util.moveFile

/**
 * Atomic file move depends on Java implementation and filesystem of [dir].
 * @param dir directory where atomic moves will occur.
 * @throws IOException if not supported.
 */
fun testAtomicFileMove(dir: File) {
    val source = File.createTempFile("PowerOnSelfTest-", null, dir)
    val destination = File.createTempFile("PowerOnSelfTest-", null, dir)
    try {
        moveFile(source, destination)
    } finally {
        source.delete()
        destination.delete()
    }
}
