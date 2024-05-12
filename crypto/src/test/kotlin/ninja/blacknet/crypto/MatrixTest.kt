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

class MatrixTest {
    @Test
    fun product() {
        val a = Matrix(PallasField, 3, 2, arrayOf(
            PallasFieldElement(17), PallasFieldElement(18),
            PallasFieldElement(33), PallasFieldElement(34),
            PallasFieldElement(49), PallasFieldElement(50),
        ))
        val b = Vector(arrayOf(PallasFieldElement(2), PallasFieldElement(3)))
        val c = Vector(arrayOf(
            PallasFieldElement(88),
            PallasFieldElement(168),
            PallasFieldElement(248),
        ))
        assertEquals(c, a * b)
    }
}
