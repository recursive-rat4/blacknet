/*
 * Copyright (c) 2020 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.byte

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
    components { b1, b2 ->
        return byteArrayOf(b1, b2)
    }
}

/**
 * Returns the byte array representation of this [Int] value in the big-endian byte order.
 *
 * @return the yielded [ByteArray]
 */
fun Int.toByteArray(): ByteArray {
    components { b1, b2, b3, b4 ->
        return byteArrayOf(b1, b2, b3, b4)
    }
}

/**
 * Returns the byte array representation of this [Long] value in the big-endian byte order.
 *
 * @return the yielded [ByteArray]
 */
fun Long.toByteArray(): ByteArray {
    components { b1, b2, b3, b4, b5, b6, b7, b8 ->
        return byteArrayOf(b1, b2, b3, b4, b5, b6, b7, b8)
    }
}

/**
 * Rotates the bytes of the big-endian representation of this [Short] value with the given [wheel].
 *
 * @param wheel the wheel function
 * @return the result of the [wheel]
 */
inline fun <T> Short.components(wheel: (b1: Byte, b2: Byte) -> T): T {
    return wheel(
            (this.toInt()  shr 1 * Byte.SIZE_BITS  ).toByte(),
            (this.toInt()/*shr 0 * Byte.SIZE_BITS*/).toByte()
    )
}

/**
 * Rotates the bytes of the big-endian representation of this [Int] value with the given [wheel].
 *
 * @param wheel the wheel function
 * @return the result of the [wheel]
 */
inline fun <T> Int.components(wheel: (b1: Byte, b2: Byte, b3: Byte, b4: Byte) -> T): T {
    return wheel(
            (this  shr 3 * Byte.SIZE_BITS  ).toByte(),
            (this  shr 2 * Byte.SIZE_BITS  ).toByte(),
            (this  shr 1 * Byte.SIZE_BITS  ).toByte(),
            (this/*shr 0 * Byte.SIZE_BITS*/).toByte()
    )
}

/**
 * Rotates the bytes of the big-endian representation of this [Long] value with the given [wheel].
 *
 * @param wheel the wheel function
 * @return the result of the [wheel]
 */
inline fun <T> Long.components(wheel: (b1: Byte, b2: Byte, b3: Byte, b4: Byte, b5: Byte, b6: Byte, b7: Byte, b8: Byte) -> T): T {
    return wheel(
            (this  shr 7 * Byte.SIZE_BITS  ).toByte(),
            (this  shr 6 * Byte.SIZE_BITS  ).toByte(),
            (this  shr 5 * Byte.SIZE_BITS  ).toByte(),
            (this  shr 4 * Byte.SIZE_BITS  ).toByte(),
            (this  shr 3 * Byte.SIZE_BITS  ).toByte(),
            (this  shr 2 * Byte.SIZE_BITS  ).toByte(),
            (this  shr 1 * Byte.SIZE_BITS  ).toByte(),
            (this/*shr 0 * Byte.SIZE_BITS*/).toByte()
    )
}
