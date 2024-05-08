/*
 * Copyright (c) 2024 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.crypto

abstract class EllipticCurveGroupElementAffine<
    E : EllipticCurveGroupElementAffine<E, G, BE, BF, SE, SF>, G : EllipticCurveGroup<G, E, BE, BF, SE, SF>,
    BE : PrimeFieldElement<BE, BF>, BF : PrimeField<BF, BE>,
    SE : PrimeFieldElement<SE, SF>, SF : PrimeField<SF, SE>,
> protected constructor(
    private val x: BE,
    private val y: BE,
) : EllipticCurveGroupElement<E, G, BE, BF, SE, SF>() {
    override val INFINITY: E
        get() = group.INFINITY_AFFINE //UPSTREAM don't create uninitialized objects

    override fun equals(other: Any?) = other is EllipticCurveGroupElementAffine<E, G, BE, BF, SE, SF> && x == other.x && y == other.y && group === other.group
    override fun hashCode() = x.hashCode() xor y.hashCode()
    override fun toString() = if (this != INFINITY) "(${x.toString()}, ${y.toString()})" else "Infinity"

    override operator fun unaryMinus(): E {
        return if (this != INFINITY)
            group.elementAffine(x, -y)
        else
            INFINITY
    }

    @Suppress("UNCHECKED_CAST")
    override operator fun plus(other: E): E {
        if (this == INFINITY)
            return other
        if (other == INFINITY)
            return this as E

        return if (x != other.x) {
            val k = (other.y - y) / (other.x - x)
            val xr = k * k - x - other.x
            val yr = k * (x - xr) - y
            group.elementAffine(xr, yr)
        } else if (y == other.y) {
            val k = (group.THREE * x * x + group.a) / (group.TWO * y)
            val xr = k * k - x - x
            val yr = k * (x - xr) - y
            group.elementAffine(xr, yr)
        } else {
            INFINITY
        }
    }
}
