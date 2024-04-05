/*
 * Copyright (c) 2019-2024 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.rpc

import com.sun.management.HotSpotDiagnosticMXBean
import com.sun.management.HotSpotDiagnosticMXBean.ThreadDumpFormat.TEXT_PLAIN
import io.github.oshai.kotlinlogging.KotlinLogging
import io.ktor.http.HttpStatusCode
import io.ktor.server.application.call
import io.ktor.server.response.respond
import io.ktor.server.routing.Route
import io.ktor.server.routing.get
import java.io.PrintStream
import java.lang.management.ManagementFactory
import java.nio.file.Files
import java.nio.file.StandardOpenOption.APPEND
import java.nio.file.StandardOpenOption.WRITE
import kotlinx.coroutines.debug.DebugProbes
import ninja.blacknet.Kernel
import ninja.blacknet.Version
import ninja.blacknet.stateDir
import ninja.blacknet.time.currentTimeSeconds

private val logger = KotlinLogging.logger {}

fun Route.debug() {
    get("/api/dumpcoroutines") {
        if (Kernel.config().debugcoroutines) {
            val file = stateDir.resolve("coroutines_${currentTimeSeconds()}.log")
            val stream = PrintStream(file.toString())
            stream.println("${Version.name} ${Version.version}")
            DebugProbes.dumpCoroutines(stream)
            stream.close()
            call.respond(file.toAbsolutePath())
        } else {
            call.respond(HttpStatusCode.BadRequest, "Not enabled in config or failed at runtime")
        }
    }

    get("/api/dumpthreads") {
        val filePath = stateDir.resolve("threads_${currentTimeSeconds()}.dump").toAbsolutePath()
        val file = filePath.toString()
        ManagementFactory.getPlatformMXBean(HotSpotDiagnosticMXBean::class.java).dumpThreads(file, TEXT_PLAIN)
        Files.newBufferedWriter(filePath, WRITE, APPEND).use {
            it.newLine()
            it.write("=".repeat(80))
            it.newLine()
            it.write("${Version.name} ${Version.version}")
            it.newLine()
        }
        call.respond(file)
    }
}
