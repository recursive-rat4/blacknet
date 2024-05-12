/*
 * Copyright (c) 2024 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.crypto

import kotlin.test.Test
import kotlin.test.assertEquals

class VectorTest {
    @Test
    fun `get`() {
        val a = arrayOf(
            PallasFieldElement(100),
            PallasFieldElement(101),
            PallasFieldElement(110),
            PallasFieldElement(111),
        )
        val v = Vector(a)
        a.forEachIndexed { i, e ->
            assertEquals(e, v[i])
        }
    }

    @Test
    fun `Hadamard product`() {
        val a = Vector(arrayOf(
            PallasFieldElement(2),
            PallasFieldElement(2),
            PallasFieldElement(2),
        ))
        val b = Vector(arrayOf(
            PallasFieldElement(1),
            PallasFieldElement(2),
            PallasFieldElement(4),
        ))
        val c = Vector(arrayOf(
            PallasFieldElement(2),
            PallasFieldElement(4),
            PallasFieldElement(8),
        ))
        assertEquals(c, a * b)
        assertEquals(c, b * a)
    }

    @Test
    fun `Hadamard summation`() {
        val a = Vector(arrayOf(
            PallasFieldElement(0),
            PallasFieldElement(4),
            PallasFieldElement(2),
        ))
        val b = Vector(arrayOf(
            PallasFieldElement(7),
            PallasFieldElement(3),
            PallasFieldElement(5),
        ))
        val c = Vector(arrayOf(
            PallasFieldElement(7),
            PallasFieldElement(7),
            PallasFieldElement(7),
        ))
        assertEquals(c, a + b)
        assertEquals(c, b + a)
    }

    @Test
    fun `Scalar product`() {
        val a = Vector(arrayOf(
            PallasFieldElement(4),
            PallasFieldElement(5),
            PallasFieldElement(6),
        ))
        val b = PallasFieldElement(2)
        val c = Vector(arrayOf(
            PallasFieldElement(8),
            PallasFieldElement(10),
            PallasFieldElement(12),
        ))
        assertEquals(c, a * b)
    }
}
