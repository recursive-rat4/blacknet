/*
 * Copyright (c) 2024 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.crypto

import java.util.Random

abstract class EllipticCurveGroup<
    G : EllipticCurveGroup<G, EA, EP, BE, BF, SE, SF>,
    EA : EllipticCurveGroupElementAffine<EA, G, EP, BE, BF, SE, SF>,
    EP : EllipticCurveGroupElementProjective<EP, G, EA, BE, BF, SE, SF>,
    BE : PrimeFieldElement<BE, BF>, BF : PrimeField<BF, BE>,
    SE : PrimeFieldElement<SE, SF>, SF : PrimeField<SF, SE>,
> protected constructor(
    internal val base: BF,
    internal val scalar: SF,
) {
    internal abstract val a: BE
    internal abstract val b: BE

    internal abstract fun elementAffine(x: BE, y: BE): EA
    internal abstract fun elementProjective(x: BE, y: BE, z: BE): EP

    internal abstract val INFINITY_AFFINE: EA
    internal abstract val INFINITY_PROJECTIVE: EP

    fun randomAffine(random: Random): EA {
        while (true) {
            val x = base.random(random)
            val y = (x * x.square() + a * x + b).sqrt() ?: continue
            return elementAffine(x, if (random.nextBoolean()) y else -y)
        }
    }

    fun randomProjective(random: Random): EP {
        while (true) {
            val x = base.random(random)
            val y = (x * x.square() + a * x + b).sqrt() ?: continue
            return elementProjective(x, if (random.nextBoolean()) y else -y, base.ONE)
        }
    }
}
