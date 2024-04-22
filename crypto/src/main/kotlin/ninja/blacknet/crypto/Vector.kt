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
import java.util.Random

class Vector<E : PrimeFieldElement<E, F>, F : PrimeField<F, E>> internal constructor(
    private val array: Array<E>,
) {
    override fun equals(other: Any?) = other is Vector<E, F> && Arrays.equals(array, other.array)
    override fun hashCode() = Arrays.hashCode(array)
    override fun toString() = Arrays.toString(array)

    @Suppress("UNCHECKED_CAST")
    operator fun plus(other: Vector<E, F>) = Vector<E, F>(
        Array<PrimeFieldElement<E, F>>(array.size) { i ->
            array[i] + other.array[i]
        } as Array<E>
    )

    @Suppress("UNCHECKED_CAST")
    operator fun times(other: Vector<E, F>) = Vector<E, F>(
        Array<PrimeFieldElement<E, F>>(array.size) { i ->
            array[i] * other.array[i]
        } as Array<E>
    )
}
