/*
 * Copyright (c) 2024 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.serialization.bbf

import java.io.OutputStream

class ByteArrayOutputStream(
    private val bytes: ByteArray,
) : OutputStream() {
    private var i = 0

    override fun write(b: Int) {
        bytes[i] = b.toByte()
        i += 1
    }

    override fun write(b: ByteArray, off: Int, len: Int) {
        System.arraycopy(b, off, bytes, i, len)
        i += len
    }
}
