/*
 * Copyright (c) 2019-2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.api

import io.ktor.application.application
import io.ktor.application.call
import io.ktor.application.log
import io.ktor.http.HttpStatusCode
import io.ktor.response.respond
import io.ktor.routing.Route
import io.ktor.routing.get
import kotlinx.coroutines.debug.DebugProbes
import kotlinx.coroutines.launch
import ninja.blacknet.Config
import ninja.blacknet.Runtime
import ninja.blacknet.Version
import ninja.blacknet.dataDir
import ninja.blacknet.time.SystemClock
import java.io.File
import java.io.PrintStream

fun Route.debug() {
    get("/api/dumpcoroutines") {
        if (Config.instance.debugcoroutines) {
            val file = File(dataDir, "coroutines_${SystemClock.seconds}.log")
            val stream = PrintStream(file)
            stream.println("${Version.name} ${Version.revision}")
            DebugProbes.dumpCoroutines(stream)
            stream.close()
            call.respond(file.absolutePath)
        } else {
            call.respond(HttpStatusCode.BadRequest, "Not enabled in config or failed at runtime")
        }
    }

    get("/api/关机") {
        application.log.warn("正在关机着私人应用程序接口服务器。これはわたすのパソコンです。")
        Runtime.launch {
            call.respond(HttpStatusCode.Gone)
        }.join()
        kotlin.system.exitProcess(0)
    }

    get("/api/抛出") {
        application.log.warn("正在抛出着私人应用程序接口服务器异常。")
        throw RuntimeException("一条测试异常消息")
    }
}
