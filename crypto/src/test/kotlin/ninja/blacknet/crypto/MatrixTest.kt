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

class MatrixTest {
    @Test
    fun product() {
        val a = Matrix(PallasField, 3, 2, arrayOf(
            PallasFieldElement(BigInteger.valueOf(17)), PallasFieldElement(BigInteger.valueOf(18)),
            PallasFieldElement(BigInteger.valueOf(33)), PallasFieldElement(BigInteger.valueOf(34)),
            PallasFieldElement(BigInteger.valueOf(49)), PallasFieldElement(BigInteger.valueOf(50)),
        ))
        val b = Vector(arrayOf(PallasFieldElement(BigInteger.valueOf(2)), PallasFieldElement(BigInteger.valueOf(3))))
        val c = Vector(arrayOf(
            PallasFieldElement(BigInteger.valueOf(88)),
            PallasFieldElement(BigInteger.valueOf(168)),
            PallasFieldElement(BigInteger.valueOf(248)),
        ))
        assertEquals(c, a * b)
    }
}
