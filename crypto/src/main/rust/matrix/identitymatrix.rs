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
use serde::{Deserialize, Serialize};

/// The `n × n` matrix with ones on the leading diagonal and zeros otherwise.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct IdentityMatrix {
    dimension: usize,
}

impl IdentityMatrix {
    /// Construct a new matrix.
    pub const fn new(dimension: usize) -> Self {
        Self { dimension }
    }

    /// The number of rows.
    pub const fn rows(&self) -> usize {
        self.dimension
    }

    /// The number of columns.
    pub const fn columns(&self) -> usize {
        self.dimension
    }

    pub fn trace<T: One + Zero + Sum<T>>(&self) -> T {
        (0..self.dimension).map(|_| T::ONE).sum()
    }
}

impl<T: One + Zero + PartialEq> PartialEq<DenseMatrix<T>> for IdentityMatrix {
    #[inline]
    fn eq(&self, rps: &DenseMatrix<T>) -> bool {
        *rps == *self
    }
}

impl<T: One + Zero + PartialEq> PartialEq<IdentityMatrix> for DenseMatrix<T> {
    fn eq(&self, rps: &IdentityMatrix) -> bool {
        if self.rows() != rps.dimension || self.columns() != rps.dimension {
            return false;
        }
        for i in 0..self.rows() {
            for j in 0..self.columns() {
                let e = &self[(i, j)];
                if (i != j && *e != T::ZERO) || (i == j && *e != T::ONE) {
                    return false;
                }
            }
        }
        true
    }
}

impl<T: One + Zero + Clone> From<IdentityMatrix> for DenseMatrix<T> {
    fn from(matrix: IdentityMatrix) -> Self {
        let n = matrix.dimension;
        let mut elements = vec![T::ZERO; n * n];
        for i in 0..n {
            elements[i * n + i] = T::ONE;
        }
        Self::new(n, n, elements)
    }
}

impl<T: One + Zero + Clone> Tensor<DenseMatrix<T>> for IdentityMatrix {
    type Output = DenseMatrix<T>;

    #[inline]
    fn tensor(self, rps: DenseMatrix<T>) -> Self::Output {
        self.tensor(&rps)
    }
}

impl<T: One + Zero + Clone> Tensor<&DenseMatrix<T>> for IdentityMatrix {
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

impl<T: One + Zero + Clone> Tensor<DenseMatrix<T>> for &IdentityMatrix {
    type Output = DenseMatrix<T>;

    #[inline]
    fn tensor(self, rps: DenseMatrix<T>) -> Self::Output {
        (*self).tensor(&rps)
    }
}

impl<T: One + Zero + Clone> Tensor<&DenseMatrix<T>> for &IdentityMatrix {
    type Output = DenseMatrix<T>;

    #[inline]
    fn tensor(self, rps: &DenseMatrix<T>) -> Self::Output {
        (*self).tensor(rps)
    }
}
