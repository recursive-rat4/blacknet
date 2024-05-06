/*
 * Copyright (c) 2024 Pavel Vasin
 *
 * Licensed under the Jelurida Public License version 1.1
 * for the Blacknet Public Blockchain Platform (the "License");
 * you may not use this file except in compliance with the License.
 * See the LICENSE.txt file at the top-level directory of this distribution.
 */

package ninja.blacknet.crypto

/*
 * Non-Interactive and Information-Theoretic Secure Verifiable Secret Sharing
 * Torben Pryds Pedersen
 * 1991
 * https://www.cs.cornell.edu/courses/cs754/2001fa/129.PDF
 */

class PedersenCommitment<
    E : EllipticCurveGroupElement<E, G, BE, BF, SE, SF>, G : EllipticCurveGroup<G, E, BE, BF, SE, SF>,
    BE : PrimeFieldElement<BE, BF>, BF : PrimeField<BF, BE>,
    SE : PrimeFieldElement<SE, SF>, SF : PrimeField<SF, SE>,
> internal constructor(
    private val g: E,
    private val h: E,
) {
    fun commit(s: SE, t: SE): E = g * s + h * t

    fun open(e: E, s: SE, t: SE): Boolean = e == commit(s, t)
}
