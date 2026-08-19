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
    AdditiveCommutativeMagma, AdditiveSemigroup, Algebra, Array, Commutator, Double, LeftOne,
    LeftZero, MultiplicativeSemigroup, One, RightOne, RightZero, Ring, RingOps, Semimodule,
    Semiring, SemiringOps, Set, Square, UnitalSemiring, VectorRing, Zero,
};
use crate::symmetric::{Absorb, Duplexer, Squeeze};
use core::iter::{Product, Sum};
use core::mem::{MaybeUninit, transmute_copy};
use core::ops::{Add, AddAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign};
#[cfg(feature = "rayon")]
use rayon::iter::IntoParallelIterator;
use serde::{Deserialize, Serialize};
use zeroize::Zeroize;

/// A ring of square matrices.
pub type MatrixRing<R, const N: usize, const NN: usize> = MatrixSpace<R, N, N, NN>;

/// A space of `m × n` matrices.
#[derive(Clone, Copy, Deserialize, Debug, Eq, PartialEq, Serialize, Zeroize)]
#[serde(bound(
    deserialize = "[R; MN]: Deserialize<'de>",
    serialize = "[R; MN]: Serialize"
))]
#[zeroize(bound = "R: Zeroize")]
pub struct MatrixSpace<R: Semiring, const M: usize, const N: usize, const MN: usize> {
    elements: Array<R, MN>,
}

impl<R: Semiring, const M: usize, const N: usize, const MN: usize> MatrixSpace<R, M, N, MN> {
    /// Construct a new matrix.
    pub const fn new(elements: Array<R, MN>) -> Self {
        const {
            assert!(M * N == MN);
        }
        Self { elements }
    }

    /// Fill a new matrix with single `element`.
    pub const fn fill(element: R) -> Self
    where
        R: Copy,
    {
        let elements = [element; MN];
        Self::new(Array::new(elements))
    }

    /// The number of rows.
    pub const fn rows() -> usize {
        M
    }

    /// The number of columns.
    pub const fn columns() -> usize {
        N
    }

    /// Transpose.
    pub fn transpose(&self) -> MatrixSpace<R, N, M, MN>
    where
        R: Copy,
    {
        let mut m = MatrixSpace::<R, N, M, MN>::ZERO;
        for j in 0..N {
            for i in 0..M {
                m[(j, i)] = self[(i, j)];
            }
        }
        m
    }
}

impl<R: Semiring, const N: usize, const NN: usize> MatrixSpace<R, N, N, NN> {
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
        let elements = Array::new(elements);
        Self { elements }
    }

    /// Compute the trace.
    pub fn trace(&self) -> R {
        (0..N).map(|i| &self[(i, i)]).sum()
    }
}

impl<R: Semiring, const M: usize, const N: usize, const MN: usize> Default
    for MatrixSpace<R, M, N, MN>
{
    #[inline]
    fn default() -> Self {
        Self::ZERO
    }
}

impl<R: Semiring + Copy, const N: usize, const NN: usize> From<R> for MatrixSpace<R, N, N, NN> {
    fn from(scalar: R) -> Self {
        Self::const_from(scalar)
    }
}

impl<R: Semiring, const M: usize, const N: usize, const MN: usize> From<[R; MN]>
    for MatrixSpace<R, M, N, MN>
{
    #[inline]
    fn from(elements: [R; MN]) -> Self {
        let elements = Array::new(elements);
        Self { elements }
    }
}

impl<R: Semiring, const M: usize, const N: usize, const MN: usize> AsRef<[R; MN]>
    for MatrixSpace<R, M, N, MN>
{
    #[inline]
    fn as_ref(&self) -> &[R; MN] {
        &self.elements
    }
}

impl<R: Semiring, const M: usize, const N: usize, const MN: usize> AsMut<[R; MN]>
    for MatrixSpace<R, M, N, MN>
{
    #[inline]
    fn as_mut(&mut self) -> &mut [R; MN] {
        &mut self.elements
    }
}

impl<R: Semiring, const M: usize, const N: usize, const MN: usize> Index<usize>
    for MatrixSpace<R, M, N, MN>
{
    type Output = R;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.elements[index]
    }
}

impl<R: Semiring, const M: usize, const N: usize, const MN: usize> IndexMut<usize>
    for MatrixSpace<R, M, N, MN>
{
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.elements[index]
    }
}

impl<R: Semiring, const M: usize, const N: usize, const MN: usize> Index<(usize, usize)>
    for MatrixSpace<R, M, N, MN>
{
    type Output = R;

    #[inline]
    fn index(&self, (i, j): (usize, usize)) -> &Self::Output {
        &self.elements[i * N + j]
    }
}

impl<R: Semiring, const M: usize, const N: usize, const MN: usize> IndexMut<(usize, usize)>
    for MatrixSpace<R, M, N, MN>
{
    #[inline]
    fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut Self::Output {
        &mut self.elements[i * N + j]
    }
}

impl<R: Semiring, const M: usize, const N: usize, const MN: usize> IntoIterator
    for MatrixSpace<R, M, N, MN>
{
    type Item = R;
    type IntoIter = core::array::IntoIter<R, MN>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.elements.into_iter()
    }
}

impl<'a, R: Semiring, const M: usize, const N: usize, const MN: usize> IntoIterator
    for &'a MatrixSpace<R, M, N, MN>
{
    type Item = &'a R;
    type IntoIter = core::slice::Iter<'a, R>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.elements.iter()
    }
}

impl<'a, R: Semiring, const M: usize, const N: usize, const MN: usize> IntoIterator
    for &'a mut MatrixSpace<R, M, N, MN>
{
    type Item = &'a mut R;
    type IntoIter = core::slice::IterMut<'a, R>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.elements.iter_mut()
    }
}

#[cfg(feature = "rayon")]
impl<R: Semiring + Send, const M: usize, const N: usize, const MN: usize> IntoParallelIterator
    for MatrixSpace<R, M, N, MN>
{
    type Item = R;
    type Iter = rayon::array::IntoIter<R, MN>;

    #[inline]
    fn into_par_iter(self) -> Self::Iter {
        self.elements.into_par_iter()
    }
}

#[cfg(feature = "rayon")]
impl<'a, R: Semiring + Sync, const M: usize, const N: usize, const MN: usize> IntoParallelIterator
    for &'a MatrixSpace<R, M, N, MN>
{
    type Item = &'a R;
    type Iter = rayon::slice::Iter<'a, R>;

    #[inline]
    fn into_par_iter(self) -> Self::Iter {
        (&self.elements).into_par_iter()
    }
}

#[cfg(feature = "rayon")]
impl<'a, R: Semiring + Send, const M: usize, const N: usize, const MN: usize> IntoParallelIterator
    for &'a mut MatrixSpace<R, M, N, MN>
{
    type Item = &'a mut R;
    type Iter = rayon::slice::IterMut<'a, R>;

    #[inline]
    fn into_par_iter(self) -> Self::Iter {
        (&mut self.elements).into_par_iter()
    }
}

impl<R: Semiring, const M: usize, const N: usize, const MN: usize> Add
    for MatrixSpace<R, M, N, MN>
{
    type Output = Self;

    #[inline]
    fn add(self, rps: Self) -> Self::Output {
        Self::new(self.elements + rps.elements)
    }
}

impl<R: Semiring, const M: usize, const N: usize, const MN: usize> Add<&Self>
    for MatrixSpace<R, M, N, MN>
{
    type Output = Self;

    #[inline]
    fn add(self, rps: &Self) -> Self::Output {
        Self::new(self.elements + &rps.elements)
    }
}

impl<R: Semiring, const M: usize, const N: usize, const MN: usize> Add<MatrixSpace<R, M, N, MN>>
    for &MatrixSpace<R, M, N, MN>
where
    for<'a> &'a R: SemiringOps<R>,
{
    type Output = MatrixSpace<R, M, N, MN>;

    #[inline]
    fn add(self, rps: MatrixSpace<R, M, N, MN>) -> Self::Output {
        Self::Output::new(&self.elements + rps.elements)
    }
}

impl<'a, R: Semiring, const M: usize, const N: usize, const MN: usize>
    Add<&'a MatrixSpace<R, M, N, MN>> for &MatrixSpace<R, M, N, MN>
where
    for<'b> &'b R: SemiringOps<R>,
{
    type Output = MatrixSpace<R, M, N, MN>;

    #[inline]
    fn add(self, rps: &'a MatrixSpace<R, M, N, MN>) -> Self::Output {
        Self::Output::new(&self.elements + &rps.elements)
    }
}

impl<R: Semiring, const M: usize, const N: usize, const MN: usize> AddAssign
    for MatrixSpace<R, M, N, MN>
{
    #[inline]
    fn add_assign(&mut self, rps: Self) {
        self.elements += rps.elements
    }
}

impl<R: Semiring, const M: usize, const N: usize, const MN: usize> AddAssign<&Self>
    for MatrixSpace<R, M, N, MN>
{
    #[inline]
    fn add_assign(&mut self, rps: &Self) {
        self.elements += &rps.elements
    }
}

impl<R: Semiring, const M: usize, const N: usize, const MN: usize> Double
    for MatrixSpace<R, M, N, MN>
{
    type Output = Self;

    #[inline]
    fn double(self) -> Self {
        Self::new(self.elements.double())
    }
}

impl<R: Semiring, const M: usize, const N: usize, const MN: usize> Double
    for &MatrixSpace<R, M, N, MN>
where
    for<'a> &'a R: SemiringOps<R>,
{
    type Output = MatrixSpace<R, M, N, MN>;

    #[inline]
    fn double(self) -> Self::Output {
        Self::Output::new((&self.elements).double())
    }
}

impl<R: Ring, const M: usize, const N: usize, const MN: usize> Neg for MatrixSpace<R, M, N, MN> {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Self::new(-self.elements)
    }
}

impl<R: Ring, const M: usize, const N: usize, const MN: usize> Neg for &MatrixSpace<R, M, N, MN>
where
    for<'a> &'a R: RingOps<R>,
{
    type Output = MatrixSpace<R, M, N, MN>;

    #[inline]
    fn neg(self) -> Self::Output {
        Self::Output::new(-&self.elements)
    }
}

impl<R: Ring, const M: usize, const N: usize, const MN: usize> Sub for MatrixSpace<R, M, N, MN> {
    type Output = Self;

    #[inline]
    fn sub(self, rps: Self) -> Self::Output {
        Self::new(self.elements - rps.elements)
    }
}

impl<R: Ring, const M: usize, const N: usize, const MN: usize> Sub<&Self>
    for MatrixSpace<R, M, N, MN>
{
    type Output = Self;

    #[inline]
    fn sub(self, rps: &Self) -> Self::Output {
        Self::new(self.elements - &rps.elements)
    }
}

impl<R: Ring, const M: usize, const N: usize, const MN: usize> Sub<MatrixSpace<R, M, N, MN>>
    for &MatrixSpace<R, M, N, MN>
where
    for<'a> &'a R: RingOps<R>,
{
    type Output = MatrixSpace<R, M, N, MN>;

    #[inline]
    fn sub(self, rps: MatrixSpace<R, M, N, MN>) -> Self::Output {
        Self::Output::new(&self.elements - rps.elements)
    }
}

impl<'a, R: Ring, const M: usize, const N: usize, const MN: usize> Sub<&'a MatrixSpace<R, M, N, MN>>
    for &MatrixSpace<R, M, N, MN>
where
    for<'b> &'b R: RingOps<R>,
{
    type Output = MatrixSpace<R, M, N, MN>;

    #[inline]
    fn sub(self, rps: &'a MatrixSpace<R, M, N, MN>) -> Self::Output {
        Self::Output::new(&self.elements - &rps.elements)
    }
}

impl<R: Ring, const M: usize, const N: usize, const MN: usize> SubAssign
    for MatrixSpace<R, M, N, MN>
{
    #[inline]
    fn sub_assign(&mut self, rps: Self) {
        self.elements -= rps.elements
    }
}

impl<R: Ring, const M: usize, const N: usize, const MN: usize> SubAssign<&Self>
    for MatrixSpace<R, M, N, MN>
{
    #[inline]
    fn sub_assign(&mut self, rps: &Self) {
        self.elements -= &rps.elements
    }
}

impl<R: Semiring, const N: usize, const NN: usize> Mul for MatrixSpace<R, N, N, NN>
where
    for<'a> &'a R: SemiringOps<R>,
{
    type Output = Self;

    #[inline]
    fn mul(self, rps: Self) -> Self::Output {
        &self * &rps
    }
}

impl<R: Semiring, const N: usize, const NN: usize> Mul<&Self> for MatrixSpace<R, N, N, NN>
where
    for<'a> &'a R: SemiringOps<R>,
{
    type Output = Self;

    #[inline]
    fn mul(self, rps: &Self) -> Self::Output {
        &self * rps
    }
}

impl<R: Semiring, const N: usize, const NN: usize> Mul<MatrixSpace<R, N, N, NN>>
    for &MatrixSpace<R, N, N, NN>
where
    for<'a> &'a R: SemiringOps<R>,
{
    type Output = MatrixSpace<R, N, N, NN>;

    #[inline]
    fn mul(self, rps: MatrixSpace<R, N, N, NN>) -> Self::Output {
        self * &rps
    }
}

impl<'a, R: Semiring, const N: usize, const NN: usize> Mul<&'a MatrixSpace<R, N, N, NN>>
    for &MatrixSpace<R, N, N, NN>
where
    for<'b> &'b R: SemiringOps<R>,
{
    type Output = MatrixSpace<R, N, N, NN>;

    fn mul(self, rps: &'a MatrixSpace<R, N, N, NN>) -> Self::Output {
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
        Self::Output::new(Array::new(elements))
    }
}

impl<R: Semiring, const N: usize, const NN: usize> MulAssign for MatrixSpace<R, N, N, NN>
where
    for<'a> &'a R: SemiringOps<R>,
{
    #[inline]
    fn mul_assign(&mut self, rps: Self) {
        *self = &*self * &rps
    }
}

impl<R: Semiring, const N: usize, const NN: usize> MulAssign<&Self> for MatrixSpace<R, N, N, NN>
where
    for<'a> &'a R: SemiringOps<R>,
{
    #[inline]
    fn mul_assign(&mut self, rps: &Self) {
        *self = &*self * rps
    }
}

impl<R: Semiring, const N: usize, const NN: usize> Square for MatrixSpace<R, N, N, NN>
where
    for<'a> &'a R: SemiringOps<R>,
{
    type Output = Self;

    #[inline]
    fn square(self) -> Self {
        &self * &self
    }
}

impl<R: Semiring, const N: usize, const NN: usize> Square for &MatrixSpace<R, N, N, NN>
where
    for<'a> &'a R: SemiringOps<R>,
{
    type Output = MatrixSpace<R, N, N, NN>;

    #[inline]
    fn square(self) -> Self::Output {
        self * self
    }
}

impl<R: Semiring, const M: usize, const N: usize, const MN: usize> Mul<R>
    for MatrixSpace<R, M, N, MN>
{
    type Output = Self;

    #[inline]
    fn mul(self, rps: R) -> Self::Output {
        self * &rps
    }
}

impl<R: Semiring, const M: usize, const N: usize, const MN: usize> Mul<&R>
    for MatrixSpace<R, M, N, MN>
{
    type Output = Self;

    #[inline]
    fn mul(self, rps: &R) -> Self::Output {
        Self::new(self.elements * rps)
    }
}

impl<R: Semiring, const M: usize, const N: usize, const MN: usize> Mul<R>
    for &MatrixSpace<R, M, N, MN>
where
    for<'a> &'a R: SemiringOps<R>,
{
    type Output = MatrixSpace<R, M, N, MN>;

    #[inline]
    fn mul(self, rps: R) -> Self::Output {
        self * &rps
    }
}

impl<R: Semiring, const M: usize, const N: usize, const MN: usize> Mul<&R>
    for &MatrixSpace<R, M, N, MN>
where
    for<'a> &'a R: SemiringOps<R>,
{
    type Output = MatrixSpace<R, M, N, MN>;

    #[inline]
    fn mul(self, rps: &R) -> Self::Output {
        Self::Output::new(&self.elements * rps)
    }
}

impl<R: Semiring, const M: usize, const N: usize, const MN: usize> MulAssign<R>
    for MatrixSpace<R, M, N, MN>
{
    #[inline]
    fn mul_assign(&mut self, rps: R) {
        *self *= &rps
    }
}

impl<R: Semiring, const M: usize, const N: usize, const MN: usize> MulAssign<&R>
    for MatrixSpace<R, M, N, MN>
{
    #[inline]
    fn mul_assign(&mut self, rps: &R) {
        self.elements *= rps
    }
}

impl<R: Semiring, const M: usize, const N: usize, const MN: usize> Mul<VectorRing<R, N>>
    for MatrixSpace<R, M, N, MN>
where
    for<'a> &'a R: SemiringOps<R>,
{
    type Output = VectorRing<R, M>;

    #[inline]
    fn mul(self, rps: VectorRing<R, N>) -> Self::Output {
        &self * &rps
    }
}

impl<R: Semiring, const M: usize, const N: usize, const MN: usize> Mul<&VectorRing<R, N>>
    for MatrixSpace<R, M, N, MN>
where
    for<'a> &'a R: SemiringOps<R>,
{
    type Output = VectorRing<R, M>;

    #[inline]
    fn mul(self, rps: &VectorRing<R, N>) -> Self::Output {
        &self * rps
    }
}

impl<R: Semiring, const M: usize, const N: usize, const MN: usize> Mul<VectorRing<R, N>>
    for &MatrixSpace<R, M, N, MN>
where
    for<'a> &'a R: SemiringOps<R>,
{
    type Output = VectorRing<R, M>;

    #[inline]
    fn mul(self, rps: VectorRing<R, N>) -> Self::Output {
        self * &rps
    }
}

impl<R: Semiring, const M: usize, const N: usize, const MN: usize> Mul<&VectorRing<R, N>>
    for &MatrixSpace<R, M, N, MN>
where
    for<'a> &'a R: SemiringOps<R>,
{
    type Output = VectorRing<R, M>;

    fn mul(self, rps: &VectorRing<R, N>) -> Self::Output {
        VectorRing::<R, M>::new(Array::from_fn(|i| {
            (0..N).map(|j| &self[(i, j)] * &rps[j]).sum()
        }))
    }
}

impl<R: Semiring, const M: usize, const N: usize, const MN: usize> Mul<MatrixSpace<R, M, N, MN>>
    for VectorRing<R, M>
where
    for<'a> &'a R: SemiringOps<R>,
{
    type Output = VectorRing<R, N>;

    #[inline]
    fn mul(self, rps: MatrixSpace<R, M, N, MN>) -> Self::Output {
        &self * &rps
    }
}

impl<R: Semiring, const M: usize, const N: usize, const MN: usize> Mul<&MatrixSpace<R, M, N, MN>>
    for VectorRing<R, M>
where
    for<'a> &'a R: SemiringOps<R>,
{
    type Output = VectorRing<R, N>;

    #[inline]
    fn mul(self, rps: &MatrixSpace<R, M, N, MN>) -> Self::Output {
        &self * rps
    }
}

impl<R: Semiring, const M: usize, const N: usize, const MN: usize> Mul<MatrixSpace<R, M, N, MN>>
    for &VectorRing<R, M>
where
    for<'a> &'a R: SemiringOps<R>,
{
    type Output = VectorRing<R, N>;

    #[inline]
    fn mul(self, rps: MatrixSpace<R, M, N, MN>) -> Self::Output {
        self * &rps
    }
}

impl<R: Semiring, const M: usize, const N: usize, const MN: usize> Mul<&MatrixSpace<R, M, N, MN>>
    for &VectorRing<R, M>
where
    for<'a> &'a R: SemiringOps<R>,
{
    type Output = VectorRing<R, N>;

    fn mul(self, rps: &MatrixSpace<R, M, N, MN>) -> Self::Output {
        VectorRing::<R, N>::new(Array::from_fn(|j| {
            (0..M).map(|i| &self[i] * &rps[(i, j)]).sum()
        }))
    }
}

impl<R: Ring, const N: usize, const NN: usize> Commutator for MatrixSpace<R, N, N, NN>
where
    for<'a> &'a R: RingOps<R>,
{
    type Output = Self;

    fn commutator(self, rps: Self) -> Self::Output {
        &self * &rps - rps * self
    }
}

impl<R: Ring, const N: usize, const NN: usize> Commutator<&Self> for MatrixSpace<R, N, N, NN>
where
    for<'a> &'a R: RingOps<R>,
{
    type Output = Self;

    fn commutator(self, rps: &Self) -> Self::Output {
        &self * rps - rps * self
    }
}

impl<R: Ring, const N: usize, const NN: usize> Commutator<MatrixSpace<R, N, N, NN>>
    for &MatrixSpace<R, N, N, NN>
where
    for<'a> &'a R: RingOps<R>,
{
    type Output = MatrixSpace<R, N, N, NN>;

    fn commutator(self, rps: MatrixSpace<R, N, N, NN>) -> Self::Output {
        self * &rps - rps * self
    }
}

impl<'a, R: Ring, const N: usize, const NN: usize> Commutator<&'a MatrixSpace<R, N, N, NN>>
    for &MatrixSpace<R, N, N, NN>
where
    for<'b> &'b R: RingOps<R>,
{
    type Output = MatrixSpace<R, N, N, NN>;

    fn commutator(self, rps: &'a MatrixSpace<R, N, N, NN>) -> Self::Output {
        self * rps - rps * self
    }
}

impl<R: Semiring, const M: usize, const N: usize, const MN: usize> Sum
    for MatrixSpace<R, M, N, MN>
{
    #[inline]
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        Self::new(iter.map(|i| i.elements).sum())
    }
}

impl<'a, R: Semiring + Clone, const M: usize, const N: usize, const MN: usize> Sum<&'a Self>
    for MatrixSpace<R, M, N, MN>
{
    #[inline]
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        Self::new(iter.map(|i| &i.elements).sum())
    }
}

impl<R: UnitalSemiring, const N: usize, const NN: usize> Product for MatrixSpace<R, N, N, NN>
where
    for<'a> &'a R: SemiringOps<R>,
{
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|lps, rps| lps * rps).unwrap_or(Self::LEFT_ONE)
    }
}

impl<'a, R: UnitalSemiring + Clone, const N: usize, const NN: usize> Product<&'a Self>
    for MatrixSpace<R, N, N, NN>
where
    for<'b> &'b R: SemiringOps<R>,
{
    fn product<I: Iterator<Item = &'a Self>>(mut iter: I) -> Self {
        let first = match iter.next() {
            Some(i) => i.clone(),
            None => return Self::LEFT_ONE,
        };
        iter.fold(first, |lps, rps| lps * rps)
    }
}

impl<R: Semiring, const M: usize, const N: usize, const MN: usize> LeftZero
    for MatrixSpace<R, M, N, MN>
{
    const LEFT_ZERO: Self = Self::ZERO;
}

impl<R: Semiring, const M: usize, const N: usize, const MN: usize> RightZero
    for MatrixSpace<R, M, N, MN>
{
    const RIGHT_ZERO: Self = Self::ZERO;
}

impl<R: Semiring, const M: usize, const N: usize, const MN: usize> Zero
    for MatrixSpace<R, M, N, MN>
{
    const ZERO: Self = Self::new(Array::<R, MN>::ZERO);
}

impl<R: UnitalSemiring, const N: usize, const NN: usize> LeftOne for MatrixSpace<R, N, N, NN> {
    const LEFT_ONE: Self = Self::ONE;
}

impl<R: UnitalSemiring, const N: usize, const NN: usize> RightOne for MatrixSpace<R, N, N, NN> {
    const RIGHT_ONE: Self = Self::ONE;
}

impl<R: UnitalSemiring, const N: usize, const NN: usize> One for MatrixSpace<R, N, N, NN> {
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
        let elements = Array::new(elements);
        Self { elements }
    };
}

impl<R: Semiring, const M: usize, const N: usize, const MN: usize> Set
    for MatrixSpace<R, M, N, MN>
{
}

impl<R: Semiring, const M: usize, const N: usize, const MN: usize> AdditiveCommutativeMagma
    for MatrixSpace<R, M, N, MN>
{
}

impl<R: Semiring, const M: usize, const N: usize, const MN: usize> AdditiveSemigroup
    for MatrixSpace<R, M, N, MN>
{
}

impl<R: Semiring, const N: usize, const NN: usize> MultiplicativeSemigroup
    for MatrixSpace<R, N, N, NN>
where
    for<'a> &'a R: SemiringOps<R>,
{
}

impl<R: Semiring + Clone, const M: usize, const N: usize, const MN: usize> Semimodule<R>
    for MatrixSpace<R, M, N, MN>
{
}

impl<R: Ring + Copy, const N: usize, const NN: usize> Algebra<R> for MatrixSpace<R, N, N, NN> where
    for<'a> &'a R: RingOps<R>
{
}

impl<Msg, R: Semiring + Absorb<Msg>, const M: usize, const N: usize, const MN: usize> Absorb<Msg>
    for MatrixSpace<R, M, N, MN>
{
    fn absorb_into<D: Duplexer<Msg = Msg>>(self, duplex: &mut D) {
        duplex.absorb(self.elements)
    }
}

impl<Msg, R: Semiring + Squeeze<Msg>, const M: usize, const N: usize, const MN: usize> Squeeze<Msg>
    for MatrixSpace<R, M, N, MN>
{
    fn squeeze_from<D: Duplexer<Msg = Msg>>(duplex: &mut D) -> Self {
        Self::new(Array::squeeze_from(duplex))
    }
}
