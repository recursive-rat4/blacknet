/*
 * Copyright (c) 2019-2024 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet

import io.ktor.server.engine.ApplicationEngine
import ninja.blacknet.util.Resources

object Version {
    val name: String
    val version: String
    val revision: String

    val http_server: String
    val http_server_version: String
    val http_server_engine: String
    val http_server_engine_version: String

    init {
        name = when (val suffix = mode.agentSuffix) {
            null -> AGENT_NAME
            else -> AGENT_NAME + '-' + suffix
        }

        attributes(Kernel::class.java).run {
            version = getValue("Implementation-Version")
            revision = getValue("Build-Revision") ?: version
        }

        attributes(ApplicationEngine::class.java).run {
            http_server = "Ktor"
            http_server_version = getValue("Implementation-Version")
        }

        Kernel.ktorEngine::class.run {
            http_server_engine = simpleName ?: http_server
            http_server_engine_version = attributes(java).getValue("Implementation-Version")
        }
    }

    private fun attributes(context: Class<*>) = Resources.jar(context).use {
        it.getManifest().getMainAttributes()
    }
}
