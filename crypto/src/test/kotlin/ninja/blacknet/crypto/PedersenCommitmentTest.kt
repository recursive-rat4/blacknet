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
import kotlin.test.assertFalse
import kotlin.test.assertTrue

class PedersenCommitmentTest {
    @Test
    fun single() {
        val g = VestaGroupElement(
            VestaFieldElement(BigInteger("33074014122d93a8ac69e0bbc472768ebf2760c0e53f73abf0e395d8b1b5b478", 16)),
            VestaFieldElement(BigInteger("3604f572d11bae3cccf8a6895d8e06b3c7388e54a5acda9f5e62d33a72bbc566", 16)),
        )
        val h = VestaGroupElement(
            VestaFieldElement(BigInteger("245a92dbb72f4e95e0be3595344d0bc58978c7b7c9c1a5b2128d9d7eb3d6328a", 16)),
            VestaFieldElement(BigInteger("11bac7e68bd74ee7a7a43f6b1f9e206e8b8ac7c8d2bae596ef891c301155ad1e", 16)),
        )
        val cs = PedersenCommitment(g, h)
        val m1 = PallasFieldElement(BigInteger("9e21902c37d0c6dc4c1c8143faefa86a192cac72bdc0d89828a2d1ce3d813b3", 16))
        val m2 = PallasFieldElement(BigInteger("374bb94b3a48c4cadbc80878bf5082692a25001e84865cbd73f3f0cb7308bc72", 16))
        val r1 = PallasFieldElement(BigInteger("1ab0bd7178dbc83ec8ec11aa0bf46e5cae406812d865fa9a96beccac98aa0f5d", 16))
        val r2 = PallasFieldElement(BigInteger("10af23b9642c311b7b270d22fd0cb8efbcdee017d8d25246dedeb7bf06064906", 16))
        val c1 = VestaGroupElement(
            VestaFieldElement(BigInteger("3e8cadd38b46b13201817a1aee9717d725593b85200de9a1e0d17d9360e6b861", 16)),
            VestaFieldElement(BigInteger("bf3b36d73b5f244cff3a65e8e8130cfacfa79fb1c3cd0404f5bac1b50b5778d", 16)),
        )
        val c2 = VestaGroupElement(
            VestaFieldElement(BigInteger("462e663bdd1b93aff1bf6c6aa7ef8e71521ddc1494e4727a9baf78b87946eef", 16)),
            VestaFieldElement(BigInteger("3342441f4969d6bff28fe055db320b90794c17a419b102c56ad8179c9a76459e", 16)),
        )
        assertTrue(cs.open(c1, m1, r1), "Opening")
        assertFalse(cs.open(c2, m1, r1), "Binding")
        assertFalse(cs.open(c1, r1, m1), "Positional binding")
        assertTrue(cs.open(c1 + c2, m1 + m2, r1 + r2), "Homomorphism")
    }
}
