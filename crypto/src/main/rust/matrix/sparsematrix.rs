/*
 * Copyright (c) 2024-2026 Pavel Vasin
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

use crate::algebra::{Presemiring, Ring, Zero};
use crate::matrix::{DenseMatrix, DenseVector};
use alloc::vec;
use alloc::vec::Vec;
use core::iter::repeat_n;
use core::ops::{Mul, Neg};
use serde::{Deserialize, Serialize};

/// A sparse matrix in CSR format.
///
/// <https://arxiv.org/abs/2404.06047>
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SparseMatrix<T: Zero> {
    columns: usize,
    r_index: Vec<usize>,
    c_index: Vec<usize>,
    elements: Vec<T>,
}

impl<T: Zero> SparseMatrix<T> {
    /// Construct a new matrix.
    /// # Safety
    /// Arguments must be valid, and in particular `elements` doesn't contain zeroes.
    pub const unsafe fn new(
        columns: usize,
        r_index: Vec<usize>,
        c_index: Vec<usize>,
        elements: Vec<T>,
    ) -> Self {
        Self {
            columns,
            r_index,
            c_index,
            elements,
        }
    }

    pub fn pad_to_power_of_two(self) -> Self {
        let n = self.rows().next_power_of_two() - self.rows();
        let e = *self.r_index.last().expect("Not empty matrix");
        Self {
            columns: self.columns.next_power_of_two(),
            r_index: self.r_index.into_iter().chain(repeat_n(e, n)).collect(),
            c_index: self.c_index,
            elements: self.elements,
        }
    }

    /// The number of rows.
    pub const fn rows(&self) -> usize {
        self.r_index.len() - 1
    }

    /// The number of columns.
    pub const fn columns(&self) -> usize {
        self.columns
    }

    /// The nonzero entries.
    pub const fn elements(&self) -> &Vec<T> {
        &self.elements
    }
}

impl<T: Zero> Default for SparseMatrix<T> {
    fn default() -> Self {
        Self {
            columns: 0,
            r_index: vec![0],
            c_index: Vec::new(),
            elements: Vec::new(),
        }
    }
}

impl<R: Ring> Neg for SparseMatrix<R> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            columns: self.columns,
            r_index: self.r_index,
            c_index: self.c_index,
            elements: self.elements.into_iter().map(Neg::neg).collect(),
        }
    }
}

impl<R: Presemiring> Mul<&DenseVector<R>> for &SparseMatrix<R> {
    type Output = DenseVector<R>;

    fn mul(self, rps: &DenseVector<R>) -> Self::Output {
        (0..self.rows())
            .map(|i| {
                let row_start = self.r_index[i];
                let row_end = self.r_index[i + 1];
                (row_start..row_end)
                    .map(|j| {
                        let column = self.c_index[j];
                        self.elements[j] * rps[column]
                    })
                    .sum()
            })
            .collect()
    }
}

impl<R: Presemiring + Eq> From<&DenseMatrix<R>> for SparseMatrix<R> {
    fn from(dense: &DenseMatrix<R>) -> Self {
        let mut builder = SparseMatrixBuilder::<R>::new(dense.rows(), dense.columns());
        for i in 0..dense.rows() {
            for j in 0..dense.columns() {
                let e = dense[(i, j)];
                if e != R::ZERO {
                    builder.column(j, e);
                }
            }
            builder.row();
        }
        builder.build()
    }
}

impl<R: Presemiring> From<&SparseMatrix<R>> for DenseMatrix<R> {
    fn from(sparse: &SparseMatrix<R>) -> Self {
        let mut dense = DenseMatrix::<R>::fill(sparse.rows(), sparse.columns(), R::ZERO);
        for i in 0..sparse.rows() {
            let row_start = sparse.r_index[i];
            let row_end = sparse.r_index[i + 1];
            for j in row_start..row_end {
                let column = sparse.c_index[j];
                dense[(i, column)] = sparse.elements[j];
            }
        }
        dense
    }
}

pub struct SparseMatrixBuilder<T: Zero> {
    columns: usize,
    r_index: Vec<usize>,
    c_index: Vec<usize>,
    elements: Vec<T>,
}

impl<T: Zero> SparseMatrixBuilder<T> {
    pub fn new(rows: usize, columns: usize) -> Self {
        let mut r_index = Vec::<usize>::with_capacity(rows + 1);
        r_index.push(0);
        Self {
            columns,
            r_index,
            c_index: Vec::new(),
            elements: Vec::new(),
        }
    }

    /// # Safety
    /// `element` is not zero.
    pub unsafe fn column_unchecked(&mut self, column: usize, element: T) {
        self.c_index.push(column);
        self.elements.push(element);
    }

    pub fn row(&mut self) {
        self.r_index.push(self.elements.len());
    }

    pub fn build(self) -> SparseMatrix<T> {
        SparseMatrix {
            columns: self.columns,
            r_index: self.r_index,
            c_index: self.c_index,
            elements: self.elements,
        }
    }
}

impl<T: Zero + Eq> SparseMatrixBuilder<T> {
    pub fn column(&mut self, column: usize, element: T) {
        if element != T::ZERO {
            unsafe { self.column_unchecked(column, element) };
        }
    }
}
