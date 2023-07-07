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

/**
 * A JSON-RPC request that must be replied with a [Response] if an [id] is present, otherwise it's a notification.
 */
@Serializable
internal class Request private constructor(
    private val jsonrpc: Version,
    internal val method: Method,
    internal val params: Params? = null,
    private val id: Id? = null
) {
    fun isNotification(): Boolean = id == null
}
