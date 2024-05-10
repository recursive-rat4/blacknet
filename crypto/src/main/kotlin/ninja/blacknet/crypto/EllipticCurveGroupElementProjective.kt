/*
 * Copyright (c) 2024 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.crypto

abstract class EllipticCurveGroupElementProjective<
    EP : EllipticCurveGroupElementProjective<EP, G, EA, BE, BF, SE, SF>, G : EllipticCurveGroup<G, EA, EP, BE, BF, SE, SF>,
    EA : EllipticCurveGroupElementAffine<EA, G, EP, BE, BF, SE, SF>,
    BE : PrimeFieldElement<BE, BF>, BF : PrimeField<BF, BE>,
    SE : PrimeFieldElement<SE, SF>, SF : PrimeField<SF, SE>,
> protected constructor(
    private val x: BE,
    private val y: BE,
    private val z: BE,
) : EllipticCurveGroupElement<EP, G, EA, EP, BE, BF, SE, SF>() {
    override val INFINITY: EP
        get() = group.INFINITY_PROJECTIVE

    override fun equals(other: Any?): Boolean {
        other as? EllipticCurveGroupElementProjective<EP, G, EA, BE, BF, SE, SF> ?: return false
        val i1 = z == group.base.ZERO
        val i2 = other.z == group.base.ZERO
        return if (i1 && i2)
            true
        else if (i1 || i2)
            false
        else
            (x * other.z == z * other.x) && (y * other.z == z * other.y)
    }
    override fun hashCode(): Int {
        return if (this != INFINITY) {
            val a = z.inv()
            (x * a).hashCode() xor (y * a).hashCode()
        } else {
            0
        }
    }
    override fun toString() = if (this != INFINITY) "(${x.toString()}, ${y.toString()}, ${z.toString()})" else "Infinity"

    override operator fun unaryMinus(): EP {
        return if (this != INFINITY)
            group.elementProjective(x, -y, z)
        else
            INFINITY
    }

    @Suppress("UNCHECKED_CAST")
    override operator fun plus(other: EP): EP {
        if (this == INFINITY)
            return other
        if (other == INFINITY)
            return this as EP

        val u1 = other.y * z
        val u2 = y * other.z
        val v1 = other.x * z
        val v2 = x * other.z

        return if (v1 != v2) {
            val u = u1 - u2
            val uu = u * u
            val v = v1 - v2
            val vv = v * v
            val vvv = v * vv
            val w = z * other.z
            val r = vv * v2
            val a = uu * w - vvv - group.TWO * r
            val xr = v * a
            val yr = u * (r - a) - vvv * u2
            val zr = vvv * w
            group.elementProjective(xr, yr, zr)
        } else if (u1 == u2) {
            val w = group.a * z * z + group.THREE * x * x
            val s = y * z
            val sss = s * s * s
            val r = y * s
            val b = x * r
            val h = w * w - group.EIGHT * b
            val xr = group.TWO * h * s
            val yr = w * (group.FOUR * b - h) - group.EIGHT * r * r
            val zr = group.EIGHT * sss
            group.elementProjective(xr, yr, zr)
        } else {
            INFINITY
        }
    }

    internal fun scale(): EP {
        return if (this != INFINITY) {
            val a = z.inv()
            group.elementProjective(x * a, y * a, group.base.ONE)
        } else {
            INFINITY
        }
    }
}
