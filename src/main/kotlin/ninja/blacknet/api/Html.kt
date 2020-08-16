/*
 * Copyright (c) 2018-2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.api

import io.ktor.application.call
import io.ktor.http.content.static
import io.ktor.response.respondRedirect
import io.ktor.routing.Route
import io.ktor.routing.get
import ninja.blacknet.Main
import ninja.blacknet.ktor.content.resources

fun Route.html() {
    get("/") {
        call.respondRedirect("static/index.html")
    }

    static("static") {
        resources(Main::class.java, "html")
    }
}
