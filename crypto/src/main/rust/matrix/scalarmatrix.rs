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

use crate::algebra::Zero;
use crate::matrix::{DenseMatrix, DenseVector};
use alloc::vec;
use alloc::vec::Vec;
use core::iter::{Sum, repeat_n, zip};
use core::ops::Mul;
use serde::{Deserialize, Serialize};

/// A `n × n` matrix with equal entries on the leading diagonal and empty otherwise..
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct ScalarMatrix<T> {
    dimension: usize,
    scalar: T,
}

impl<T> ScalarMatrix<T> {
    /// Construct a new matrix.
    pub const fn new(dimension: usize, scalar: T) -> Self {
        Self { dimension, scalar }
    }

    pub fn pad_to_power_of_two(self) -> Self {
        Self {
            dimension: self.dimension.next_power_of_two(),
            scalar: self.scalar,
        }
    }

    /// A number of rows.
    pub const fn rows(&self) -> usize {
        self.dimension
    }

    /// A number of columns.
    pub const fn columns(&self) -> usize {
        self.dimension
    }

    pub fn trace(&self) -> T
    where
        T: for<'a> Sum<&'a T>,
    {
        repeat_n(&self.scalar, self.dimension).sum()
    }
}

impl<T: Clone> ScalarMatrix<DenseMatrix<T>> {
    pub fn transpose(&self) -> Self {
        Self {
            dimension: self.dimension,
            scalar: self.scalar.transpose(),
        }
    }
}

impl<T> From<ScalarMatrix<T>> for (usize, T) {
    fn from(matrix: ScalarMatrix<T>) -> Self {
        (matrix.dimension, matrix.scalar)
    }
}

impl<T: Zero + Clone> From<&ScalarMatrix<T>> for DenseMatrix<T> {
    fn from(matrix: &ScalarMatrix<T>) -> Self {
        let n = matrix.dimension;
        let mut elements = vec![T::ZERO; n * n];
        for i in 0..n {
            elements[i * n + i] = matrix.scalar.clone();
        }
        Self::new(n, n, elements)
    }
}

impl<T: Sum> Mul<&ScalarMatrix<DenseMatrix<T>>> for &DenseMatrix<T>
where
    for<'a> &'a T: Mul<Output = T>,
{
    type Output = DenseMatrix<T>;

    fn mul(self, rps: &ScalarMatrix<DenseMatrix<T>>) -> Self::Output {
        debug_assert!(self.columns() == rps.dimension * rps.scalar.rows());
        let rps_columns = rps.dimension * rps.scalar.columns();
        // Iterative algorithm
        let mut elements = Vec::<T>::with_capacity(self.rows() * rps_columns);
        for row in self.iter_row() {
            for i in 0..rps.dimension {
                for j in 0..rps.scalar.columns() {
                    elements.push(
                        zip(
                            row.iter().skip(i * rps.scalar.rows()),
                            rps.scalar.iter_row().map(|row| &row[j]),
                        )
                        .map(|(l, r)| l * r)
                        .sum(),
                    )
                }
            }
        }
        DenseMatrix::new(self.rows(), rps_columns, elements)
    }
}

impl<T: Sum> Mul<&DenseVector<T>> for &ScalarMatrix<DenseMatrix<T>>
where
    for<'a> &'a T: Mul<Output = T>,
{
    type Output = DenseVector<T>;

    fn mul(self, rps: &DenseVector<T>) -> Self::Output {
        debug_assert!(self.dimension * self.scalar.columns() == rps.dimension());
        let lps = repeat_n(&self.scalar, self.dimension);
        let rps = rps.chunks_exact(self.scalar.columns());
        zip(lps, rps)
            .flat_map(|(lps, rps)| {
                lps.iter_row()
                    .map(move |row| zip(row, rps).map(|(l, r)| l * r).sum())
            })
            .collect()
    }
}
