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

use crate::matrixdense::MatrixDense;
use crate::ring::Ring;
use crate::vectordense::VectorDense;
use alloc::vec::Vec;
use core::ops::{Mul, Neg};
use serde::{Deserialize, Serialize};

// https://arxiv.org/abs/2404.06047
// CSR format

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct MatrixSparse<R: Ring> {
    columns: usize,
    r_index: Vec<usize>,
    c_index: Vec<usize>,
    elements: Vec<R>,
}

impl<R: Ring> MatrixSparse<R> {
    pub const fn new(
        columns: usize,
        r_index: Vec<usize>,
        c_index: Vec<usize>,
        elements: Vec<R>,
    ) -> Self {
        Self {
            columns,
            r_index,
            c_index,
            elements,
        }
    }

    pub const fn rows(&self) -> usize {
        self.r_index.len() - 1
    }

    pub const fn columns(&self) -> usize {
        self.columns
    }
}

impl<R: Ring> Neg for MatrixSparse<R> {
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

impl<R: Ring> Mul<&VectorDense<R>> for &MatrixSparse<R> {
    type Output = VectorDense<R>;

    fn mul(self, rps: &VectorDense<R>) -> Self::Output {
        let mut v = VectorDense::<R>::fill(self.rows(), R::ZERO);
        for i in 0..self.rows() {
            let row_start = self.r_index[i];
            let row_end = self.r_index[i + 1];
            for j in row_start..row_end {
                let column = self.c_index[j];
                v[i] += self.elements[j] * rps[column];
            }
        }
        v
    }
}

impl<R: Ring> From<&MatrixDense<R>> for MatrixSparse<R> {
    fn from(dense: &MatrixDense<R>) -> Self {
        let mut builder = MatrixSparseBuilder::<R>::new(dense.rows(), dense.columns());
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

impl<R: Ring> From<&MatrixSparse<R>> for MatrixDense<R> {
    fn from(sparse: &MatrixSparse<R>) -> Self {
        let mut dense = MatrixDense::<R>::fill(sparse.rows(), sparse.columns(), R::ZERO);
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

pub struct MatrixSparseBuilder<R: Ring> {
    columns: usize,
    r_index: Vec<usize>,
    c_index: Vec<usize>,
    elements: Vec<R>,
}

impl<R: Ring> MatrixSparseBuilder<R> {
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

    pub fn column(&mut self, column: usize, element: R) {
        self.c_index.push(column);
        self.elements.push(element);
    }

    pub fn row(&mut self) {
        self.r_index.push(self.elements.len());
    }

    pub fn build(self) -> MatrixSparse<R> {
        MatrixSparse::<R>::new(self.columns, self.r_index, self.c_index, self.elements)
    }
}
