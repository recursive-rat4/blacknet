/*
 * Copyright (c) 2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

@file:Suppress("NOTHING_TO_INLINE")

package ninja.blacknet.byte

/**
 * Byte components of primitive types.
 */

/**
 * Returns the short from a big-endian representation.
 */
inline fun Short.Companion.fromBytes(b1: Byte, b2: Byte): Short = com.google.common.primitives.Shorts.fromBytes(b1, b2)

/**
 * Returns the int from a big-endian representation.
 */
inline fun Int.Companion.fromBytes(b1: Byte, b2: Byte, b3: Byte, b4: Byte): Int = com.google.common.primitives.Ints.fromBytes(b1, b2, b3, b4)

/**
 * Returns the long from a big-endian representation.
 */
inline fun Long.Companion.fromBytes(b1: Byte, b2: Byte, b3: Byte, b4: Byte, b5: Byte, b6: Byte, b7: Byte, b8: Byte): Long = com.google.common.primitives.Longs.fromBytes(b1, b2, b3, b4, b5, b6, b7, b8)

/**
 * Returns the big-endian representation.
 */
inline fun Short.toByteArray(): ByteArray = com.google.common.primitives.Shorts.toByteArray(this)

/**
 * Returns the big-endian representation.
 */
inline fun Int.toByteArray(): ByteArray = com.google.common.primitives.Ints.toByteArray(this)

/**
 * Returns the big-endian representation.
 */
inline fun Long.toByteArray(): ByteArray = com.google.common.primitives.Longs.toByteArray(this)

/**
 * Returns the first byte of the big-endian representation.
 */
inline operator fun Short.component1() = (this.toInt() shr 1 * Byte.SIZE_BITS).toByte()

/**
 * Returns the second byte of the big-endian representation.
 */
inline operator fun Short.component2() = toByte()
//inline operator fun Short.component2() = (this.toInt() shr 0 * Byte.SIZE_BITS).toByte()

/**
 * Returns the first byte of the big-endian representation.
 */
inline operator fun Int.component1() = (this shr 3 * Byte.SIZE_BITS).toByte()

/**
 * Returns the second byte of the big-endian representation.
 */
inline operator fun Int.component2() = (this shr 2 * Byte.SIZE_BITS).toByte()

/**
 * Returns the third byte of the big-endian representation.
 */
inline operator fun Int.component3() = (this shr 1 * Byte.SIZE_BITS).toByte()

/**
 * Returns the fourth byte of the big-endian representation.
 */
inline operator fun Int.component4() = toByte()
//inline operator fun Int.component4() = (this shr 0 * Byte.SIZE_BITS).toByte()

/**
 * Returns the first byte of the big-endian representation.
 */
inline operator fun Long.component1() = (this shr 7 * Byte.SIZE_BITS).toByte()

/**
 * Returns the second byte of the big-endian representation.
 */
inline operator fun Long.component2() = (this shr 6 * Byte.SIZE_BITS).toByte()

/**
 * Returns the third byte of the big-endian representation.
 */
inline operator fun Long.component3() = (this shr 5 * Byte.SIZE_BITS).toByte()

/**
 * Returns the fourth byte of the big-endian representation.
 */
inline operator fun Long.component4() = (this shr 4 * Byte.SIZE_BITS).toByte()

/**
 * Returns the fifth byte of the big-endian representation.
 */
inline operator fun Long.component5() = (this shr 3 * Byte.SIZE_BITS).toByte()

/**
 * Returns the sixth byte of the big-endian representation.
 */
inline operator fun Long.component6() = (this shr 2 * Byte.SIZE_BITS).toByte()

/**
 * Returns the seventh byte of the big-endian representation.
 */
inline operator fun Long.component7() = (this shr 1 * Byte.SIZE_BITS).toByte()

/**
 * Returns the eighth byte of the big-endian representation.
 */
inline operator fun Long.component8() = toByte()
//inline operator fun Long.component8() = (this shr 0 * Byte.SIZE_BITS).toByte()
