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

use crate::algebra::{IntegerModRing, PolynomialRing, UnitalRing};
use crate::branchless::BlSelect;
use crate::commitmentscheme::{BindingCommitmentScheme, CommitmentScheme};
use crate::matrix::{DenseMatrix, DenseVector};
use crate::norm::{EuclideanNorm, InfinityNorm, L2, LInf, NormBound};
use crate::random::{
    BinaryUniformDistribution, Distribution, UniformBitGenerator, UniformModDistribution,
};
use core::array;
use core::iter::repeat_with;
use core::ops::Mul;

/// Ajtai hash.
pub struct AjtaiHash<R: UnitalRing, Lp, Length> {
    a: DenseMatrix<R>,
    norm_bound: NormBound<Lp, Length>,
}

impl<R: UnitalRing, Lp, Length> AjtaiHash<R, Lp, Length> {
    /// Construct with given setup and norm bound.
    pub const fn new(a: DenseMatrix<R>, norm_bound: NormBound<Lp, Length>) -> Self {
        Self { a, norm_bound }
    }

    /// Set another norm bound.
    pub fn set_norm_bound(&mut self, norm_bound: NormBound<Lp, Length>) {
        self.norm_bound = norm_bound;
    }

    /// Short Integer Solution
    pub fn sis<G: UniformBitGenerator>(g: &mut G, m: u32, n: u32) -> DenseMatrix<R>
    where
        R: IntegerModRing,
    {
        let mut umd = UniformModDistribution::<R>::new();
        DenseMatrix::<R>::new(
            m,
            n,
            repeat_with(|| umd.sample(g))
                .take(m as usize * n as usize)
                .collect(),
        )
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
        DenseMatrix::<R>::new(
            m,
            n,
            repeat_with(|| R::from(array::from_fn(|_| umd.sample(g))))
                .take(m as usize * n as usize)
                .collect(),
        )
    }
}

impl<R: UnitalRing + Eq, Message: EuclideanNorm> BindingCommitmentScheme<Message>
    for AjtaiHash<R, L2, f64>
where
    for<'a, 'b> &'a DenseMatrix<R>: Mul<&'b Message, Output = DenseVector<R>>,
{
    type Commitment = DenseVector<R>;
    type Opening = ();

    fn commit(&self, m: &Message) -> (DenseVector<R>, ()) {
        (&self.a * m, ())
    }

    fn open(&self, c: &DenseVector<R>, m: &Message, _o: &()) -> bool {
        self.norm_bound.check(m) && &self.a * m == *c
    }
}

impl<R: UnitalRing + Eq, Length: Ord, Message: InfinityNorm<Length>>
    BindingCommitmentScheme<Message> for AjtaiHash<R, LInf, Length>
where
    for<'a, 'b> &'a DenseMatrix<R>: Mul<&'b Message, Output = DenseVector<R>>,
{
    type Commitment = DenseVector<R>;
    type Opening = ();

    fn commit(&self, m: &Message) -> (DenseVector<R>, ()) {
        (&self.a * m, ())
    }

    fn open(&self, c: &DenseVector<R>, m: &Message, _o: &()) -> bool {
        self.norm_bound.check(m) && &self.a * m == *c
    }
}

/// Ajtai commitment scheme.
pub struct AjtaiCommitment<R: UnitalRing, Lp, Length> {
    a: DenseMatrix<R>,
    b: DenseMatrix<R>,
    norm_bound: NormBound<Lp, Length>,
}

impl<R: UnitalRing, Lp, Length> AjtaiCommitment<R, Lp, Length> {
    /// Construct with given setup and norm bound.
    pub fn new(setup: (DenseMatrix<R>, DenseMatrix<R>), norm_bound: NormBound<Lp, Length>) -> Self {
        Self {
            a: setup.0,
            b: setup.1,
            norm_bound,
        }
    }

    /// Set another norm bound.
    pub fn set_norm_bound(&mut self, norm_bound: NormBound<Lp, Length>) {
        self.norm_bound = norm_bound;
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
            DenseMatrix::<R>::new(
                m,
                n,
                repeat_with(|| umd.sample(g))
                    .take(m as usize * n as usize)
                    .collect(),
            ),
            DenseMatrix::<R>::new(
                m,
                k,
                repeat_with(|| umd.sample(g))
                    .take(m as usize * k as usize)
                    .collect(),
            ),
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
            DenseMatrix::<R>::new(
                m,
                n,
                repeat_with(|| R::from(array::from_fn(|_| umd.sample(g))))
                    .take(m as usize * n as usize)
                    .collect(),
            ),
            DenseMatrix::<R>::new(
                m,
                k,
                repeat_with(|| R::from(array::from_fn(|_| umd.sample(g))))
                    .take(m as usize * k as usize)
                    .collect(),
            ),
        )
    }
}

impl<R: UnitalRing + Eq + BlSelect<Output = R>, Message: EuclideanNorm> CommitmentScheme<Message>
    for AjtaiCommitment<R, L2, f64>
where
    for<'a, 'b> &'a DenseMatrix<R>: Mul<&'b Message, Output = DenseVector<R>>,
    for<'a, 'b> &'a DenseMatrix<R>: Mul<&'b DenseVector<R>, Output = DenseVector<R>>,
    DenseVector<R>: EuclideanNorm,
{
    type Commitment = DenseVector<R>;
    type Opening = DenseVector<R>;

    fn commit<RNG: UniformBitGenerator>(
        &self,
        m: &Message,
        rng: &mut RNG,
    ) -> (DenseVector<R>, DenseVector<R>) {
        let mut bud = BinaryUniformDistribution::new();
        let r: DenseVector<R> = repeat_with(|| bud.sample(rng))
            .take(self.b.columns() as usize)
            .collect();
        (&self.a * m + &self.b * &r, r)
    }

    fn open(&self, c: &DenseVector<R>, m: &Message, o: &DenseVector<R>) -> bool {
        self.norm_bound.check(m) && self.norm_bound.check(o) && &self.a * m + &self.b * o == *c
    }
}

impl<R: UnitalRing + Eq + BlSelect<Output = R>, Length: Ord, Message: InfinityNorm<Length>>
    CommitmentScheme<Message> for AjtaiCommitment<R, LInf, Length>
where
    for<'a, 'b> &'a DenseMatrix<R>: Mul<&'b Message, Output = DenseVector<R>>,
    for<'a, 'b> &'a DenseMatrix<R>: Mul<&'b DenseVector<R>, Output = DenseVector<R>>,
    DenseVector<R>: InfinityNorm<Length>,
{
    type Commitment = DenseVector<R>;
    type Opening = DenseVector<R>;

    fn commit<RNG: UniformBitGenerator>(
        &self,
        m: &Message,
        rng: &mut RNG,
    ) -> (DenseVector<R>, DenseVector<R>) {
        let mut bud = BinaryUniformDistribution::new();
        let r: DenseVector<R> = repeat_with(|| bud.sample(rng))
            .take(self.b.columns() as usize)
            .collect();
        (&self.a * m + &self.b * &r, r)
    }

    fn open(&self, c: &DenseVector<R>, m: &Message, o: &DenseVector<R>) -> bool {
        self.norm_bound.check(m) && self.norm_bound.check(o) && &self.a * m + &self.b * o == *c
    }
}
