/*
 * Copyright (c) 2018-2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.codec.base

/**
 * Bech32 address format.
 *
 * Bitcoin improvement proposal 173 "Base32 address format for native v0-16 witness outputs"
 */
object Bech32 : AbstractBech32() {
    override val POLYMOD_CONST: Int = 1
}
