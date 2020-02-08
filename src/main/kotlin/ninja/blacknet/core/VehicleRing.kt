/*
 * Copyright (c) 2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.core

import kotlinx.serialization.Serializable
import ninja.blacknet.crypto.Salt

/**
 * 树车年轮「樹車年輪」
 */
@Serializable
class VehicleRing(private val int: Int) {
    override fun equals(other: Any?): Boolean = (other is VehicleRing) && int == other.int
    override fun hashCode(): Int = Salt.hashCode { x(int) }
    override fun toString(): String = int.toString()
}
