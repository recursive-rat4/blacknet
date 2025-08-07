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

use crate::ring::Ring;
use crate::vectordense::VectorDense;
use core::iter::zip;
use core::ops::{Add, AddAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Debug, Eq, PartialEq)]
pub struct MatrixDense<R: Ring> {
    rows: usize,
    columns: usize,
    elements: Vec<R>,
}

impl<R: Ring> MatrixDense<R> {
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

impl<R: Ring> Index<(usize, usize)> for MatrixDense<R> {
    type Output = R;

    #[inline]
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (i, j) = index;
        &self.elements[i * self.columns + j]
    }
}

impl<R: Ring> IndexMut<(usize, usize)> for MatrixDense<R> {
    #[inline]
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (i, j) = index;
        &mut self.elements[i * self.columns + j]
    }
}

impl<R: Ring> Add for MatrixDense<R> {
    type Output = MatrixDense<R>;

    fn add(self, rps: Self) -> Self::Output {
        MatrixDense::new(
            self.rows,
            self.columns,
            zip(self.elements, rps.elements)
                .map(|(l, r)| l + r)
                .collect(),
        )
    }
}

impl<R: Ring> AddAssign for MatrixDense<R> {
    fn add_assign(&mut self, rps: Self) {
        zip(self.elements.iter_mut(), rps.elements).for_each(|(l, r)| *l += r);
    }
}

impl<R: Ring> Add<&MatrixDense<R>> for MatrixDense<R> {
    type Output = MatrixDense<R>;

    fn add(self, rps: &MatrixDense<R>) -> Self::Output {
        MatrixDense::new(
            self.rows,
            self.columns,
            zip(self.elements, rps.elements.iter())
                .map(|(l, &r)| l + r)
                .collect(),
        )
    }
}

impl<R: Ring> AddAssign<&MatrixDense<R>> for MatrixDense<R> {
    fn add_assign(&mut self, rps: &MatrixDense<R>) {
        zip(self.elements.iter_mut(), rps.elements.iter()).for_each(|(l, &r)| *l += r);
    }
}

impl<R: Ring> Add<MatrixDense<R>> for &MatrixDense<R> {
    type Output = MatrixDense<R>;

    fn add(self, rps: MatrixDense<R>) -> Self::Output {
        MatrixDense::new(
            self.rows,
            self.columns,
            zip(self.elements.iter(), rps.elements)
                .map(|(&l, r)| l + r)
                .collect(),
        )
    }
}

impl<R: Ring> Add for &MatrixDense<R> {
    type Output = MatrixDense<R>;

    fn add(self, rps: Self) -> Self::Output {
        MatrixDense::new(
            self.rows,
            self.columns,
            zip(self.elements.iter(), rps.elements.iter())
                .map(|(&l, &r)| l + r)
                .collect(),
        )
    }
}

impl<R: Ring> Neg for MatrixDense<R> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        MatrixDense::new(
            self.rows,
            self.columns,
            self.elements.into_iter().map(|e| -e).collect(),
        )
    }
}

impl<R: Ring> Neg for &MatrixDense<R> {
    type Output = MatrixDense<R>;

    fn neg(self) -> Self::Output {
        MatrixDense::new(
            self.rows,
            self.columns,
            self.elements.iter().map(|&e| -e).collect(),
        )
    }
}

impl<R: Ring> Sub for MatrixDense<R> {
    type Output = MatrixDense<R>;

    fn sub(self, rps: Self) -> Self::Output {
        MatrixDense::new(
            self.rows,
            self.columns,
            zip(self.elements, rps.elements)
                .map(|(l, r)| l - r)
                .collect(),
        )
    }
}

impl<R: Ring> SubAssign for MatrixDense<R> {
    fn sub_assign(&mut self, rps: Self) {
        zip(self.elements.iter_mut(), rps.elements).for_each(|(l, r)| *l -= r);
    }
}

impl<R: Ring> Sub<&MatrixDense<R>> for MatrixDense<R> {
    type Output = MatrixDense<R>;

    fn sub(self, rps: &MatrixDense<R>) -> Self::Output {
        MatrixDense::new(
            self.rows,
            self.columns,
            zip(self.elements, rps.elements.iter())
                .map(|(l, &r)| l - r)
                .collect(),
        )
    }
}

impl<R: Ring> SubAssign<&MatrixDense<R>> for MatrixDense<R> {
    fn sub_assign(&mut self, rps: &MatrixDense<R>) {
        zip(self.elements.iter_mut(), rps.elements.iter()).for_each(|(l, &r)| *l -= r);
    }
}

impl<R: Ring> Sub<MatrixDense<R>> for &MatrixDense<R> {
    type Output = MatrixDense<R>;

    fn sub(self, rps: MatrixDense<R>) -> Self::Output {
        MatrixDense::new(
            self.rows,
            self.columns,
            zip(self.elements.iter(), rps.elements)
                .map(|(&l, r)| l - r)
                .collect(),
        )
    }
}

impl<R: Ring> Sub for &MatrixDense<R> {
    type Output = MatrixDense<R>;

    fn sub(self, rps: Self) -> Self::Output {
        MatrixDense::new(
            self.rows,
            self.columns,
            zip(self.elements.iter(), rps.elements.iter())
                .map(|(&l, &r)| l - r)
                .collect(),
        )
    }
}

impl<R: Ring> Mul for MatrixDense<R> {
    type Output = Self;

    #[inline]
    fn mul(self, rps: Self) -> Self::Output {
        &self * &rps
    }
}

impl<R: Ring> MulAssign for MatrixDense<R> {
    #[inline]
    fn mul_assign(&mut self, rps: Self) {
        *self = &*self * &rps
    }
}

impl<R: Ring> Mul<&MatrixDense<R>> for MatrixDense<R> {
    type Output = Self;

    #[inline]
    fn mul(self, rps: &MatrixDense<R>) -> Self::Output {
        &self * rps
    }
}

impl<R: Ring> MulAssign<&MatrixDense<R>> for MatrixDense<R> {
    #[inline]
    fn mul_assign(&mut self, rps: &MatrixDense<R>) {
        *self = &*self * rps
    }
}

impl<R: Ring> Mul<MatrixDense<R>> for &MatrixDense<R> {
    type Output = MatrixDense<R>;

    #[inline]
    fn mul(self, rps: MatrixDense<R>) -> Self::Output {
        self * &rps
    }
}

impl<R: Ring> Mul for &MatrixDense<R> {
    type Output = MatrixDense<R>;

    fn mul(self, rps: &MatrixDense<R>) -> Self::Output {
        // Iterative algorithm
        let mut r = MatrixDense::fill(self.rows, rps.columns, R::ZERO);
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

impl<R: Ring> Mul<R> for MatrixDense<R> {
    type Output = Self;

    fn mul(self, rps: R) -> Self::Output {
        MatrixDense::new(
            self.rows,
            self.columns,
            self.elements.into_iter().map(|e| e * rps).collect(),
        )
    }
}

impl<R: Ring> MulAssign<R> for MatrixDense<R> {
    fn mul_assign(&mut self, rps: R) {
        self.elements.iter_mut().for_each(|e| *e *= rps);
    }
}

impl<R: Ring> Mul<R> for &MatrixDense<R> {
    type Output = MatrixDense<R>;

    fn mul(self, rps: R) -> Self::Output {
        MatrixDense::new(
            self.rows,
            self.columns,
            self.elements.iter().map(|&e| e * rps).collect(),
        )
    }
}

impl<R: Ring> Mul<&VectorDense<R>> for &MatrixDense<R> {
    type Output = VectorDense<R>;

    fn mul(self, rps: &VectorDense<R>) -> Self::Output {
        let mut r = VectorDense::fill(self.rows, R::ZERO);
        for i in 0..self.rows {
            for j in 0..self.columns {
                r[i] += self[(i, j)] * rps[j]
            }
        }
        r
    }
}

impl<R: Ring> Mul<&MatrixDense<R>> for &VectorDense<R> {
    type Output = VectorDense<R>;

    fn mul(self, rps: &MatrixDense<R>) -> Self::Output {
        let mut r = VectorDense::fill(rps.columns, R::ZERO);
        for i in 0..rps.rows {
            for j in 0..rps.columns {
                r[j] += self[i] * rps[(i, j)]
            }
        }
        r
    }
}
