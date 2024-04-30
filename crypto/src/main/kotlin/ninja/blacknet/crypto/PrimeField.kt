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
import java.util.Random

abstract class PrimeField<F : PrimeField<F, E>, E : PrimeFieldElement<E, F>> protected constructor(
    internal val order: BigInteger,
) {
    private val bits: Int = (order - BigInteger.ONE).bitLength()

    internal val S: BigInteger
    internal val Q: BigInteger

    init {
        require(order > BigInteger.TWO) { "Odd prime" }

        var s = BigInteger.ZERO
        var q = order - BigInteger.ONE
        while (q mod BigInteger.TWO == BigInteger.ZERO) {
            s += BigInteger.ONE
            q /= BigInteger.TWO
        }
        S = s
        Q = q
    }

    internal abstract fun element(n: BigInteger): E

    //internal val ZERO: E = element(BigInteger.ZERO)
    internal abstract val ZERO: E //UPSTREAM don't create uninitialized objects
    internal abstract val ONE: E

    internal fun random(random: Random): E {
        while (true) {
            val n = BigInteger(bits, random)
            if (n < order)
                return element(n)
        }
    }

    @Suppress("UNCHECKED_CAST")
    internal fun random(random: Random, n: Int) = Vector<E, F>(
        Array<PrimeFieldElement<E, F>>(n) {
            random(random)
        } as Array<E>
    )
}
