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
use crate::matrix::{DenseMatrix, DenseVector};
use crate::norm::{EuclideanNorm, InfinityNorm, L2, LInf, NormBound};
use crate::random::UniformGenerator;
use core::ops::Mul;

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

impl<R: UnitalRing + Eq, Message: EuclideanNorm> CommitmentScheme<Message>
    for AjtaiCommitment<R, L2, f64>
where
    for<'a, 'b> &'a DenseMatrix<R>: Mul<&'b Message, Output = DenseVector<R>>,
{
    type Commitment = DenseVector<R>;
    type Randomness = ();

    fn commit(&self, m: &Message, _r: &()) -> DenseVector<R> {
        &self.a * m
    }

    fn open(&self, c: &DenseVector<R>, m: &Message, _r: &()) -> bool {
        self.norm_bound.check(m) && &self.a * m == *c
    }
}

impl<R: UnitalRing + Eq, Length: Ord, Message: InfinityNorm<Length>> CommitmentScheme<Message>
    for AjtaiCommitment<R, LInf, Length>
where
    for<'a, 'b> &'a DenseMatrix<R>: Mul<&'b Message, Output = DenseVector<R>>,
{
    type Commitment = DenseVector<R>;
    type Randomness = ();

    fn commit(&self, m: &Message, _r: &()) -> DenseVector<R> {
        &self.a * m
    }

    fn open(&self, c: &DenseVector<R>, m: &Message, _r: &()) -> bool {
        self.norm_bound.check(m) && &self.a * m == *c
    }
}
