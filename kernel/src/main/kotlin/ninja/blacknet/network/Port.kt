/*
 * Copyright (c) 2019-2024 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

@file:Suppress("INLINE_CLASS_DEPRECATED")

package ninja.blacknet.network

import kotlinx.serialization.Serializable

/**
 * Network port
 */
@Serializable
inline class Port(
    internal val value: UShort
) : Comparable<Port> {
    override fun toString(): String = value.toString()
    override fun compareTo(other: Port): Int = value.compareTo(other.value)

    /**
     * From Java representation
     */
    constructor(java: Int) : this(java.toUShort()) {
        require(java in 0..65535) { "Port must be in range 0..65535, actual value $java" }
    }

    /**
     * To Java representation
     */
    fun toJava(): Int = value.toInt()
}
