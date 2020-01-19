/*
 * Copyright (c) 2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.util

import ninja.blacknet.SystemService

/**
 * The number of bytes in a binary representation of a [Byte].
 */
@SystemService
val Byte.Companion.SIZE get() = 1

/**
 * The number of bytes in a binary representation of a [Short].
 */
@SystemService
val Short.Companion.SIZE get() = 2

/**
 * The number of bytes in a binary representation of a [Int].
 */
@SystemService
val Int.Companion.SIZE get() = 4

/**
 * The number of bytes in a binary representation of a [Long].
 */
@SystemService
val Long.Companion.SIZE get() = 8

/**
 * The number of bytes in a binary representation of a [Float].
 */
@SystemService
val Float.Companion.SIZE get() = 4

/**
 * The number of bytes in a binary representation of a [Double].
 */
@SystemService
val Double.Companion.SIZE get() = 8

/**
 * The number of bits in a binary representation of a [Byte].
 */
@SystemService
val Byte.Companion.BITS get() = 8

/**
 * The number of bits in a binary representation of a [Short].
 */
@SystemService
val Short.Companion.BITS get() = 16

/**
 * The number of bits in a binary representation of a [Int].
 */
@SystemService
val Int.Companion.BITS get() = 32

/**
 * The number of bits in a binary representation of a [Long].
 */
@SystemService
val Long.Companion.BITS get() = 64

/**
 * The number of bits in a binary representation of a [Float].
 */
@SystemService
val Float.Companion.BITS get() = 32

/**
 * The number of bits in a binary representation of a [Double].
 */
@SystemService
val Double.Companion.BITS get() = 64
