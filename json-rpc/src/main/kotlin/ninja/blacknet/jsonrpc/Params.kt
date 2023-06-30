/*
 * Copyright (c) 2023 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.jsonrpc

import kotlinx.serialization.Serializable
import kotlinx.serialization.json.JsonElement
import kotlinx.serialization.json.JsonPrimitive

/**
 * A type for the `params` field in a [Request].
 */
@Serializable
internal inline class Params private constructor(
    private val value: JsonElement
) {
    //TODO what types should be passed to constructors?

    init {
        require(value !is JsonPrimitive) { "$value is not object or array" }
    }
}
