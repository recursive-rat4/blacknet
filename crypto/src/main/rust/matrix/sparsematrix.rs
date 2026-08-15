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

use crate::algebra::{AdditiveGroup, AdditiveGroupOps, Zero};
use crate::matrix::{DenseMatrix, DenseVector};
use alloc::vec;
use alloc::vec::Vec;
use core::iter::{Sum, repeat_n, zip};
use core::ops::{Mul, Neg};
use serde::{Deserialize, Serialize};

/// A sparse matrix in CSR format.
///
/// <https://arxiv.org/abs/2404.06047>
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SparseMatrix<T: Zero> {
    columns: u32,
    r_index: Vec<u32>,
    c_index: Vec<u32>,
    elements: Vec<T>,
}

impl<T: Zero> SparseMatrix<T> {
    /// Construct a new matrix.
    /// # Safety
    /// Arguments must be valid, and in particular `elements` doesn't contain zeroes.
    pub const unsafe fn new(
        columns: u32,
        r_index: Vec<u32>,
        c_index: Vec<u32>,
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
        let e = *self.r_index.last().expect("Not empty row index");
        Self {
            columns: self.columns.next_power_of_two(),
            r_index: self
                .r_index
                .into_iter()
                .chain(repeat_n(e, n as usize))
                .collect(),
            c_index: self.c_index,
            elements: self.elements,
        }
    }

    /// The number of rows.
    pub const fn rows(&self) -> u32 {
        (self.r_index.len() - 1) as u32
    }

    /// The number of columns.
    pub const fn columns(&self) -> u32 {
        self.columns
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

impl<T: Zero> AsRef<[T]> for SparseMatrix<T> {
    #[inline]
    fn as_ref(&self) -> &[T] {
        &self.elements
    }
}

impl<T: Zero> AsMut<[T]> for SparseMatrix<T> {
    #[inline]
    fn as_mut(&mut self) -> &mut [T] {
        &mut self.elements
    }
}

impl<G: AdditiveGroup> Neg for SparseMatrix<G> {
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

impl<G: AdditiveGroup> Neg for &SparseMatrix<G>
where
    for<'a> &'a G: AdditiveGroupOps<G>,
{
    type Output = SparseMatrix<G>;

    fn neg(self) -> Self::Output {
        Self::Output {
            columns: self.columns,
            r_index: self.r_index.clone(),
            c_index: self.c_index.clone(),
            elements: self.elements.iter().map(Neg::neg).collect(),
        }
    }
}

impl<T: Zero + Sum> Mul<&DenseVector<T>> for &SparseMatrix<T>
where
    for<'a> &'a T: Mul<Output = T>,
{
    type Output = DenseVector<T>;

    fn mul(self, rps: &DenseVector<T>) -> Self::Output {
        debug_assert!(self.columns == rps.dimension());
        self.r_index
            .array_windows::<2>()
            .map(|&[row_start, row_end]| {
                let [row_start, row_end] = [row_start as usize, row_end as usize];
                zip(
                    &self.c_index[row_start..row_end],
                    &self.elements[row_start..row_end],
                )
                .map(|(&j, l)| l * &rps[j])
                .sum()
            })
            .collect()
    }
}

impl<T: Zero + Clone + Eq> From<&DenseMatrix<T>> for SparseMatrix<T> {
    fn from(dense: &DenseMatrix<T>) -> Self {
        let mut builder = SparseMatrixBuilder::<T>::with_dim(dense.rows(), dense.columns());
        for i in 0..dense.rows() {
            for j in 0..dense.columns() {
                let e = &dense[(i, j)];
                builder.column_ref(j, e);
            }
            builder.row();
        }
        builder.build()
    }
}

impl<T: Zero + Clone> From<&SparseMatrix<T>> for DenseMatrix<T> {
    fn from(sparse: &SparseMatrix<T>) -> Self {
        let mut dense = DenseMatrix::<T>::fill(sparse.rows(), sparse.columns(), T::ZERO);
        sparse
            .r_index
            .array_windows::<2>()
            .enumerate()
            .for_each(|(i, &[row_start, row_end])| {
                let [row_start, row_end] = [row_start as usize, row_end as usize];
                zip(
                    &sparse.c_index[row_start..row_end],
                    &sparse.elements[row_start..row_end],
                )
                .for_each(|(&j, e)| dense[(i as u32, j)] = e.clone());
            });
        dense
    }
}

/// Sparse matrix builder accepts entries in the row-major order.
/// Known to be zero entries may be skipped.
pub struct SparseMatrixBuilder<T: Zero> {
    columns: u32,
    r_index: Vec<u32>,
    c_index: Vec<u32>,
    elements: Vec<T>,
}

impl<T: Zero> SparseMatrixBuilder<T> {
    /// Construct a new builder.
    pub fn new() -> Self {
        Self {
            columns: 0,
            r_index: vec![0],
            c_index: Vec::new(),
            elements: Vec::new(),
        }
    }

    /// Construct a new builder.
    pub fn with_dim(rows: u32, columns: u32) -> Self {
        let mut r_index = Vec::with_capacity(rows as usize + 1);
        r_index.push(0);
        Self {
            columns,
            r_index,
            c_index: Vec::new(),
            elements: Vec::new(),
        }
    }

    /// Set the number of columns.
    pub const fn columns(&mut self, columns: u32) {
        self.columns = columns;
    }

    /// Push a next column of current row.
    /// # Safety
    /// `element` is not zero.
    pub unsafe fn column_unchecked(&mut self, column: u32, element: T) {
        self.c_index.push(column);
        self.elements.push(element);
    }

    /// Finish current row.
    pub fn row(&mut self) {
        self.r_index.push(self.elements.len() as u32);
    }

    /// Build the matrix.
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
    /// Push a next column of current row.
    pub fn column(&mut self, column: u32, element: T) {
        if element != T::ZERO {
            unsafe { self.column_unchecked(column, element) };
        }
    }

    /// Push a next column of current row.
    pub fn column_ref(&mut self, column: u32, element: &T)
    where
        T: Clone,
    {
        if *element != T::ZERO {
            unsafe { self.column_unchecked(column, element.clone()) };
        }
    }
}

impl<T: Zero> Default for SparseMatrixBuilder<T> {
    fn default() -> Self {
        Self::new()
    }
}
