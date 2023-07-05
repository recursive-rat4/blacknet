/*
 * Copyright (c) 2019-2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet

import ninja.blacknet.util.Resources

object Version {
    val name: String
    val version: String
    val revision: String

    init {
        val jar = Resources.jar(Version::class.java)
        val attributes = jar.getManifest().getMainAttributes()
        jar.close()

        name = if (regtest)
            "$AGENT_NAME-regtest"
        else
            AGENT_NAME

        version = attributes.getValue("Implementation-Version")

        revision = attributes.getValue("Build-Revision")
                ?: version
    }

    const val http_server = "Ktor"

    const val http_server_version = "2.1.3"

    const val http_server_engine = "CIO"

    const val http_server_engine_version = "2.1.3"
}
