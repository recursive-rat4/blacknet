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

use crate::algebra::{IntegerRing, PolynomialRing, UnitalRing};
use crate::matrix::{DenseMatrix, DenseVector, SparseVector};
use crate::norm::{EuclideanNorm, InfinityNorm};
use crate::random::UniformGenerator;

// https://www.cs.sjsu.edu/faculty/pollett/masters/Semesters/Spring21/michaela/files/Ajtai96.pdf

/// Ajtai commitment scheme
pub struct AjtaiCommitment<R: UnitalRing> {
    a: DenseMatrix<R>,
}

impl<R: UnitalRing> AjtaiCommitment<R> {
    /// Construct with given setup.
    pub const fn new(a: DenseMatrix<R>) -> Self {
        Self { a }
    }

    /// Short Integer Solution
    pub fn sis(
        g: &mut impl UniformGenerator<Output = R>,
        rows: usize,
        columns: usize,
    ) -> DenseMatrix<R>
    where
        R: IntegerRing,
    {
        DenseMatrix::<R>::new(
            rows,
            columns,
            (0..rows * columns).map(|_| g.generate()).collect(),
        )
    }

    /// Module Short Integer Solution
    pub fn msis<Z: IntegerRing>(
        g: &mut impl UniformGenerator<Output = R>,
        rows: usize,
        columns: usize,
    ) -> DenseMatrix<R>
    where
        R: PolynomialRing<Z>,
    {
        DenseMatrix::<R>::new(
            rows,
            columns,
            (0..rows * columns).map(|_| g.generate()).collect(),
        )
    }

    /// Commit a dense message.
    pub fn commit_dense(&self, m: &DenseVector<R>) -> DenseVector<R> {
        &self.a * m
    }

    /// Commit a sparse message.
    pub fn commit_sparse(&self, m: &SparseVector<R>) -> DenseVector<R> {
        &self.a * m
    }
}

//RUST currently requires std for sqrt, https://github.com/rust-lang/rust/issues/137578
impl<R: UnitalRing + Eq + EuclideanNorm> AjtaiCommitment<R> {
    /// Open commitment under Euclidean norm bound.
    #[cfg(feature = "std")]
    pub fn open_dense_l2(&self, c: &DenseVector<R>, m: &DenseVector<R>, bound: f64) -> bool {
        m.euclidean_norm() < bound && &self.a * m == *c
    }

    /// Open commitment under Euclidean norm bound.
    #[cfg(feature = "std")]
    pub fn open_sparse_l2(&self, c: &DenseVector<R>, m: &SparseVector<R>, bound: f64) -> bool {
        m.euclidean_norm() < bound && &self.a * m == *c
    }
}

impl<R: UnitalRing + Eq> AjtaiCommitment<R> {
    /// Open commitment under infinity norm bound.
    pub fn open_dense_linf<Length: Ord>(
        &self,
        c: &DenseVector<R>,
        m: &DenseVector<R>,
        bound: &Length,
    ) -> bool
    where
        R: InfinityNorm<Length>,
    {
        m.check_infinity_norm(bound) && &self.a * m == *c
    }

    /// Open commitment under infinity norm bound.
    pub fn open_sparse_linf<Length: Ord>(
        &self,
        c: &DenseVector<R>,
        m: &SparseVector<R>,
        bound: &Length,
    ) -> bool
    where
        R: InfinityNorm<Length>,
    {
        m.check_infinity_norm(bound) && &self.a * m == *c
    }
}
