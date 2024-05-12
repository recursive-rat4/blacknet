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

    override val bits = 255
    override val S = BigInteger.valueOf(32)
    override val Q = BigInteger("40000000000000000000000000000000224698fc094cf91b992d30ed", 16)

    override val ZERO = PallasFieldElement(0)
    override val ONE = PallasFieldElement(1)

    override val TWO = PallasFieldElement(2)
    override val THREE = PallasFieldElement(3)
    override val FOUR = PallasFieldElement(4)
    override val EIGHT = PallasFieldElement(8)
}

class PallasFieldElement internal constructor(
    n: BigInteger,
) : PrimeFieldElement<PallasFieldElement, PallasField>(n) {
    override val field = PallasField

    constructor(n: Long) : this(BigInteger.valueOf(n))
    constructor(string: String, radix: Int) : this(BigInteger(string, radix))
}

object VestaField : PrimeField<VestaField, VestaFieldElement>(
    BigInteger("40000000000000000000000000000000224698fc0994a8dd8c46eb2100000001", 16),
) {
    override fun element(n: BigInteger) = VestaFieldElement(n)

    override val bits = 255
    override val S = BigInteger.valueOf(32)
    override val Q = BigInteger("40000000000000000000000000000000224698fc0994a8dd8c46eb21", 16)

    override val ZERO = VestaFieldElement(0)
    override val ONE = VestaFieldElement(1)

    override val TWO = VestaFieldElement(2)
    override val THREE = VestaFieldElement(3)
    override val FOUR = VestaFieldElement(4)
    override val EIGHT = VestaFieldElement(8)
}

class VestaFieldElement internal constructor(
    n: BigInteger,
) : PrimeFieldElement<VestaFieldElement, VestaField>(n) {
    override val field = VestaField

    constructor(n: Long) : this(BigInteger.valueOf(n))
    constructor(string: String, radix: Int) : this(BigInteger(string, radix))
}

object PallasGroup : EllipticCurveGroup<
    PallasGroup, PallasGroupElementAffine, PallasGroupElementProjective,
    PallasFieldElement, PallasField,
    VestaFieldElement, VestaField,
>(PallasField, VestaField) {
    override val a = PallasField.ZERO
    override val b = PallasFieldElement(5)

    override fun elementAffine(x: PallasFieldElement, y: PallasFieldElement) = PallasGroupElementAffine(x, y)
    override fun elementProjective(x: PallasFieldElement, y: PallasFieldElement, z: PallasFieldElement) = PallasGroupElementProjective(x, y, z)

    override val INFINITY_AFFINE = elementAffine(PallasField.ZERO, PallasField.ZERO)
    override val INFINITY_PROJECTIVE = elementProjective(PallasField.ZERO, PallasField.ZERO, PallasField.ZERO)
}

class PallasGroupElementAffine internal constructor(
    x: PallasFieldElement,
    y: PallasFieldElement,
) : EllipticCurveGroupElementAffine<
    PallasGroupElementAffine, PallasGroup, PallasGroupElementProjective,
    PallasFieldElement, PallasField,
    VestaFieldElement, VestaField,
>(x, y) {
    override val group = PallasGroup
}

class PallasGroupElementProjective internal constructor(
    x: PallasFieldElement,
    y: PallasFieldElement,
    z: PallasFieldElement,
) : EllipticCurveGroupElementProjective<
    PallasGroupElementProjective, PallasGroup, PallasGroupElementAffine,
    PallasFieldElement, PallasField,
    VestaFieldElement, VestaField,
>(x, y, z) {
    override val group = PallasGroup
}

object VestaGroup : EllipticCurveGroup<
    VestaGroup, VestaGroupElementAffine, VestaGroupElementProjective,
    VestaFieldElement, VestaField,
    PallasFieldElement, PallasField,
>(VestaField, PallasField) {
    override val a = VestaField.ZERO
    override val b = VestaFieldElement(5)

    override fun elementAffine(x: VestaFieldElement, y: VestaFieldElement) = VestaGroupElementAffine(x, y)
    override fun elementProjective(x: VestaFieldElement, y: VestaFieldElement, z: VestaFieldElement) = VestaGroupElementProjective(x, y, z)

    override val INFINITY_AFFINE = elementAffine(VestaField.ZERO, VestaField.ZERO)
    override val INFINITY_PROJECTIVE = elementProjective(VestaField.ZERO, VestaField.ZERO, VestaField.ZERO)
}

class VestaGroupElementAffine internal constructor(
    x: VestaFieldElement,
    y: VestaFieldElement,
) : EllipticCurveGroupElementAffine<
    VestaGroupElementAffine, VestaGroup, VestaGroupElementProjective,
    VestaFieldElement, VestaField,
    PallasFieldElement, PallasField,
>(x, y) {
    override val group = VestaGroup
}

class VestaGroupElementProjective internal constructor(
    x: VestaFieldElement,
    y: VestaFieldElement,
    z: VestaFieldElement,
) : EllipticCurveGroupElementProjective<
    VestaGroupElementProjective, VestaGroup, VestaGroupElementAffine,
    VestaFieldElement, VestaField,
    PallasFieldElement, PallasField,
>(x, y, z) {
    override val group = VestaGroup
}
