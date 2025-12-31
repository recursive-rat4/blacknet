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

use crate::algebra::Ring;
use crate::algebra::{Double, Square};
use crate::algebra::{Presemiring, Semiring};
use crate::matrix::DenseMatrix;
use alloc::borrow::{Borrow, BorrowMut};
use alloc::vec;
use alloc::vec::Vec;
use core::fmt::{Debug, Formatter, Result};
use core::iter::{chain, repeat_n, zip};
use core::ops::{
    Add, AddAssign, Deref, DerefMut, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};
use serde::{Deserialize, Serialize};

/// A row (column) vector is a `1 ⨉ n` (`m ⨉ 1`) matrix.
///
/// # Panics
///
/// In debug builds, panic on incompatible dimensions.
///
/// # Safety
///
/// In release builds, undefined behaviour on incompatible dimensions.
#[derive(Clone, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct DenseVector<R: Presemiring> {
    elements: Vec<R>,
}

impl<R: Presemiring> DenseVector<R> {
    /// Construct a new vector.
    pub const fn new(elements: Vec<R>) -> Self {
        Self { elements }
    }

    /// Fill a new `n`-dimensional vector with a single `element`.
    pub fn fill(n: usize, element: R) -> Self {
        Self {
            elements: vec![element; n],
        }
    }

    pub fn pad_to_power_of_two(&self) -> Self {
        let n = self.elements.len().next_power_of_two() - self.elements.len();
        Self {
            elements: self
                .elements
                .iter()
                .copied()
                .chain(repeat_n(R::ZERO, n))
                .collect(),
        }
    }

    /// The number of dimensions.
    pub const fn dimension(&self) -> usize {
        self.elements.len()
    }

    /// The entries.
    pub const fn elements(&self) -> &Vec<R> {
        &self.elements
    }

    /// Concatenate horizontally.
    pub fn cat(&self, rps: &Self) -> Self {
        chain(self, rps).copied().collect()
    }

    /// Compute the dot product.
    pub fn dot(&self, rps: &Self) -> R {
        debug_assert_eq!(self.elements.len(), rps.elements.len());
        zip(self, rps).map(|(&l, &r)| l * r).sum()
    }

    /// Compute the tensor product.
    pub fn tensor(&self, rps: &Self) -> DenseMatrix<R> {
        let rows = self.elements.len();
        let columns = rps.elements.len();
        let mut elements = Vec::<R>::with_capacity(rows * columns);
        for i in 0..rows {
            for j in 0..columns {
                elements.push(self.elements[i] * rps.elements[j])
            }
        }
        DenseMatrix::new(rows, columns, elements)
    }
}

impl<R: Semiring> DenseVector<R> {
    /// The `n`-dimensional multiplicative identity.
    pub fn identity(n: usize) -> Self {
        Self {
            elements: vec![R::ONE; n],
        }
    }
}

impl<R: Presemiring, const N: usize> From<[R; N]> for DenseVector<R> {
    fn from(elements: [R; N]) -> Self {
        Self {
            elements: elements.into(),
        }
    }
}

impl<R: Presemiring> From<Vec<R>> for DenseVector<R> {
    #[inline]
    fn from(elements: Vec<R>) -> Self {
        Self { elements }
    }
}

impl<R: Presemiring> From<DenseVector<R>> for Vec<R> {
    #[inline]
    fn from(vector: DenseVector<R>) -> Self {
        vector.elements
    }
}

impl<R: Presemiring + Debug> Debug for DenseVector<R> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self.elements)
    }
}

impl<R: Presemiring> AsRef<[R]> for DenseVector<R> {
    #[inline]
    fn as_ref(&self) -> &[R] {
        &self.elements
    }
}

impl<R: Presemiring> AsMut<[R]> for DenseVector<R> {
    #[inline]
    fn as_mut(&mut self) -> &mut [R] {
        self
    }
}

impl<R: Presemiring> Borrow<[R]> for DenseVector<R> {
    #[inline]
    fn borrow(&self) -> &[R] {
        &self.elements
    }
}

impl<R: Presemiring> BorrowMut<[R]> for DenseVector<R> {
    #[inline]
    fn borrow_mut(&mut self) -> &mut [R] {
        &mut self.elements
    }
}

impl<R: Presemiring> Deref for DenseVector<R> {
    type Target = [R];

    #[inline]
    fn deref(&self) -> &[R] {
        &self.elements
    }
}

impl<R: Presemiring> DerefMut for DenseVector<R> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.elements
    }
}

impl<R: Presemiring> Index<usize> for DenseVector<R> {
    type Output = R;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.elements[index]
    }
}

impl<R: Presemiring> IndexMut<usize> for DenseVector<R> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.elements[index]
    }
}

impl<R: Presemiring> FromIterator<R> for DenseVector<R> {
    #[inline]
    fn from_iter<I: IntoIterator<Item = R>>(iter: I) -> Self {
        Self {
            elements: iter.into_iter().collect(),
        }
    }
}

impl<R: Presemiring> IntoIterator for DenseVector<R> {
    type Item = R;
    type IntoIter = alloc::vec::IntoIter<R>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.elements.into_iter()
    }
}

impl<'a, R: Presemiring> IntoIterator for &'a DenseVector<R> {
    type Item = &'a R;
    type IntoIter = core::slice::Iter<'a, R>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.elements.iter()
    }
}

impl<'a, R: Presemiring> IntoIterator for &'a mut DenseVector<R> {
    type Item = &'a mut R;
    type IntoIter = core::slice::IterMut<'a, R>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.elements.iter_mut()
    }
}

impl<R: Presemiring> Add for DenseVector<R> {
    type Output = Self;

    fn add(self, rps: Self) -> Self::Output {
        debug_assert_eq!(self.elements.len(), rps.elements.len());
        zip(self, rps).map(|(l, r)| l + r).collect()
    }
}

impl<R: Presemiring> AddAssign for DenseVector<R> {
    fn add_assign(&mut self, rps: Self) {
        debug_assert_eq!(self.elements.len(), rps.elements.len());
        zip(self, rps).for_each(|(l, r)| *l += r);
    }
}

impl<R: Presemiring> Double for DenseVector<R> {
    type Output = Self;

    fn double(self) -> Self::Output {
        self.into_iter().map(Double::double).collect()
    }
}

impl<R: Presemiring> Double for &DenseVector<R> {
    type Output = DenseVector<R>;

    fn double(self) -> Self::Output {
        self.into_iter().copied().map(Double::double).collect()
    }
}

impl<R: Presemiring> Add<&DenseVector<R>> for DenseVector<R> {
    type Output = Self;

    fn add(self, rps: &DenseVector<R>) -> Self::Output {
        debug_assert_eq!(self.elements.len(), rps.elements.len());
        zip(self, rps).map(|(l, &r)| l + r).collect()
    }
}

impl<R: Presemiring> AddAssign<&DenseVector<R>> for DenseVector<R> {
    fn add_assign(&mut self, rps: &DenseVector<R>) {
        debug_assert_eq!(self.elements.len(), rps.elements.len());
        zip(self, rps).for_each(|(l, &r)| *l += r);
    }
}

impl<R: Presemiring> Add<DenseVector<R>> for &DenseVector<R> {
    type Output = DenseVector<R>;

    fn add(self, rps: DenseVector<R>) -> Self::Output {
        debug_assert_eq!(self.elements.len(), rps.elements.len());
        zip(self, rps).map(|(&l, r)| l + r).collect()
    }
}

impl<R: Presemiring> Add for &DenseVector<R> {
    type Output = DenseVector<R>;

    fn add(self, rps: Self) -> Self::Output {
        debug_assert_eq!(self.elements.len(), rps.elements.len());
        zip(self, rps).map(|(&l, &r)| l + r).collect()
    }
}

impl<R: Ring> Neg for DenseVector<R> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self.into_iter().map(Neg::neg).collect()
    }
}

impl<R: Ring> Neg for &DenseVector<R> {
    type Output = DenseVector<R>;

    fn neg(self) -> Self::Output {
        self.into_iter().map(|&e| -e).collect()
    }
}

impl<R: Ring> Sub for DenseVector<R> {
    type Output = Self;

    fn sub(self, rps: Self) -> Self::Output {
        debug_assert_eq!(self.elements.len(), rps.elements.len());
        zip(self, rps).map(|(l, r)| l - r).collect()
    }
}

impl<R: Ring> SubAssign for DenseVector<R> {
    fn sub_assign(&mut self, rps: Self) {
        debug_assert_eq!(self.elements.len(), rps.elements.len());
        zip(self, rps).for_each(|(l, r)| *l -= r);
    }
}

impl<R: Ring> Sub<&DenseVector<R>> for DenseVector<R> {
    type Output = Self;

    fn sub(self, rps: &DenseVector<R>) -> Self::Output {
        debug_assert_eq!(self.elements.len(), rps.elements.len());
        zip(self, rps).map(|(l, &r)| l - r).collect()
    }
}

impl<R: Ring> SubAssign<&DenseVector<R>> for DenseVector<R> {
    fn sub_assign(&mut self, rps: &DenseVector<R>) {
        debug_assert_eq!(self.elements.len(), rps.elements.len());
        zip(self, rps).for_each(|(l, &r)| *l -= r);
    }
}

impl<R: Ring> Sub<DenseVector<R>> for &DenseVector<R> {
    type Output = DenseVector<R>;

    fn sub(self, rps: DenseVector<R>) -> Self::Output {
        debug_assert_eq!(self.elements.len(), rps.elements.len());
        zip(self, rps).map(|(&l, r)| l - r).collect()
    }
}

impl<R: Ring> Sub for &DenseVector<R> {
    type Output = DenseVector<R>;

    fn sub(self, rps: Self) -> Self::Output {
        debug_assert_eq!(self.elements.len(), rps.elements.len());
        zip(self, rps).map(|(&l, &r)| l - r).collect()
    }
}

impl<R: Presemiring> Mul for DenseVector<R> {
    type Output = Self;

    fn mul(self, rps: Self) -> Self::Output {
        debug_assert_eq!(self.elements.len(), rps.elements.len());
        zip(self, rps).map(|(l, r)| l * r).collect()
    }
}

impl<R: Presemiring> MulAssign for DenseVector<R> {
    fn mul_assign(&mut self, rps: Self) {
        debug_assert_eq!(self.elements.len(), rps.elements.len());
        zip(self, rps).for_each(|(l, r)| *l *= r);
    }
}

impl<R: Presemiring> Mul<&DenseVector<R>> for DenseVector<R> {
    type Output = Self;

    fn mul(self, rps: &DenseVector<R>) -> Self::Output {
        debug_assert_eq!(self.elements.len(), rps.elements.len());
        zip(self, rps).map(|(l, &r)| l * r).collect()
    }
}

impl<R: Presemiring> MulAssign<&DenseVector<R>> for DenseVector<R> {
    fn mul_assign(&mut self, rps: &DenseVector<R>) {
        debug_assert_eq!(self.elements.len(), rps.elements.len());
        zip(self, rps).for_each(|(l, &r)| *l *= r);
    }
}

impl<R: Presemiring> Mul<DenseVector<R>> for &DenseVector<R> {
    type Output = DenseVector<R>;

    fn mul(self, rps: DenseVector<R>) -> Self::Output {
        debug_assert_eq!(self.elements.len(), rps.elements.len());
        zip(self, rps).map(|(&l, r)| l * r).collect()
    }
}

impl<R: Presemiring> Mul for &DenseVector<R> {
    type Output = DenseVector<R>;

    fn mul(self, rps: Self) -> Self::Output {
        debug_assert_eq!(self.elements.len(), rps.elements.len());
        zip(self, rps).map(|(&l, &r)| l * r).collect()
    }
}

impl<R: Presemiring> Mul<R> for DenseVector<R> {
    type Output = Self;

    fn mul(self, rps: R) -> Self::Output {
        self.into_iter().map(|l| l * rps).collect()
    }
}

impl<R: Presemiring> MulAssign<R> for DenseVector<R> {
    fn mul_assign(&mut self, rps: R) {
        self.into_iter().for_each(|l| *l *= rps);
    }
}

impl<R: Presemiring> Mul<R> for &DenseVector<R> {
    type Output = DenseVector<R>;

    fn mul(self, rps: R) -> Self::Output {
        self.into_iter().map(|&l| l * rps).collect()
    }
}

impl<R: Presemiring> Square for DenseVector<R> {
    type Output = Self;

    fn square(self) -> Self::Output {
        self.into_iter().map(Square::square).collect()
    }
}

impl<R: Presemiring> Square for &DenseVector<R> {
    type Output = DenseVector<R>;

    fn square(self) -> Self::Output {
        self.into_iter().copied().map(Square::square).collect()
    }
}
