/*
 * Copyright (c) 2019 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet

import ninja.blacknet.util.Resources
import java.util.jar.Manifest

object Version {
    val name: String
    val version: String
    val revision: String

    init {
        val stream = Resources.stream(this, "META-INF/MANIFEST.MF")
        val attributes = Manifest(stream).getMainAttributes()
        stream.close()

        name = if (Config.instance.regtest)
            "Blacknet-regtest"
        else
            "Blacknet"

        version = attributes.getValue("Implementation-Version")

        revision = attributes.getValue("Build-Revision")
                ?: version
    }

    const val http_server = "Ktor"

    const val http_server_version = "1.3.2"

    const val http_server_engine = "Netty"

    const val http_server_engine_version = "4.1.44.Final"
}
