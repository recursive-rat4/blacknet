/*
 * Copyright (c) 2019 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.api

import io.ktor.application.Application
import io.ktor.application.install
import io.ktor.features.DefaultHeaders
import io.ktor.routing.routing
import kotlinx.serialization.Serializable
import ninja.blacknet.ktor.requests.Request
import ninja.blacknet.ktor.requests.TextContent
import ninja.blacknet.ktor.requests.get
import ninja.blacknet.ktor.requests.respondJson

fun Application.PublicServer() {
    install(DefaultHeaders) {
        APIServer.configureHeaders(this)
    }

    routing {
        @Serializable
        class Supply : Request {
            override suspend fun handle(): TextContent {
                return respondJson(SupplyInfo.serializer(), SupplyInfo.get())
            }
        }

        get(Supply.serializer(), "/api/v1/supply")
    }
}
