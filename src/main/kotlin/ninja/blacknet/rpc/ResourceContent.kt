/*
 * Copyright (c) 2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.rpc

import io.ktor.http.ContentType
import io.ktor.http.defaultForFilePath
import io.ktor.server.application.call
import io.ktor.server.http.content.JarFileContent
import io.ktor.server.response.respond
import io.ktor.server.routing.Route
import io.ktor.server.routing.get
import ninja.blacknet.util.Resources

fun Route.resource(context: Class<*>, resourceName: String) {
    get(resourceName) {
        call.respond(JarFileContent(Resources.file(context), resourceName, ContentType.defaultForFilePath(resourceName)))
    }
}

fun Route.resources(context: Class<*>, resourceName: String) {
    get("{path...}") {
        val path = resourceName + "/" + (call.parameters.getAll("path")?.joinToString("/") ?: return@get)
        call.respond(JarFileContent(Resources.file(context), path, ContentType.defaultForFilePath(path)))
    }
}
