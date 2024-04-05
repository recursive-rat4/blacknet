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
import java.util.Arrays

class ZeroInputStream(
) : InputStream() {
    override fun read(): Int {
        return 0
    }

    override fun read(b: ByteArray, off: Int, len: Int): Int {
        Arrays.fill(b, off, off + len, 0)
        return len
    }
}
