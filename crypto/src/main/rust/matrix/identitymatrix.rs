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
use crate::matrix::{DenseMatrix, ScalarMatrix};
use alloc::vec;
use core::iter::Sum;
use serde::{Deserialize, Serialize};

/// The `n × n` matrix with ones on the leading diagonal and zeros otherwise.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct IdentityMatrix {
    dimension: u32,
}

impl IdentityMatrix {
    /// Construct a new matrix.
    pub const fn new(dimension: u32) -> Self {
        Self { dimension }
    }

    /// The number of rows.
    pub const fn rows(&self) -> u32 {
        self.dimension
    }

    /// The number of columns.
    pub const fn columns(&self) -> u32 {
        self.dimension
    }

    pub fn trace<T: One + Sum<T>>(&self) -> T {
        (0..self.dimension).map(|_| T::ONE).sum()
    }
}

impl<T: One + Zero + Clone> From<IdentityMatrix> for DenseMatrix<T> {
    fn from(matrix: IdentityMatrix) -> Self {
        let n = matrix.dimension as usize;
        let mut elements = vec![T::ZERO; n * n];
        for i in 0..n {
            elements[i * n + i] = T::ONE;
        }
        Self::new(n as u32, n as u32, elements)
    }
}

impl<T> Tensor<DenseMatrix<T>> for IdentityMatrix {
    type Output = ScalarMatrix<DenseMatrix<T>>;

    #[inline]
    fn tensor(self, rps: DenseMatrix<T>) -> Self::Output {
        ScalarMatrix::new(self.dimension, rps)
    }
}

impl<T: Clone> Tensor<&DenseMatrix<T>> for IdentityMatrix {
    type Output = ScalarMatrix<DenseMatrix<T>>;

    #[inline]
    fn tensor(self, rps: &DenseMatrix<T>) -> Self::Output {
        self.tensor(rps.clone())
    }
}

impl<T> Tensor<DenseMatrix<T>> for &IdentityMatrix {
    type Output = ScalarMatrix<DenseMatrix<T>>;

    #[inline]
    fn tensor(self, rps: DenseMatrix<T>) -> Self::Output {
        (*self).tensor(rps)
    }
}

impl<T: Clone> Tensor<&DenseMatrix<T>> for &IdentityMatrix {
    type Output = ScalarMatrix<DenseMatrix<T>>;

    #[inline]
    fn tensor(self, rps: &DenseMatrix<T>) -> Self::Output {
        (*self).tensor(rps.clone())
    }
}
