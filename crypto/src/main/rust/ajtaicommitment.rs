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

use crate::distribution::UniformGenerator;
use crate::integer::Integer;
use crate::matrix::{DenseMatrix, DenseVector, SparseVector};
use crate::norm::{EuclideanNorm, InfinityNorm};
use crate::ring::CommutativeRing;

// https://www.cs.sjsu.edu/faculty/pollett/masters/Semesters/Spring21/michaela/files/Ajtai96.pdf

pub struct AjtaiCommitment<R: CommutativeRing> {
    a: DenseMatrix<R>,
}

impl<R: CommutativeRing> AjtaiCommitment<R> {
    pub const fn new(a: DenseMatrix<R>) -> Self {
        Self { a }
    }

    pub fn setup(
        g: &mut impl UniformGenerator<Output = R>,
        rows: usize,
        columns: usize,
    ) -> DenseMatrix<R> {
        DenseMatrix::<R>::new(
            rows,
            columns,
            (0..rows * columns).map(|_| g.generate()).collect(),
        )
    }

    pub fn commit_dense(&self, m: &DenseVector<R>) -> DenseVector<R> {
        &self.a * m
    }

    pub fn commit_sparse(&self, m: &SparseVector<R>) -> DenseVector<R> {
        &self.a * m
    }
}

//RUST currently requires std for sqrt, https://github.com/rust-lang/rust/issues/137578
impl<R: CommutativeRing + Eq + EuclideanNorm> AjtaiCommitment<R> {
    #[cfg(feature = "std")]
    pub fn open_dense_l2(&self, c: &DenseVector<R>, m: &DenseVector<R>, bound: f64) -> bool {
        m.euclidean_norm() < bound && &self.a * m == *c
    }

    #[cfg(feature = "std")]
    pub fn open_sparse_l2(&self, c: &DenseVector<R>, m: &SparseVector<R>, bound: f64) -> bool {
        m.euclidean_norm() < bound && &self.a * m == *c
    }
}

impl<R: CommutativeRing + Eq> AjtaiCommitment<R> {
    pub fn open_dense_linf<Int: Integer>(
        &self,
        c: &DenseVector<R>,
        m: &DenseVector<R>,
        bound: Int,
    ) -> bool
    where
        R: InfinityNorm<Int>,
    {
        m.check_infinity_norm(bound) && &self.a * m == *c
    }

    pub fn open_sparse_linf<Int: Integer>(
        &self,
        c: &DenseVector<R>,
        m: &SparseVector<R>,
        bound: Int,
    ) -> bool
    where
        R: InfinityNorm<Int>,
    {
        m.check_infinity_norm(bound) && &self.a * m == *c
    }
}
