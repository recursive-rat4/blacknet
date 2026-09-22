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

use crate::algebra::{One, Zero};
use crate::matrix::{DenseVector, SparseMatrix};
use alloc::vec;
use alloc::vec::Vec;
use core::iter::{Sum, repeat_n};
use core::ops::Mul;
use serde::{Deserialize, Serialize};

/// A sparse matrix in CSR format over subset `{0, 1}`.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SparseBinaryMatrix {
    columns: u32,
    r_index: Vec<u32>,
    c_index: Vec<u32>,
}

impl SparseBinaryMatrix {
    /// Construct a new matrix.
    /// # Safety
    /// Arguments must be valid.
    pub const unsafe fn new(columns: u32, r_index: Vec<u32>, c_index: Vec<u32>) -> Self {
        Self {
            columns,
            r_index,
            c_index,
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

    /// The row index.
    pub fn r_index(&self) -> &[u32] {
        &self.r_index
    }

    /// The column index.
    pub fn c_index(&self) -> &[u32] {
        &self.c_index
    }
}

impl Default for SparseBinaryMatrix {
    fn default() -> Self {
        Self {
            columns: 0,
            r_index: vec![0],
            c_index: Vec::new(),
        }
    }
}

impl<T: for<'a> Sum<&'a T>> Mul<&DenseVector<T>> for &SparseBinaryMatrix {
    type Output = DenseVector<T>;

    fn mul(self, rps: &DenseVector<T>) -> Self::Output {
        debug_assert!(self.columns == rps.dimension());
        self.r_index
            .array_windows::<2>()
            .map(|&[row_start, row_end]| {
                let [row_start, row_end] = [row_start as usize, row_end as usize];
                self.c_index[row_start..row_end]
                    .iter()
                    .map(|&j| &rps[j])
                    .sum()
            })
            .collect()
    }
}

impl<T: One + Zero + Eq> TryFrom<&SparseMatrix<T>> for SparseBinaryMatrix {
    type Error = ();

    fn try_from(sparse: &SparseMatrix<T>) -> Result<Self, Self::Error> {
        let elements: &[T] = sparse.as_ref();
        if elements.iter().all(|e| *e == T::ONE) {
            Ok(unsafe {
                Self::new(
                    sparse.columns(),
                    sparse.r_index().to_vec(),
                    sparse.c_index().to_vec(),
                )
            })
        } else {
            Err(())
        }
    }
}

impl<T: One + Zero + Clone> From<SparseBinaryMatrix> for SparseMatrix<T> {
    fn from(binary: SparseBinaryMatrix) -> Self {
        let elements = vec![T::ONE; binary.c_index.len()];
        unsafe { SparseMatrix::<T>::new(binary.columns, binary.r_index, binary.c_index, elements) }
    }
}

/// Sparse binary matrix builder accepts entries in the row-major order.
/// Known to be zero entries may be skipped.
pub struct SparseBinaryMatrixBuilder {
    columns: u32,
    r_index: Vec<u32>,
    c_index: Vec<u32>,
}

impl SparseBinaryMatrixBuilder {
    /// Construct a new builder.
    pub fn new() -> Self {
        Self {
            columns: 0,
            r_index: vec![0],
            c_index: Vec::new(),
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
        }
    }

    /// Set the number of columns.
    pub const fn columns(&mut self, columns: u32) {
        self.columns = columns;
    }

    /// Push a next column of current row.
    /// # Safety
    /// The element is one.
    pub unsafe fn column_unchecked(&mut self, column: u32) {
        self.c_index.push(column);
    }

    /// Finish current row.
    pub fn row(&mut self) {
        self.r_index.push(self.c_index.len() as u32);
    }

    /// Build the matrix.
    pub fn build(self) -> SparseBinaryMatrix {
        SparseBinaryMatrix {
            columns: self.columns,
            r_index: self.r_index,
            c_index: self.c_index,
        }
    }
}

impl SparseBinaryMatrixBuilder {
    /// Push a next column of current row.
    /// # Safety
    /// The element is one or zero.
    pub fn column<T: One + Eq>(&mut self, column: u32, element: &T) {
        if *element == T::ONE {
            unsafe { self.column_unchecked(column) };
        }
    }
}

impl Default for SparseBinaryMatrixBuilder {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}
