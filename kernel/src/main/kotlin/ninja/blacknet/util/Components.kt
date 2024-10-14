/*
 * Copyright (c) 2020-2024 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.util

import com.google.common.primitives.Shorts
import com.google.common.primitives.Ints
import com.google.common.primitives.Longs

/**
 * Byte components of primitive types.
 */

/**
 * Returns a short value from the byte parameters in the big-endian order.
 *
 * @param b1 the first byte of the short
 * @param b2 the second byte of the short
 * @return the yielded [Short]
 */
fun Short.Companion.fromBytes(b1: Byte, b2: Byte): Short {
    return (((b1.toInt()/*and 0xZZ*/)  shl  (1 * Byte.SIZE_BITS))  or
            ((b2.toInt()  and 0xFF  )/*shl  (0 * Byte.SIZE_BITS))*/)).toShort()
}

/**
 * Returns an int value from the byte parameters in the big-endian order.
 *
 * @param b1 the first byte of the int
 * @param b2 the second byte of the int
 * @param b3 the third byte of the int
 * @param b4 the fourth byte of the int
 * @return the yielded [Int]
 */
fun Int.Companion.fromBytes(b1: Byte, b2: Byte, b3: Byte, b4: Byte): Int {
    return (((b1.toInt()/*and 0xZZ*/)  shl  (3 * Byte.SIZE_BITS))  or
            ((b2.toInt()  and 0xFF  )  shl  (2 * Byte.SIZE_BITS))  or
            ((b3.toInt()  and 0xFF  )  shl  (1 * Byte.SIZE_BITS))  or
            ((b4.toInt()  and 0xFF  )/*shl  (0 * Byte.SIZE_BITS))*/))
}

/**
 * Returns a long value from the byte parameters in the big-endian order.
 *
 * @param b1 the first byte of the long
 * @param b2 the second byte of the long
 * @param b3 the third byte of the long
 * @param b4 the fourth byte of the long
 * @param b5 the fifth byte of the long
 * @param b6 the sixth byte of the long
 * @param b7 the seventh byte of the long
 * @param b8 the eighth byte of the long
 * @return the yielded [Long]
 */
fun Long.Companion.fromBytes(b1: Byte, b2: Byte, b3: Byte, b4: Byte, b5: Byte, b6: Byte, b7: Byte, b8: Byte): Long {
    return (((b1.toLong()/*and 0xZZ*/)  shl  (7 * Byte.SIZE_BITS))  or
            ((b2.toLong()  and 0xFF  )  shl  (6 * Byte.SIZE_BITS))  or
            ((b3.toLong()  and 0xFF  )  shl  (5 * Byte.SIZE_BITS))  or
            ((b4.toLong()  and 0xFF  )  shl  (4 * Byte.SIZE_BITS))  or
            ((b5.toLong()  and 0xFF  )  shl  (3 * Byte.SIZE_BITS))  or
            ((b6.toLong()  and 0xFF  )  shl  (2 * Byte.SIZE_BITS))  or
            ((b7.toLong()  and 0xFF  )  shl  (1 * Byte.SIZE_BITS))  or
            ((b8.toLong()  and 0xFF  )/*shl  (0 * Byte.SIZE_BITS))*/))
}

/**
 * Returns the byte array representation of this [Short] value in the big-endian byte order.
 *
 * @return the yielded [ByteArray]
 */
fun Short.toByteArray(): ByteArray {
    return Shorts.toByteArray(this);
}

/**
 * Returns the byte array representation of this [Int] value in the big-endian byte order.
 *
 * @return the yielded [ByteArray]
 */
fun Int.toByteArray(): ByteArray {
    return Ints.toByteArray(this);
}

/**
 * Returns the byte array representation of this [Long] value in the big-endian byte order.
 *
 * @return the yielded [ByteArray]
 */
fun Long.toByteArray(): ByteArray {
    return Longs.toByteArray(this);
}
