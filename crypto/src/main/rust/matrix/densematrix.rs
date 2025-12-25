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

use crate::matrix::DenseVector;
use crate::operation::{Double, Square};
use crate::ring::Ring;
use crate::semiring::Presemiring;
use alloc::vec;
use alloc::vec::Vec;
use core::iter::zip;
use core::ops::{Add, AddAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DenseMatrix<R: Presemiring> {
    rows: usize,
    columns: usize,
    elements: Vec<R>,
}

impl<R: Presemiring> DenseMatrix<R> {
    pub const fn new(rows: usize, columns: usize, elements: Vec<R>) -> Self {
        Self {
            rows,
            columns,
            elements,
        }
    }

    pub fn fill(rows: usize, columns: usize, element: R) -> Self {
        Self::new(rows, columns, vec![element; rows * columns])
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
        Self::new(self.rows + m, self.columns + n, elements)
    }

    pub const fn rows(&self) -> usize {
        self.rows
    }

    pub const fn columns(&self) -> usize {
        self.columns
    }

    pub const fn elements(&self) -> &Vec<R> {
        &self.elements
    }

    pub fn cat(&self, rps: &Self) -> Self {
        let mut elements = Vec::<R>::with_capacity(self.rows * (self.columns + rps.columns));
        for i in 0..self.rows {
            for j in 0..self.columns {
                elements.push(self[(i, j)])
            }
            for j in 0..rps.columns {
                elements.push(rps[(i, j)])
            }
        }
        Self::new(self.rows, self.columns + rps.columns, elements)
    }

    pub fn trace(&self) -> R {
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
        Self::new(self.columns, self.rows, elements)
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
    type Output = DenseMatrix<R>;

    fn add(self, rps: Self) -> Self::Output {
        DenseMatrix::new(
            self.rows,
            self.columns,
            zip(self.elements, rps.elements)
                .map(|(l, r)| l + r)
                .collect(),
        )
    }
}

impl<R: Presemiring> AddAssign for DenseMatrix<R> {
    fn add_assign(&mut self, rps: Self) {
        zip(self.elements.iter_mut(), rps.elements).for_each(|(l, r)| *l += r);
    }
}

impl<R: Presemiring> Double for DenseMatrix<R> {
    type Output = Self;

    fn double(self) -> Self::Output {
        DenseMatrix::new(
            self.rows,
            self.columns,
            self.elements.into_iter().map(Double::double).collect(),
        )
    }
}

impl<R: Presemiring> Add<&DenseMatrix<R>> for DenseMatrix<R> {
    type Output = DenseMatrix<R>;

    fn add(self, rps: &DenseMatrix<R>) -> Self::Output {
        DenseMatrix::new(
            self.rows,
            self.columns,
            zip(self.elements, rps.elements.iter())
                .map(|(l, &r)| l + r)
                .collect(),
        )
    }
}

impl<R: Presemiring> AddAssign<&DenseMatrix<R>> for DenseMatrix<R> {
    fn add_assign(&mut self, rps: &DenseMatrix<R>) {
        zip(self.elements.iter_mut(), rps.elements.iter()).for_each(|(l, &r)| *l += r);
    }
}

impl<R: Presemiring> Add<DenseMatrix<R>> for &DenseMatrix<R> {
    type Output = DenseMatrix<R>;

    fn add(self, rps: DenseMatrix<R>) -> Self::Output {
        DenseMatrix::new(
            self.rows,
            self.columns,
            zip(self.elements.iter(), rps.elements)
                .map(|(&l, r)| l + r)
                .collect(),
        )
    }
}

impl<R: Presemiring> Add for &DenseMatrix<R> {
    type Output = DenseMatrix<R>;

    fn add(self, rps: Self) -> Self::Output {
        DenseMatrix::new(
            self.rows,
            self.columns,
            zip(self.elements.iter(), rps.elements.iter())
                .map(|(&l, &r)| l + r)
                .collect(),
        )
    }
}

impl<R: Ring> Neg for DenseMatrix<R> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        DenseMatrix::new(
            self.rows,
            self.columns,
            self.elements.into_iter().map(Neg::neg).collect(),
        )
    }
}

impl<R: Ring> Neg for &DenseMatrix<R> {
    type Output = DenseMatrix<R>;

    fn neg(self) -> Self::Output {
        DenseMatrix::new(
            self.rows,
            self.columns,
            self.elements.iter().map(|&e| -e).collect(),
        )
    }
}

impl<R: Ring> Sub for DenseMatrix<R> {
    type Output = DenseMatrix<R>;

    fn sub(self, rps: Self) -> Self::Output {
        DenseMatrix::new(
            self.rows,
            self.columns,
            zip(self.elements, rps.elements)
                .map(|(l, r)| l - r)
                .collect(),
        )
    }
}

impl<R: Ring> SubAssign for DenseMatrix<R> {
    fn sub_assign(&mut self, rps: Self) {
        zip(self.elements.iter_mut(), rps.elements).for_each(|(l, r)| *l -= r);
    }
}

impl<R: Ring> Sub<&DenseMatrix<R>> for DenseMatrix<R> {
    type Output = DenseMatrix<R>;

    fn sub(self, rps: &DenseMatrix<R>) -> Self::Output {
        DenseMatrix::new(
            self.rows,
            self.columns,
            zip(self.elements, rps.elements.iter())
                .map(|(l, &r)| l - r)
                .collect(),
        )
    }
}

impl<R: Ring> SubAssign<&DenseMatrix<R>> for DenseMatrix<R> {
    fn sub_assign(&mut self, rps: &DenseMatrix<R>) {
        zip(self.elements.iter_mut(), rps.elements.iter()).for_each(|(l, &r)| *l -= r);
    }
}

impl<R: Ring> Sub<DenseMatrix<R>> for &DenseMatrix<R> {
    type Output = DenseMatrix<R>;

    fn sub(self, rps: DenseMatrix<R>) -> Self::Output {
        DenseMatrix::new(
            self.rows,
            self.columns,
            zip(self.elements.iter(), rps.elements)
                .map(|(&l, r)| l - r)
                .collect(),
        )
    }
}

impl<R: Ring> Sub for &DenseMatrix<R> {
    type Output = DenseMatrix<R>;

    fn sub(self, rps: Self) -> Self::Output {
        DenseMatrix::new(
            self.rows,
            self.columns,
            zip(self.elements.iter(), rps.elements.iter())
                .map(|(&l, &r)| l - r)
                .collect(),
        )
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
        DenseMatrix::new(
            self.rows,
            self.columns,
            self.elements.into_iter().map(|e| e * rps).collect(),
        )
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
        DenseMatrix::new(
            self.rows,
            self.columns,
            self.elements.iter().map(|&e| e * rps).collect(),
        )
    }
}

impl<R: Presemiring> Mul<&DenseVector<R>> for &DenseMatrix<R> {
    type Output = DenseVector<R>;

    fn mul(self, rps: &DenseVector<R>) -> Self::Output {
        let mut r = DenseVector::fill(self.rows, R::ZERO);
        for i in 0..self.rows {
            for j in 0..self.columns {
                r[i] += self[(i, j)] * rps[j]
            }
        }
        r
    }
}

impl<R: Presemiring> Mul<&DenseMatrix<R>> for &DenseVector<R> {
    type Output = DenseVector<R>;

    fn mul(self, rps: &DenseMatrix<R>) -> Self::Output {
        let mut r = DenseVector::fill(rps.columns, R::ZERO);
        for i in 0..rps.rows {
            for j in 0..rps.columns {
                r[j] += self[i] * rps[(i, j)]
            }
        }
        r
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
