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

use crate::algebra::{AlgebraOps, IntegerModRing, PolynomialRing, Tensor, UnitalAlgebra};
use crate::integer::Integer;
use crate::matrix::{DenseMatrix, DenseVector, IdentityMatrix, ScalarMatrix};
use alloc::{vec, vec::Vec};
use core::iter::zip;

// https://eprint.iacr.org/2018/946

pub struct Params<Z: IntegerModRing> {
    radix: Z,
    mask: <Z::Int as Integer>::Limb,
    shift: <Z::Int as Integer>::Limb,
    digits: u32,
}

impl<Z: IntegerModRing> Params<Z> {
    pub const fn new(
        radix: Z,
        mask: <Z::Int as Integer>::Limb,
        shift: <Z::Int as Integer>::Limb,
        digits: u32,
    ) -> Self {
        Self {
            radix,
            mask,
            shift,
            digits,
        }
    }
}

fn decompose_impl<Z: IntegerModRing, R: PolynomialRing<Z>>(
    polynomial: &R,
    params: &Params<Z>,
    pieces: &mut [R],
) {
    for (i, coefficient) in polynomial.borrow().iter().enumerate() {
        let mut representative = coefficient.canonical();
        for piece in pieces.iter_mut() {
            piece[i] = Z::with_limb(representative & params.mask);
            representative >>= params.shift;
        }
    }
}

fn decompose_slice<Z: IntegerModRing, R: PolynomialRing<Z> + Clone>(
    slice: &[R],
    params: &Params<Z>,
) -> Vec<R> {
    let mut pieces = vec![R::ZERO; slice.len() * params.digits as usize];
    for (polynomial, pieces) in zip(slice, pieces.chunks_exact_mut(params.digits as usize)) {
        decompose_impl(polynomial, params, pieces)
    }
    pieces
}

pub fn decompose_integer<Z: IntegerModRing>(integer: &Z, params: &Params<Z>) -> DenseVector<Z> {
    let mut representative = integer.canonical();
    let mut pieces = Vec::<Z>::with_capacity(params.digits as usize);
    for _ in 0..params.digits {
        let piece = Z::with_limb(representative & params.mask);
        pieces.push(piece);
        representative >>= params.shift;
    }
    pieces.into()
}

pub fn decompose_polynomial<Z: IntegerModRing, R: PolynomialRing<Z> + Clone>(
    polynomial: &R,
    params: &Params<Z>,
) -> DenseVector<R> {
    let mut pieces = vec![R::ZERO; params.digits as usize];
    decompose_impl(polynomial, params, &mut pieces);
    pieces.into()
}

pub fn decompose_vector<Z: IntegerModRing, R: PolynomialRing<Z> + Clone>(
    vector: &DenseVector<R>,
    params: &Params<Z>,
) -> DenseVector<R> {
    let pieces = decompose_slice(vector, params);
    pieces.into()
}

pub fn decompose_matrix<Z: IntegerModRing, R: PolynomialRing<Z> + Clone>(
    matrix: &DenseMatrix<R>,
    params: &Params<Z>,
) -> DenseMatrix<R> {
    let pieces = decompose_slice(matrix.as_ref(), params);
    DenseMatrix::new(matrix.rows(), matrix.columns() * params.digits, pieces)
}

pub fn matrix<Z: IntegerModRing + Clone, R: PolynomialRing<Z> + Clone>(
    m: u32,
    n: u32,
    params: &Params<Z>,
) -> ScalarMatrix<DenseMatrix<R>> {
    debug_assert!(n >= 2);
    let mut powers = Vec::<R>::with_capacity(n as usize);
    powers.push(R::ONE);
    powers.push(params.radix.clone().into());
    let mut power = params.radix.clone();
    for _ in 2..n {
        power *= &params.radix;
        powers.push(power.clone().into());
    }

    let powers = DenseMatrix::<R>::new(1, n, powers);
    let identity = IdentityMatrix::new(m);
    identity.tensor(powers)
}

pub fn vector<Z: IntegerModRing + Clone, A: UnitalAlgebra<Z> + Clone>(
    algebra: A,
    params: &Params<Z>,
) -> DenseVector<A>
where
    for<'a> &'a A: AlgebraOps<Z, A>,
{
    let mut powers = Vec::<A>::with_capacity(params.digits as usize);
    powers.push(algebra.clone());
    let mut power = params.radix.clone();
    for _ in 1..params.digits - 1 {
        powers.push(&algebra * &power);
        power *= &params.radix;
    }
    powers.push(algebra * power);
    powers.into()
}
