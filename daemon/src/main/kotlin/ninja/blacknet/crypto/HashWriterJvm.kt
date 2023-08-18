/*
 * Copyright (c) 2018-2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.crypto

import java.security.MessageDigest

class HashWriterJvm(
        algorithm: String
) : HashWriter {
    private val digest = MessageDigest.getInstance(algorithm)
    private val digestLength = digest.getDigestLength()

    override fun writeByte(value: Byte) {
        digest.update(value)
    }

    override fun writeByteArray(value: ByteArray) {
        digest.update(value)
    }

    override fun writeByteArray(value: ByteArray, offset: Int, length: Int) {
        digest.update(value, offset, length)
    }

    override fun reset() {
        digest.reset()
    }

    override fun finish(): ByteArray {
        return digest.digest()
    }
}
