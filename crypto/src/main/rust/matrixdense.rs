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

use crate::ring::Ring;
use core::ops::{Index, IndexMut};

#[derive(Debug, Eq, PartialEq)]
pub struct MatrixDense<R: Ring> {
    rows: usize,
    columns: usize,
    elements: Vec<R>,
}

impl<R: Ring> MatrixDense<R> {
    pub const fn new(rows: usize, columns: usize, elements: Vec<R>) -> Self {
        Self {
            rows,
            columns,
            elements,
        }
    }

    pub fn fill(rows: usize, columns: usize, element: R) -> Self {
        Self {
            rows,
            columns,
            elements: vec![element; rows * columns],
        }
    }

    pub const fn rows(&self) -> usize {
        self.rows
    }

    pub const fn columns(&self) -> usize {
        self.columns
    }
}

impl<R: Ring> Index<(usize, usize)> for MatrixDense<R> {
    type Output = R;

    #[inline]
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (row, column) = index;
        &self.elements[row * self.columns + column]
    }
}

impl<R: Ring> IndexMut<(usize, usize)> for MatrixDense<R> {
    #[inline]
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (row, column) = index;
        &mut self.elements[row * self.columns + column]
    }
}
