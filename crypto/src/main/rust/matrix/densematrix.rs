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

use crate::algebra::{Double, One, Square, Tensor, Zero};
use crate::matrix::DenseVector;
use alloc::vec;
use alloc::vec::Vec;
use core::iter::{Sum, zip};
use core::ops::{Add, AddAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign};
use serde::{Deserialize, Serialize};

/// A matrix in the row-major order.
///
/// # Panics
///
/// In debug builds, panic on incompatible dimensions.
///
/// # Safety
///
/// In release builds, undefined behaviour on incompatible dimensions.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct DenseMatrix<T> {
    rows: usize,
    columns: usize,
    elements: Vec<T>,
}

impl<T> DenseMatrix<T> {
    /// Construct a new matrix.
    pub const fn new(rows: usize, columns: usize, elements: Vec<T>) -> Self {
        debug_assert!(rows * columns == elements.len());
        Self {
            rows,
            columns,
            elements,
        }
    }

    /// Fill a new `m ⨉ n` matrix with a single `element`.
    pub fn fill(rows: usize, columns: usize, element: T) -> Self
    where
        T: Clone,
    {
        Self {
            rows,
            columns,
            elements: vec![element; rows * columns],
        }
    }

    pub fn pad_to_power_of_two(&self) -> Self
    where
        T: Zero + Clone,
    {
        let m = self.rows.next_power_of_two() - self.rows;
        let n = self.columns.next_power_of_two() - self.columns;
        let mut elements = Vec::<T>::with_capacity((self.rows + m) * (self.columns + n));
        for i in 0..self.rows {
            for j in 0..self.columns {
                elements.push(self[(i, j)].clone())
            }
            for _j in 0..n {
                elements.push(T::ZERO)
            }
        }
        for _ in 0..m * (self.columns + n) {
            elements.push(T::ZERO)
        }
        Self {
            rows: self.rows + m,
            columns: self.columns + n,
            elements,
        }
    }

    /// The number of rows.
    pub const fn rows(&self) -> usize {
        self.rows
    }

    /// The number of columns.
    pub const fn columns(&self) -> usize {
        self.columns
    }

    /// The entries in row-major order.
    pub const fn elements(&self) -> &Vec<T> {
        &self.elements
    }

    /// Iterate rows.
    pub fn iter_row(&self) -> impl ExactSizeIterator<Item = &[T]> {
        self.elements.chunks_exact(self.columns)
    }

    /// Concatenate horizontally.
    pub fn concat(&self, rps: &Self) -> Self
    where
        T: Clone,
    {
        debug_assert!(self.rows == rps.rows);
        let mut elements = Vec::<T>::with_capacity(self.rows * (self.columns + rps.columns));
        for i in 0..self.rows {
            elements.extend_from_slice(&self.elements[i * self.columns..(i + 1) * self.columns]);
            elements.extend_from_slice(&rps.elements[i * rps.columns..(i + 1) * rps.columns]);
        }
        Self {
            rows: self.rows,
            columns: self.columns + rps.columns,
            elements,
        }
    }

    /// Convert a `m ⨉ n` matrix into a `1 ⨉ mn` row vector.
    #[inline]
    pub fn vectorize(self) -> DenseVector<T> {
        self.elements.into()
    }

    pub fn trace(&self) -> T
    where
        T: for<'a> Sum<&'a T>,
    {
        debug_assert!(self.rows == self.columns);
        (0..self.rows).map(|i| &self[(i, i)]).sum()
    }

    pub fn transpose(&self) -> Self
    where
        T: Clone,
    {
        let mut elements = Vec::<T>::with_capacity(self.elements.len());
        for j in 0..self.columns {
            for i in 0..self.rows {
                elements.push(self[(i, j)].clone());
            }
        }
        Self {
            rows: self.columns,
            columns: self.rows,
            elements,
        }
    }

    /// The `n ⨉ n` multiplicative identity.
    pub fn identity(n: usize) -> Self
    where
        T: One + Zero + Clone,
    {
        let mut elements = vec![T::ZERO; n * n];
        for i in 0..n {
            elements[i * n + i] = T::ONE;
        }
        Self {
            rows: n,
            columns: n,
            elements,
        }
    }

    /// The face-splitting product
    pub fn row_tensor(&self, rps: &Self) -> Self
    where
        for<'a> &'a T: Mul<Output = T>,
    {
        debug_assert!(self.rows == rps.rows);
        let rows = self.rows;
        let columns = self.columns * rps.columns;
        let mut elements = Vec::<T>::with_capacity(rows * columns);
        for i in 0..rows {
            for j in 0..self.columns {
                for k in 0..rps.columns {
                    elements.push(&self[(i, j)] * &rps[(i, k)])
                }
            }
        }
        Self {
            rows,
            columns,
            elements,
        }
    }

    /// The Khatri–Rao product
    pub fn column_tensor(&self, rps: &Self) -> Self
    where
        for<'a> &'a T: Mul<Output = T>,
    {
        debug_assert!(self.columns == rps.columns);
        let rows = self.rows * rps.rows;
        let columns = self.columns;
        let mut elements = Vec::<T>::with_capacity(rows * columns);
        for i in 0..self.rows {
            for j in 0..rps.rows {
                for k in 0..columns {
                    elements.push(&self[(i, k)] * &rps[(j, k)])
                }
            }
        }
        Self {
            rows,
            columns,
            elements,
        }
    }
}

impl<T> From<DenseMatrix<T>> for (usize, usize, Vec<T>) {
    fn from(matrix: DenseMatrix<T>) -> Self {
        (matrix.rows, matrix.columns, matrix.elements)
    }
}

impl<T> Index<(usize, usize)> for DenseMatrix<T> {
    type Output = T;

    #[inline]
    fn index(&self, (i, j): (usize, usize)) -> &Self::Output {
        &self.elements[i * self.columns + j]
    }
}

impl<T> IndexMut<(usize, usize)> for DenseMatrix<T> {
    #[inline]
    fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut Self::Output {
        &mut self.elements[i * self.columns + j]
    }
}

impl<T: Add<Output = T>> Add for DenseMatrix<T> {
    type Output = Self;

    fn add(self, rps: Self) -> Self::Output {
        debug_assert!(self.rows == rps.rows && self.columns == rps.columns);
        Self {
            rows: self.rows,
            columns: self.columns,
            elements: zip(self.elements, rps.elements)
                .map(|(l, r)| l + r)
                .collect(),
        }
    }
}

impl<T: AddAssign> AddAssign for DenseMatrix<T> {
    fn add_assign(&mut self, rps: Self) {
        debug_assert!(self.rows == rps.rows && self.columns == rps.columns);
        zip(self.elements.iter_mut(), rps.elements).for_each(|(l, r)| *l += r);
    }
}

impl<T: Double<Output = T>> Double for DenseMatrix<T> {
    type Output = Self;

    fn double(self) -> Self::Output {
        Self {
            rows: self.rows,
            columns: self.columns,
            elements: self.elements.into_iter().map(Double::double).collect(),
        }
    }
}

impl<T: for<'a> Add<&'a T, Output = T>> Add<&DenseMatrix<T>> for DenseMatrix<T> {
    type Output = Self;

    fn add(self, rps: &DenseMatrix<T>) -> Self::Output {
        debug_assert!(self.rows == rps.rows && self.columns == rps.columns);
        Self {
            rows: self.rows,
            columns: self.columns,
            elements: zip(self.elements, rps.elements.iter())
                .map(|(l, r)| l + r)
                .collect(),
        }
    }
}

impl<T: for<'a> AddAssign<&'a T>> AddAssign<&DenseMatrix<T>> for DenseMatrix<T> {
    fn add_assign(&mut self, rps: &DenseMatrix<T>) {
        debug_assert!(self.rows == rps.rows && self.columns == rps.columns);
        zip(self.elements.iter_mut(), rps.elements.iter()).for_each(|(l, r)| *l += r);
    }
}

impl<T> Add<DenseMatrix<T>> for &DenseMatrix<T>
where
    for<'a> &'a T: Add<T, Output = T>,
{
    type Output = DenseMatrix<T>;

    fn add(self, rps: DenseMatrix<T>) -> Self::Output {
        debug_assert!(self.rows == rps.rows && self.columns == rps.columns);
        Self::Output {
            rows: self.rows,
            columns: self.columns,
            elements: zip(self.elements.iter(), rps.elements)
                .map(|(l, r)| l + r)
                .collect(),
        }
    }
}

impl<T> Add for &DenseMatrix<T>
where
    for<'a> &'a T: Add<Output = T>,
{
    type Output = DenseMatrix<T>;

    fn add(self, rps: Self) -> Self::Output {
        debug_assert!(self.rows == rps.rows && self.columns == rps.columns);
        Self::Output {
            rows: self.rows,
            columns: self.columns,
            elements: zip(self.elements.iter(), rps.elements.iter())
                .map(|(l, r)| l + r)
                .collect(),
        }
    }
}

impl<T: Neg<Output = T>> Neg for DenseMatrix<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            rows: self.rows,
            columns: self.columns,
            elements: self.elements.into_iter().map(Neg::neg).collect(),
        }
    }
}

impl<T> Neg for &DenseMatrix<T>
where
    for<'a> &'a T: Neg<Output = T>,
{
    type Output = DenseMatrix<T>;

    fn neg(self) -> Self::Output {
        Self::Output {
            rows: self.rows,
            columns: self.columns,
            elements: self.elements.iter().map(Neg::neg).collect(),
        }
    }
}

impl<T: Sub<Output = T>> Sub for DenseMatrix<T> {
    type Output = Self;

    fn sub(self, rps: Self) -> Self::Output {
        debug_assert!(self.rows == rps.rows && self.columns == rps.columns);
        Self {
            rows: self.rows,
            columns: self.columns,
            elements: zip(self.elements, rps.elements)
                .map(|(l, r)| l - r)
                .collect(),
        }
    }
}

impl<T: SubAssign> SubAssign for DenseMatrix<T> {
    fn sub_assign(&mut self, rps: Self) {
        debug_assert!(self.rows == rps.rows && self.columns == rps.columns);
        zip(self.elements.iter_mut(), rps.elements).for_each(|(l, r)| *l -= r);
    }
}

impl<T: for<'a> Sub<&'a T, Output = T>> Sub<&DenseMatrix<T>> for DenseMatrix<T> {
    type Output = Self;

    fn sub(self, rps: &DenseMatrix<T>) -> Self::Output {
        debug_assert!(self.rows == rps.rows && self.columns == rps.columns);
        Self {
            rows: self.rows,
            columns: self.columns,
            elements: zip(self.elements, rps.elements.iter())
                .map(|(l, r)| l - r)
                .collect(),
        }
    }
}

impl<T: for<'a> SubAssign<&'a T>> SubAssign<&DenseMatrix<T>> for DenseMatrix<T> {
    fn sub_assign(&mut self, rps: &DenseMatrix<T>) {
        debug_assert!(self.rows == rps.rows && self.columns == rps.columns);
        zip(self.elements.iter_mut(), rps.elements.iter()).for_each(|(l, r)| *l -= r);
    }
}

impl<T> Sub<DenseMatrix<T>> for &DenseMatrix<T>
where
    for<'a> &'a T: Sub<T, Output = T>,
{
    type Output = DenseMatrix<T>;

    fn sub(self, rps: DenseMatrix<T>) -> Self::Output {
        debug_assert!(self.rows == rps.rows && self.columns == rps.columns);
        Self::Output {
            rows: self.rows,
            columns: self.columns,
            elements: zip(self.elements.iter(), rps.elements)
                .map(|(l, r)| l - r)
                .collect(),
        }
    }
}

impl<T> Sub for &DenseMatrix<T>
where
    for<'a> &'a T: Sub<Output = T>,
{
    type Output = DenseMatrix<T>;

    fn sub(self, rps: Self) -> Self::Output {
        debug_assert!(self.rows == rps.rows && self.columns == rps.columns);
        Self::Output {
            rows: self.rows,
            columns: self.columns,
            elements: zip(self.elements.iter(), rps.elements.iter())
                .map(|(l, r)| l - r)
                .collect(),
        }
    }
}

impl<T: Zero + AddAssign + Clone> Mul for DenseMatrix<T>
where
    for<'a> &'a T: Mul<Output = T>,
{
    type Output = Self;

    #[inline]
    fn mul(self, rps: Self) -> Self::Output {
        &self * &rps
    }
}

impl<T: Zero + AddAssign + Clone> MulAssign for DenseMatrix<T>
where
    for<'a> &'a T: Mul<Output = T>,
{
    #[inline]
    fn mul_assign(&mut self, rps: Self) {
        *self = &*self * &rps
    }
}

impl<T: Zero + AddAssign + Clone> Mul<&DenseMatrix<T>> for DenseMatrix<T>
where
    for<'a> &'a T: Mul<Output = T>,
{
    type Output = Self;

    #[inline]
    fn mul(self, rps: &DenseMatrix<T>) -> Self::Output {
        &self * rps
    }
}

impl<T: Zero + AddAssign + Clone> MulAssign<&DenseMatrix<T>> for DenseMatrix<T>
where
    for<'a> &'a T: Mul<Output = T>,
{
    #[inline]
    fn mul_assign(&mut self, rps: &DenseMatrix<T>) {
        *self = &*self * rps
    }
}

impl<T: Zero + AddAssign + Clone> Mul<DenseMatrix<T>> for &DenseMatrix<T>
where
    for<'a> &'a T: Mul<Output = T>,
{
    type Output = DenseMatrix<T>;

    #[inline]
    fn mul(self, rps: DenseMatrix<T>) -> Self::Output {
        self * &rps
    }
}

impl<T: Zero + AddAssign + Clone> Mul for &DenseMatrix<T>
where
    for<'a> &'a T: Mul<Output = T>,
{
    type Output = DenseMatrix<T>;

    fn mul(self, rps: &DenseMatrix<T>) -> Self::Output {
        debug_assert!(self.columns == rps.rows);
        // Iterative algorithm
        let mut r = DenseMatrix::fill(self.rows, rps.columns, T::ZERO);
        for i in 0..self.rows {
            for j in 0..rps.columns {
                for k in 0..self.columns {
                    r[(i, j)] += &self[(i, k)] * &rps[(k, j)];
                }
            }
        }
        r
    }
}

impl<T: for<'a> Mul<&'a T, Output = T>> Mul<T> for DenseMatrix<T> {
    type Output = Self;

    #[inline]
    fn mul(self, rps: T) -> Self::Output {
        self * &rps
    }
}

impl<T: for<'a> Mul<&'a T, Output = T>> Mul<&T> for DenseMatrix<T> {
    type Output = Self;

    fn mul(self, rps: &T) -> Self::Output {
        Self {
            rows: self.rows,
            columns: self.columns,
            elements: self.elements.into_iter().map(|e| e * rps).collect(),
        }
    }
}

impl<T: for<'a> MulAssign<&'a T>> MulAssign<T> for DenseMatrix<T> {
    #[inline]
    fn mul_assign(&mut self, rps: T) {
        *self *= &rps
    }
}

impl<T: for<'a> MulAssign<&'a T>> MulAssign<&T> for DenseMatrix<T> {
    fn mul_assign(&mut self, rps: &T) {
        self.elements.iter_mut().for_each(|e| *e *= rps);
    }
}

impl<T> Mul<T> for &DenseMatrix<T>
where
    for<'a> &'a T: Mul<Output = T>,
{
    type Output = DenseMatrix<T>;

    #[inline]
    fn mul(self, rps: T) -> Self::Output {
        self * &rps
    }
}

impl<T> Mul<&T> for &DenseMatrix<T>
where
    for<'a> &'a T: Mul<Output = T>,
{
    type Output = DenseMatrix<T>;

    fn mul(self, rps: &T) -> Self::Output {
        Self::Output {
            rows: self.rows,
            columns: self.columns,
            elements: self.elements.iter().map(|e| e * rps).collect(),
        }
    }
}

impl<T: Sum> Mul<DenseVector<T>> for DenseMatrix<T>
where
    for<'a> &'a T: Mul<Output = T>,
{
    type Output = DenseVector<T>;

    #[inline]
    fn mul(self, rps: DenseVector<T>) -> Self::Output {
        &self * &rps
    }
}

impl<T: Sum> Mul<&DenseVector<T>> for DenseMatrix<T>
where
    for<'a> &'a T: Mul<Output = T>,
{
    type Output = DenseVector<T>;

    #[inline]
    fn mul(self, rps: &DenseVector<T>) -> Self::Output {
        &self * rps
    }
}

impl<T: Sum> Mul<DenseVector<T>> for &DenseMatrix<T>
where
    for<'a> &'a T: Mul<Output = T>,
{
    type Output = DenseVector<T>;

    #[inline]
    fn mul(self, rps: DenseVector<T>) -> Self::Output {
        self * &rps
    }
}

impl<T: Sum> Mul<&DenseVector<T>> for &DenseMatrix<T>
where
    for<'a> &'a T: Mul<Output = T>,
{
    type Output = DenseVector<T>;

    fn mul(self, rps: &DenseVector<T>) -> Self::Output {
        debug_assert!(self.columns == rps.dimension());
        (0..self.rows())
            .map(|i| (0..self.columns()).map(|j| &self[(i, j)] * &rps[j]).sum())
            .collect()
    }
}

impl<T: Sum> Mul<DenseMatrix<T>> for DenseVector<T>
where
    for<'a> &'a T: Mul<Output = T>,
{
    type Output = DenseVector<T>;

    #[inline]
    fn mul(self, rps: DenseMatrix<T>) -> Self::Output {
        &self * &rps
    }
}

impl<T: Sum> Mul<&DenseMatrix<T>> for DenseVector<T>
where
    for<'a> &'a T: Mul<Output = T>,
{
    type Output = DenseVector<T>;

    #[inline]
    fn mul(self, rps: &DenseMatrix<T>) -> Self::Output {
        &self * rps
    }
}

impl<T: Sum> Mul<DenseMatrix<T>> for &DenseVector<T>
where
    for<'a> &'a T: Mul<Output = T>,
{
    type Output = DenseVector<T>;

    #[inline]
    fn mul(self, rps: DenseMatrix<T>) -> Self::Output {
        self * &rps
    }
}

impl<T: Sum> Mul<&DenseMatrix<T>> for &DenseVector<T>
where
    for<'a> &'a T: Mul<Output = T>,
{
    type Output = DenseVector<T>;

    fn mul(self, rps: &DenseMatrix<T>) -> Self::Output {
        debug_assert!(self.dimension() == rps.rows);
        (0..rps.columns())
            .map(|j| (0..rps.rows()).map(|i| &self[i] * &rps[(i, j)]).sum())
            .collect()
    }
}

impl<T: Zero + AddAssign + Clone> Square for DenseMatrix<T>
where
    for<'a> &'a T: Mul<Output = T>,
{
    type Output = Self;

    #[inline]
    fn square(self) -> Self::Output {
        &self * &self
    }
}

impl<T: Zero + AddAssign + Clone> Square for &DenseMatrix<T>
where
    for<'a> &'a T: Mul<Output = T>,
{
    type Output = DenseMatrix<T>;

    #[inline]
    fn square(self) -> Self::Output {
        self * self
    }
}

impl<T> Tensor for DenseMatrix<T>
where
    for<'a> &'a T: Mul<Output = T>,
{
    type Output = DenseMatrix<T>;

    #[inline]
    fn tensor(self, rps: Self) -> Self::Output {
        (&self).tensor(&rps)
    }
}

impl<T> Tensor<&Self> for DenseMatrix<T>
where
    for<'a> &'a T: Mul<Output = T>,
{
    type Output = DenseMatrix<T>;

    #[inline]
    fn tensor(self, rps: &Self) -> Self::Output {
        (&self).tensor(rps)
    }
}

impl<T> Tensor<DenseMatrix<T>> for &DenseMatrix<T>
where
    for<'a> &'a T: Mul<Output = T>,
{
    type Output = DenseMatrix<T>;

    #[inline]
    fn tensor(self, rps: DenseMatrix<T>) -> Self::Output {
        self.tensor(&rps)
    }
}

impl<T> Tensor for &DenseMatrix<T>
where
    for<'a> &'a T: Mul<Output = T>,
{
    type Output = DenseMatrix<T>;

    fn tensor(self, rps: Self) -> Self::Output {
        // Kronecker product
        let rows = self.rows * rps.rows;
        let columns = self.columns * rps.columns;
        let mut elements = Vec::<T>::with_capacity(rows * columns);
        for i in 0..self.rows {
            for j in 0..rps.rows {
                for k in 0..self.columns {
                    for l in 0..rps.columns {
                        elements.push(&self[(i, k)] * &rps[(j, l)])
                    }
                }
            }
        }
        Self::Output {
            rows,
            columns,
            elements,
        }
    }
}
