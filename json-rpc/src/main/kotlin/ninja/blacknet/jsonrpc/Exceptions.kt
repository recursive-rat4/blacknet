/*
 * Copyright (c) 2023 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.jsonrpc

import kotlinx.serialization.json.JsonElement

/**
 * A JSON-RPC exception that can be sent as [Error].
 */
public open class Exception(
    public val code: Int,
    override val message: String,
    public val data: JsonElement? = null,
    cause: Throwable? = null,
) : kotlin.Exception(message, cause) {
    /**
     * An application-defined error.
     */
    public constructor(message: String, data: JsonElement? = null) : this(-31000, message, data)
}

/**
 * Invalid JSON was received by the server.
 * An error occurred on the server while parsing the JSON text.
 */
internal class ParseError(data: JsonElement? = null, cause: Throwable? = null) :
    Exception(-32700, "Parse error", data, cause)

/**
 * The JSON sent is not a valid Request object.
 */
internal class InvalidRequest(data: JsonElement? = null, cause: Throwable? = null) :
    Exception(-32600, "Invalid Request", data, cause)

/**
 * The method does not exist / is not available.
 */
internal class MethodNotFound(data: JsonElement? = null, cause: Throwable? = null) :
    Exception(-32601, "Method not found", data, cause)

/**
 * Invalid method parameter(s).
 */
internal class InvalidParams(data: JsonElement? = null, cause: Throwable? = null) :
    Exception(-32602, "Invalid params", data, cause)

/**
 * Internal JSON-RPC error.
 */
internal class InternalError(data: JsonElement? = null, cause: Throwable? = null) :
    Exception(-32603, "Internal error", data, cause)
