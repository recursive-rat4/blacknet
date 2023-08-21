/*
 * Copyright (c) 2020-2023 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.codec.base

import java.util.HexFormat

public val Base16: HexFormat = HexFormat.of().run {
    if (System.getProperty("ninja.blacknet.codec.base.hex.lowercase") != "true")
        withUpperCase()
    else
        withLowerCase()
}

public fun HexFormat.encode(bytes: ByteArray): String = formatHex(bytes)

public fun HexFormat.decode(string: String): ByteArray = parseHex(string)
