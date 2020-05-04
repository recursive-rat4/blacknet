/*
 * Copyright (c) 2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.ktor.requests

import io.ktor.application.ApplicationCall
import io.ktor.application.ApplicationCallPipeline
import io.ktor.application.call
import io.ktor.http.HttpMethod
import io.ktor.request.receiveParameters
import io.ktor.routing.Route
import io.ktor.routing.route
import io.ktor.util.AttributeKey
import io.ktor.util.Attributes
import kotlinx.serialization.DeserializationStrategy

interface Request {
    suspend fun handle(call: ApplicationCall): Unit
}

fun <T : Request> Route.get(
        serializer: DeserializationStrategy<T>,
        path: String
) {
    route(path, HttpMethod.Get) {
        handle(HttpMethod.Get, serializer)
    }
}

fun <T : Request> Route.post(
        serializer: DeserializationStrategy<T>,
        path: String
) {
    route(path, HttpMethod.Post) {
        handle(HttpMethod.Post, serializer)
    }
}

private fun <T : Request> Route.handle(
        method: HttpMethod,
        serializer: DeserializationStrategy<T>
) {
    intercept(ApplicationCallPipeline.Features) {
        call.attributes[requestKey] = RequestDecoder(RequestReader(when (method) {
            HttpMethod.Get -> call.parameters
            HttpMethod.Post -> call.receiveParameters()
            else -> throw Error("超文本傳輸協議請求方法 ${method.value} 的支持尚未實現")
        })).decode(serializer)
    }
    handle {
        @Suppress("UNCHECKED_CAST")
        (call.attributes[requestKey] as T).handle(call)
    }
}

// Warning: nay map put should be converted to assigment
private operator fun <T : Any> Attributes.set(key: AttributeKey<T>, value: T): Unit = put(key, value)

private val requestKey = AttributeKey<Any>("請求鍵")
