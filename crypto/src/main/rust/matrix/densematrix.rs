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

use crate::algebra::{Double, Presemiring, Ring, Semiring, Square, Tensor};
use crate::matrix::DenseVector;
use alloc::vec;
use alloc::vec::Vec;
use core::iter::zip;
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
pub struct DenseMatrix<R: Presemiring> {
    rows: usize,
    columns: usize,
    elements: Vec<R>,
}

impl<R: Presemiring> DenseMatrix<R> {
    /// Construct a new matrix.
    pub const fn new(rows: usize, columns: usize, elements: Vec<R>) -> Self {
        debug_assert!(rows * columns == elements.len());
        Self {
            rows,
            columns,
            elements,
        }
    }

    /// Fill a new `m ⨉ n` matrix with a single `element`.
    pub fn fill(rows: usize, columns: usize, element: R) -> Self {
        Self {
            rows,
            columns,
            elements: vec![element; rows * columns],
        }
    }

    pub fn pad_to_power_of_two(&self) -> Self {
        let m = self.rows.next_power_of_two() - self.rows;
        let n = self.columns.next_power_of_two() - self.columns;
        let mut elements = Vec::<R>::with_capacity((self.rows + m) * (self.columns + n));
        for i in 0..self.rows {
            for j in 0..self.columns {
                elements.push(self[(i, j)])
            }
            for _j in 0..n {
                elements.push(R::ZERO)
            }
        }
        for _ in 0..m * (self.columns + n) {
            elements.push(R::ZERO)
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
    pub const fn elements(&self) -> &Vec<R> {
        &self.elements
    }

    /// Concatenate horizontally.
    pub fn cat(&self, rps: &Self) -> Self {
        debug_assert!(self.rows == rps.rows);
        let mut elements = Vec::<R>::with_capacity(self.rows * (self.columns + rps.columns));
        for i in 0..self.rows {
            for j in 0..self.columns {
                elements.push(self[(i, j)])
            }
            for j in 0..rps.columns {
                elements.push(rps[(i, j)])
            }
        }
        Self {
            rows: self.rows,
            columns: self.columns + rps.columns,
            elements,
        }
    }

    pub fn trace(&self) -> R {
        debug_assert!(self.rows == self.columns);
        let mut sigma = R::ZERO;
        for i in 0..self.rows {
            sigma += self[(i, i)]
        }
        sigma
    }

    pub fn transpose(&self) -> Self {
        let mut elements = Vec::<R>::with_capacity(self.elements.len());
        for j in 0..self.columns {
            for i in 0..self.rows {
                elements.push(self[(i, j)]);
            }
        }
        Self {
            rows: self.columns,
            columns: self.rows,
            elements,
        }
    }
}

impl<R: Semiring> DenseMatrix<R> {
    /// The `n ⨉ n` multiplicative identity.
    pub fn identity(n: usize) -> Self {
        let mut elements = vec![R::ZERO; n * n];
        for i in 0..n {
            elements[i * n + i] = R::ONE;
        }
        Self {
            rows: n,
            columns: n,
            elements,
        }
    }
}

impl<R: Presemiring> From<DenseMatrix<R>> for (usize, usize, Vec<R>) {
    fn from(matrix: DenseMatrix<R>) -> Self {
        (matrix.rows, matrix.columns, matrix.elements)
    }
}

impl<R: Presemiring> Index<(usize, usize)> for DenseMatrix<R> {
    type Output = R;

    #[inline]
    fn index(&self, (i, j): (usize, usize)) -> &Self::Output {
        &self.elements[i * self.columns + j]
    }
}

impl<R: Presemiring> IndexMut<(usize, usize)> for DenseMatrix<R> {
    #[inline]
    fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut Self::Output {
        &mut self.elements[i * self.columns + j]
    }
}

impl<R: Presemiring> Add for DenseMatrix<R> {
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

impl<R: Presemiring> AddAssign for DenseMatrix<R> {
    fn add_assign(&mut self, rps: Self) {
        debug_assert!(self.rows == rps.rows && self.columns == rps.columns);
        zip(self.elements.iter_mut(), rps.elements).for_each(|(l, r)| *l += r);
    }
}

impl<R: Presemiring> Double for DenseMatrix<R> {
    type Output = Self;

    fn double(self) -> Self::Output {
        Self {
            rows: self.rows,
            columns: self.columns,
            elements: self.elements.into_iter().map(Double::double).collect(),
        }
    }
}

impl<R: Presemiring> Add<&DenseMatrix<R>> for DenseMatrix<R> {
    type Output = Self;

    fn add(self, rps: &DenseMatrix<R>) -> Self::Output {
        debug_assert!(self.rows == rps.rows && self.columns == rps.columns);
        Self {
            rows: self.rows,
            columns: self.columns,
            elements: zip(self.elements, rps.elements.iter())
                .map(|(l, &r)| l + r)
                .collect(),
        }
    }
}

impl<R: Presemiring> AddAssign<&DenseMatrix<R>> for DenseMatrix<R> {
    fn add_assign(&mut self, rps: &DenseMatrix<R>) {
        debug_assert!(self.rows == rps.rows && self.columns == rps.columns);
        zip(self.elements.iter_mut(), rps.elements.iter()).for_each(|(l, &r)| *l += r);
    }
}

impl<R: Presemiring> Add<DenseMatrix<R>> for &DenseMatrix<R> {
    type Output = DenseMatrix<R>;

    fn add(self, rps: DenseMatrix<R>) -> Self::Output {
        debug_assert!(self.rows == rps.rows && self.columns == rps.columns);
        Self::Output {
            rows: self.rows,
            columns: self.columns,
            elements: zip(self.elements.iter(), rps.elements)
                .map(|(&l, r)| l + r)
                .collect(),
        }
    }
}

impl<R: Presemiring> Add for &DenseMatrix<R> {
    type Output = DenseMatrix<R>;

    fn add(self, rps: Self) -> Self::Output {
        debug_assert!(self.rows == rps.rows && self.columns == rps.columns);
        Self::Output {
            rows: self.rows,
            columns: self.columns,
            elements: zip(self.elements.iter(), rps.elements.iter())
                .map(|(&l, &r)| l + r)
                .collect(),
        }
    }
}

impl<R: Ring> Neg for DenseMatrix<R> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            rows: self.rows,
            columns: self.columns,
            elements: self.elements.into_iter().map(Neg::neg).collect(),
        }
    }
}

impl<R: Ring> Neg for &DenseMatrix<R> {
    type Output = DenseMatrix<R>;

    fn neg(self) -> Self::Output {
        Self::Output {
            rows: self.rows,
            columns: self.columns,
            elements: self.elements.iter().map(|&e| -e).collect(),
        }
    }
}

impl<R: Ring> Sub for DenseMatrix<R> {
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

impl<R: Ring> SubAssign for DenseMatrix<R> {
    fn sub_assign(&mut self, rps: Self) {
        debug_assert!(self.rows == rps.rows && self.columns == rps.columns);
        zip(self.elements.iter_mut(), rps.elements).for_each(|(l, r)| *l -= r);
    }
}

impl<R: Ring> Sub<&DenseMatrix<R>> for DenseMatrix<R> {
    type Output = Self;

    fn sub(self, rps: &DenseMatrix<R>) -> Self::Output {
        debug_assert!(self.rows == rps.rows && self.columns == rps.columns);
        Self {
            rows: self.rows,
            columns: self.columns,
            elements: zip(self.elements, rps.elements.iter())
                .map(|(l, &r)| l - r)
                .collect(),
        }
    }
}

impl<R: Ring> SubAssign<&DenseMatrix<R>> for DenseMatrix<R> {
    fn sub_assign(&mut self, rps: &DenseMatrix<R>) {
        debug_assert!(self.rows == rps.rows && self.columns == rps.columns);
        zip(self.elements.iter_mut(), rps.elements.iter()).for_each(|(l, &r)| *l -= r);
    }
}

impl<R: Ring> Sub<DenseMatrix<R>> for &DenseMatrix<R> {
    type Output = DenseMatrix<R>;

    fn sub(self, rps: DenseMatrix<R>) -> Self::Output {
        debug_assert!(self.rows == rps.rows && self.columns == rps.columns);
        Self::Output {
            rows: self.rows,
            columns: self.columns,
            elements: zip(self.elements.iter(), rps.elements)
                .map(|(&l, r)| l - r)
                .collect(),
        }
    }
}

impl<R: Ring> Sub for &DenseMatrix<R> {
    type Output = DenseMatrix<R>;

    fn sub(self, rps: Self) -> Self::Output {
        debug_assert!(self.rows == rps.rows && self.columns == rps.columns);
        Self::Output {
            rows: self.rows,
            columns: self.columns,
            elements: zip(self.elements.iter(), rps.elements.iter())
                .map(|(&l, &r)| l - r)
                .collect(),
        }
    }
}

impl<R: Presemiring> Mul for DenseMatrix<R> {
    type Output = Self;

    #[inline]
    fn mul(self, rps: Self) -> Self::Output {
        &self * &rps
    }
}

impl<R: Presemiring> MulAssign for DenseMatrix<R> {
    #[inline]
    fn mul_assign(&mut self, rps: Self) {
        *self = &*self * &rps
    }
}

impl<R: Presemiring> Mul<&DenseMatrix<R>> for DenseMatrix<R> {
    type Output = Self;

    #[inline]
    fn mul(self, rps: &DenseMatrix<R>) -> Self::Output {
        &self * rps
    }
}

impl<R: Presemiring> MulAssign<&DenseMatrix<R>> for DenseMatrix<R> {
    #[inline]
    fn mul_assign(&mut self, rps: &DenseMatrix<R>) {
        *self = &*self * rps
    }
}

impl<R: Presemiring> Mul<DenseMatrix<R>> for &DenseMatrix<R> {
    type Output = DenseMatrix<R>;

    #[inline]
    fn mul(self, rps: DenseMatrix<R>) -> Self::Output {
        self * &rps
    }
}

impl<R: Presemiring> Mul for &DenseMatrix<R> {
    type Output = DenseMatrix<R>;

    fn mul(self, rps: &DenseMatrix<R>) -> Self::Output {
        debug_assert!(self.columns == rps.rows);
        // Iterative algorithm
        let mut r = DenseMatrix::fill(self.rows, rps.columns, R::ZERO);
        for i in 0..self.rows {
            for j in 0..rps.columns {
                for k in 0..self.columns {
                    r[(i, j)] += self[(i, k)] * rps[(k, j)];
                }
            }
        }
        r
    }
}

impl<R: Presemiring> Mul<R> for DenseMatrix<R> {
    type Output = Self;

    fn mul(self, rps: R) -> Self::Output {
        Self {
            rows: self.rows,
            columns: self.columns,
            elements: self.elements.into_iter().map(|e| e * rps).collect(),
        }
    }
}

impl<R: Presemiring> MulAssign<R> for DenseMatrix<R> {
    fn mul_assign(&mut self, rps: R) {
        self.elements.iter_mut().for_each(|e| *e *= rps);
    }
}

impl<R: Presemiring> Mul<R> for &DenseMatrix<R> {
    type Output = DenseMatrix<R>;

    fn mul(self, rps: R) -> Self::Output {
        Self::Output {
            rows: self.rows,
            columns: self.columns,
            elements: self.elements.iter().map(|&e| e * rps).collect(),
        }
    }
}

impl<R: Presemiring> Mul<&DenseVector<R>> for &DenseMatrix<R> {
    type Output = DenseVector<R>;

    fn mul(self, rps: &DenseVector<R>) -> Self::Output {
        debug_assert!(self.columns == rps.dimension());
        (0..self.rows())
            .map(|i| (0..self.columns()).map(|j| self[(i, j)] * rps[j]).sum())
            .collect()
    }
}

impl<R: Presemiring> Mul<&DenseMatrix<R>> for &DenseVector<R> {
    type Output = DenseVector<R>;

    fn mul(self, rps: &DenseMatrix<R>) -> Self::Output {
        debug_assert!(self.dimension() == rps.rows);
        (0..rps.columns())
            .map(|j| (0..rps.rows()).map(|i| self[i] * rps[(i, j)]).sum())
            .collect()
    }
}

impl<R: Presemiring> Square for DenseMatrix<R> {
    type Output = Self;

    #[inline]
    fn square(self) -> Self::Output {
        &self * &self
    }
}

impl<R: Presemiring> Square for &DenseMatrix<R> {
    type Output = DenseMatrix<R>;

    #[inline]
    fn square(self) -> Self::Output {
        self * self
    }
}

impl<R: Presemiring> Tensor for DenseMatrix<R> {
    type Output = DenseMatrix<R>;

    #[inline]
    fn tensor(self, rps: Self) -> Self::Output {
        (&self).tensor(&rps)
    }
}

impl<R: Presemiring> Tensor<&Self> for DenseMatrix<R> {
    type Output = DenseMatrix<R>;

    #[inline]
    fn tensor(self, rps: &Self) -> Self::Output {
        (&self).tensor(rps)
    }
}

impl<R: Presemiring> Tensor<DenseMatrix<R>> for &DenseMatrix<R> {
    type Output = DenseMatrix<R>;

    #[inline]
    fn tensor(self, rps: DenseMatrix<R>) -> Self::Output {
        self.tensor(&rps)
    }
}

impl<R: Presemiring> Tensor for &DenseMatrix<R> {
    type Output = DenseMatrix<R>;

    fn tensor(self, rps: Self) -> Self::Output {
        // Kronecker product
        let rows = self.rows * rps.rows;
        let columns = self.columns * rps.columns;
        let mut elements = Vec::<R>::with_capacity(rows * columns);
        for i in 0..self.rows {
            for j in 0..rps.rows {
                for k in 0..self.columns {
                    for l in 0..rps.columns {
                        elements.push(self[(i, k)] * rps[(j, l)])
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
