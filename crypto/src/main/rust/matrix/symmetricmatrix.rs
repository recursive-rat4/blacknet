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
use alloc::vec;
use alloc::vec::Vec;
use core::iter::zip;
use core::ops::{Add, AddAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign};
use serde::{Deserialize, Serialize};

/// A square matrix that is equal to its transpose.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SymmetricMatrix<R: Presemiring> {
    dimension: usize,
    elements: Vec<R>,
}

impl<R: Presemiring> SymmetricMatrix<R> {
    pub const fn new(dimension: usize, elements: Vec<R>) -> Self {
        Self {
            dimension,
            elements,
        }
    }

    pub fn fill(dimension: usize, element: R) -> Self {
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
    pub const fn elements(&self) -> &Vec<R> {
        &self.elements
    }

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

    const fn size(dimension: usize) -> usize {
        (dimension * (dimension + 1)) >> 1
    }
}

impl<R: Presemiring> From<&SymmetricMatrix<R>> for DenseMatrix<R> {
    fn from(symmetric: &SymmetricMatrix<R>) -> Self {
        let mut elements = Vec::<R>::with_capacity(symmetric.rows() * symmetric.columns());
        for i in 0..symmetric.rows() {
            for j in 0..symmetric.columns() {
                elements.push(symmetric[(i, j)]);
            }
        }
        DenseMatrix::new(symmetric.rows(), symmetric.columns(), elements)
    }
}

impl<R: Presemiring> Index<(usize, usize)> for SymmetricMatrix<R> {
    type Output = R;

    #[inline]
    fn index(&self, (mut i, mut j): (usize, usize)) -> &Self::Output {
        if j > i {
            (i, j) = (j, i);
        }
        &self.elements[Self::size(i) + j]
    }
}

impl<R: Presemiring> IndexMut<(usize, usize)> for SymmetricMatrix<R> {
    #[inline]
    fn index_mut(&mut self, (mut i, mut j): (usize, usize)) -> &mut Self::Output {
        if j > i {
            (i, j) = (j, i);
        }
        &mut self.elements[Self::size(i) + j]
    }
}

impl<R: Presemiring> Add for SymmetricMatrix<R> {
    type Output = Self;

    fn add(self, rps: Self) -> Self::Output {
        Self {
            dimension: self.dimension,
            elements: zip(self.elements, rps.elements)
                .map(|(l, r)| l + r)
                .collect(),
        }
    }
}

impl<R: Presemiring> AddAssign for SymmetricMatrix<R> {
    fn add_assign(&mut self, rps: Self) {
        zip(self.elements.iter_mut(), rps.elements).for_each(|(l, r)| *l += r);
    }
}

impl<R: Presemiring> Double for SymmetricMatrix<R> {
    type Output = Self;

    fn double(self) -> Self::Output {
        Self {
            dimension: self.dimension,
            elements: self.elements.into_iter().map(Double::double).collect(),
        }
    }
}

impl<R: Presemiring> Add<&SymmetricMatrix<R>> for SymmetricMatrix<R> {
    type Output = Self;

    fn add(self, rps: &SymmetricMatrix<R>) -> Self::Output {
        Self {
            dimension: self.dimension,
            elements: zip(self.elements, rps.elements.iter())
                .map(|(l, &r)| l + r)
                .collect(),
        }
    }
}

impl<R: Presemiring> AddAssign<&SymmetricMatrix<R>> for SymmetricMatrix<R> {
    fn add_assign(&mut self, rps: &SymmetricMatrix<R>) {
        zip(self.elements.iter_mut(), rps.elements.iter()).for_each(|(l, &r)| *l += r);
    }
}

impl<R: Presemiring> Add<SymmetricMatrix<R>> for &SymmetricMatrix<R> {
    type Output = SymmetricMatrix<R>;

    fn add(self, rps: SymmetricMatrix<R>) -> Self::Output {
        Self::Output {
            dimension: self.dimension,
            elements: zip(self.elements.iter(), rps.elements)
                .map(|(&l, r)| l + r)
                .collect(),
        }
    }
}

impl<R: Presemiring> Add for &SymmetricMatrix<R> {
    type Output = SymmetricMatrix<R>;

    fn add(self, rps: Self) -> Self::Output {
        Self::Output {
            dimension: self.dimension,
            elements: zip(self.elements.iter(), rps.elements.iter())
                .map(|(&l, &r)| l + r)
                .collect(),
        }
    }
}

impl<R: Ring> Neg for SymmetricMatrix<R> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            dimension: self.dimension,
            elements: self.elements.into_iter().map(Neg::neg).collect(),
        }
    }
}

impl<R: Ring> Neg for &SymmetricMatrix<R> {
    type Output = SymmetricMatrix<R>;

    fn neg(self) -> Self::Output {
        Self::Output {
            dimension: self.dimension,
            elements: self.elements.iter().map(|&e| -e).collect(),
        }
    }
}

impl<R: Ring> Sub for SymmetricMatrix<R> {
    type Output = Self;

    fn sub(self, rps: Self) -> Self::Output {
        Self {
            dimension: self.dimension,
            elements: zip(self.elements, rps.elements)
                .map(|(l, r)| l - r)
                .collect(),
        }
    }
}

impl<R: Ring> SubAssign for SymmetricMatrix<R> {
    fn sub_assign(&mut self, rps: Self) {
        zip(self.elements.iter_mut(), rps.elements).for_each(|(l, r)| *l -= r);
    }
}

impl<R: Ring> Sub<&SymmetricMatrix<R>> for SymmetricMatrix<R> {
    type Output = Self;

    fn sub(self, rps: &SymmetricMatrix<R>) -> Self::Output {
        Self {
            dimension: self.dimension,
            elements: zip(self.elements, rps.elements.iter())
                .map(|(l, &r)| l - r)
                .collect(),
        }
    }
}

impl<R: Ring> SubAssign<&SymmetricMatrix<R>> for SymmetricMatrix<R> {
    fn sub_assign(&mut self, rps: &SymmetricMatrix<R>) {
        zip(self.elements.iter_mut(), rps.elements.iter()).for_each(|(l, &r)| *l -= r);
    }
}

impl<R: Ring> Sub<SymmetricMatrix<R>> for &SymmetricMatrix<R> {
    type Output = SymmetricMatrix<R>;

    fn sub(self, rps: SymmetricMatrix<R>) -> Self::Output {
        Self::Output {
            dimension: self.dimension,
            elements: zip(self.elements.iter(), rps.elements)
                .map(|(&l, r)| l - r)
                .collect(),
        }
    }
}

impl<R: Ring> Sub for &SymmetricMatrix<R> {
    type Output = SymmetricMatrix<R>;

    fn sub(self, rps: Self) -> Self::Output {
        Self::Output {
            dimension: self.dimension,
            elements: zip(self.elements.iter(), rps.elements.iter())
                .map(|(&l, &r)| l - r)
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
        (0..self.rows())
            .map(|i| (0..self.columns()).map(|j| self[(i, j)] * rps[j]).sum())
            .collect()
    }
}

impl<R: Presemiring> Mul<&SymmetricMatrix<R>> for &DenseVector<R> {
    type Output = DenseVector<R>;

    fn mul(self, rps: &SymmetricMatrix<R>) -> Self::Output {
        (0..rps.columns())
            .map(|j| (0..rps.rows()).map(|i| self[i] * rps[(i, j)]).sum())
            .collect()
    }
}
