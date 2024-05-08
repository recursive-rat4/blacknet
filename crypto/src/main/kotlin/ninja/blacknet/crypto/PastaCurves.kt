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
    override val ZERO = PallasFieldElement(BigInteger.ZERO)
    override val ONE = PallasFieldElement(BigInteger.ONE)
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
    override val ZERO = VestaFieldElement(BigInteger.ZERO)
    override val ONE = VestaFieldElement(BigInteger.ONE)
}

class VestaFieldElement internal constructor(
    n: BigInteger,
) : PrimeFieldElement<VestaFieldElement, VestaField>(n) {
    override val field = VestaField
}

object PallasGroup : EllipticCurveGroup<
    PallasGroup, PallasGroupElementAffine,
    PallasFieldElement, PallasField,
    VestaFieldElement, VestaField,
>(PallasField, VestaField) {
    override val a = PallasField.ZERO
    override val b = PallasFieldElement(BigInteger.valueOf(5))

    override fun elementAffine(x: PallasFieldElement, y: PallasFieldElement) = PallasGroupElementAffine(x, y)

    override val INFINITY_AFFINE = elementAffine(PallasField.ZERO, PallasField.ZERO)
}

class PallasGroupElementAffine internal constructor(
    x: PallasFieldElement,
    y: PallasFieldElement,
) : EllipticCurveGroupElementAffine<
    PallasGroupElementAffine, PallasGroup,
    PallasFieldElement, PallasField,
    VestaFieldElement, VestaField,
>(x, y) {
    override val group = PallasGroup
}

object VestaGroup : EllipticCurveGroup<
    VestaGroup, VestaGroupElementAffine,
    VestaFieldElement, VestaField,
    PallasFieldElement, PallasField,
>(VestaField, PallasField) {
    override val a = VestaField.ZERO
    override val b = VestaFieldElement(BigInteger.valueOf(5))

    override fun elementAffine(x: VestaFieldElement, y: VestaFieldElement) = VestaGroupElementAffine(x, y)

    override val INFINITY_AFFINE = elementAffine(VestaField.ZERO, VestaField.ZERO)
}

class VestaGroupElementAffine internal constructor(
    x: VestaFieldElement,
    y: VestaFieldElement,
) : EllipticCurveGroupElementAffine<
    VestaGroupElementAffine, VestaGroup,
    VestaFieldElement, VestaField,
    PallasFieldElement, PallasField,
>(x, y) {
    override val group = VestaGroup
}
