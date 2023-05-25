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
) : Comparable<Milliseconds> {
    override fun toString(): String = milliseconds.toString()

    override fun compareTo(other: Milliseconds): Int = milliseconds.compareTo(other.milliseconds)

    public companion object {
        /**
         * The zero value that is represented by [Milliseconds].
         */
        public val ZERO: Milliseconds = Milliseconds(0)

        /**
         * The minimum value that can be represented by [Milliseconds].
         */
        public val MIN_VALUE: Milliseconds = Milliseconds(Long.MIN_VALUE)

        /**
         * The maximum value that can be represented by [Milliseconds].
         */
        public val MAX_VALUE: Milliseconds = Milliseconds(Long.MAX_VALUE)
    }
}
