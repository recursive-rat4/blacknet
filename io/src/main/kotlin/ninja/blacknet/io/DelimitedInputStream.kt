/*
 * Copyright (c) 2024 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.io

import java.io.IOException
import java.io.InputStream

class DelimitedInputStream(
    private val stream: InputStream,
) : InputStream() {
    private var message: Int = Int.MAX_VALUE

    fun begin(size: Int) {
        message = size
    }

    fun end() {
        if (message == 0)
            message = Int.MAX_VALUE
        else
            throw IOException("$message trailing bytes")
    }

    override fun read(): Int = stream.read().also {
        if (it >= 0) {
            message -= 1
            if (message < 0)
                throw IOException("${-message} bytes read past end of message")
        }
    }

    override fun read(b: ByteArray, off: Int, len: Int): Int = stream.read(b, off, len).also {
        if (it > 0) {
            message -= it
            if (message < 0)
                throw IOException("${-message} bytes read past end of message")
        }
    }

    override fun available(): Int = stream.available()

    override fun close(): Unit = stream.close()

    override fun skip(n: Long): Long {
        return if (n > 0) {
            val s = stream.skip(n)
            if (s <= message) {
                message -= s.toInt()
                s
            } else {
                throw IOException("${-(message - s)} bytes read past end of message")
            }
        } else {
            0
        }
    }

    override fun skipNBytes(n: Long) {
        if (n > 0) {
            if (n <= message) {
                stream.skipNBytes(n)
                message -= n.toInt()
            } else {
                throw IOException("${-(message - n)} bytes read past end of message")
            }
        }
    }
}

fun InputStream.delimited(): DelimitedInputStream = DelimitedInputStream(this)
