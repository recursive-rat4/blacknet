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

use crate::algebra::{AlgebraOps, IntegerModRing, PolynomialRing, RingOps, Tensor, UnitalAlgebra};
use crate::integer::Integer;
use crate::matrix::{DenseMatrix, DenseVector, IdentityMatrix, ScalarMatrix};
use alloc::{vec, vec::Vec};
use core::{
    cell::OnceCell,
    iter::{once, zip},
};

// https://eprint.iacr.org/2018/946

pub struct Gadget<Z: IntegerModRing> {
    radix: Z,
    mask: <Z::Int as Integer>::Limb,
    shift: <Z::Int as Integer>::Limb,
    digits: u32,
    powers: OnceCell<Vec<Z>>,
}

impl<Z: IntegerModRing> Gadget<Z> {
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
            powers: OnceCell::new(),
        }
    }

    pub const fn digits(&self) -> u32 {
        self.digits
    }

    fn powers(&self) -> &[Z]
    where
        Z: Clone,
    {
        self.powers.get_or_init(|| {
            let mut powers = Vec::<Z>::with_capacity(self.digits as usize);
            let mut power = self.radix.clone();
            for _ in 2..self.digits {
                powers.push(power.clone());
                power *= &self.radix;
            }
            if self.digits > 1 {
                powers.push(power);
            }
            powers
        })
    }

    fn decompose_impl<R: PolynomialRing<Z>>(&self, polynomial: &R, pieces: &mut [R]) {
        for (i, coefficient) in polynomial.borrow().iter().enumerate() {
            let mut representative = coefficient.canonical();
            for piece in pieces.iter_mut() {
                piece[i] = Z::with_limb(representative & self.mask);
                representative >>= self.shift;
            }
        }
    }

    pub fn decompose_sequence<R: PolynomialRing<Z> + Clone>(&self, sequence: &[R]) -> Vec<R> {
        let mut pieces = vec![R::ZERO; sequence.len() * self.digits as usize];
        for (polynomial, pieces) in zip(sequence, pieces.chunks_exact_mut(self.digits as usize)) {
            self.decompose_impl(polynomial, pieces)
        }
        pieces
    }

    pub fn decompose_integer(&self, integer: &Z) -> DenseVector<Z> {
        let mut representative = integer.canonical();
        let mut pieces = Vec::<Z>::with_capacity(self.digits as usize);
        for _ in 0..self.digits {
            let piece = Z::with_limb(representative & self.mask);
            pieces.push(piece);
            representative >>= self.shift;
        }
        pieces.into()
    }

    pub fn decompose_polynomial<R: PolynomialRing<Z> + Clone>(
        &self,
        polynomial: &R,
    ) -> DenseVector<R> {
        let mut pieces = vec![R::ZERO; self.digits as usize];
        self.decompose_impl(polynomial, &mut pieces);
        pieces.into()
    }

    pub fn decompose_vector<R: PolynomialRing<Z> + Clone>(
        &self,
        vector: &DenseVector<R>,
    ) -> DenseVector<R> {
        let pieces = self.decompose_sequence(vector);
        pieces.into()
    }

    pub fn decompose_matrix<R: PolynomialRing<Z> + Clone>(
        &self,
        matrix: &DenseMatrix<R>,
    ) -> DenseMatrix<R> {
        let pieces = self.decompose_sequence(matrix.as_ref());
        DenseMatrix::new(matrix.rows(), matrix.columns() * self.digits, pieces)
    }

    fn compose_impl<R: PolynomialRing<Z> + Clone>(&self, pieces: &[R]) -> R
    where
        Z: Clone,
        for<'a> &'a R: AlgebraOps<Z, R>,
    {
        debug_assert!(pieces.len() == self.digits as usize);
        let mut pieces = pieces.iter();
        let Some(polynomial) = pieces.next().cloned() else {
            return R::ZERO;
        };
        once(polynomial)
            .chain(zip(pieces, self.powers()).map(|(piece, power)| piece * power))
            .sum()
    }

    pub fn compose_sequence<R: PolynomialRing<Z> + Clone>(&self, sequence: &[R]) -> Vec<R>
    where
        Z: Clone,
        for<'a> &'a R: AlgebraOps<Z, R>,
    {
        debug_assert!(sequence.len().is_multiple_of(self.digits as usize));
        sequence
            .chunks_exact(self.digits as usize)
            .map(|pieces| self.compose_impl::<R>(pieces))
            .collect()
    }

    pub fn compose_integer(&self, pieces: &DenseVector<Z>) -> Z
    where
        Z: Clone,
        for<'a> &'a Z: RingOps<Z>,
    {
        debug_assert!(pieces.dimension() == self.digits);
        let mut pieces = pieces.into_iter();
        let Some(integer) = pieces.next().cloned() else {
            return Z::ZERO;
        };
        once(integer)
            .chain(zip(pieces, self.powers()).map(|(piece, power)| piece * power))
            .sum()
    }

    pub fn compose_polynomial<R: PolynomialRing<Z> + Clone>(&self, pieces: &DenseVector<R>) -> R
    where
        Z: Clone,
        for<'a> &'a R: AlgebraOps<Z, R>,
    {
        self.compose_impl::<R>(pieces)
    }

    pub fn compose_vector<R: PolynomialRing<Z> + Clone>(
        &self,
        pieces: &DenseVector<R>,
    ) -> DenseVector<R>
    where
        Z: Clone,
        for<'a> &'a R: AlgebraOps<Z, R>,
    {
        self.compose_sequence::<R>(pieces).into()
    }

    pub fn compose_matrix<R: PolynomialRing<Z> + Clone>(
        &self,
        matrix: &DenseMatrix<R>,
    ) -> DenseMatrix<R>
    where
        Z: Clone,
        for<'a> &'a R: AlgebraOps<Z, R>,
    {
        let elements = self.compose_sequence::<R>(matrix.as_ref());
        DenseMatrix::new(matrix.rows(), matrix.columns() / self.digits, elements)
    }

    pub fn matrix<R: PolynomialRing<Z> + Clone>(
        &self,
        m: u32,
        n: u32,
    ) -> ScalarMatrix<DenseMatrix<R>>
    where
        Z: Clone,
    {
        let powers = once(R::ONE)
            .chain(self.powers().iter().map(|power| R::from(power.clone())))
            .collect();
        let powers = DenseMatrix::<R>::new(1, n, powers);
        let identity = IdentityMatrix::new(m);
        identity.tensor(powers)
    }

    pub fn vector<A: UnitalAlgebra<Z> + Clone>(&self, algebra: &A) -> DenseVector<A>
    where
        Z: Clone,
        for<'a> &'a A: AlgebraOps<Z, A>,
    {
        once(algebra.clone())
            .chain(self.powers().iter().map(|power| algebra * power))
            .collect()
    }
}
