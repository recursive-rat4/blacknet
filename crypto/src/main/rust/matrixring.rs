/*
 * Copyright (c) 2024-2025 Pavel Vasin
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

use crate::algebra::{Algebra, UnitalAlgebra};
use crate::freemodule::FreeModule;
use crate::magma::{AdditiveCommutativeMagma, AdditiveMagma, MultiplicativeMagma};
use crate::module::Module;
use crate::monoid::{AdditiveMonoid, MultiplicativeMonoid};
use crate::operation::{Double, Square};
use crate::ring::{Ring, UnitalRing};
use crate::semigroup::{
    AdditiveSemigroup, LeftOne, LeftZero, MultiplicativeSemigroup, RightOne, RightZero,
};
use crate::semiring::Presemiring;
use core::array;
use core::iter::{Product, Sum};
use core::ops::{Add, AddAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign};
use serde::{Deserialize, Serialize};

/// A ring of square matrices.
#[derive(Clone, Copy, Deserialize, Debug, Eq, PartialEq, Serialize)]
#[serde(bound(
    deserialize = "[R; NN]: Deserialize<'de>",
    serialize = "[R; NN]: Serialize"
))]
pub struct MatrixRing<R: Ring, const N: usize, const NN: usize> {
    elements: [R; NN],
}

impl<R: Ring, const N: usize, const NN: usize> MatrixRing<R, N, NN> {
    /// Construct a new matrix.
    pub const fn new(elements: [R; NN]) -> Self {
        const {
            assert!(N * N == NN);
        }
        Self { elements }
    }

    /// Fill a new matrix with single `element`.
    pub const fn fill(element: R) -> Self {
        Self {
            elements: [element; NN],
        }
    }

    /// Map from the scalar ring into the matrix ring.
    pub const fn const_from(scalar: R) -> Self {
        let mut elements = [R::ZERO; NN];
        let mut i = 0;
        while i < N {
            elements[i * N] = scalar;
            i += 1;
        }
        Self { elements }
    }

    /// The number of rows.
    pub const fn rows() -> usize {
        N
    }

    /// The number of columns.
    pub const fn columns() -> usize {
        N
    }

    /// The entries.
    pub const fn elements(self) -> [R; NN] {
        self.elements
    }

    /// Compute the trace.
    pub fn trace(&self) -> R {
        let mut sigma = R::ZERO;
        for i in 0..N {
            sigma += self[(i, i)]
        }
        sigma
    }

    /// Transpose.
    pub fn transpose(&self) -> Self {
        let mut m = Self::ZERO;
        for j in 0..N {
            for i in 0..N {
                m[(j, i)] = self[(i, j)];
            }
        }
        m
    }
}

impl<R: Ring, const N: usize, const NN: usize> Default for MatrixRing<R, N, NN> {
    fn default() -> Self {
        Self::ZERO
    }
}

impl<R: Ring, const N: usize, const NN: usize> From<R> for MatrixRing<R, N, NN> {
    fn from(scalar: R) -> Self {
        Self::const_from(scalar)
    }
}

impl<R: Ring, const N: usize, const NN: usize> Index<usize> for MatrixRing<R, N, NN> {
    type Output = R;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.elements[index]
    }
}

impl<R: Ring, const N: usize, const NN: usize> IndexMut<usize> for MatrixRing<R, N, NN> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.elements[index]
    }
}

impl<R: Ring, const N: usize, const NN: usize> Index<(usize, usize)> for MatrixRing<R, N, NN> {
    type Output = R;

    #[inline]
    fn index(&self, (i, j): (usize, usize)) -> &Self::Output {
        &self.elements[i * N + j]
    }
}

impl<R: Ring, const N: usize, const NN: usize> IndexMut<(usize, usize)> for MatrixRing<R, N, NN> {
    #[inline]
    fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut Self::Output {
        &mut self.elements[i * N + j]
    }
}

impl<R: Ring, const N: usize, const NN: usize> IntoIterator for MatrixRing<R, N, NN> {
    type Item = R;
    type IntoIter = core::array::IntoIter<R, NN>;

    fn into_iter(self) -> Self::IntoIter {
        self.elements.into_iter()
    }
}

impl<R: Ring, const N: usize, const NN: usize> Add for MatrixRing<R, N, NN> {
    type Output = Self;

    fn add(self, rps: Self) -> Self::Output {
        Self {
            elements: array::from_fn(|i| self.elements[i] + rps.elements[i]),
        }
    }
}

impl<R: Ring, const N: usize, const NN: usize> Add<&Self> for MatrixRing<R, N, NN> {
    type Output = Self;

    #[inline]
    fn add(self, rps: &Self) -> Self::Output {
        self + *rps
    }
}

impl<R: Ring, const N: usize, const NN: usize> AddAssign for MatrixRing<R, N, NN> {
    #[inline]
    fn add_assign(&mut self, rps: Self) {
        *self = *self + rps
    }
}

impl<R: Ring, const N: usize, const NN: usize> AddAssign<&Self> for MatrixRing<R, N, NN> {
    #[inline]
    fn add_assign(&mut self, rps: &Self) {
        *self = *self + *rps
    }
}

impl<R: Ring, const N: usize, const NN: usize> Double for MatrixRing<R, N, NN> {
    type Output = Self;

    fn double(self) -> Self {
        Self {
            elements: array::from_fn(|i| self.elements[i].double()),
        }
    }
}

impl<R: Ring, const N: usize, const NN: usize> Neg for MatrixRing<R, N, NN> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            elements: array::from_fn(|i| -self.elements[i]),
        }
    }
}

impl<R: Ring, const N: usize, const NN: usize> Sub for MatrixRing<R, N, NN> {
    type Output = Self;

    fn sub(self, rps: Self) -> Self::Output {
        Self {
            elements: array::from_fn(|i| self.elements[i] - rps.elements[i]),
        }
    }
}

impl<R: Ring, const N: usize, const NN: usize> Sub<&Self> for MatrixRing<R, N, NN> {
    type Output = Self;

    #[inline]
    fn sub(self, rps: &Self) -> Self::Output {
        self - *rps
    }
}

impl<R: Ring, const N: usize, const NN: usize> SubAssign for MatrixRing<R, N, NN> {
    #[inline]
    fn sub_assign(&mut self, rps: Self) {
        *self = *self - rps
    }
}

impl<R: Ring, const N: usize, const NN: usize> SubAssign<&Self> for MatrixRing<R, N, NN> {
    #[inline]
    fn sub_assign(&mut self, rps: &Self) {
        *self = *self - *rps
    }
}

impl<R: Ring, const N: usize, const NN: usize> Mul for MatrixRing<R, N, NN> {
    type Output = Self;

    fn mul(self, rps: Self) -> Self::Output {
        // Iterative algorithm
        let mut m = Self::ZERO;
        for i in 0..N {
            for j in 0..N {
                for k in 0..N {
                    m[(i, j)] += self[(i, k)] * rps[(k, j)];
                }
            }
        }
        m
    }
}

impl<R: Ring, const N: usize, const NN: usize> Mul<&Self> for MatrixRing<R, N, NN> {
    type Output = Self;

    #[inline]
    fn mul(self, rps: &Self) -> Self::Output {
        self * *rps
    }
}

impl<R: Ring, const N: usize, const NN: usize> MulAssign for MatrixRing<R, N, NN> {
    #[inline]
    fn mul_assign(&mut self, rps: Self) {
        *self = *self * rps
    }
}

impl<R: Ring, const N: usize, const NN: usize> MulAssign<&Self> for MatrixRing<R, N, NN> {
    #[inline]
    fn mul_assign(&mut self, rps: &Self) {
        *self = *self * *rps
    }
}

impl<R: Ring, const N: usize, const NN: usize> Square for MatrixRing<R, N, NN> {
    type Output = Self;

    #[inline]
    fn square(self) -> Self {
        self * self
    }
}

impl<R: Ring, const N: usize, const NN: usize> Mul<R> for MatrixRing<R, N, NN> {
    type Output = Self;

    fn mul(self, rps: R) -> Self::Output {
        Self {
            elements: array::from_fn(|i| self.elements[i] * rps),
        }
    }
}

impl<R: Ring, const N: usize, const NN: usize> Mul<&R> for MatrixRing<R, N, NN> {
    type Output = Self;

    #[inline]
    fn mul(self, rps: &R) -> Self::Output {
        self * *rps
    }
}

impl<R: Ring, const N: usize, const NN: usize> MulAssign<R> for MatrixRing<R, N, NN> {
    #[inline]
    fn mul_assign(&mut self, rps: R) {
        *self = *self * rps
    }
}

impl<R: Ring, const N: usize, const NN: usize> MulAssign<&R> for MatrixRing<R, N, NN> {
    #[inline]
    fn mul_assign(&mut self, rps: &R) {
        *self = *self * *rps
    }
}

impl<R: Ring, const N: usize, const NN: usize> Mul<FreeModule<R, N>> for MatrixRing<R, N, NN> {
    type Output = FreeModule<R, N>;

    fn mul(self, rps: FreeModule<R, N>) -> Self::Output {
        let mut m = FreeModule::<R, N>::IDENTITY;
        for i in 0..N {
            for j in 0..N {
                m[i] += self[(i, j)] * rps[j]
            }
        }
        m
    }
}

impl<R: Ring, const N: usize, const NN: usize> Mul<MatrixRing<R, N, NN>> for FreeModule<R, N> {
    type Output = FreeModule<R, N>;

    fn mul(self, rps: MatrixRing<R, N, NN>) -> Self::Output {
        let mut m = FreeModule::<R, N>::IDENTITY;
        for i in 0..N {
            for j in 0..N {
                m[j] += self[i] * rps[(i, j)]
            }
        }
        m
    }
}

impl<R: Ring, const N: usize, const NN: usize> Sum for MatrixRing<R, N, NN> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|lps, rps| lps + rps).unwrap_or(Self::LEFT_ZERO)
    }
}

impl<'a, R: Ring, const N: usize, const NN: usize> Sum<&'a Self> for MatrixRing<R, N, NN> {
    #[inline]
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.copied().sum()
    }
}

impl<R: UnitalRing, const N: usize, const NN: usize> Product for MatrixRing<R, N, NN> {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|lps, rps| lps * rps).unwrap_or(Self::LEFT_ONE)
    }
}

impl<'a, R: UnitalRing, const N: usize, const NN: usize> Product<&'a Self>
    for MatrixRing<R, N, NN>
{
    #[inline]
    fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.copied().product()
    }
}

impl<R: Ring, const N: usize, const NN: usize> LeftZero for MatrixRing<R, N, NN> {
    const LEFT_ZERO: Self = Self::const_from(R::LEFT_ZERO);
}

impl<R: Ring, const N: usize, const NN: usize> RightZero for MatrixRing<R, N, NN> {
    const RIGHT_ZERO: Self = Self::const_from(R::RIGHT_ZERO);
}

impl<R: UnitalRing, const N: usize, const NN: usize> LeftOne for MatrixRing<R, N, NN> {
    const LEFT_ONE: Self = Self::const_from(R::LEFT_ONE);
}

impl<R: UnitalRing, const N: usize, const NN: usize> RightOne for MatrixRing<R, N, NN> {
    const RIGHT_ONE: Self = Self::const_from(R::RIGHT_ONE);
}

impl<R: Ring, const N: usize, const NN: usize> AdditiveMagma for MatrixRing<R, N, NN> {}

impl<R: Ring, const N: usize, const NN: usize> AdditiveCommutativeMagma for MatrixRing<R, N, NN> {}

impl<R: Ring, const N: usize, const NN: usize> AdditiveSemigroup for MatrixRing<R, N, NN> {}

impl<R: Ring, const N: usize, const NN: usize> AdditiveMonoid for MatrixRing<R, N, NN> {
    const IDENTITY: Self = Self::const_from(R::ZERO);
}

impl<R: Ring, const N: usize, const NN: usize> MultiplicativeMagma for MatrixRing<R, N, NN> {}

impl<R: Ring, const N: usize, const NN: usize> MultiplicativeSemigroup for MatrixRing<R, N, NN> {}

impl<R: UnitalRing, const N: usize, const NN: usize> MultiplicativeMonoid for MatrixRing<R, N, NN> {
    const IDENTITY: Self = Self::const_from(R::ONE);
}

impl<R: Ring, const N: usize, const NN: usize> Module<R> for MatrixRing<R, N, NN> {}

impl<R: Ring, const N: usize, const NN: usize> Algebra<R> for MatrixRing<R, N, NN> {}

impl<R: UnitalRing, const N: usize, const NN: usize> UnitalAlgebra<R> for MatrixRing<R, N, NN> {}
