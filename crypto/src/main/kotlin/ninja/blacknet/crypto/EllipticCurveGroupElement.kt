/*
 * Copyright (c) 2024 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.crypto

abstract class EllipticCurveGroupElement<
    E : EllipticCurveGroupElement<E, G, EA, EP, BE, BF, SE, SF>, G : EllipticCurveGroup<G, EA, EP, BE, BF, SE, SF>,
    EA : EllipticCurveGroupElementAffine<EA, G, EP, BE, BF, SE, SF>,
    EP : EllipticCurveGroupElementProjective<EP, G, EA, BE, BF, SE, SF>,
    BE : PrimeFieldElement<BE, BF>, BF : PrimeField<BF, BE>,
    SE : PrimeFieldElement<SE, SF>, SF : PrimeField<SF, SE>,
> protected constructor(
) {
    protected abstract val group: G
    protected abstract val INFINITY: E

    override abstract fun equals(other: Any?): Boolean
    override abstract fun hashCode(): Int
    override abstract fun toString(): String

    abstract operator fun unaryMinus(): E

    abstract operator fun plus(other: E): E

    @Suppress("UNCHECKED_CAST")
    operator fun times(other: SE): E {
        // Double-and-add method
        var r = INFINITY
        var t = this as E
        for (i in 0 until group.scalar.bits) {
            if (other[i])
                r += t
            t += t
        }
        return r
    }
}
