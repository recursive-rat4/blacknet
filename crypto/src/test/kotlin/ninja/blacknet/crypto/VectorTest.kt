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
import kotlin.test.Test
import kotlin.test.assertEquals

class VectorTest {
    @Test
    fun `Hadamard product`() {
        val a = Vector(arrayOf(
            PallasFieldElement(BigInteger.valueOf(2)),
            PallasFieldElement(BigInteger.valueOf(2)),
            PallasFieldElement(BigInteger.valueOf(2)),
        ))
        val b = Vector(arrayOf(
            PallasFieldElement(BigInteger.valueOf(1)),
            PallasFieldElement(BigInteger.valueOf(2)),
            PallasFieldElement(BigInteger.valueOf(4)),
        ))
        val c = Vector(arrayOf(
            PallasFieldElement(BigInteger.valueOf(2)),
            PallasFieldElement(BigInteger.valueOf(4)),
            PallasFieldElement(BigInteger.valueOf(8)),
        ))
        assertEquals(c, a * b)
        assertEquals(c, b * a)
    }

    @Test
    fun `Hadamard summation`() {
        val a = Vector(arrayOf(
            PallasFieldElement(BigInteger.valueOf(0)),
            PallasFieldElement(BigInteger.valueOf(4)),
            PallasFieldElement(BigInteger.valueOf(2)),
        ))
        val b = Vector(arrayOf(
            PallasFieldElement(BigInteger.valueOf(7)),
            PallasFieldElement(BigInteger.valueOf(3)),
            PallasFieldElement(BigInteger.valueOf(5)),
        ))
        val c = Vector(arrayOf(
            PallasFieldElement(BigInteger.valueOf(7)),
            PallasFieldElement(BigInteger.valueOf(7)),
            PallasFieldElement(BigInteger.valueOf(7)),
        ))
        assertEquals(c, a + b)
        assertEquals(c, b + a)
    }
}
