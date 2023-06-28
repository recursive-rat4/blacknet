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

/**
 * A type for the `error` field in a response.
 */
@Serializable
internal class Error private constructor(
    private val code: Int,
    private val message: String,
    private val data: JsonElement? = null
)
