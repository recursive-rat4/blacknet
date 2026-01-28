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
use crate::commitmentscheme::CommitmentScheme;
use crate::matrix::{DenseMatrix, DenseVector, SparseVector};
#[cfg(feature = "std")]
use crate::norm::{EuclideanNorm, L2};
use crate::norm::{InfinityNorm, LInf, NormBound};
use crate::random::UniformGenerator;

// https://www.cs.sjsu.edu/faculty/pollett/masters/Semesters/Spring21/michaela/files/Ajtai96.pdf

/// Ajtai commitment scheme
pub struct AjtaiCommitment<R: UnitalRing, Lp, Length> {
    a: DenseMatrix<R>,
    norm_bound: NormBound<Lp, Length>,
}

impl<R: UnitalRing, Lp, Length> AjtaiCommitment<R, Lp, Length> {
    /// Construct with given setup and norm bound.
    pub const fn new(a: DenseMatrix<R>, norm_bound: NormBound<Lp, Length>) -> Self {
        Self { a, norm_bound }
    }

    /// Set another norm bound.
    pub fn set_norm_bound(&mut self, norm_bound: NormBound<Lp, Length>) {
        self.norm_bound = norm_bound;
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
}

//RUST currently requires std for sqrt, https://github.com/rust-lang/rust/issues/137578
#[cfg(feature = "std")]
impl<R: UnitalRing + EuclideanNorm + Eq> CommitmentScheme<DenseVector<R>>
    for AjtaiCommitment<R, L2, f64>
{
    type Commitment = DenseVector<R>;
    type Randomness = ();

    fn commit(&self, m: &DenseVector<R>, _r: &()) -> DenseVector<R> {
        &self.a * m
    }

    fn open(&self, c: &DenseVector<R>, m: &DenseVector<R>, _r: &()) -> bool {
        self.norm_bound.check(m) && &self.a * m == *c
    }
}

#[cfg(feature = "std")]
impl<R: UnitalRing + EuclideanNorm + Eq> CommitmentScheme<SparseVector<R>>
    for AjtaiCommitment<R, L2, f64>
{
    type Commitment = DenseVector<R>;
    type Randomness = ();

    fn commit(&self, m: &SparseVector<R>, _r: &()) -> DenseVector<R> {
        &self.a * m
    }

    fn open(&self, c: &DenseVector<R>, m: &SparseVector<R>, _r: &()) -> bool {
        self.norm_bound.check(m) && &self.a * m == *c
    }
}

impl<R: UnitalRing + InfinityNorm<Length> + Eq, Length: Ord> CommitmentScheme<DenseVector<R>>
    for AjtaiCommitment<R, LInf, Length>
{
    type Commitment = DenseVector<R>;
    type Randomness = ();

    fn commit(&self, m: &DenseVector<R>, _r: &()) -> DenseVector<R> {
        &self.a * m
    }

    fn open(&self, c: &DenseVector<R>, m: &DenseVector<R>, _r: &()) -> bool {
        self.norm_bound.check(m) && &self.a * m == *c
    }
}

impl<R: UnitalRing + InfinityNorm<Length> + Eq, Length: Ord> CommitmentScheme<SparseVector<R>>
    for AjtaiCommitment<R, LInf, Length>
{
    type Commitment = DenseVector<R>;
    type Randomness = ();

    fn commit(&self, m: &SparseVector<R>, _r: &()) -> DenseVector<R> {
        &self.a * m
    }

    fn open(&self, c: &DenseVector<R>, m: &SparseVector<R>, _r: &()) -> bool {
        self.norm_bound.check(m) && &self.a * m == *c
    }
}
