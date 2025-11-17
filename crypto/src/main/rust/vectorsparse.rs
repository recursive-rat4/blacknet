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

use crate::matrixdense::MatrixDense;
use crate::ring::Ring;
use crate::vectordense::VectorDense;
use alloc::vec::Vec;
use core::iter::zip;
use core::ops::{Mul, Neg};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct VectorSparse<R: Ring> {
    dimension: usize,
    index: Vec<usize>,
    elements: Vec<R>,
}

impl<R: Ring> VectorSparse<R> {
    pub const fn new(dimension: usize, index: Vec<usize>, elements: Vec<R>) -> Self {
        Self {
            dimension,
            index,
            elements,
        }
    }

    pub const fn dimension(&self) -> usize {
        self.dimension
    }

    pub const fn elements(&self) -> &Vec<R> {
        &self.elements
    }
}

impl<R: Ring> Neg for VectorSparse<R> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            dimension: self.dimension,
            index: self.index,
            elements: self.elements.into_iter().map(Neg::neg).collect(),
        }
    }
}

impl<R: Ring> Mul<&MatrixDense<R>> for &VectorSparse<R> {
    type Output = VectorDense<R>;

    fn mul(self, rps: &MatrixDense<R>) -> Self::Output {
        let mut v = VectorDense::<R>::fill(rps.columns(), R::ZERO);
        let lps_nnz = self.index.len();
        for i in 0..lps_nnz {
            for j in 0..rps.columns() {
                let row = self.index[i];
                v[j] += self.elements[i] * rps[(row, j)];
            }
        }
        v
    }
}

impl<R: Ring> Mul<&VectorSparse<R>> for &MatrixDense<R> {
    type Output = VectorDense<R>;

    fn mul(self, rps: &VectorSparse<R>) -> Self::Output {
        let mut v = VectorDense::<R>::fill(self.rows(), R::ZERO);
        let rps_nnz = rps.index.len();
        for i in 0..self.rows() {
            for j in 0..rps_nnz {
                let column = rps.index[j];
                v[i] += self[(i, column)] * rps.elements[j];
            }
        }
        v
    }
}

impl<R: Ring> From<&VectorDense<R>> for VectorSparse<R> {
    fn from(dense: &VectorDense<R>) -> Self {
        let mut index = Vec::<usize>::new();
        let mut elements = Vec::<R>::new();
        for i in 0..dense.dimension() {
            let e = dense[i];
            if e != R::ZERO {
                index.push(i);
                elements.push(e);
            }
        }
        VectorSparse::new(dense.dimension(), index, elements)
    }
}

impl<R: Ring> From<&VectorSparse<R>> for VectorDense<R> {
    fn from(sparse: &VectorSparse<R>) -> Self {
        let mut dense = VectorDense::fill(sparse.dimension(), R::ZERO);
        zip(sparse.index.iter(), sparse.elements.iter()).for_each(|(&i, &e)| dense[i] = e);
        dense
    }
}
