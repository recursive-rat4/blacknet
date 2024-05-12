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
import kotlin.test.assertFalse
import kotlin.test.assertTrue

class CustomizableConstraintSystemTest {
    @Test
    fun r1cs() {
        // x = w²
        val a = Matrix(PallasField, 1, 3, arrayOf(PallasField.ZERO, PallasField.ZERO, PallasField.ONE))
        val b = Matrix(PallasField, 1, 3, arrayOf(PallasField.ZERO, PallasField.ZERO, PallasField.ONE))
        val c = Matrix(PallasField, 1, 3, arrayOf(PallasField.ONE, PallasField.ZERO, PallasField.ZERO))
        val z1 = Vector(arrayOf(
            PallasFieldElement(9),
            PallasField.ONE,
            PallasFieldElement(2),
        ))
        val z2 = Vector(arrayOf(
            PallasFieldElement(4),
            PallasField.ONE,
            PallasFieldElement(2),
        ))
        val ccs = CustomizableConstraintSystem(
            PallasField, 1, 3,
            arrayOf(a, b, c),
            arrayOf(arrayOf(0, 1), arrayOf(2)),
            arrayOf(PallasFieldElement(1), -PallasFieldElement(1)),
        )
        assertFalse(ccs.isSatisfied(z1))
        assertTrue(ccs.isSatisfied(z2))
    }
}
