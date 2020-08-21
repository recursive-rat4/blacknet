/*
 * Copyright (c) 2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.logging

import java.util.logging.LogManager
import kotlin.Unit

class LogManager() : LogManager() {
    override fun reset() = Unit

    fun shutDown() = super.reset()
}
