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

use crate::algebra::{IntegerModRing, PolynomialRing, RingOps, UnitalRing};
use crate::branchless::BlSelect;
use crate::commitmentscheme::{BindingCommitmentScheme, CommitmentScheme};
use crate::matrix::{DenseMatrix, DenseVector};
use crate::norm::Norm;
use crate::random::{
    BinaryUniformDistribution, Distribution, UniformBitGenerator, UniformModDistribution,
};
use core::array;
use core::fmt;

/// Ajtai hash.
pub struct AjtaiHash<Lp, R: UnitalRing + Norm<Lp>> {
    a: DenseMatrix<R>,
    bound: <R as Norm<Lp>>::Length,
}

impl<Lp, R: UnitalRing + Norm<Lp>> AjtaiHash<Lp, R> {
    /// Construct with given setup and norm bound.
    pub const fn new(a: DenseMatrix<R>, bound: <R as Norm<Lp>>::Length) -> Self {
        Self { a, bound }
    }

    /// Set another norm bound.
    pub fn set_norm_bound(&mut self, bound: <R as Norm<Lp>>::Length) {
        self.bound = bound;
    }

    /// Short Integer Solution
    pub fn sis<G: UniformBitGenerator>(g: &mut G, m: u32, n: u32) -> DenseMatrix<R>
    where
        R: IntegerModRing,
    {
        let mut umd = UniformModDistribution::<R>::new();
        DenseMatrix::<R>::fill_with(m, n, || umd.sample(g))
    }

    /// Module Short Integer Solution
    pub fn msis<Z: IntegerModRing, const N: usize, G: UniformBitGenerator>(
        g: &mut G,
        m: u32,
        n: u32,
    ) -> DenseMatrix<R>
    where
        R: PolynomialRing<Z> + From<[Z; N]>,
    {
        let mut umd = UniformModDistribution::<Z>::new();
        DenseMatrix::<R>::fill_with(m, n, || R::from(array::from_fn(|_| umd.sample(g))))
    }
}

impl<Lp, R: UnitalRing + Norm<Lp> + Eq> BindingCommitmentScheme<DenseVector<R>> for AjtaiHash<Lp, R>
where
    for<'a> &'a R: RingOps<R>,
    DenseVector<R>: Norm<Lp, Length = R::Length>,
{
    type Commitment = DenseVector<R>;
    type Opening = ();
    type Error = Error;

    fn commit(&self, m: &DenseVector<R>) -> (DenseVector<R>, ()) {
        (&self.a * m, ())
    }

    fn open(&self, c: &DenseVector<R>, m: &DenseVector<R>, _o: &()) -> Result<(), Error> {
        if self.a.columns() != m.dimension() {
            return Err(Error::Dimension);
        }
        if !m.check_norm(&self.bound) {
            return Err(Error::Norm);
        }
        if &self.a * m != *c {
            return Err(Error::Solution);
        }
        Ok(())
    }
}

/// Ajtai commitment scheme.
pub struct AjtaiCommitment<Lp, R: UnitalRing + Norm<Lp>> {
    a: DenseMatrix<R>,
    b: DenseMatrix<R>,
    bound: <R as Norm<Lp>>::Length,
}

impl<Lp, R: UnitalRing + Norm<Lp>> AjtaiCommitment<Lp, R> {
    /// Construct with given setup and norm bound.
    pub fn new(setup: (DenseMatrix<R>, DenseMatrix<R>), bound: <R as Norm<Lp>>::Length) -> Self {
        Self {
            a: setup.0,
            b: setup.1,
            bound,
        }
    }

    /// Set another norm bound.
    pub fn set_norm_bound(&mut self, bound: <R as Norm<Lp>>::Length) {
        self.bound = bound;
    }

    /// Short Integer Solution
    pub fn sis<G: UniformBitGenerator>(
        g: &mut G,
        m: u32,
        n: u32,
        k: u32,
    ) -> (DenseMatrix<R>, DenseMatrix<R>)
    where
        R: IntegerModRing,
    {
        let mut umd = UniformModDistribution::<R>::new();
        (
            DenseMatrix::<R>::fill_with(m, n, || umd.sample(g)),
            DenseMatrix::<R>::fill_with(m, k, || umd.sample(g)),
        )
    }

    /// Module Short Integer Solution
    pub fn msis<Z: IntegerModRing, const N: usize, G: UniformBitGenerator>(
        g: &mut G,
        m: u32,
        n: u32,
        k: u32,
    ) -> (DenseMatrix<R>, DenseMatrix<R>)
    where
        R: PolynomialRing<Z> + From<[Z; N]>,
    {
        let mut umd = UniformModDistribution::<Z>::new();
        (
            DenseMatrix::<R>::fill_with(m, n, || R::from(array::from_fn(|_| umd.sample(g)))),
            DenseMatrix::<R>::fill_with(m, k, || R::from(array::from_fn(|_| umd.sample(g)))),
        )
    }
}

impl<Lp, R: UnitalRing + Norm<Lp> + Eq + BlSelect<Output = R>> CommitmentScheme<DenseVector<R>>
    for AjtaiCommitment<Lp, R>
where
    for<'a> &'a R: RingOps<R>,
    DenseVector<R>: Norm<Lp, Length = R::Length>,
{
    type Commitment = DenseVector<R>;
    type Opening = DenseVector<R>;
    type Error = Error;

    fn commit<RNG: UniformBitGenerator>(
        &self,
        m: &DenseVector<R>,
        rng: &mut RNG,
    ) -> (DenseVector<R>, DenseVector<R>) {
        let mut bud = BinaryUniformDistribution::new();
        let r = DenseVector::<R>::fill_with(self.b.columns(), || bud.sample(rng));
        (&self.a * m + &self.b * &r, r)
    }

    fn open(
        &self,
        c: &DenseVector<R>,
        m: &DenseVector<R>,
        o: &DenseVector<R>,
    ) -> Result<(), Error> {
        if self.a.columns() != m.dimension() || self.b.columns() != o.dimension() {
            return Err(Error::Dimension);
        }
        if !m.check_norm(&self.bound) || !o.check_norm(&self.bound) {
            return Err(Error::Norm);
        }
        if &self.a * m + &self.b * o != *c {
            return Err(Error::Solution);
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum Error {
    Dimension,
    Norm,
    Solution,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Dimension => f.write_str("Dimension mismath"),
            Error::Norm => f.write_str("Norm is out of bound"),
            Error::Solution => f.write_str("Not a solution"),
        }
    }
}

impl core::error::Error for Error {}
