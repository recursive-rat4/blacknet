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
import kotlinx.serialization.json.JsonNull

internal inline class Routing private constructor(
    private val routes: HashMap<String, DeserializationStrategy<Handler<*>>>
) {
    constructor() : this(HashMap())

    fun <T> add(method: String, handler: DeserializationStrategy<Handler<T>>) {
        routes.put(method, handler)
    }

    fun <T> handle(request: Request): T {
        @Suppress("UNCHECKED_CAST")
        val serializer = routes.get(request.method.value) as? DeserializationStrategy<Handler<T>> ?: throw MethodNotFound()
        val handler: Handler<T> = Json.decodeFromJsonElement(serializer, request.params?.value ?: JsonNull)
        return handler.handle()
    }
}
