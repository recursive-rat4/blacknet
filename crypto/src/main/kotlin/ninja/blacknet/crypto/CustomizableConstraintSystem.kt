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
 * Customizable constraint systems for succinct arguments
 * Srinath Setty, Justin Thaler, Riad Wahby
 * May 3, 2023
 * https://eprint.iacr.org/2023/552
 */

class CustomizableConstraintSystem<E : PrimeFieldElement<E, F>, F : PrimeField<F, E>> internal constructor(
    private val field: F,
    private val rows: Int,
    private val columns: Int,
    private val m: Array<Matrix<E, F>>,
    private val s: Array<Array<Int>>,
    private val c: Array<E>,
) {
    fun isSatisfied(z: Vector<E, F>): Boolean {
        var sigma = Vector(Array<PrimeFieldElement<E, F>>(rows) { field.ZERO } as Array<E>)
        for (i in 0 until c.size) {
            var circle = Vector(Array<PrimeFieldElement<E, F>>(rows) { field.ONE } as Array<E>)
            for (j in s[i]) {
                circle = m[j] * z * circle
            }
            sigma = sigma + circle * c[i]
        }
        return sigma == Vector(Array<PrimeFieldElement<E, F>>(rows) { field.ZERO } as Array<E>)
    }
}
