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
        n * other.n.inv() mod field.order
    )

    operator fun unaryMinus(): E = field.element(
        field.order - n mod field.order
    )

    fun inv(): E = field.element(
        n.inv()
    )

    fun sqrt(): E? {
        // Tonelli–Shanks algorithm
        when (n.isQuadraticResidue()) {
            BigInteger.ONE -> {
                var z = BigInteger.TWO
                while (z.isQuadraticResidue() < BigInteger.TWO)
                    z += BigInteger.ONE
                var m = field.S
                var c = z.modPow(field.Q, field.order)
                var t = n.modPow(field.Q, field.order)
                var r = n.modPow((field.Q + BigInteger.ONE) / BigInteger.TWO, field.order)
                while (true) {
                    if (t == BigInteger.ZERO)
                        return field.ZERO
                    else if (t == BigInteger.ONE)
                        return field.element(r)
                    else {
                        var i = BigInteger.ONE
                        while (t.modPow(BigInteger.TWO.pow(i.intValueExact()), field.order) != BigInteger.ONE)
                            i += BigInteger.ONE
                        val b = c.modPow(BigInteger.TWO.pow((m - i - BigInteger.ONE).intValueExact()), field.order)
                        m = i
                        c = b.pow(2) mod field.order
                        t = t * b.pow(2) mod field.order
                        r = r * b mod field.order
                    }
                }
            }
            BigInteger.ZERO -> return field.ZERO
            else -> return null
        }
    }

    internal operator fun get(index: Int) = n.testBit(index)

    // Legendre symbol
    private fun BigInteger.isQuadraticResidue() = modPow((field.order - BigInteger.ONE) / BigInteger.TWO, field.order)

    private fun BigInteger.inv() = modInverse(field.order)
}

internal infix fun BigInteger.mod(mod: BigInteger) = mod(mod)
