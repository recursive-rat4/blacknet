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

use crate::algebra::{
    AdditiveCommutativeMagma, AdditiveMonoid, AdditiveSemigroup, Algebra, Double, FreeModule,
    LeftOne, LeftZero, MultiplicativeMonoid, MultiplicativeSemigroup, One, RightOne, RightZero,
    Ring, RingOps, Semimodule, Set, Square, UnitalAlgebra, UnitalRing, Zero,
};
use core::array;
use core::iter::{Product, Sum, zip};
use core::mem::{MaybeUninit, transmute_copy};
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
    pub const fn fill(element: R) -> Self
    where
        R: Copy,
    {
        Self {
            elements: [element; NN],
        }
    }

    /// Map from the scalar ring into the matrix ring.
    pub const fn const_from(scalar: R) -> Self
    where
        R: Copy,
    {
        let mut elements = [R::ZERO; NN];
        let mut i = 0;
        while i < N {
            elements[i * N + i] = scalar;
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

    /// Compute the trace.
    pub fn trace(&self) -> R {
        let mut sigma = R::ZERO;
        for i in 0..N {
            sigma += &self[(i, i)]
        }
        sigma
    }

    /// Transpose.
    pub fn transpose(&self) -> Self
    where
        R: Copy,
    {
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

impl<R: Ring + Copy, const N: usize, const NN: usize> From<R> for MatrixRing<R, N, NN> {
    fn from(scalar: R) -> Self {
        Self::const_from(scalar)
    }
}

impl<R: Ring, const N: usize, const NN: usize> AsRef<[R; NN]> for MatrixRing<R, N, NN> {
    #[inline]
    fn as_ref(&self) -> &[R; NN] {
        &self.elements
    }
}

impl<R: Ring, const N: usize, const NN: usize> AsMut<[R; NN]> for MatrixRing<R, N, NN> {
    #[inline]
    fn as_mut(&mut self) -> &mut [R; NN] {
        &mut self.elements
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

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.elements.into_iter()
    }
}

impl<'a, R: Ring, const N: usize, const NN: usize> IntoIterator for &'a MatrixRing<R, N, NN> {
    type Item = &'a R;
    type IntoIter = core::slice::Iter<'a, R>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.elements.iter()
    }
}

impl<'a, R: Ring, const N: usize, const NN: usize> IntoIterator for &'a mut MatrixRing<R, N, NN> {
    type Item = &'a mut R;
    type IntoIter = core::slice::IterMut<'a, R>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.elements.iter_mut()
    }
}

impl<R: Ring, const N: usize, const NN: usize> Add for MatrixRing<R, N, NN> {
    type Output = Self;

    fn add(self, rps: Self) -> Self::Output {
        let mut lps = self;
        zip(&mut lps, rps).for_each(|(l, r)| *l += r);
        lps
    }
}

impl<R: Ring, const N: usize, const NN: usize> Add<&Self> for MatrixRing<R, N, NN> {
    type Output = Self;

    fn add(self, rps: &Self) -> Self::Output {
        let mut lps = self;
        zip(&mut lps, rps).for_each(|(l, r)| *l += r);
        lps
    }
}

impl<R: Ring, const N: usize, const NN: usize> Add<MatrixRing<R, N, NN>> for &MatrixRing<R, N, NN>
where
    for<'a> &'a R: RingOps<R>,
{
    type Output = MatrixRing<R, N, NN>;

    fn add(self, rps: MatrixRing<R, N, NN>) -> Self::Output {
        let mut rps = rps;
        zip(self, &mut rps).for_each(|(l, r)| *r += l);
        rps
    }
}

impl<'a, R: Ring, const N: usize, const NN: usize> Add<&'a MatrixRing<R, N, NN>>
    for &MatrixRing<R, N, NN>
where
    for<'b> &'b R: RingOps<R>,
{
    type Output = MatrixRing<R, N, NN>;

    fn add(self, rps: &'a MatrixRing<R, N, NN>) -> Self::Output {
        let elements = array::from_fn(|i| &self.elements[i] + &rps.elements[i]);
        Self::Output { elements }
    }
}

impl<R: Ring, const N: usize, const NN: usize> AddAssign for MatrixRing<R, N, NN> {
    fn add_assign(&mut self, rps: Self) {
        zip(self, rps).for_each(|(l, r)| *l += r);
    }
}

impl<R: Ring, const N: usize, const NN: usize> AddAssign<&Self> for MatrixRing<R, N, NN> {
    fn add_assign(&mut self, rps: &Self) {
        zip(self, rps).for_each(|(l, r)| *l += r);
    }
}

impl<R: Ring, const N: usize, const NN: usize> Double for MatrixRing<R, N, NN> {
    type Output = Self;

    fn double(self) -> Self {
        Self {
            elements: self.elements.map(Double::double),
        }
    }
}

impl<R: Ring, const N: usize, const NN: usize> Double for &MatrixRing<R, N, NN>
where
    for<'a> &'a R: RingOps<R>,
{
    type Output = MatrixRing<R, N, NN>;

    fn double(self) -> Self::Output {
        let elements = array::from_fn(|i| (&self.elements[i]).double());
        Self::Output { elements }
    }
}

impl<R: Ring, const N: usize, const NN: usize> Neg for MatrixRing<R, N, NN> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            elements: self.elements.map(Neg::neg),
        }
    }
}

impl<R: Ring, const N: usize, const NN: usize> Neg for &MatrixRing<R, N, NN>
where
    for<'a> &'a R: RingOps<R>,
{
    type Output = MatrixRing<R, N, NN>;

    fn neg(self) -> Self::Output {
        let elements = array::from_fn(|i| -&self.elements[i]);
        Self::Output { elements }
    }
}

impl<R: Ring, const N: usize, const NN: usize> Sub for MatrixRing<R, N, NN> {
    type Output = Self;

    fn sub(self, rps: Self) -> Self::Output {
        let mut lps = self;
        zip(&mut lps, rps).for_each(|(l, r)| *l -= r);
        lps
    }
}

impl<R: Ring, const N: usize, const NN: usize> Sub<&Self> for MatrixRing<R, N, NN> {
    type Output = Self;

    fn sub(self, rps: &Self) -> Self::Output {
        let mut lps = self;
        zip(&mut lps, rps).for_each(|(l, r)| *l -= r);
        lps
    }
}

impl<R: Ring, const N: usize, const NN: usize> Sub<MatrixRing<R, N, NN>> for &MatrixRing<R, N, NN>
where
    for<'a> &'a R: RingOps<R>,
{
    type Output = MatrixRing<R, N, NN>;

    fn sub(self, rps: MatrixRing<R, N, NN>) -> Self::Output {
        let mut rps = rps;
        zip(self, &mut rps).for_each(|(l, r)| *r = l - &*r);
        rps
    }
}

impl<'a, R: Ring, const N: usize, const NN: usize> Sub<&'a MatrixRing<R, N, NN>>
    for &MatrixRing<R, N, NN>
where
    for<'b> &'b R: RingOps<R>,
{
    type Output = MatrixRing<R, N, NN>;

    fn sub(self, rps: &'a MatrixRing<R, N, NN>) -> Self::Output {
        let elements = array::from_fn(|i| &self.elements[i] - &rps.elements[i]);
        Self::Output { elements }
    }
}

impl<R: Ring, const N: usize, const NN: usize> SubAssign for MatrixRing<R, N, NN> {
    fn sub_assign(&mut self, rps: Self) {
        zip(self, rps).for_each(|(l, r)| *l -= r);
    }
}

impl<R: Ring, const N: usize, const NN: usize> SubAssign<&Self> for MatrixRing<R, N, NN> {
    fn sub_assign(&mut self, rps: &Self) {
        zip(self, rps).for_each(|(l, r)| *l -= r);
    }
}

impl<R: Ring, const N: usize, const NN: usize> Mul for MatrixRing<R, N, NN>
where
    for<'a> &'a R: RingOps<R>,
{
    type Output = Self;

    #[inline]
    fn mul(self, rps: Self) -> Self::Output {
        &self * &rps
    }
}

impl<R: Ring, const N: usize, const NN: usize> Mul<&Self> for MatrixRing<R, N, NN>
where
    for<'a> &'a R: RingOps<R>,
{
    type Output = Self;

    #[inline]
    fn mul(self, rps: &Self) -> Self::Output {
        &self * rps
    }
}

impl<R: Ring, const N: usize, const NN: usize> Mul<MatrixRing<R, N, NN>> for &MatrixRing<R, N, NN>
where
    for<'a> &'a R: RingOps<R>,
{
    type Output = MatrixRing<R, N, NN>;

    #[inline]
    fn mul(self, rps: MatrixRing<R, N, NN>) -> Self::Output {
        self * &rps
    }
}

impl<'a, R: Ring, const N: usize, const NN: usize> Mul<&'a MatrixRing<R, N, NN>>
    for &MatrixRing<R, N, NN>
where
    for<'b> &'b R: RingOps<R>,
{
    type Output = MatrixRing<R, N, NN>;

    fn mul(self, rps: &'a MatrixRing<R, N, NN>) -> Self::Output {
        // Iterative algorithm
        let mut out = [const { MaybeUninit::<R>::uninit() }; NN];
        for i in 0..N {
            for j in 0..N {
                let mut m = R::ZERO;
                for k in 0..N {
                    m += &self[(i, k)] * &rps[(k, j)];
                }
                out[i * N + j].write(m);
            }
        }
        let elements = out.map(|i| unsafe { i.assume_init() });
        Self::Output { elements }
    }
}

impl<R: Ring, const N: usize, const NN: usize> MulAssign for MatrixRing<R, N, NN>
where
    for<'a> &'a R: RingOps<R>,
{
    #[inline]
    fn mul_assign(&mut self, rps: Self) {
        *self = &*self * &rps
    }
}

impl<R: Ring, const N: usize, const NN: usize> MulAssign<&Self> for MatrixRing<R, N, NN>
where
    for<'a> &'a R: RingOps<R>,
{
    #[inline]
    fn mul_assign(&mut self, rps: &Self) {
        *self = &*self * rps
    }
}

impl<R: Ring, const N: usize, const NN: usize> Square for MatrixRing<R, N, NN>
where
    for<'a> &'a R: RingOps<R>,
{
    type Output = Self;

    #[inline]
    fn square(self) -> Self {
        &self * &self
    }
}

impl<R: Ring, const N: usize, const NN: usize> Square for &MatrixRing<R, N, NN>
where
    for<'a> &'a R: RingOps<R>,
{
    type Output = MatrixRing<R, N, NN>;

    #[inline]
    fn square(self) -> Self::Output {
        self * self
    }
}

impl<R: Ring, const N: usize, const NN: usize> Mul<R> for MatrixRing<R, N, NN> {
    type Output = Self;

    #[inline]
    fn mul(self, rps: R) -> Self::Output {
        self * &rps
    }
}

impl<R: Ring, const N: usize, const NN: usize> Mul<&R> for MatrixRing<R, N, NN> {
    type Output = Self;

    fn mul(self, rps: &R) -> Self::Output {
        let mut lps = self;
        lps.elements.iter_mut().for_each(|l| *l *= rps);
        lps
    }
}

impl<R: Ring, const N: usize, const NN: usize> Mul<R> for &MatrixRing<R, N, NN>
where
    for<'a> &'a R: RingOps<R>,
{
    type Output = MatrixRing<R, N, NN>;

    #[inline]
    fn mul(self, rps: R) -> Self::Output {
        self * &rps
    }
}

impl<R: Ring, const N: usize, const NN: usize> Mul<&R> for &MatrixRing<R, N, NN>
where
    for<'a> &'a R: RingOps<R>,
{
    type Output = MatrixRing<R, N, NN>;

    fn mul(self, rps: &R) -> Self::Output {
        let elements = array::from_fn(|i| &self.elements[i] * rps);
        Self::Output { elements }
    }
}

impl<R: Ring, const N: usize, const NN: usize> MulAssign<R> for MatrixRing<R, N, NN> {
    #[inline]
    fn mul_assign(&mut self, rps: R) {
        *self *= &rps
    }
}

impl<R: Ring, const N: usize, const NN: usize> MulAssign<&R> for MatrixRing<R, N, NN> {
    fn mul_assign(&mut self, rps: &R) {
        self.elements.iter_mut().for_each(|l| *l *= rps);
    }
}

impl<R: Ring + Copy, const N: usize, const NN: usize> Mul<FreeModule<R, N>>
    for MatrixRing<R, N, NN>
{
    type Output = FreeModule<R, N>;

    fn mul(self, rps: FreeModule<R, N>) -> Self::Output {
        let mut m = FreeModule::<R, N>::ZERO;
        for i in 0..N {
            for j in 0..N {
                m[i] += self[(i, j)] * rps[j]
            }
        }
        m
    }
}

impl<R: Ring + Copy, const N: usize, const NN: usize> Mul<MatrixRing<R, N, NN>>
    for FreeModule<R, N>
{
    type Output = FreeModule<R, N>;

    fn mul(self, rps: MatrixRing<R, N, NN>) -> Self::Output {
        let mut m = FreeModule::<R, N>::ZERO;
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

impl<'a, R: Ring + Clone, const N: usize, const NN: usize> Sum<&'a Self> for MatrixRing<R, N, NN> {
    fn sum<I: Iterator<Item = &'a Self>>(mut iter: I) -> Self {
        let first = match iter.next() {
            Some(i) => i.clone(),
            None => return Self::ZERO,
        };
        iter.fold(first, |lps, rps| lps + rps)
    }
}

impl<R: UnitalRing, const N: usize, const NN: usize> Product for MatrixRing<R, N, NN>
where
    for<'a> &'a R: RingOps<R>,
{
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|lps, rps| lps * rps).unwrap_or(Self::LEFT_ONE)
    }
}

impl<'a, R: UnitalRing + Clone, const N: usize, const NN: usize> Product<&'a Self>
    for MatrixRing<R, N, NN>
where
    for<'b> &'b R: RingOps<R>,
{
    fn product<I: Iterator<Item = &'a Self>>(mut iter: I) -> Self {
        let first = match iter.next() {
            Some(i) => i.clone(),
            None => return Self::LEFT_ONE,
        };
        iter.fold(first, |lps, rps| lps * rps)
    }
}

impl<R: Ring, const N: usize, const NN: usize> LeftZero for MatrixRing<R, N, NN> {
    const LEFT_ZERO: Self = Self::ZERO;
}

impl<R: Ring, const N: usize, const NN: usize> RightZero for MatrixRing<R, N, NN> {
    const RIGHT_ZERO: Self = Self::ZERO;
}

impl<R: Ring, const N: usize, const NN: usize> Zero for MatrixRing<R, N, NN> {
    const ZERO: Self = {
        let elements = [R::ZERO; NN];
        Self { elements }
    };
}

impl<R: UnitalRing, const N: usize, const NN: usize> LeftOne for MatrixRing<R, N, NN> {
    const LEFT_ONE: Self = Self::ONE;
}

impl<R: UnitalRing, const N: usize, const NN: usize> RightOne for MatrixRing<R, N, NN> {
    const RIGHT_ONE: Self = Self::ONE;
}

impl<R: UnitalRing, const N: usize, const NN: usize> One for MatrixRing<R, N, NN> {
    const ONE: Self = {
        let mut t = [const { MaybeUninit::<R>::uninit() }; NN];
        let mut i = 0;
        while i < N {
            let mut j = 0;
            while j < N {
                t[i * N + j].write(if i != j { R::ZERO } else { R::ONE });
                j += 1;
            }
            i += 1;
        }
        let elements: [R; NN] = unsafe { transmute_copy(&t) };
        Self { elements }
    };
}

impl<R: Ring, const N: usize, const NN: usize> Set for MatrixRing<R, N, NN> {}

impl<R: Ring, const N: usize, const NN: usize> AdditiveCommutativeMagma for MatrixRing<R, N, NN> {}

impl<R: Ring, const N: usize, const NN: usize> AdditiveSemigroup for MatrixRing<R, N, NN> {}

impl<R: Ring + Clone, const N: usize, const NN: usize> AdditiveMonoid for MatrixRing<R, N, NN> {}

impl<R: Ring, const N: usize, const NN: usize> MultiplicativeSemigroup for MatrixRing<R, N, NN> where
    for<'a> &'a R: RingOps<R>
{
}

impl<R: UnitalRing + Clone, const N: usize, const NN: usize> MultiplicativeMonoid
    for MatrixRing<R, N, NN>
where
    for<'a> &'a R: RingOps<R>,
{
}

impl<R: Ring + Clone, const N: usize, const NN: usize> Semimodule<R> for MatrixRing<R, N, NN> {}

impl<R: Ring + Copy, const N: usize, const NN: usize> Algebra<R> for MatrixRing<R, N, NN> where
    for<'a> &'a R: RingOps<R>
{
}

impl<R: UnitalRing + Copy, const N: usize, const NN: usize> UnitalAlgebra<R>
    for MatrixRing<R, N, NN>
where
    for<'a> &'a R: RingOps<R>,
{
}
