/*
 * Copyright (c) 2024 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.io

import java.io.EOFException
import java.io.InputStream

class ByteArrayInputStream(
    private val bytes: ByteArray,
) : InputStream() {
    private var i = 0

    override fun read(): Int {
        val byte = bytes[i]
        i += 1
        return byte.toUByte().toInt()
    }

    override fun read(b: ByteArray, off: Int, len: Int): Int {
        System.arraycopy(bytes, i, b, off, len)
        i += len
        return len
    }

    override fun available(): Int {
        return bytes.size - i
    }

    override fun skip(n: Long): Long {
        return if (n > 0) {
            val s = minOf(n, available().toLong()).toInt()
            i += s
            s.toLong()
        } else {
            0
        }
    }

    override fun skipNBytes(n: Long) {
        if (available() >= n)
            i += n.toInt()
        else
            throw EOFException()
    }
}

fun ByteArray.inputStream(): ByteArrayInputStream = ByteArrayInputStream(this)
