/*
 * Copyright (c) 2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.db

import ninja.blacknet.contract.BAppId

object BAppDB {
    private val bapps: Map<BAppId, *> = emptyMap<BAppId, Unit>()

    init {

    }

    fun isInteresting(id: BAppId): Boolean {
        return bapps.containsKey(id)
    }
}
