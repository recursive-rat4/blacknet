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

use crate::algebra::{Double, Presemiring};
use crate::matrix::{DenseMatrix, DenseVector};
use alloc::vec;
use alloc::vec::Vec;
use core::iter::zip;
use core::ops::{Add, AddAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign};
use serde::{Deserialize, Serialize};

/// A square matrix that is equal to its transpose.
///
/// # Panics
///
/// In debug builds, panic on incompatible dimensions.
///
/// # Safety
///
/// In release builds, undefined behaviour on incompatible dimensions.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct SymmetricMatrix<T> {
    dimension: usize,
    elements: Vec<T>,
}

impl<T> SymmetricMatrix<T> {
    /// Construct a new matrix given the lower triangular entries.
    pub const fn new(dimension: usize, elements: Vec<T>) -> Self {
        debug_assert!(Self::size(dimension) == elements.len());
        Self {
            dimension,
            elements,
        }
    }

    /// Fill a new `n ⨉ n` matrix with a single `element`.
    pub fn fill(dimension: usize, element: T) -> Self
    where
        T: Clone,
    {
        Self {
            dimension,
            elements: vec![element; Self::size(dimension)],
        }
    }

    /// The number of rows.
    pub const fn rows(&self) -> usize {
        self.dimension
    }

    /// The number of columns.
    pub const fn columns(&self) -> usize {
        self.dimension
    }

    /// The lower triangular entries.
    pub const fn elements(&self) -> &Vec<T> {
        &self.elements
    }

    const fn size(dimension: usize) -> usize {
        (dimension * (dimension + 1)) >> 1
    }
}

impl<R: Presemiring> SymmetricMatrix<R> {
    pub fn trace(&self) -> R {
        let mut sigma = R::ZERO;
        for i in 0..self.dimension {
            sigma += self[(i, i)]
        }
        sigma
    }

    pub const fn transpose(&self) -> &Self {
        self
    }
}

impl<T: Clone> From<&SymmetricMatrix<T>> for DenseMatrix<T> {
    fn from(symmetric: &SymmetricMatrix<T>) -> Self {
        let mut elements = Vec::<T>::with_capacity(symmetric.rows() * symmetric.columns());
        for i in 0..symmetric.rows() {
            for j in 0..symmetric.columns() {
                elements.push(symmetric[(i, j)].clone());
            }
        }
        DenseMatrix::new(symmetric.rows(), symmetric.columns(), elements)
    }
}

impl<T> Index<(usize, usize)> for SymmetricMatrix<T> {
    type Output = T;

    #[inline]
    fn index(&self, (mut i, mut j): (usize, usize)) -> &Self::Output {
        if j > i {
            (i, j) = (j, i);
        }
        &self.elements[Self::size(i) + j]
    }
}

impl<T> IndexMut<(usize, usize)> for SymmetricMatrix<T> {
    #[inline]
    fn index_mut(&mut self, (mut i, mut j): (usize, usize)) -> &mut Self::Output {
        if j > i {
            (i, j) = (j, i);
        }
        &mut self.elements[Self::size(i) + j]
    }
}

impl<T: Add<Output = T>> Add for SymmetricMatrix<T> {
    type Output = Self;

    fn add(self, rps: Self) -> Self::Output {
        debug_assert!(self.dimension == rps.dimension);
        Self {
            dimension: self.dimension,
            elements: zip(self.elements, rps.elements)
                .map(|(l, r)| l + r)
                .collect(),
        }
    }
}

impl<T: AddAssign> AddAssign for SymmetricMatrix<T> {
    fn add_assign(&mut self, rps: Self) {
        debug_assert!(self.dimension == rps.dimension);
        zip(self.elements.iter_mut(), rps.elements).for_each(|(l, r)| *l += r);
    }
}

impl<T: Double<Output = T>> Double for SymmetricMatrix<T> {
    type Output = Self;

    fn double(self) -> Self::Output {
        Self {
            dimension: self.dimension,
            elements: self.elements.into_iter().map(Double::double).collect(),
        }
    }
}

impl<T: for<'a> Add<&'a T, Output = T>> Add<&SymmetricMatrix<T>> for SymmetricMatrix<T> {
    type Output = Self;

    fn add(self, rps: &SymmetricMatrix<T>) -> Self::Output {
        debug_assert!(self.dimension == rps.dimension);
        Self {
            dimension: self.dimension,
            elements: zip(self.elements, rps.elements.iter())
                .map(|(l, r)| l + r)
                .collect(),
        }
    }
}

impl<T: for<'a> AddAssign<&'a T>> AddAssign<&SymmetricMatrix<T>> for SymmetricMatrix<T> {
    fn add_assign(&mut self, rps: &SymmetricMatrix<T>) {
        debug_assert!(self.dimension == rps.dimension);
        zip(self.elements.iter_mut(), rps.elements.iter()).for_each(|(l, r)| *l += r);
    }
}

impl<T> Add<SymmetricMatrix<T>> for &SymmetricMatrix<T>
where
    for<'a> &'a T: Add<T, Output = T>,
{
    type Output = SymmetricMatrix<T>;

    fn add(self, rps: SymmetricMatrix<T>) -> Self::Output {
        debug_assert!(self.dimension == rps.dimension);
        Self::Output {
            dimension: self.dimension,
            elements: zip(self.elements.iter(), rps.elements)
                .map(|(l, r)| l + r)
                .collect(),
        }
    }
}

impl<T> Add for &SymmetricMatrix<T>
where
    for<'a> &'a T: Add<Output = T>,
{
    type Output = SymmetricMatrix<T>;

    fn add(self, rps: Self) -> Self::Output {
        debug_assert!(self.dimension == rps.dimension);
        Self::Output {
            dimension: self.dimension,
            elements: zip(self.elements.iter(), rps.elements.iter())
                .map(|(l, r)| l + r)
                .collect(),
        }
    }
}

impl<T: Neg<Output = T>> Neg for SymmetricMatrix<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            dimension: self.dimension,
            elements: self.elements.into_iter().map(Neg::neg).collect(),
        }
    }
}

impl<T> Neg for &SymmetricMatrix<T>
where
    for<'a> &'a T: Neg<Output = T>,
{
    type Output = SymmetricMatrix<T>;

    fn neg(self) -> Self::Output {
        Self::Output {
            dimension: self.dimension,
            elements: self.elements.iter().map(Neg::neg).collect(),
        }
    }
}

impl<T: Sub<Output = T>> Sub for SymmetricMatrix<T> {
    type Output = Self;

    fn sub(self, rps: Self) -> Self::Output {
        debug_assert!(self.dimension == rps.dimension);
        Self {
            dimension: self.dimension,
            elements: zip(self.elements, rps.elements)
                .map(|(l, r)| l - r)
                .collect(),
        }
    }
}

impl<T: SubAssign> SubAssign for SymmetricMatrix<T> {
    fn sub_assign(&mut self, rps: Self) {
        debug_assert!(self.dimension == rps.dimension);
        zip(self.elements.iter_mut(), rps.elements).for_each(|(l, r)| *l -= r);
    }
}

impl<T: for<'a> Sub<&'a T, Output = T>> Sub<&SymmetricMatrix<T>> for SymmetricMatrix<T> {
    type Output = Self;

    fn sub(self, rps: &SymmetricMatrix<T>) -> Self::Output {
        debug_assert!(self.dimension == rps.dimension);
        Self {
            dimension: self.dimension,
            elements: zip(self.elements, rps.elements.iter())
                .map(|(l, r)| l - r)
                .collect(),
        }
    }
}

impl<T: for<'a> SubAssign<&'a T>> SubAssign<&SymmetricMatrix<T>> for SymmetricMatrix<T> {
    fn sub_assign(&mut self, rps: &SymmetricMatrix<T>) {
        debug_assert!(self.dimension == rps.dimension);
        zip(self.elements.iter_mut(), rps.elements.iter()).for_each(|(l, r)| *l -= r);
    }
}

impl<T> Sub<SymmetricMatrix<T>> for &SymmetricMatrix<T>
where
    for<'a> &'a T: Sub<T, Output = T>,
{
    type Output = SymmetricMatrix<T>;

    fn sub(self, rps: SymmetricMatrix<T>) -> Self::Output {
        debug_assert!(self.dimension == rps.dimension);
        Self::Output {
            dimension: self.dimension,
            elements: zip(self.elements.iter(), rps.elements)
                .map(|(l, r)| l - r)
                .collect(),
        }
    }
}

impl<T> Sub for &SymmetricMatrix<T>
where
    for<'a> &'a T: Sub<Output = T>,
{
    type Output = SymmetricMatrix<T>;

    fn sub(self, rps: Self) -> Self::Output {
        debug_assert!(self.dimension == rps.dimension);
        Self::Output {
            dimension: self.dimension,
            elements: zip(self.elements.iter(), rps.elements.iter())
                .map(|(l, r)| l - r)
                .collect(),
        }
    }
}

impl<R: Presemiring> Mul<R> for SymmetricMatrix<R> {
    type Output = Self;

    fn mul(self, rps: R) -> Self::Output {
        Self {
            dimension: self.dimension,
            elements: self.elements.into_iter().map(|e| e * rps).collect(),
        }
    }
}

impl<R: Presemiring> MulAssign<R> for SymmetricMatrix<R> {
    fn mul_assign(&mut self, rps: R) {
        self.elements.iter_mut().for_each(|e| *e *= rps);
    }
}

impl<R: Presemiring> Mul<R> for &SymmetricMatrix<R> {
    type Output = SymmetricMatrix<R>;

    fn mul(self, rps: R) -> Self::Output {
        Self::Output {
            dimension: self.dimension,
            elements: self.elements.iter().map(|&e| e * rps).collect(),
        }
    }
}

impl<R: Presemiring> Mul<&DenseVector<R>> for &SymmetricMatrix<R> {
    type Output = DenseVector<R>;

    fn mul(self, rps: &DenseVector<R>) -> Self::Output {
        debug_assert!(self.dimension == rps.dimension());
        (0..self.rows())
            .map(|i| (0..self.columns()).map(|j| self[(i, j)] * rps[j]).sum())
            .collect()
    }
}

impl<R: Presemiring> Mul<&SymmetricMatrix<R>> for &DenseVector<R> {
    type Output = DenseVector<R>;

    fn mul(self, rps: &SymmetricMatrix<R>) -> Self::Output {
        debug_assert!(self.dimension() == rps.dimension);
        (0..rps.columns())
            .map(|j| (0..rps.rows()).map(|i| self[i] * rps[(i, j)]).sum())
            .collect()
    }
}
