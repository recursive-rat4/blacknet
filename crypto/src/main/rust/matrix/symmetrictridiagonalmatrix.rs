/*
 * Copyright (c) 2025 Pavel Vasin
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

use crate::matrix::{DenseMatrix, DenseVector};
use crate::operation::Double;
use crate::ring::Ring;
use crate::semiring::Presemiring;
use alloc::vec::Vec;
use core::iter::zip;
use core::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use serde::{Deserialize, Serialize};

/// A matrix that is both symmetric and tridiagonal.
///
/// # Panics
///
/// In debug builds, panic on incompatible dimensions.
///
/// # Safety
///
/// In release builds, undefined behaviour on incompatible dimensions.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SymmetricTridiagonalMatrix<R: Presemiring> {
    elements: Vec<R>,
}

impl<R: Presemiring> SymmetricTridiagonalMatrix<R> {
    /// Construct a new matrix given the concatenation of leading and following diagonals.
    pub const fn new(elements: Vec<R>) -> Self {
        debug_assert!(elements.len() & 1 == 1 || elements.len() == 0);
        Self { elements }
    }

    /// The number of rows.
    pub const fn rows(&self) -> usize {
        (self.elements.len() + 1) >> 1
    }

    /// The number of columns.
    pub const fn columns(&self) -> usize {
        (self.elements.len() + 1) >> 1
    }

    pub fn trace(&self) -> R {
        self.elements[0..self.columns()].iter().sum()
    }

    pub const fn transpose(&self) -> &Self {
        self
    }

    fn index(&self, i: usize, j: usize) -> R {
        if i == j {
            self.elements[i]
        } else if i == j + 1 {
            self.elements[self.columns() + j]
        } else if j == i + 1 {
            self.elements[self.columns() + i]
        } else {
            R::ZERO
        }
    }
}

impl<R: Presemiring> From<&SymmetricTridiagonalMatrix<R>> for DenseMatrix<R> {
    fn from(stm: &SymmetricTridiagonalMatrix<R>) -> Self {
        let mut elements = Vec::<R>::with_capacity(stm.rows() * stm.columns());
        for i in 0..stm.rows() {
            for j in 0..stm.columns() {
                elements.push(stm.index(i, j))
            }
        }
        DenseMatrix::new(stm.rows(), stm.columns(), elements)
    }
}

impl<R: Presemiring> Add for SymmetricTridiagonalMatrix<R> {
    type Output = Self;

    fn add(self, rps: Self) -> Self::Output {
        debug_assert!(self.rows() == rps.rows() && self.columns() == rps.columns());
        Self {
            elements: zip(self.elements, rps.elements)
                .map(|(l, r)| l + r)
                .collect(),
        }
    }
}

impl<R: Presemiring> AddAssign for SymmetricTridiagonalMatrix<R> {
    fn add_assign(&mut self, rps: Self) {
        debug_assert!(self.rows() == rps.rows() && self.columns() == rps.columns());
        zip(self.elements.iter_mut(), rps.elements).for_each(|(l, r)| *l += r);
    }
}

impl<R: Presemiring> Double for SymmetricTridiagonalMatrix<R> {
    type Output = Self;

    fn double(self) -> Self::Output {
        Self {
            elements: self.elements.into_iter().map(Double::double).collect(),
        }
    }
}

impl<R: Presemiring> Add<&SymmetricTridiagonalMatrix<R>> for SymmetricTridiagonalMatrix<R> {
    type Output = Self;

    fn add(self, rps: &SymmetricTridiagonalMatrix<R>) -> Self::Output {
        debug_assert!(self.rows() == rps.rows() && self.columns() == rps.columns());
        Self {
            elements: zip(self.elements, rps.elements.iter())
                .map(|(l, &r)| l + r)
                .collect(),
        }
    }
}

impl<R: Presemiring> AddAssign<&SymmetricTridiagonalMatrix<R>> for SymmetricTridiagonalMatrix<R> {
    fn add_assign(&mut self, rps: &SymmetricTridiagonalMatrix<R>) {
        debug_assert!(self.rows() == rps.rows() && self.columns() == rps.columns());
        zip(self.elements.iter_mut(), rps.elements.iter()).for_each(|(l, &r)| *l += r);
    }
}

impl<R: Presemiring> Add<SymmetricTridiagonalMatrix<R>> for &SymmetricTridiagonalMatrix<R> {
    type Output = SymmetricTridiagonalMatrix<R>;

    fn add(self, rps: SymmetricTridiagonalMatrix<R>) -> Self::Output {
        debug_assert!(self.rows() == rps.rows() && self.columns() == rps.columns());
        Self::Output {
            elements: zip(self.elements.iter(), rps.elements)
                .map(|(&l, r)| l + r)
                .collect(),
        }
    }
}

impl<R: Presemiring> Add for &SymmetricTridiagonalMatrix<R> {
    type Output = SymmetricTridiagonalMatrix<R>;

    fn add(self, rps: Self) -> Self::Output {
        debug_assert!(self.rows() == rps.rows() && self.columns() == rps.columns());
        Self::Output {
            elements: zip(self.elements.iter(), rps.elements.iter())
                .map(|(&l, &r)| l + r)
                .collect(),
        }
    }
}

impl<R: Ring> Neg for SymmetricTridiagonalMatrix<R> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            elements: self.elements.into_iter().map(Neg::neg).collect(),
        }
    }
}

impl<R: Ring> Neg for &SymmetricTridiagonalMatrix<R> {
    type Output = SymmetricTridiagonalMatrix<R>;

    fn neg(self) -> Self::Output {
        Self::Output {
            elements: self.elements.iter().map(|&e| -e).collect(),
        }
    }
}

impl<R: Ring> Sub for SymmetricTridiagonalMatrix<R> {
    type Output = Self;

    fn sub(self, rps: Self) -> Self::Output {
        debug_assert!(self.rows() == rps.rows() && self.columns() == rps.columns());
        Self {
            elements: zip(self.elements, rps.elements)
                .map(|(l, r)| l - r)
                .collect(),
        }
    }
}

impl<R: Ring> SubAssign for SymmetricTridiagonalMatrix<R> {
    fn sub_assign(&mut self, rps: Self) {
        debug_assert!(self.rows() == rps.rows() && self.columns() == rps.columns());
        zip(self.elements.iter_mut(), rps.elements).for_each(|(l, r)| *l -= r);
    }
}

impl<R: Ring> Sub<&SymmetricTridiagonalMatrix<R>> for SymmetricTridiagonalMatrix<R> {
    type Output = Self;

    fn sub(self, rps: &SymmetricTridiagonalMatrix<R>) -> Self::Output {
        debug_assert!(self.rows() == rps.rows() && self.columns() == rps.columns());
        Self {
            elements: zip(self.elements, rps.elements.iter())
                .map(|(l, &r)| l - r)
                .collect(),
        }
    }
}

impl<R: Ring> SubAssign<&SymmetricTridiagonalMatrix<R>> for SymmetricTridiagonalMatrix<R> {
    fn sub_assign(&mut self, rps: &SymmetricTridiagonalMatrix<R>) {
        debug_assert!(self.rows() == rps.rows() && self.columns() == rps.columns());
        zip(self.elements.iter_mut(), rps.elements.iter()).for_each(|(l, &r)| *l -= r);
    }
}

impl<R: Ring> Sub<SymmetricTridiagonalMatrix<R>> for &SymmetricTridiagonalMatrix<R> {
    type Output = SymmetricTridiagonalMatrix<R>;

    fn sub(self, rps: SymmetricTridiagonalMatrix<R>) -> Self::Output {
        debug_assert!(self.rows() == rps.rows() && self.columns() == rps.columns());
        Self::Output {
            elements: zip(self.elements.iter(), rps.elements)
                .map(|(&l, r)| l - r)
                .collect(),
        }
    }
}

impl<R: Ring> Sub for &SymmetricTridiagonalMatrix<R> {
    type Output = SymmetricTridiagonalMatrix<R>;

    fn sub(self, rps: Self) -> Self::Output {
        debug_assert!(self.rows() == rps.rows() && self.columns() == rps.columns());
        Self::Output {
            elements: zip(self.elements.iter(), rps.elements.iter())
                .map(|(&l, &r)| l - r)
                .collect(),
        }
    }
}

impl<R: Presemiring> Mul<R> for SymmetricTridiagonalMatrix<R> {
    type Output = Self;

    fn mul(self, rps: R) -> Self::Output {
        Self {
            elements: self.elements.into_iter().map(|e| e * rps).collect(),
        }
    }
}

impl<R: Presemiring> MulAssign<R> for SymmetricTridiagonalMatrix<R> {
    fn mul_assign(&mut self, rps: R) {
        self.elements.iter_mut().for_each(|e| *e *= rps);
    }
}

impl<R: Presemiring> Mul<R> for &SymmetricTridiagonalMatrix<R> {
    type Output = SymmetricTridiagonalMatrix<R>;

    fn mul(self, rps: R) -> Self::Output {
        Self::Output {
            elements: self.elements.iter().map(|&e| e * rps).collect(),
        }
    }
}

impl<R: Presemiring> Mul<&DenseVector<R>> for &SymmetricTridiagonalMatrix<R> {
    type Output = DenseVector<R>;

    fn mul(self, rps: &DenseVector<R>) -> Self::Output {
        debug_assert!(self.columns() == rps.dimension());
        (0..self.rows())
            .map(|i| (0..self.columns()).map(|j| self.index(i, j) * rps[j]).sum())
            .collect()
    }
}

impl<R: Presemiring> Mul<&SymmetricTridiagonalMatrix<R>> for &DenseVector<R> {
    type Output = DenseVector<R>;

    fn mul(self, rps: &SymmetricTridiagonalMatrix<R>) -> Self::Output {
        debug_assert!(self.dimension() == rps.rows());
        (0..rps.columns())
            .map(|j| (0..rps.rows()).map(|i| self[i] * rps.index(i, j)).sum())
            .collect()
    }
}
