/*
 * Copyright (c) 2024 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.io

import java.io.OutputStream

class CountedOutputStream(
    private val stream: OutputStream,
) : OutputStream() {
    @Volatile
    public var bytesWritten: Long = 0
        private set

    override fun write(b: Int) = stream.write(b).also {
        bytesWritten += 1
    }

    override fun write(b: ByteArray, off: Int, len: Int) = stream.write(b, off, len).also {
        bytesWritten += len
    }

    override fun flush(): Unit = stream.flush()

    override fun close(): Unit = stream.close()
}

fun OutputStream.counted(): CountedOutputStream = CountedOutputStream(this)
