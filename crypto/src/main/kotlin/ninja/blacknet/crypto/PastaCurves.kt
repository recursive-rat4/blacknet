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

/*
 * The Pasta Curves for Halo 2 and Beyond
 * Daira Hopwood
 * November 23, 2020
 * https://electriccoin.co/blog/the-pasta-curves-for-halo-2-and-beyond/
 */

object PallasField : PrimeField<PallasField, PallasFieldElement>(
    BigInteger("40000000000000000000000000000000224698fc094cf91b992d30ed00000001", 16),
) {
    override fun element(n: BigInteger) = PallasFieldElement(n)
}

class PallasFieldElement internal constructor(
    n: BigInteger,
) : PrimeFieldElement<PallasFieldElement, PallasField>(n) {
    override val field = PallasField
}

object VestaField : PrimeField<VestaField, VestaFieldElement>(
    BigInteger("40000000000000000000000000000000224698fc0994a8dd8c46eb2100000001", 16),
) {
    override fun element(n: BigInteger) = VestaFieldElement(n)
}

class VestaFieldElement internal constructor(
    n: BigInteger,
) : PrimeFieldElement<VestaFieldElement, VestaField>(n) {
    override val field = VestaField
}
