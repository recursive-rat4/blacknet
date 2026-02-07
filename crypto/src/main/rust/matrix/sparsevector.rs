/*
 * Copyright (c) 2025-2026 Pavel Vasin
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

use crate::algebra::{AdditiveGroup, Zero};
use crate::matrix::{DenseMatrix, DenseVector};
use alloc::vec::Vec;
use core::iter::{Sum, zip};
use core::ops::{Mul, Neg};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct SparseVector<T: Zero> {
    dimension: usize,
    index: Vec<usize>,
    elements: Vec<T>,
}

impl<T: Zero> SparseVector<T> {
    pub const fn new(dimension: usize, index: Vec<usize>, elements: Vec<T>) -> Self {
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

    pub const fn elements(&self) -> &Vec<T> {
        &self.elements
    }
}

impl<G: AdditiveGroup> Neg for SparseVector<G> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            dimension: self.dimension,
            index: self.index,
            elements: self.elements.into_iter().map(Neg::neg).collect(),
        }
    }
}

impl<T: Zero + Sum> Mul<&DenseMatrix<T>> for &SparseVector<T>
where
    for<'a> &'a T: Mul<Output = T>,
{
    type Output = DenseVector<T>;

    fn mul(self, rps: &DenseMatrix<T>) -> Self::Output {
        let lps_nnz = self.index.len();
        (0..rps.columns())
            .map(|j| {
                (0..lps_nnz)
                    .map(|i| {
                        let row = self.index[i];
                        &self.elements[i] * &rps[(row, j)]
                    })
                    .sum()
            })
            .collect()
    }
}

impl<T: Zero + Sum> Mul<&SparseVector<T>> for &DenseMatrix<T>
where
    for<'a> &'a T: Mul<Output = T>,
{
    type Output = DenseVector<T>;

    fn mul(self, rps: &SparseVector<T>) -> Self::Output {
        let rps_nnz = rps.index.len();
        (0..self.rows())
            .map(|i| {
                (0..rps_nnz)
                    .map(|j| {
                        let column = rps.index[j];
                        &self[(i, column)] * &rps.elements[j]
                    })
                    .sum()
            })
            .collect()
    }
}

impl<T: Zero + Clone + Eq> From<&DenseVector<T>> for SparseVector<T> {
    fn from(dense: &DenseVector<T>) -> Self {
        let dimension = dense.dimension();
        let mut index = Vec::<usize>::new();
        let mut elements = Vec::<T>::new();
        for i in 0..dimension {
            let e = &dense[i];
            if *e != T::ZERO {
                index.push(i);
                elements.push(e.clone());
            }
        }
        Self {
            dimension,
            index,
            elements,
        }
    }
}

impl<T: Zero + Clone> From<&SparseVector<T>> for DenseVector<T> {
    fn from(sparse: &SparseVector<T>) -> Self {
        let mut dense = DenseVector::fill(sparse.dimension(), T::ZERO);
        zip(sparse.index.iter(), sparse.elements.iter()).for_each(|(&i, e)| dense[i] = e.clone());
        dense
    }
}
