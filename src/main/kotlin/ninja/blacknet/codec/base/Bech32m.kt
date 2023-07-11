/*
 * Copyright (c) 2018-2023 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.codec.base

/**
 * Bech32m address format.
 *
 * Bitcoin improvement proposal 350 "Bech32m format for v1+ witness addresses"
 */
object Bech32m : AbstractBech32() {
    override val POLYMOD_CONST: Int = 0x2BC830A3
}
