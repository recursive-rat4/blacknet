/*
 * Copyright (c) 2018-2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.crypto

import javax.crypto.Mac
import javax.crypto.spec.SecretKeySpec

class KeyedHashWriterJvm(
        algorithm: String,
        key: ByteArray
) : HashWriter {
    private val mac = Mac.getInstance(algorithm).also {
        it.init(SecretKeySpec(key, algorithm))
    }

    override fun writeByte(value: Byte) {
        mac.update(value)
    }

    override fun writeByteArray(value: ByteArray) {
        mac.update(value)
    }

    override fun writeByteArray(value: ByteArray, offset: Int, length: Int) {
        mac.update(value, offset, length)
    }

    override fun reset() {
        mac.reset()
    }

    override fun finish(): ByteArray {
        return mac.doFinal()
    }
}
