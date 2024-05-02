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
    E : EllipticCurveGroupElement<E, G, BE, BF, SE, SF>, G : EllipticCurveGroup<G, E, BE, BF, SE, SF>,
    BE : PrimeFieldElement<BE, BF>, BF : PrimeField<BF, BE>,
    SE : PrimeFieldElement<SE, SF>, SF : PrimeField<SF, SE>,
> protected constructor(
    private val x: BE,
    private val y: BE,
) {
    protected abstract val group: G

    override fun equals(other: Any?) = other is EllipticCurveGroupElement<E, G, BE, BF, SE, SF> && x == other.x && y == other.y && group === other.group
    override fun hashCode() = x.hashCode() xor y.hashCode()
    override fun toString() = if (this != group.INFINITY) "(${x.toString()}, ${y.toString()})" else "Infinity"

    operator fun unaryMinus(): E {
        return if (this != group.INFINITY)
            group.element(x, -y)
        else
            group.INFINITY
    }

    @Suppress("UNCHECKED_CAST")
    operator fun plus(other: E): E {
        if (this == group.INFINITY)
            return other
        if (other == group.INFINITY)
            return this as E

        return if (x != other.x) {
            val k = (other.y - y) / (other.x - x)
            val xr = k * k - x - other.x
            val yr = k * (x - xr) - y
            group.element(xr, yr)
        } else if (y == other.y) {
            val k = (group.THREE * x * x + group.a) / (group.TWO * y)
            val xr = k * k - x - x
            val yr = k * (x - xr) - y
            group.element(xr, yr)
        } else {
            group.INFINITY
        }
    }

    @Suppress("UNCHECKED_CAST")
    operator fun times(other: SE): E {
        // Double-and-add method
        var r = group.INFINITY
        var t = this as E
        for (i in 0 until group.scalar.bits) {
            if (other[i])
                r += t
            t += t
        }
        return r
    }
}
