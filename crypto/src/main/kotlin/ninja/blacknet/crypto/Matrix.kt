/*
 * Copyright (c) 2024 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.crypto

import java.math.BigInteger
import java.util.Arrays

class Matrix<E : PrimeFieldElement<E, F>, F : PrimeField<F, E>> internal constructor(
    private val field: F,
    private val rows: Int,
    private val columns: Int,
    private val data: Array<E>,
) {
    override fun equals(other: Any?) = other is Matrix<E, F> && rows == other.rows && Arrays.equals(data, other.data)
    override fun hashCode() = Arrays.hashCode(data)
    override fun toString() = Arrays.toString(data)

    private operator fun get(i: Int, j: Int): E = data[i * columns + j]

    @Suppress("UNCHECKED_CAST")
    operator fun times(vector: Vector<E, F>) = Vector<E, F>(
        Array<PrimeFieldElement<E, F>>(rows) { i ->
            var acc = field.ZERO
            for (j in 0 until columns)
                acc = acc + get(i, j) * vector[j]
            acc
        } as Array<E>
    )
}
