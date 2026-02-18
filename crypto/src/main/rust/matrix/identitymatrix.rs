/*
 * Copyright (c) 2026 Pavel Vasin
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

use crate::algebra::{One, Tensor, Zero};
use crate::matrix::DenseMatrix;
use alloc::vec;
use core::iter::Sum;
use core::marker::PhantomData;
use serde::{Deserialize, Serialize};

/// The `n ⨉ n` matrix with ones on the leading diagonal and zeros otherwise.
#[derive(Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct IdentityMatrix<T: One + Zero> {
    dimension: usize,
    #[serde(skip)]
    phantom: PhantomData<T>,
}

impl<T: One + Zero> IdentityMatrix<T> {
    /// Construct a new matrix.
    pub const fn new(dimension: usize) -> Self {
        Self {
            dimension,
            phantom: PhantomData,
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

    pub fn trace(&self) -> T
    where
        T: Sum<T>,
    {
        (0..self.dimension).map(|_| T::ONE).sum()
    }
}

impl<T: One + Zero> Clone for IdentityMatrix<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: One + Zero> Copy for IdentityMatrix<T> {}

impl<T: One + Zero + Clone> From<IdentityMatrix<T>> for DenseMatrix<T> {
    fn from(matrix: IdentityMatrix<T>) -> Self {
        let n = matrix.dimension;
        let mut elements = vec![T::ZERO; n * n];
        for i in 0..n {
            elements[i * n + i] = T::ONE;
        }
        Self::new(n, n, elements)
    }
}

impl<T: One + Zero + Clone> Tensor<DenseMatrix<T>> for IdentityMatrix<T> {
    type Output = DenseMatrix<T>;

    #[inline]
    fn tensor(self, rps: DenseMatrix<T>) -> Self::Output {
        self.tensor(&rps)
    }
}

impl<T: One + Zero + Clone> Tensor<&DenseMatrix<T>> for IdentityMatrix<T> {
    type Output = DenseMatrix<T>;

    fn tensor(self, rps: &DenseMatrix<T>) -> Self::Output {
        let rows = self.rows() * rps.rows();
        let columns = self.columns() * rps.columns();
        let mut elements = vec![T::ZERO; rows * columns];
        for i in 0..self.rows() {
            for (j, row) in rps.iter_row().enumerate() {
                let offset = (i * rps.rows() + j) * columns + i * rps.columns();
                elements[offset..offset + rps.columns()].clone_from_slice(row)
            }
        }
        DenseMatrix::new(rows, columns, elements)
    }
}

impl<T: One + Zero + Clone> Tensor<DenseMatrix<T>> for &IdentityMatrix<T> {
    type Output = DenseMatrix<T>;

    #[inline]
    fn tensor(self, rps: DenseMatrix<T>) -> Self::Output {
        (*self).tensor(&rps)
    }
}

impl<T: One + Zero + Clone> Tensor<&DenseMatrix<T>> for &IdentityMatrix<T> {
    type Output = DenseMatrix<T>;

    #[inline]
    fn tensor(self, rps: &DenseMatrix<T>) -> Self::Output {
        (*self).tensor(rps)
    }
}
