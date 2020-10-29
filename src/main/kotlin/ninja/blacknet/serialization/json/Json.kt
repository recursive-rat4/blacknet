/*
 * Copyright (c) 2019 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.serialization.json

import kotlinx.serialization.json.Json

/**
 * Instance of JSON serialization.
 */
public val json: Json = Json {
    prettyPrint = System.getProperty("ninja.blacknet.serialization.json.indented")?.toBoolean() ?: false
    prettyPrintIndent = "    "
}
