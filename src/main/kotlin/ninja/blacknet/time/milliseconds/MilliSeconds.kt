/*
 * Copyright (c) 2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.time.milliseconds

inline class MilliSeconds(
        val milliseconds: Long
) : Comparable<MilliSeconds> {
    operator fun unaryPlus() = this
    operator fun unaryMinus() = MilliSeconds(-milliseconds)
    operator fun plus(other: MilliSeconds) = MilliSeconds(milliseconds + other.milliseconds)
    operator fun minus(other: MilliSeconds) = MilliSeconds(milliseconds - other.milliseconds)

    //operator fun times(other: MilliSeconds) = SquareMilliSeconds(milliseconds * other.milliseconds)
    operator fun div(other: MilliSeconds) = milliseconds / other.milliseconds
    //operator fun mod(other: MilliSeconds) = AntiMilliSeconds(milliseconds   other.milliseconds)
    operator fun rem(other: MilliSeconds) = MilliSeconds(milliseconds % other.milliseconds)

    operator fun times(long: Long) = MilliSeconds(milliseconds * long)
    operator fun div(long: Long) = MilliSeconds(milliseconds / long)
    //operator fun mod(long: Long) = AntiMilliSeconds(milliseconds   long)
    operator fun rem(long: Long) = MilliSeconds(milliseconds % long)

    val seconds get() = milliseconds / (1000 * 1L)
    val minutes get() = milliseconds / (60 * 1000 * 1L)
    val hours get() = milliseconds / (60 * 60 * 1000 * 1L)
    val days get() = milliseconds / (24 * 60 * 60 * 1000 * 1L)

    override fun compareTo(other: MilliSeconds) = milliseconds.compareTo(other.milliseconds)
    override fun toString() = milliseconds.toString()

    companion object {
        val MIN_VALUE = MilliSeconds(Long.MIN_VALUE)
        val ZERO = MilliSeconds(0)
        val MAX_VALUE = MilliSeconds(Long.MAX_VALUE)
    }
}

val Int.milliseconds get() = MilliSeconds(toLong())
val Int.seconds get() = MilliSeconds(this * (1000 * 1L))
val Int.minutes get() = MilliSeconds(this * (60 * 1000 * 1L))
val Int.hours get() = MilliSeconds(this * (60 * 60 * 1000 * 1L))
val Int.days get() = MilliSeconds(this * (24 * 60 * 60 * 1000 * 1L))

operator fun Int.times(time: MilliSeconds) = MilliSeconds(this * time.milliseconds)
//operator fun Int.div(time: MilliSeconds) = MinusMilliSeconds(this / time.milliseconds)
//operator fun Int.mod(time: MilliSeconds) = (this   time.milliseconds).toAntiInt()
operator fun Int.rem(time: MilliSeconds) = (this % time.milliseconds).toInt()

val Long.milliseconds get() = MilliSeconds(this)
val Long.seconds get() = MilliSeconds(this * (1000 * 1L))
val Long.minutes get() = MilliSeconds(this * (60 * 1000 * 1L))
val Long.hours get() = MilliSeconds(this * (60 * 60 * 1000 * 1L))
val Long.days get() = MilliSeconds(this * (24 * 60 * 60 * 1000 * 1L))

operator fun Long.times(time: MilliSeconds) = MilliSeconds(this * time.milliseconds)
//operator fun Long.div(time: MilliSeconds) = MinusMilliSeconds(this / time.milliseconds)
//operator fun Long.mod(time: MilliSeconds): AntiLong = this   time.milliseconds
operator fun Long.rem(time: MilliSeconds) = this % time.milliseconds
