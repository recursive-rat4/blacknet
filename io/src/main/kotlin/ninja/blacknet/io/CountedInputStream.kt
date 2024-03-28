/*
 * Copyright (c) 2024 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.io

import java.io.InputStream

class CountedInputStream(
    private val stream: InputStream,
) : InputStream() {
    @Volatile
    public var bytesRead: Long = 0
        private set

    override fun read(): Int = stream.read().also {
        if (it >= 0)
            bytesRead += 1
    }

    override fun read(b: ByteArray, off: Int, len: Int): Int = stream.read(b, off, len).also {
        if (it > 0)
            bytesRead += it
    }

    override fun available(): Int = stream.available()

    override fun close(): Unit = stream.close()
}

fun InputStream.counted(): CountedInputStream = CountedInputStream(this)
