/*
 * Copyright (c) 2019-2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.rpc

import io.ktor.application.call
import io.ktor.http.HttpStatusCode
import io.ktor.response.respond
import io.ktor.routing.Route
import io.ktor.routing.get
import kotlinx.coroutines.debug.DebugProbes
import mu.KotlinLogging
import ninja.blacknet.Config
import ninja.blacknet.Version
import ninja.blacknet.dataDir
import ninja.blacknet.time.currentTimeSeconds
import java.io.File
import java.io.PrintStream

private val logger = KotlinLogging.logger {}

fun Route.debug() {
    get("/api/dumpcoroutines") {
        if (Config.instance.debugcoroutines) {
            val file = File(dataDir, "coroutines_${currentTimeSeconds()}.log")
            val stream = PrintStream(file)
            stream.println("${Version.name} ${Version.revision}")
            DebugProbes.dumpCoroutines(stream)
            stream.close()
            call.respond(file.absolutePath)
        } else {
            call.respond(HttpStatusCode.BadRequest, "Not enabled in config or failed at runtime")
        }
    }
}
