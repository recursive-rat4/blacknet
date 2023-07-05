/*
 * Copyright (c) 2018-2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.rpc

import io.ktor.server.application.call
import io.ktor.server.http.content.static
import io.ktor.server.response.respondRedirect
import io.ktor.server.routing.Route
import io.ktor.server.routing.get
import ninja.blacknet.Main

fun Route.html() {
    get("/") {
        call.respondRedirect("static/index.html")
    }

    static("static") {
        resources(Main::class.java, "html")
    }
}
