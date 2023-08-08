/*
 * Copyright (c) 2023 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.jsonrpc

import kotlinx.serialization.DeserializationStrategy
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.JsonArray
import kotlinx.serialization.json.JsonNull
import kotlinx.serialization.json.JsonObject

/**
 * Routing for [Request]s.
 */
internal inline class Routing private constructor(
    private val routes: HashMap<String, DeserializationStrategy<Handler<*>>>
) {
    constructor() : this(HashMap())

    /**
     * Register a [handler] for a [method] name.
     */
    fun <T> add(method: String, handler: DeserializationStrategy<Handler<T>>) {
        routes.put(method, handler)
        //XXX what if handler already registered?
    }

    /**
     * Handle a [request] by a registered handler.
     */
    fun <T> handle(request: Request): Either<Error, T> {
        @Suppress("UNCHECKED_CAST")
        val serializer = routes.get(request.method.value) as? DeserializationStrategy<Handler<T>> ?: return Left(Error.methodNotFound())
        val handler: Handler<T> = when (val jsonElement = request.params?.value ?: JsonNull) {
            is JsonObject -> Json.decodeNamedParameters(serializer, jsonElement)
            is JsonArray -> Json.decodePositionalParameters(serializer, jsonElement)
            else -> TODO()
        }
        return handler.handle()
    }
}
