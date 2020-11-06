/*
 * Copyright (c) 2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.rpc.requests

import io.ktor.application.ApplicationCallPipeline
import io.ktor.application.call
import io.ktor.http.ContentType
import io.ktor.http.HttpMethod
import io.ktor.http.HttpStatusCode
import io.ktor.request.receiveParameters
import io.ktor.response.respond
import io.ktor.routing.Route
import io.ktor.routing.route
import io.ktor.util.AttributeKey
import kotlinx.serialization.DeserializationStrategy
import kotlinx.serialization.SerializationStrategy
import ninja.blacknet.serialization.json.json
import ninja.blacknet.serialization.textModule

typealias TextContent = io.ktor.http.content.TextContent

/**
 * A respond to a client with a plain [text].
 *
 * @param text the plain text content
 */
fun respondText(text: String)
        = TextContent(text, ContentType.Text.Plain, HttpStatusCode.OK)

/**
 * A respond to a client with a plain text [message], using the [HttpStatusCode.BadRequest].
 *
 * @param message the plain text message
 */
fun respondError(message: String)
        = TextContent(message, ContentType.Text.Plain, HttpStatusCode.BadRequest)

/**
 * A respond to a client with a [value], using provided [serializer] and the [ContentType.Application.Json].
 *
 * @param serializer the serialization strategy
 * @param value the object serializable to JSON
 */
fun <T> respondJson(serializer: SerializationStrategy<T>, value: T)
        = TextContent(json.encodeToString(serializer, value), ContentType.Application.Json, HttpStatusCode.OK)

interface Request {
    /**
     * Handle a HTTP request to respond with a text content.
     */
    suspend fun handle(): TextContent
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
        call.attributes.put(requestKey, requestFormat.decodeFromParameters(serializer, when (method) {
            HttpMethod.Get -> call.parameters
            HttpMethod.Post -> call.receiveParameters()
            else -> throw Error("超文本傳輸協議請求方法 ${method.value} 的支持尚未實現")
        }))
    }
    handle {
        @Suppress("UNCHECKED_CAST")
        call.respond((call.attributes[requestKey] as T).handle())
    }
}

private val requestKey = AttributeKey<Any>("請求鍵")

private val requestFormat = RequestFormat(
        serializersModule = textModule
)
