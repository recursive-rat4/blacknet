/*
 * Copyright (c) 2020-2023 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.time

/**
 * A timestamp or a time interval measured in milliseconds. The value may be negative.
 */
public inline class Milliseconds(
    /*TODO private*/public val milliseconds: Long
) {
    override fun toString(): String = milliseconds.toString()
}
