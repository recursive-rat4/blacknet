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

use crate::algebra::{AdditiveGroup, AdditiveGroupOps, Concat, Zero};
use crate::matrix::{DenseMatrix, DenseVector};
use alloc::vec::Vec;
use core::iter::{Sum, zip};
use core::ops::{Mul, Neg};
use serde::{Deserialize, Serialize};

/// A sparse vector.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct SparseVector<T: Zero> {
    dimension: u32,
    index: Vec<u32>,
    elements: Vec<T>,
}

impl<T: Zero> SparseVector<T> {
    pub const fn new(dimension: u32, index: Vec<u32>, elements: Vec<T>) -> Self {
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

    pub const fn dimension(&self) -> u32 {
        self.dimension
    }
}

impl<T: Zero> AsRef<[T]> for SparseVector<T> {
    #[inline]
    fn as_ref(&self) -> &[T] {
        &self.elements
    }
}

impl<T: Zero> AsMut<[T]> for SparseVector<T> {
    #[inline]
    fn as_mut(&mut self) -> &mut [T] {
        &mut self.elements
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

impl<G: AdditiveGroup> Neg for &SparseVector<G>
where
    for<'a> &'a G: AdditiveGroupOps<G>,
{
    type Output = SparseVector<G>;

    fn neg(self) -> Self::Output {
        Self::Output {
            dimension: self.dimension,
            index: self.index.clone(),
            elements: self.elements.iter().map(Neg::neg).collect(),
        }
    }
}

impl<T: Zero + Sum> Mul<&DenseMatrix<T>> for &SparseVector<T>
where
    for<'a> &'a T: Mul<Output = T>,
{
    type Output = DenseVector<T>;

    fn mul(self, rps: &DenseMatrix<T>) -> Self::Output {
        debug_assert!(self.dimension == rps.rows());
        (0..rps.columns())
            .map(|j| {
                zip(&self.index, &self.elements)
                    .map(|(&i, l)| l * &rps[(i, j)])
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
        debug_assert!(self.columns() == rps.dimension);
        self.iter_row()
            .map(|row| {
                zip(&rps.index, &rps.elements)
                    .map(|(&j, r)| &row[j as usize] * r)
                    .sum()
            })
            .collect()
    }
}

impl<T: Zero + Clone + Eq> From<&DenseVector<T>> for SparseVector<T> {
    fn from(dense: &DenseVector<T>) -> Self {
        let dimension = dense.dimension();
        let mut index = Vec::new();
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
        zip(&sparse.index, &sparse.elements).for_each(|(&i, e)| dense[i] = e.clone());
        dense
    }
}

impl<T: Zero> Concat for SparseVector<T> {
    type Output = Self;

    fn concat(self, mut rps: Self) -> Self::Output {
        let dimension = self.dimension + rps.dimension;
        let mut index = self.index;
        index.reserve(rps.dimension as usize);
        for i in rps.index {
            index.push(self.dimension + i)
        }
        let mut elements = self.elements;
        elements.append(&mut rps.elements);
        Self {
            dimension,
            index,
            elements,
        }
    }
}

impl<T: Zero + Clone> Concat<&Self> for SparseVector<T> {
    type Output = Self;

    fn concat(self, rps: &Self) -> Self::Output {
        let dimension = self.dimension + rps.dimension;
        let mut index = self.index;
        index.reserve(rps.dimension as usize);
        for i in &rps.index {
            index.push(self.dimension + i)
        }
        let mut elements = self.elements;
        elements.extend_from_slice(&rps.elements);
        Self {
            dimension,
            index,
            elements,
        }
    }
}

impl<T: Zero + Clone> Concat<SparseVector<T>> for &SparseVector<T> {
    type Output = SparseVector<T>;

    fn concat(self, mut rps: SparseVector<T>) -> Self::Output {
        let dimension = self.dimension + rps.dimension;
        let nnz = self.index.len() + rps.index.len();
        let mut index = Vec::with_capacity(nnz);
        index.extend_from_slice(&self.index);
        for i in rps.index {
            index.push(self.dimension + i)
        }
        let mut elements = Vec::<T>::with_capacity(nnz);
        elements.extend_from_slice(&self.elements);
        elements.append(&mut rps.elements);
        Self::Output {
            dimension,
            index,
            elements,
        }
    }
}

impl<T: Zero + Clone> Concat for &SparseVector<T> {
    type Output = SparseVector<T>;

    fn concat(self, rps: Self) -> Self::Output {
        let dimension = self.dimension + rps.dimension;
        let nnz = self.index.len() + rps.index.len();
        let mut index = Vec::with_capacity(nnz);
        index.extend_from_slice(&self.index);
        for i in &rps.index {
            index.push(self.dimension + i)
        }
        let mut elements = Vec::<T>::with_capacity(nnz);
        elements.extend_from_slice(&self.elements);
        elements.extend_from_slice(&rps.elements);
        Self::Output {
            dimension,
            index,
            elements,
        }
    }
}
