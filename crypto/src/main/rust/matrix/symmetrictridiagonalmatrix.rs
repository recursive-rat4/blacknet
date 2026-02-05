/*
 * Copyright (c) 2025-2026 Pavel Vasin
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

use crate::algebra::{Double, Presemiring, Zero};
use crate::matrix::{DenseMatrix, DenseVector};
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
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct SymmetricTridiagonalMatrix<T: Zero> {
    elements: Vec<T>,
}

impl<T: Zero> SymmetricTridiagonalMatrix<T> {
    /// Construct a new matrix given the concatenation of leading and following diagonals.
    pub const fn new(elements: Vec<T>) -> Self {
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

    fn index(&self, i: usize, j: usize) -> T
    where
        T: Clone,
    {
        if i == j {
            self.elements[i].clone()
        } else if i == j + 1 {
            self.elements[self.columns() + j].clone()
        } else if j == i + 1 {
            self.elements[self.columns() + i].clone()
        } else {
            T::ZERO
        }
    }
}

impl<R: Presemiring> SymmetricTridiagonalMatrix<R> {
    pub fn trace(&self) -> R {
        self.elements[0..self.columns()].iter().sum()
    }

    pub const fn transpose(&self) -> &Self {
        self
    }
}

impl<T: Zero + Clone> From<&SymmetricTridiagonalMatrix<T>> for DenseMatrix<T> {
    fn from(stm: &SymmetricTridiagonalMatrix<T>) -> Self {
        let mut elements = Vec::<T>::with_capacity(stm.rows() * stm.columns());
        for i in 0..stm.rows() {
            for j in 0..stm.columns() {
                elements.push(stm.index(i, j))
            }
        }
        DenseMatrix::new(stm.rows(), stm.columns(), elements)
    }
}

impl<T: Zero + Add<Output = T>> Add for SymmetricTridiagonalMatrix<T> {
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

impl<T: Zero + AddAssign> AddAssign for SymmetricTridiagonalMatrix<T> {
    fn add_assign(&mut self, rps: Self) {
        debug_assert!(self.rows() == rps.rows() && self.columns() == rps.columns());
        zip(self.elements.iter_mut(), rps.elements).for_each(|(l, r)| *l += r);
    }
}

impl<T: Zero + Double<Output = T>> Double for SymmetricTridiagonalMatrix<T> {
    type Output = Self;

    fn double(self) -> Self::Output {
        Self {
            elements: self.elements.into_iter().map(Double::double).collect(),
        }
    }
}

impl<T: Zero + for<'a> Add<&'a T, Output = T>> Add<&SymmetricTridiagonalMatrix<T>>
    for SymmetricTridiagonalMatrix<T>
{
    type Output = Self;

    fn add(self, rps: &SymmetricTridiagonalMatrix<T>) -> Self::Output {
        debug_assert!(self.rows() == rps.rows() && self.columns() == rps.columns());
        Self {
            elements: zip(self.elements, rps.elements.iter())
                .map(|(l, r)| l + r)
                .collect(),
        }
    }
}

impl<T: Zero + for<'a> AddAssign<&'a T>> AddAssign<&SymmetricTridiagonalMatrix<T>>
    for SymmetricTridiagonalMatrix<T>
{
    fn add_assign(&mut self, rps: &SymmetricTridiagonalMatrix<T>) {
        debug_assert!(self.rows() == rps.rows() && self.columns() == rps.columns());
        zip(self.elements.iter_mut(), rps.elements.iter()).for_each(|(l, r)| *l += r);
    }
}

impl<T: Zero> Add<SymmetricTridiagonalMatrix<T>> for &SymmetricTridiagonalMatrix<T>
where
    for<'a> &'a T: Add<T, Output = T>,
{
    type Output = SymmetricTridiagonalMatrix<T>;

    fn add(self, rps: SymmetricTridiagonalMatrix<T>) -> Self::Output {
        debug_assert!(self.rows() == rps.rows() && self.columns() == rps.columns());
        Self::Output {
            elements: zip(self.elements.iter(), rps.elements)
                .map(|(l, r)| l + r)
                .collect(),
        }
    }
}

impl<T: Zero> Add for &SymmetricTridiagonalMatrix<T>
where
    for<'a> &'a T: Add<Output = T>,
{
    type Output = SymmetricTridiagonalMatrix<T>;

    fn add(self, rps: Self) -> Self::Output {
        debug_assert!(self.rows() == rps.rows() && self.columns() == rps.columns());
        Self::Output {
            elements: zip(self.elements.iter(), rps.elements.iter())
                .map(|(l, r)| l + r)
                .collect(),
        }
    }
}

impl<T: Zero + Neg<Output = T>> Neg for SymmetricTridiagonalMatrix<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            elements: self.elements.into_iter().map(Neg::neg).collect(),
        }
    }
}

impl<T: Zero> Neg for &SymmetricTridiagonalMatrix<T>
where
    for<'a> &'a T: Neg<Output = T>,
{
    type Output = SymmetricTridiagonalMatrix<T>;

    fn neg(self) -> Self::Output {
        Self::Output {
            elements: self.elements.iter().map(Neg::neg).collect(),
        }
    }
}

impl<T: Zero + Sub<Output = T>> Sub for SymmetricTridiagonalMatrix<T> {
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

impl<T: Zero + SubAssign> SubAssign for SymmetricTridiagonalMatrix<T> {
    fn sub_assign(&mut self, rps: Self) {
        debug_assert!(self.rows() == rps.rows() && self.columns() == rps.columns());
        zip(self.elements.iter_mut(), rps.elements).for_each(|(l, r)| *l -= r);
    }
}

impl<T: Zero + for<'a> Sub<&'a T, Output = T>> Sub<&SymmetricTridiagonalMatrix<T>>
    for SymmetricTridiagonalMatrix<T>
{
    type Output = Self;

    fn sub(self, rps: &SymmetricTridiagonalMatrix<T>) -> Self::Output {
        debug_assert!(self.rows() == rps.rows() && self.columns() == rps.columns());
        Self {
            elements: zip(self.elements, rps.elements.iter())
                .map(|(l, r)| l - r)
                .collect(),
        }
    }
}

impl<T: Zero + for<'a> SubAssign<&'a T>> SubAssign<&SymmetricTridiagonalMatrix<T>>
    for SymmetricTridiagonalMatrix<T>
{
    fn sub_assign(&mut self, rps: &SymmetricTridiagonalMatrix<T>) {
        debug_assert!(self.rows() == rps.rows() && self.columns() == rps.columns());
        zip(self.elements.iter_mut(), rps.elements.iter()).for_each(|(l, r)| *l -= r);
    }
}

impl<T: Zero> Sub<SymmetricTridiagonalMatrix<T>> for &SymmetricTridiagonalMatrix<T>
where
    for<'a> &'a T: Sub<T, Output = T>,
{
    type Output = SymmetricTridiagonalMatrix<T>;

    fn sub(self, rps: SymmetricTridiagonalMatrix<T>) -> Self::Output {
        debug_assert!(self.rows() == rps.rows() && self.columns() == rps.columns());
        Self::Output {
            elements: zip(self.elements.iter(), rps.elements)
                .map(|(l, r)| l - r)
                .collect(),
        }
    }
}

impl<T: Zero> Sub for &SymmetricTridiagonalMatrix<T>
where
    for<'a> &'a T: Sub<Output = T>,
{
    type Output = SymmetricTridiagonalMatrix<T>;

    fn sub(self, rps: Self) -> Self::Output {
        debug_assert!(self.rows() == rps.rows() && self.columns() == rps.columns());
        Self::Output {
            elements: zip(self.elements.iter(), rps.elements.iter())
                .map(|(l, r)| l - r)
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
