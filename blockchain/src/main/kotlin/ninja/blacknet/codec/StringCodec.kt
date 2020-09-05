/*
 * Copyright (c) 2019-2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.codec

/**
 * String codec encodes bytes to a string and decodes a string to bytes.
 */
interface StringCodec {
    /**
     * Returns a byte array representation of a string.
     * @param string the [String] to be decoded
     * @return the decoded [ByteArray]
     * @throws CodecException
     */
    @Throws(CodecException::class)
    fun decode(string: String): ByteArray

    /**
     * Returns a string representation of a byte array.
     * @param bytes the [ByteArray] to be encoded
     * @return the encoded [String]
     * @throws CodecException
     */
    @Throws(CodecException::class)
    fun encode(bytes: ByteArray): String
}
