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

private const val JSON_RPC_VERSION = "2.0"

/**
 * A type for the `jsonrpc` field in a [Request] or [Response].
 */
@Serializable
internal inline class Version private constructor(
    private val value: String
) {
    /**
     * Use a default version.
     */
    constructor() : this(JSON_RPC_VERSION)

    init {
        require(value == JSON_RPC_VERSION) { "Unknown JSON-RPC version $value" }
    }
}
