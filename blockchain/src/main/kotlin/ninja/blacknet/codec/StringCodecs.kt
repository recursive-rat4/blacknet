/*
 * Copyright (c) 2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.codec

import ninja.blacknet.codec.base.*

/**
 * Returns the string codec for the given name.
 * @param codec the name of the codec
 * @return the [StringCodec]
 * @throws CodecException if there is no such codec
 */
@Throws(CodecException::class)
fun stringCodec(codec: String): StringCodec {
    return when (codec) {
        "base16" -> Base16
        "base32" -> throw NotImplementedError()
        "base64" -> throw NotImplementedError()
        "hex" -> Base16
        else -> throw CodecException("No such codec $codec")
    }
}
