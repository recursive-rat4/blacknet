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

use crate::densematrix::DenseMatrix;
use crate::densevector::DenseVector;
use crate::ring::Ring;
use crate::semiring::Presemiring;
use alloc::vec::Vec;
use core::iter::zip;
use core::ops::{Mul, Neg};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SparseVector<R: Presemiring> {
    dimension: usize,
    index: Vec<usize>,
    elements: Vec<R>,
}

impl<R: Presemiring> SparseVector<R> {
    pub const fn new(dimension: usize, index: Vec<usize>, elements: Vec<R>) -> Self {
        Self {
            dimension,
            index,
            elements,
        }
    }

    pub fn pad_to_power_of_two(self) -> Self {
        Self {
            dimension: self.dimension.next_power_of_two(),
            index: self.index,
            elements: self.elements,
        }
    }

    pub const fn dimension(&self) -> usize {
        self.dimension
    }

    pub const fn elements(&self) -> &Vec<R> {
        &self.elements
    }
}

impl<R: Ring> Neg for SparseVector<R> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            dimension: self.dimension,
            index: self.index,
            elements: self.elements.into_iter().map(Neg::neg).collect(),
        }
    }
}

impl<R: Presemiring> Mul<&DenseMatrix<R>> for &SparseVector<R> {
    type Output = DenseVector<R>;

    fn mul(self, rps: &DenseMatrix<R>) -> Self::Output {
        let mut v = DenseVector::<R>::fill(rps.columns(), R::ZERO);
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

impl<R: Presemiring> Mul<&SparseVector<R>> for &DenseMatrix<R> {
    type Output = DenseVector<R>;

    fn mul(self, rps: &SparseVector<R>) -> Self::Output {
        let mut v = DenseVector::<R>::fill(self.rows(), R::ZERO);
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

impl<R: Presemiring> From<&DenseVector<R>> for SparseVector<R> {
    fn from(dense: &DenseVector<R>) -> Self {
        let mut index = Vec::<usize>::new();
        let mut elements = Vec::<R>::new();
        for i in 0..dense.dimension() {
            let e = dense[i];
            if e != R::ZERO {
                index.push(i);
                elements.push(e);
            }
        }
        SparseVector::new(dense.dimension(), index, elements)
    }
}

impl<R: Presemiring> From<&SparseVector<R>> for DenseVector<R> {
    fn from(sparse: &SparseVector<R>) -> Self {
        let mut dense = DenseVector::fill(sparse.dimension(), R::ZERO);
        zip(sparse.index.iter(), sparse.elements.iter()).for_each(|(&i, &e)| dense[i] = e);
        dense
    }
}
