/*
 * Copyright (c) 2019 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.api

import kotlinx.serialization.Serializable
import ninja.blacknet.transaction.Bundle

@Serializable
class BundleInfo(
        val magic: Int,
        val data: String
) {
    constructor(data: Bundle) : this(
            data.magic,
            data.data.toString()
    )
}
