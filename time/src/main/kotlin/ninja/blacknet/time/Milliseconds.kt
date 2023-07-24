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

    /**
     * @return the same value.
     */
    public operator fun unaryPlus(): Milliseconds = Milliseconds(+milliseconds)

    /**
     * @return the negated value.
     */
    public operator fun unaryMinus(): Milliseconds = Milliseconds(-milliseconds)

    /**
     * @return the sum of two values.
     */
    public operator fun plus(other: Milliseconds): Milliseconds = Milliseconds(milliseconds + other.milliseconds)

    /**
     * @return the difference of two values.
     */
    public operator fun minus(other: Milliseconds): Milliseconds = Milliseconds(milliseconds - other.milliseconds)

    /**
     * @return the product of two values.
     */
    public operator fun times(number: Long): Milliseconds = Milliseconds(milliseconds * number)

    /**
     * @return the quotient of two values.
     */
    public operator fun div(other: Milliseconds): Long = milliseconds / other.milliseconds

    /**
     * @return the quotient of two values.
     */
    public operator fun div(number: Long): Milliseconds = Milliseconds(milliseconds / number)

    /**
     * @return the remainder of two values.
     */
    public operator fun rem(other: Milliseconds): Milliseconds = Milliseconds(milliseconds % other.milliseconds)

    /**
     * @return the remainder of two values.
     */
    public operator fun rem(number: Long): Milliseconds = Milliseconds(milliseconds % number)

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

        /**
         * @return the converted number of seconds into [Milliseconds].
         */
        public val Int.seconds: Milliseconds get() = Milliseconds(this * 1000L)

        /**
         * @return the converted number of minutes into [Milliseconds].
         */
        public val Int.minutes: Milliseconds get() = Milliseconds(this * 60000L)

        /**
         * @return the converted number of hours into [Milliseconds].
         */
        public val Int.hours: Milliseconds get() = Milliseconds(this * 3600000L)

        /**
         * @return the converted number of days into [Milliseconds].
         */
        public val Int.days: Milliseconds get() = Milliseconds(this * 86400000L)
    }
}
