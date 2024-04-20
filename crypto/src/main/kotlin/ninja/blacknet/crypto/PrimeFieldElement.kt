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

abstract class PrimeFieldElement<E : PrimeFieldElement<E, F>, F : PrimeField<F, E>> protected constructor(
    private val n: BigInteger,
) {
    protected abstract val field: F

    override fun equals(other: Any?) = other is PrimeFieldElement<E, F> && n == other.n && field === other.field
    override fun hashCode() = n.hashCode()
    override fun toString() = n.toString(16)

    operator fun plus(other: E): E = field.element(
        n + other.n mod field.order
    )

    operator fun times(other: E): E = field.element(
        n * other.n mod field.order
    )

    operator fun minus(other: E): E = field.element(
        n - other.n mod field.order
    )

    operator fun div(other: E): E = field.element(
        n / other.n mod field.order
    )

    operator fun unaryMinus(): E = field.element(
        field.order - n mod field.order
    )

    private operator fun BigInteger.div(other: BigInteger) = multiply(other.modInverse(field.order))
    private infix fun BigInteger.mod(mod: BigInteger) = mod(mod)
}
