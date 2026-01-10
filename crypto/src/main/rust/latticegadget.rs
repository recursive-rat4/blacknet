/*
 * Copyright (c) 2024-2026 Pavel Vasin
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

use crate::algebra::{IntegerRing, PolynomialRing, Tensor};
use crate::integer::Integer;
use crate::matrix::{DenseMatrix, DenseVector};
use alloc::vec;
use alloc::vec::Vec;

// https://eprint.iacr.org/2018/946

fn decompose_impl<Z: IntegerRing, R: PolynomialRing<Z>>(
    polynomial: R,
    radix_mask: <Z::Int as Integer>::Limb,
    radix_shift: <Z::Int as Integer>::Limb,
    pieces: &mut [R],
) {
    for (i, coefficient) in polynomial.into_iter().enumerate() {
        let mut representative = coefficient.canonical();
        for piece in pieces.iter_mut() {
            piece[i] = Z::with_limb(representative & radix_mask);
            representative >>= radix_shift;
        }
    }
}

pub fn decompose<Z: IntegerRing, R: PolynomialRing<Z>>(
    polynomial: R,
    radix_mask: <Z::Int as Integer>::Limb,
    radix_shift: <Z::Int as Integer>::Limb,
    digits: usize,
) -> DenseVector<R> {
    let mut pieces = vec![R::ZERO; digits];
    decompose_impl(polynomial, radix_mask, radix_shift, &mut pieces);
    pieces.into()
}

pub fn decompose_vector<Z: IntegerRing, R: PolynomialRing<Z>>(
    vector: &DenseVector<R>,
    radix_mask: <Z::Int as Integer>::Limb,
    radix_shift: <Z::Int as Integer>::Limb,
    digits: usize,
) -> DenseVector<R> {
    let mut pieces = vec![R::ZERO; vector.dimension() * digits];
    for (i, &polynomial) in vector.iter().enumerate() {
        decompose_impl(
            polynomial,
            radix_mask,
            radix_shift,
            &mut pieces[i * digits..(i + 1) * digits],
        );
    }
    pieces.into()
}

pub fn matrix<Z: IntegerRing, R: PolynomialRing<Z>>(
    radix: R,
    m: usize,
    n: usize,
) -> DenseMatrix<R> {
    let mut powers = Vec::<R>::with_capacity(n);
    let mut power = R::ONE;
    powers.push(power);
    power = radix;
    powers.push(power);
    for _ in 2..n - 1 {
        powers.push(radix * power);
        power *= radix;
    }
    powers.push(radix * power);
    let powers = DenseVector::<R>::new(powers);
    let identity = DenseVector::<R>::identity(m);
    identity.tensor(powers)
}

pub fn vector<Z: IntegerRing, R: PolynomialRing<Z>>(
    polynomial: R,
    radix: Z,
    digits: usize,
) -> DenseVector<R> {
    let mut powers = Vec::<R>::with_capacity(digits);
    powers.push(polynomial);
    let mut power = radix;
    for _ in 1..digits - 1 {
        powers.push(polynomial * power);
        power *= radix;
    }
    powers.push(polynomial * power);
    powers.into()
}
