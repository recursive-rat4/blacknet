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
 * A type for the `error` field in a [Response].
 */
@Serializable
public class Error private constructor(
    private val code: Int,
    private val message: String,
    private val data: JsonElement? = null
) {
    public companion object {
        // Pre-defined errors -32768 to -32000

        /**
         * Invalid JSON was received by the server.
         * An error occurred on the server while parsing the JSON text.
         */
        internal fun parseError(data: JsonElement? = null) = Error(-32700, "Parse error", data)

        /**
         * The JSON sent is not a valid Request object.
         */
        internal fun invalidRequest(data: JsonElement? = null) = Error(-32600, "Invalid Request", data)

        /**
         * The method does not exist / is not available.
         */
        internal fun methodNotFound(data: JsonElement? = null) = Error(-32601, "Method not found", data)

        /**
         * Invalid method parameter(s).
         */
        internal fun invalidParams(data: JsonElement? = null) = Error(-32602, "Invalid params", data)

        /**
         * Internal JSON-RPC error.
         */
        internal fun internalError(data: JsonElement? = null) = Error(-32603, "Internal error", data)

        // Implementation-defined server-errors -32099 to -32000

        /**
         * An application-defined error.
         */
        public fun of(code: Int, message: String, data: JsonElement? = null): Error {
            require(code < -32768 || code > -32000) { "code $code is reserved for server errors" }
            return Error(code, message, data)
        }
    }
}
