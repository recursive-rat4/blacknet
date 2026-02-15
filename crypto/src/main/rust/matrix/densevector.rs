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

use crate::algebra::{Conjugate, Double, One, Set, Square, Tensor, Zero};
use crate::duplex::{Absorb, Duplex};
use crate::matrix::DenseMatrix;
use alloc::borrow::{Borrow, BorrowMut};
use alloc::vec;
use alloc::vec::Vec;
use core::fmt::{Debug, Formatter, Result};
use core::iter::{Sum, chain, repeat_n, zip};
use core::ops::{
    Add, AddAssign, Deref, DerefMut, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};
use serde::{Deserialize, Serialize};

/// A row (column) vector is a `1 ⨉ n` (`m ⨉ 1`) matrix.
///
/// Multiplication is defined as the Hadamard product.
///
/// # Panics
///
/// In debug builds, panic on incompatible dimensions.
///
/// # Safety
///
/// In release builds, undefined behaviour on incompatible dimensions.
#[derive(Clone, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct DenseVector<T> {
    elements: Vec<T>,
}

impl<T> DenseVector<T> {
    /// Construct a new vector.
    pub const fn new(elements: Vec<T>) -> Self {
        Self { elements }
    }

    /// Fill a new `n`-dimensional vector with a single `element`.
    pub fn fill(n: usize, element: T) -> Self
    where
        T: Clone,
    {
        Self {
            elements: vec![element; n],
        }
    }

    pub fn pad_to_power_of_two(&self) -> Self
    where
        T: Zero + Clone,
    {
        let n = self.elements.len().next_power_of_two() - self.elements.len();
        Self {
            elements: self
                .elements
                .iter()
                .cloned()
                .chain(repeat_n(T::ZERO, n))
                .collect(),
        }
    }

    /// The number of dimensions.
    pub const fn dimension(&self) -> usize {
        self.elements.len()
    }

    /// The entries.
    pub const fn elements(&self) -> &Vec<T> {
        &self.elements
    }

    /// Concatenate horizontally.
    pub fn cat(&self, rps: &Self) -> Self
    where
        T: Clone,
    {
        chain(self, rps).cloned().collect()
    }

    /// Compute the dot product.
    pub fn dot(&self, rps: &Self) -> T
    where
        T: Sum,
        for<'a> &'a T: Mul<Output = T>,
    {
        debug_assert_eq!(self.elements.len(), rps.elements.len());
        zip(self, rps).map(|(l, r)| l * r).sum()
    }

    /// The `n`-dimensional multiplicative identity.
    pub fn identity(n: usize) -> Self
    where
        T: One + Clone,
    {
        Self {
            elements: vec![T::ONE; n],
        }
    }
}

impl<T, const N: usize> From<[T; N]> for DenseVector<T> {
    fn from(elements: [T; N]) -> Self {
        Self {
            elements: elements.into(),
        }
    }
}

impl<T> From<Vec<T>> for DenseVector<T> {
    #[inline]
    fn from(elements: Vec<T>) -> Self {
        Self { elements }
    }
}

impl<T> From<DenseVector<T>> for Vec<T> {
    #[inline]
    fn from(vector: DenseVector<T>) -> Self {
        vector.elements
    }
}

impl<T: Debug> Debug for DenseVector<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self.elements)
    }
}

impl<T> AsRef<[T]> for DenseVector<T> {
    #[inline]
    fn as_ref(&self) -> &[T] {
        &self.elements
    }
}

impl<T> AsMut<[T]> for DenseVector<T> {
    #[inline]
    fn as_mut(&mut self) -> &mut [T] {
        self
    }
}

impl<T> Borrow<[T]> for DenseVector<T> {
    #[inline]
    fn borrow(&self) -> &[T] {
        &self.elements
    }
}

impl<T> BorrowMut<[T]> for DenseVector<T> {
    #[inline]
    fn borrow_mut(&mut self) -> &mut [T] {
        &mut self.elements
    }
}

impl<T> Deref for DenseVector<T> {
    type Target = [T];

    #[inline]
    fn deref(&self) -> &[T] {
        &self.elements
    }
}

impl<T> DerefMut for DenseVector<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.elements
    }
}

impl<T> Index<usize> for DenseVector<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.elements[index]
    }
}

impl<T> IndexMut<usize> for DenseVector<T> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.elements[index]
    }
}

impl<T> FromIterator<T> for DenseVector<T> {
    #[inline]
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        Self {
            elements: iter.into_iter().collect(),
        }
    }
}

impl<T> IntoIterator for DenseVector<T> {
    type Item = T;
    type IntoIter = alloc::vec::IntoIter<T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.elements.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a DenseVector<T> {
    type Item = &'a T;
    type IntoIter = core::slice::Iter<'a, T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.elements.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut DenseVector<T> {
    type Item = &'a mut T;
    type IntoIter = core::slice::IterMut<'a, T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.elements.iter_mut()
    }
}

impl<T: Add<Output = T>> Add for DenseVector<T> {
    type Output = Self;

    fn add(self, rps: Self) -> Self::Output {
        debug_assert_eq!(self.elements.len(), rps.elements.len());
        zip(self, rps).map(|(l, r)| l + r).collect()
    }
}

impl<T: AddAssign> AddAssign for DenseVector<T> {
    fn add_assign(&mut self, rps: Self) {
        debug_assert_eq!(self.elements.len(), rps.elements.len());
        zip(self, rps).for_each(|(l, r)| *l += r);
    }
}

impl<T: Double<Output = T>> Double for DenseVector<T> {
    type Output = Self;

    fn double(self) -> Self::Output {
        self.into_iter().map(Double::double).collect()
    }
}

impl<T> Double for &DenseVector<T>
where
    for<'a> &'a T: Double<Output = T>,
{
    type Output = DenseVector<T>;

    fn double(self) -> Self::Output {
        self.into_iter().map(Double::double).collect()
    }
}

impl<T: for<'a> Add<&'a T, Output = T>> Add<&DenseVector<T>> for DenseVector<T> {
    type Output = Self;

    fn add(self, rps: &DenseVector<T>) -> Self::Output {
        debug_assert_eq!(self.elements.len(), rps.elements.len());
        zip(self, rps).map(|(l, r)| l + r).collect()
    }
}

impl<T: for<'a> AddAssign<&'a T>> AddAssign<&DenseVector<T>> for DenseVector<T> {
    fn add_assign(&mut self, rps: &DenseVector<T>) {
        debug_assert_eq!(self.elements.len(), rps.elements.len());
        zip(self, rps).for_each(|(l, r)| *l += r);
    }
}

impl<T> Add<DenseVector<T>> for &DenseVector<T>
where
    for<'a> &'a T: Add<T, Output = T>,
{
    type Output = DenseVector<T>;

    fn add(self, rps: DenseVector<T>) -> Self::Output {
        debug_assert_eq!(self.elements.len(), rps.elements.len());
        zip(self, rps).map(|(l, r)| l + r).collect()
    }
}

impl<T> Add for &DenseVector<T>
where
    for<'a> &'a T: Add<Output = T>,
{
    type Output = DenseVector<T>;

    fn add(self, rps: Self) -> Self::Output {
        debug_assert_eq!(self.elements.len(), rps.elements.len());
        zip(self, rps).map(|(l, r)| l + r).collect()
    }
}

impl<T: Neg<Output = T>> Neg for DenseVector<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self.into_iter().map(Neg::neg).collect()
    }
}

impl<T> Neg for &DenseVector<T>
where
    for<'a> &'a T: Neg<Output = T>,
{
    type Output = DenseVector<T>;

    fn neg(self) -> Self::Output {
        self.into_iter().map(Neg::neg).collect()
    }
}

impl<T: Sub<Output = T>> Sub for DenseVector<T> {
    type Output = Self;

    fn sub(self, rps: Self) -> Self::Output {
        debug_assert_eq!(self.elements.len(), rps.elements.len());
        zip(self, rps).map(|(l, r)| l - r).collect()
    }
}

impl<T: SubAssign> SubAssign for DenseVector<T> {
    fn sub_assign(&mut self, rps: Self) {
        debug_assert_eq!(self.elements.len(), rps.elements.len());
        zip(self, rps).for_each(|(l, r)| *l -= r);
    }
}

impl<T: for<'a> Sub<&'a T, Output = T>> Sub<&DenseVector<T>> for DenseVector<T> {
    type Output = Self;

    fn sub(self, rps: &DenseVector<T>) -> Self::Output {
        debug_assert_eq!(self.elements.len(), rps.elements.len());
        zip(self, rps).map(|(l, r)| l - r).collect()
    }
}

impl<T: for<'a> SubAssign<&'a T>> SubAssign<&DenseVector<T>> for DenseVector<T> {
    fn sub_assign(&mut self, rps: &DenseVector<T>) {
        debug_assert_eq!(self.elements.len(), rps.elements.len());
        zip(self, rps).for_each(|(l, r)| *l -= r);
    }
}

impl<T> Sub<DenseVector<T>> for &DenseVector<T>
where
    for<'a> &'a T: Sub<T, Output = T>,
{
    type Output = DenseVector<T>;

    fn sub(self, rps: DenseVector<T>) -> Self::Output {
        debug_assert_eq!(self.elements.len(), rps.elements.len());
        zip(self, rps).map(|(l, r)| l - r).collect()
    }
}

impl<T> Sub for &DenseVector<T>
where
    for<'a> &'a T: Sub<Output = T>,
{
    type Output = DenseVector<T>;

    fn sub(self, rps: Self) -> Self::Output {
        debug_assert_eq!(self.elements.len(), rps.elements.len());
        zip(self, rps).map(|(l, r)| l - r).collect()
    }
}

impl<T: Mul<Output = T>> Mul for DenseVector<T> {
    type Output = Self;

    fn mul(self, rps: Self) -> Self::Output {
        debug_assert_eq!(self.elements.len(), rps.elements.len());
        zip(self, rps).map(|(l, r)| l * r).collect()
    }
}

impl<T: MulAssign> MulAssign for DenseVector<T> {
    fn mul_assign(&mut self, rps: Self) {
        debug_assert_eq!(self.elements.len(), rps.elements.len());
        zip(self, rps).for_each(|(l, r)| *l *= r);
    }
}

impl<T: for<'a> Mul<&'a T, Output = T>> Mul<&DenseVector<T>> for DenseVector<T> {
    type Output = Self;

    fn mul(self, rps: &DenseVector<T>) -> Self::Output {
        debug_assert_eq!(self.elements.len(), rps.elements.len());
        zip(self, rps).map(|(l, r)| l * r).collect()
    }
}

impl<T: for<'a> MulAssign<&'a T>> MulAssign<&DenseVector<T>> for DenseVector<T> {
    fn mul_assign(&mut self, rps: &DenseVector<T>) {
        debug_assert_eq!(self.elements.len(), rps.elements.len());
        zip(self, rps).for_each(|(l, r)| *l *= r);
    }
}

impl<T> Mul<DenseVector<T>> for &DenseVector<T>
where
    for<'a> &'a T: Mul<T, Output = T>,
{
    type Output = DenseVector<T>;

    fn mul(self, rps: DenseVector<T>) -> Self::Output {
        debug_assert_eq!(self.elements.len(), rps.elements.len());
        zip(self, rps).map(|(l, r)| l * r).collect()
    }
}

impl<T> Mul for &DenseVector<T>
where
    for<'a> &'a T: Mul<Output = T>,
{
    type Output = DenseVector<T>;

    fn mul(self, rps: Self) -> Self::Output {
        debug_assert_eq!(self.elements.len(), rps.elements.len());
        zip(self, rps).map(|(l, r)| l * r).collect()
    }
}

impl<T: for<'a> Mul<&'a T, Output = T>> Mul<T> for DenseVector<T> {
    type Output = Self;

    #[inline]
    fn mul(self, rps: T) -> Self::Output {
        self * &rps
    }
}

impl<T: for<'a> Mul<&'a T, Output = T>> Mul<&T> for DenseVector<T> {
    type Output = Self;

    fn mul(self, rps: &T) -> Self::Output {
        self.into_iter().map(|l| l * rps).collect()
    }
}

impl<T: for<'a> MulAssign<&'a T>> MulAssign<T> for DenseVector<T> {
    #[inline]
    fn mul_assign(&mut self, rps: T) {
        *self *= &rps
    }
}

impl<T: for<'a> MulAssign<&'a T>> MulAssign<&T> for DenseVector<T> {
    fn mul_assign(&mut self, rps: &T) {
        self.into_iter().for_each(|l| *l *= rps);
    }
}

impl<T> Mul<T> for &DenseVector<T>
where
    for<'a> &'a T: Mul<Output = T>,
{
    type Output = DenseVector<T>;

    #[inline]
    fn mul(self, rps: T) -> Self::Output {
        self * &rps
    }
}

impl<T> Mul<&T> for &DenseVector<T>
where
    for<'a> &'a T: Mul<Output = T>,
{
    type Output = DenseVector<T>;

    fn mul(self, rps: &T) -> Self::Output {
        self.into_iter().map(|l| l * rps).collect()
    }
}

impl<T: Square<Output = T>> Square for DenseVector<T> {
    type Output = Self;

    fn square(self) -> Self::Output {
        self.into_iter().map(Square::square).collect()
    }
}

impl<T> Square for &DenseVector<T>
where
    for<'a> &'a T: Square<Output = T>,
{
    type Output = DenseVector<T>;

    fn square(self) -> Self::Output {
        self.into_iter().map(Square::square).collect()
    }
}

impl<T: Conjugate<Output = T>> Conjugate for DenseVector<T> {
    type Output = Self;

    fn conjugate(self) -> Self::Output {
        self.into_iter().map(Conjugate::conjugate).collect()
    }
}

impl<T> Conjugate for &DenseVector<T>
where
    for<'a> &'a T: Conjugate<Output = T>,
{
    type Output = DenseVector<T>;

    fn conjugate(self) -> Self::Output {
        self.into_iter().map(Conjugate::conjugate).collect()
    }
}

impl<T> Tensor for DenseVector<T>
where
    for<'a> &'a T: Mul<Output = T>,
{
    type Output = DenseMatrix<T>;

    #[inline]
    fn tensor(self, rps: Self) -> Self::Output {
        (&self).tensor(&rps)
    }
}

impl<T> Tensor<&Self> for DenseVector<T>
where
    for<'a> &'a T: Mul<Output = T>,
{
    type Output = DenseMatrix<T>;

    #[inline]
    fn tensor(self, rps: &Self) -> Self::Output {
        (&self).tensor(rps)
    }
}

impl<T> Tensor<DenseVector<T>> for &DenseVector<T>
where
    for<'a> &'a T: Mul<Output = T>,
{
    type Output = DenseMatrix<T>;

    #[inline]
    fn tensor(self, rps: DenseVector<T>) -> Self::Output {
        self.tensor(&rps)
    }
}

impl<T> Tensor for &DenseVector<T>
where
    for<'a> &'a T: Mul<Output = T>,
{
    type Output = DenseMatrix<T>;

    fn tensor(self, rps: Self) -> Self::Output {
        // Module tensor
        let rows = self.elements.len();
        let columns = rps.elements.len();
        let mut elements = Vec::<T>::with_capacity(rows * columns);
        for i in 0..rows {
            for j in 0..columns {
                elements.push(&self.elements[i] * &rps.elements[j])
            }
        }
        DenseMatrix::new(rows, columns, elements)
    }
}

impl<S: Set, T: Absorb<S>> Absorb<S> for DenseVector<T> {
    fn absorb_into(self, duplex: &mut (impl Duplex<S> + ?Sized)) {
        duplex.absorb_iter(self.elements.into_iter())
    }
}

impl<S: Set, T: Absorb<S> + Clone> Absorb<S> for &DenseVector<T> {
    fn absorb_into(self, duplex: &mut (impl Duplex<S> + ?Sized)) {
        duplex.absorb_iter(self.elements.iter().cloned())
    }
}
