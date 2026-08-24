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

use crate::algebra::{Concat, Double, Inv, One, SemifieldOps, Square, Tensor, UnitalRing, Zero};
use crate::matrix::{DenseVector, IdentityMatrix};
use alloc::vec;
use alloc::vec::Vec;
use core::iter::{Sum, repeat_with, zip};
use core::ops::{Add, AddAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign};
#[cfg(feature = "rayon")]
use rayon::slice::ParallelSlice;
use serde::{Deserialize, Serialize};

/// A matrix in the row-major order.
#[derive(Clone, Debug, Default, Deserialize, Eq, Serialize)]
pub struct DenseMatrix<T> {
    rows: u32,
    columns: u32,
    elements: Vec<T>,
}

impl<T> DenseMatrix<T> {
    /// Construct a new matrix.
    pub const fn new(rows: u32, columns: u32, elements: Vec<T>) -> Self {
        debug_assert!(rows as usize * columns as usize == elements.len());
        Self {
            rows,
            columns,
            elements,
        }
    }

    /// Construct a square Vandermonde matrix.
    pub fn vandermonde(values: &[T]) -> Self
    where
        T: One + Clone + for<'a> MulAssign<&'a T>,
    {
        let n = values.len();
        let mut elements = Vec::<T>::with_capacity(n * n);
        for v in values {
            elements.push(T::ONE);
            let mut power = v.clone();
            for _ in 1..n {
                elements.push(power.clone());
                power *= v;
            }
        }
        Self {
            rows: n as u32,
            columns: n as u32,
            elements,
        }
    }

    /// Fill a new `m × n` matrix with a single `element`.
    pub fn fill(rows: u32, columns: u32, element: T) -> Self
    where
        T: Clone,
    {
        Self {
            rows,
            columns,
            elements: vec![element; rows as usize * columns as usize],
        }
    }

    /// Fill a new `m × n` matrix by calling `f`.
    #[inline]
    pub fn fill_with<F: FnMut() -> T>(rows: u32, columns: u32, f: F) -> Self {
        Self {
            rows,
            columns,
            elements: repeat_with(f)
                .take(rows as usize * columns as usize)
                .collect(),
        }
    }

    pub fn pad_to_power_of_two(&self) -> Self
    where
        T: Zero + Clone,
    {
        let m = self.rows.next_power_of_two() - self.rows;
        let n = self.columns.next_power_of_two() - self.columns;
        let mut elements =
            Vec::<T>::with_capacity((self.rows + m) as usize * (self.columns + n) as usize);
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
    pub const fn rows(&self) -> u32 {
        self.rows
    }

    /// The number of columns.
    pub const fn columns(&self) -> u32 {
        self.columns
    }

    /// Get i-th row as mutable slice.
    pub fn row_mut(&mut self, i: u32) -> &mut [T] {
        let begin = i as usize * self.columns as usize;
        let end = begin + self.columns as usize;
        &mut self.elements[begin..end]
    }

    /// Iterate rows.
    pub fn iter_row(&self) -> core::slice::ChunksExact<'_, T> {
        self.elements.chunks_exact(self.columns as usize)
    }

    /// Iterate rows.
    #[cfg(feature = "rayon")]
    pub fn par_iter_row(&self) -> rayon::slice::ChunksExact<'_, T>
    where
        T: Sync,
    {
        self.elements.par_chunks_exact(self.columns as usize)
    }

    /// Swap two rows.
    pub fn swap_row(&mut self, mut i: u32, mut j: u32) {
        if i > j {
            (i, j) = (j, i);
        }
        let (_, right) = self
            .elements
            .split_at_mut(i as usize * self.columns as usize);
        let (ith, right) = right.split_at_mut(self.columns as usize);
        let (_, right) = right.split_at_mut((j - i - 1) as usize * self.columns as usize);
        let (jth, _) = right.split_at_mut(self.columns as usize);
        ith.swap_with_slice(jth);
    }

    /// Convert a `m × n` matrix into a `1 × mn` row vector.
    #[inline]
    pub fn vectorize(self) -> DenseVector<T> {
        self.elements.into()
    }

    pub fn trace(&self) -> T
    where
        T: for<'a> Sum<&'a T>,
    {
        debug_assert!(self.rows == self.columns);
        self.iter_row().enumerate().map(|(i, row)| &row[i]).sum()
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

    /// The face-splitting product
    pub fn row_tensor(&self, rps: &Self) -> Self
    where
        for<'a> &'a T: Mul<Output = T>,
    {
        debug_assert!(self.rows == rps.rows);
        let rows = self.rows;
        let columns = self.columns * rps.columns;
        let mut elements = Vec::<T>::with_capacity(rows as usize * columns as usize);
        for (left, right) in zip(self.iter_row(), rps.iter_row()) {
            for l in left {
                for r in right {
                    elements.push(l * r)
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
        let mut elements = Vec::<T>::with_capacity(rows as usize * columns as usize);
        for left in self.iter_row() {
            for right in rps.iter_row() {
                for (l, r) in zip(left, right) {
                    elements.push(l * r)
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

impl<T: PartialEq> PartialEq for DenseMatrix<T> {
    fn eq(&self, rps: &Self) -> bool {
        self.columns == rps.columns && self.elements == rps.elements
    }
}

impl<T> From<DenseMatrix<T>> for (u32, u32, Vec<T>) {
    fn from(matrix: DenseMatrix<T>) -> Self {
        (matrix.rows, matrix.columns, matrix.elements)
    }
}

impl<T> AsRef<[T]> for DenseMatrix<T> {
    #[inline]
    fn as_ref(&self) -> &[T] {
        &self.elements
    }
}

impl<T> AsMut<[T]> for DenseMatrix<T> {
    #[inline]
    fn as_mut(&mut self) -> &mut [T] {
        &mut self.elements
    }
}

impl<T> Index<(u32, u32)> for DenseMatrix<T> {
    type Output = T;

    #[inline]
    fn index(&self, (i, j): (u32, u32)) -> &Self::Output {
        &self.elements[i as usize * self.columns as usize + j as usize]
    }
}

impl<T> IndexMut<(u32, u32)> for DenseMatrix<T> {
    #[inline]
    fn index_mut(&mut self, (i, j): (u32, u32)) -> &mut Self::Output {
        &mut self.elements[i as usize * self.columns as usize + j as usize]
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

impl<T> Double for &DenseMatrix<T>
where
    for<'a> &'a T: Double<Output = T>,
{
    type Output = DenseMatrix<T>;

    fn double(self) -> Self::Output {
        Self::Output {
            rows: self.rows,
            columns: self.columns,
            elements: self.elements.iter().map(Double::double).collect(),
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

impl<T: Sum> Mul for DenseMatrix<T>
where
    for<'a> &'a T: Mul<Output = T>,
{
    type Output = Self;

    #[inline]
    fn mul(self, rps: Self) -> Self::Output {
        &self * &rps
    }
}

impl<T: Sum> MulAssign for DenseMatrix<T>
where
    for<'a> &'a T: Mul<Output = T>,
{
    #[inline]
    fn mul_assign(&mut self, rps: Self) {
        *self = &*self * &rps
    }
}

impl<T: Sum> Mul<&DenseMatrix<T>> for DenseMatrix<T>
where
    for<'a> &'a T: Mul<Output = T>,
{
    type Output = Self;

    #[inline]
    fn mul(self, rps: &DenseMatrix<T>) -> Self::Output {
        &self * rps
    }
}

impl<T: Sum> MulAssign<&DenseMatrix<T>> for DenseMatrix<T>
where
    for<'a> &'a T: Mul<Output = T>,
{
    #[inline]
    fn mul_assign(&mut self, rps: &DenseMatrix<T>) {
        *self = &*self * rps
    }
}

impl<T: Sum> Mul<DenseMatrix<T>> for &DenseMatrix<T>
where
    for<'a> &'a T: Mul<Output = T>,
{
    type Output = DenseMatrix<T>;

    #[inline]
    fn mul(self, rps: DenseMatrix<T>) -> Self::Output {
        self * &rps
    }
}

impl<T: Sum> Mul for &DenseMatrix<T>
where
    for<'a> &'a T: Mul<Output = T>,
{
    type Output = DenseMatrix<T>;

    fn mul(self, rps: &DenseMatrix<T>) -> Self::Output {
        debug_assert!(self.columns == rps.rows);
        // Iterative algorithm
        let mut elements = Vec::<T>::with_capacity(self.rows as usize * rps.columns as usize);
        for row in self.iter_row() {
            for j in 0..rps.columns {
                elements.push(
                    zip(row, rps.iter_row().map(|row| &row[j as usize]))
                        .map(|(l, r)| l * r)
                        .sum(),
                )
            }
        }
        Self::Output {
            rows: self.rows,
            columns: rps.columns,
            elements,
        }
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

impl<'a, T: Sum> Mul<&'a DenseVector<T>> for &DenseMatrix<T>
where
    for<'b> &'b T: Mul<Output = T>,
{
    type Output = DenseVector<T>;

    fn mul(self, rps: &'a DenseVector<T>) -> Self::Output {
        debug_assert!(self.columns == rps.dimension());
        self.iter_row()
            .map(|row| zip(row, rps).map(|(l, r)| l * r).sum())
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

impl<'a, T: Sum> Mul<&'a DenseMatrix<T>> for &DenseVector<T>
where
    for<'b> &'b T: Mul<Output = T>,
{
    type Output = DenseVector<T>;

    fn mul(self, rps: &'a DenseMatrix<T>) -> Self::Output {
        debug_assert!(self.dimension() == rps.rows);
        (0..rps.columns)
            .map(|j| {
                zip(self, rps.iter_row().map(|row| &row[j as usize]))
                    .map(|(l, r)| l * r)
                    .sum()
            })
            .collect()
    }
}

impl<T: Sum> Square for DenseMatrix<T>
where
    for<'a> &'a T: Mul<Output = T>,
{
    type Output = Self;

    #[inline]
    fn square(self) -> Self::Output {
        &self * &self
    }
}

impl<T: Sum> Square for &DenseMatrix<T>
where
    for<'a> &'a T: Mul<Output = T>,
{
    type Output = DenseMatrix<T>;

    #[inline]
    fn square(self) -> Self::Output {
        self * self
    }
}

impl<R: UnitalRing + Clone> Inv for DenseMatrix<R>
where
    for<'a> &'a R: SemifieldOps<R>,
{
    type Output = Option<DenseMatrix<R>>;

    fn inv(mut self) -> Self::Output {
        if self.rows != self.columns {
            return None;
        }
        // Gauss–Jordan elimination
        let mut a: DenseMatrix<R> = IdentityMatrix::new(self.rows).into();
        for i in 0..self.rows {
            let (pivot, f) = (i..self.rows).find_map(|j| {
                let (f, is_inv) = (&self[(j, i)]).inv().into();
                if is_inv { Some((j, f)) } else { None }
            })?;
            if pivot != i {
                self.swap_row(i, pivot);
                a.swap_row(i, pivot);
            }

            for element in self.row_mut(i) {
                *element = &f * &*element;
            }
            for element in a.row_mut(i) {
                *element = &f * &*element;
            }

            for j in 0..self.rows {
                if i == j {
                    continue;
                }
                let f = self[(j, i)].clone();
                for k in 0..self.columns {
                    let g = self[(i, k)].clone();
                    self[(j, k)] -= &f * g;
                }
                for k in 0..a.columns {
                    let g = a[(i, k)].clone();
                    a[(j, k)] -= &f * g;
                }
            }
        }
        Some(a)
    }
}

impl<R: UnitalRing + Clone> Inv for &DenseMatrix<R>
where
    for<'a> &'a R: SemifieldOps<R>,
{
    type Output = Option<DenseMatrix<R>>;

    fn inv(self) -> Self::Output {
        self.clone().inv()
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
        let mut elements = Vec::<T>::with_capacity(rows as usize * columns as usize);
        for left in self.iter_row() {
            for right in rps.iter_row() {
                for l in left {
                    for r in right {
                        elements.push(l * r)
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

impl<T: Clone> Concat for DenseMatrix<T> {
    type Output = Self;

    #[inline]
    fn concat(self, rps: Self) -> Self::Output {
        (&self).concat(&rps)
    }
}

impl<T: Clone> Concat<&Self> for DenseMatrix<T> {
    type Output = Self;

    #[inline]
    fn concat(self, rps: &Self) -> Self::Output {
        (&self).concat(rps)
    }
}

impl<T: Clone> Concat<DenseMatrix<T>> for &DenseMatrix<T> {
    type Output = DenseMatrix<T>;

    #[inline]
    fn concat(self, rps: DenseMatrix<T>) -> Self::Output {
        self.concat(&rps)
    }
}

impl<T: Clone> Concat for &DenseMatrix<T> {
    type Output = DenseMatrix<T>;

    fn concat(self, rps: Self) -> Self::Output {
        debug_assert!(self.rows == rps.rows);
        let rows = self.rows;
        let columns = self.columns + rps.columns;
        let mut elements = Vec::<T>::with_capacity(rows as usize * columns as usize);
        zip(self.iter_row(), rps.iter_row()).for_each(|(l, r)| {
            elements.extend_from_slice(l);
            elements.extend_from_slice(r);
        });
        Self::Output {
            rows,
            columns,
            elements,
        }
    }
}
