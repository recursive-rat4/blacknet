/*
 * Copyright (c) 2023 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.jsonrpc

import kotlinx.serialization.json.JsonPrimitive

//TODO send upstream

/**
 * Returns content of current element as byte
 * @throws NumberFormatException if current element is not a valid representation of number
 */
public val JsonPrimitive.byte: Byte get() = content.toByte()

/**
 * Returns content of current element as short
 * @throws NumberFormatException if current element is not a valid representation of number
 */
public val JsonPrimitive.short: Short get() = content.toShort()

/**
 * Returns content of current element as char
 * @throws IllegalArgumentException if current element is not a valid representation of symbol
 */
public val JsonPrimitive.char: Char
    get() {
        require(content.length == 1)
        return content[0]
    }

//TODO some checks?
/**
 * Returns content of current element as string
 */
public val JsonPrimitive.string: String get() = content
