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
import org.bouncycastle.math.raw.Mod
import org.bouncycastle.math.raw.Nat
import org.bouncycastle.math.raw.Nat256
import org.bouncycastle.math.raw.Nat512

abstract class PrimeFieldElement<E : PrimeFieldElement<E, F>, F : PrimeField<F, E>> protected constructor(
    private val limbs: IntArray,
) {
    protected abstract val field: F

    override fun equals(other: Any?) = other is PrimeFieldElement<E, F> && Arrays.equals(limbs, other.limbs) && field === other.field
    override fun hashCode() = Arrays.hashCode(limbs)
    override fun toString() = Nat256.toBigInteger(limbs).toString(16)

    operator fun plus(other: E): E {
        val tt = Nat256.create()
        Nat256.add(limbs, other.limbs, tt)
        if (Nat256.gte(tt, field.order))
            Nat256.sub(tt, field.order, tt)
        return field.element(tt)
    }

    operator fun times(other: E): E {
        val tt = Nat256.create()
        val ttt = Nat256.createExt()
        Nat256.mul(limbs, other.limbs, ttt)
        reduce(ttt, tt)
        return field.element(tt)
    }

    operator fun minus(other: E): E {
        val tt = Nat256.create()
        Nat256.sub(limbs, other.limbs, tt)
        if (Nat256.gte(tt, field.order))
            Nat256.add(tt, field.order, tt)
        return field.element(tt)
    }

    operator fun div(other: E): E {
        return this * other.inv()
    }

    operator fun unaryMinus(): E {
        return if (this != field.ZERO) {
            val tt = Nat256.create()
            Nat256.sub(field.order, limbs, tt)
            field.element(tt)
        } else {
            field.ZERO
        }
    }

    fun square(): E {
        val tt = Nat256.create()
        val ttt = Nat256.createExt()
        Nat256.square(limbs, ttt)
        reduce(ttt, tt)
        return field.element(tt)
    }

    fun inv(): E {
        val tt = Nat256.create()
        Mod.checkedModOddInverseVar(field.order, limbs, tt)
        return field.element(tt)
    }

    fun sqrt(): E? {
        // Tonelli–Shanks algorithm
        val n = Nat256.toBigInteger(limbs)
        when (n.isQuadraticResidue()) {
            BigInteger.ONE -> {
                var z = BigInteger.TWO
                while (z.isQuadraticResidue() < BigInteger.TWO)
                    z += BigInteger.ONE
                var m = field.S
                var c = z.modPow(field.Q, field.orderBN)
                var t = n.modPow(field.Q, field.orderBN)
                var r = n.modPow((field.Q + BigInteger.ONE) / BigInteger.TWO, field.orderBN)
                while (true) {
                    if (t == BigInteger.ZERO)
                        return field.ZERO
                    else if (t == BigInteger.ONE)
                        return field.element(Nat256.fromBigInteger(r))
                    else {
                        var i = BigInteger.ONE
                        while (t.modPow(BigInteger.TWO.pow(i.intValueExact()), field.orderBN) != BigInteger.ONE)
                            i += BigInteger.ONE
                        val b = c.modPow(BigInteger.TWO.pow((m - i - BigInteger.ONE).intValueExact()), field.orderBN)
                        m = i
                        c = b.pow(2) mod field.orderBN
                        t = t * b.pow(2) mod field.orderBN
                        r = r * b mod field.orderBN
                    }
                }
            }
            BigInteger.ZERO -> return field.ZERO
            else -> return null
        }
    }
    private infix fun BigInteger.mod(mod: BigInteger) = mod(mod)

    internal operator fun get(index: Int) = Nat256.getBit(limbs, index) == 1

    // Legendre symbol
    private fun BigInteger.isQuadraticResidue() = modPow((field.orderBN - BigInteger.ONE) / BigInteger.TWO, field.orderBN)

    private fun reduce(xx: IntArray, z: IntArray) {
        // Barrett reduction
        val ttt = Nat256.createExt()
        val tttt = Nat.create(32)
        Nat512.mul(xx, field.m, tttt)
        Nat256.copy(tttt, 16, ttt, 0)
        Nat.mul(ttt, 0, 16, field.order, 0, 8, tttt, 0)
        Nat.subFrom(16, tttt, 0, xx, 0)
        if (Nat256.gte(xx, field.order))
            Nat256.subFrom(field.order, xx)
        Nat256.copy(xx, z)
    }
}
