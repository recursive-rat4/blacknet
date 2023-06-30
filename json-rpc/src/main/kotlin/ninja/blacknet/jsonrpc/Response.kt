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
 * A JSON-RPC response to a [Request] that matches by the [id].
 */
@Serializable
internal class Response private constructor(
    private val jsonrpc: Version,
    private val result: JsonElement? = null,
    private val error: Error? = null,
    private val id: Id
) {
    init {
        require((result != null) xor (error != null)) { "Neither success nor error" }
    }
}
