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

abstract class EllipticCurveGroup<
    G : EllipticCurveGroup<G, E, BE, BF, SE, SF>, E : EllipticCurveGroupElement<E, G, BE, BF, SE, SF>,
    BE : PrimeFieldElement<BE, BF>, BF : PrimeField<BF, BE>,
    SE : PrimeFieldElement<SE, SF>, SF : PrimeField<SF, SE>,
> protected constructor(
    private val base: BF,
    internal val scalar: SF,
) {
    init {
        require(base.order > BigInteger.valueOf(3)) { "Affine double" }
    }

    internal val TWO = base.element(BigInteger.TWO)
    internal val THREE = base.element(BigInteger.valueOf(3))

    internal abstract val a: BE
    internal abstract val b: BE

    internal abstract fun element(x: BE, y: BE): E

    internal abstract val INFINITY: E

    fun random(random: Random): E {
        while (true) {
            val x = base.random(random)
            val y = (x * x * x + a * x + b).sqrt() ?: continue
            return element(x, if (random.nextBoolean()) y else -y)
        }
    }
}
