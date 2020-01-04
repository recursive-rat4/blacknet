/*
 * Copyright (c) 2019 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet

import com.google.common.io.Resources

object Version {
    val name = if (Config.regTest) "Blacknet-regtest" else "Blacknet"

    val version: String = Resources.toString(Resources.getResource("version.txt"), Charsets.US_ASCII)

    const val http_server = "ktor"

    val http_server_version: String = Resources.toString(Resources.getResource("ktor_version.txt"), Charsets.US_ASCII)
}
