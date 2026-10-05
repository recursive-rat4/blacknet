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

use crate::algebra::{Double, RingOps, SemiringOps, Square, UnitalRing, UnitalSemiring};
use crate::circuit::builder::{
    Constant, Expression, LinearCombination, LinearMonoid, LinearSpan, Variable,
};
use alloc::vec;
use core::ops::{Add, Mul, MulAssign, Neg, Sub};

/// A linear term is a pair of a variable and a coefficient.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LinearTerm<R: UnitalSemiring> {
    pub(super) variable: Variable<R>,
    pub(super) coefficient: Constant<R>,
}

impl<R: UnitalSemiring> LinearTerm<R> {
    /// Construct a new linear term.
    pub const fn new(variable: Variable<R>, coefficient: Constant<R>) -> Self {
        Self {
            variable,
            coefficient,
        }
    }
}

impl<R: UnitalSemiring + Eq> Expression<R> for LinearTerm<R> {
    fn span(self) -> LinearSpan<R> {
        vec![self.into()].into()
    }

    fn degree(&self) -> u32 {
        if self.coefficient != Constant::ZERO {
            1
        } else {
            0
        }
    }
}

impl<R: UnitalSemiring> From<(Variable<R>, Constant<R>)> for LinearTerm<R> {
    fn from(pair: (Variable<R>, Constant<R>)) -> Self {
        Self {
            variable: pair.0,
            coefficient: pair.1,
        }
    }
}

impl<R: UnitalSemiring + Eq> Add for LinearTerm<R> {
    type Output = LinearCombination<R>;

    fn add(self, rps: Self) -> Self::Output {
        LinearCombination::with_terms([self, rps])
    }
}

impl<R: UnitalSemiring + Clone + Eq> Add<&Self> for LinearTerm<R> {
    type Output = LinearCombination<R>;

    fn add(self, rps: &Self) -> Self::Output {
        self + rps.clone()
    }
}

impl<R: UnitalSemiring + Clone + Eq> Add<LinearTerm<R>> for &LinearTerm<R> {
    type Output = LinearCombination<R>;

    fn add(self, rps: LinearTerm<R>) -> Self::Output {
        self.clone() + rps
    }
}

impl<R: UnitalSemiring + Clone + Eq> Add for &LinearTerm<R> {
    type Output = LinearCombination<R>;

    fn add(self, rps: Self) -> Self::Output {
        self.clone() + rps.clone()
    }
}

impl<R: UnitalSemiring> Double for LinearTerm<R> {
    type Output = Self;

    fn double(self) -> Self::Output {
        Self {
            variable: self.variable,
            coefficient: self.coefficient.double(),
        }
    }
}

impl<R: UnitalSemiring> Double for &LinearTerm<R>
where
    for<'a> &'a R: SemiringOps<R>,
{
    type Output = LinearTerm<R>;

    fn double(self) -> Self::Output {
        Self::Output {
            variable: self.variable,
            coefficient: (&self.coefficient).double(),
        }
    }
}

impl<R: UnitalRing> Neg for LinearTerm<R> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            variable: self.variable,
            coefficient: -self.coefficient,
        }
    }
}

impl<R: UnitalRing> Neg for &LinearTerm<R>
where
    for<'a> &'a R: RingOps<R>,
{
    type Output = LinearTerm<R>;

    fn neg(self) -> Self::Output {
        Self::Output {
            variable: self.variable,
            coefficient: -&self.coefficient,
        }
    }
}

impl<R: UnitalRing + Eq> Sub for LinearTerm<R> {
    type Output = LinearCombination<R>;

    fn sub(self, rps: Self) -> Self::Output {
        LinearCombination::with_terms([self, -rps])
    }
}

impl<R: UnitalRing + Clone + Eq> Sub<&Self> for LinearTerm<R> {
    type Output = LinearCombination<R>;

    fn sub(self, rps: &Self) -> Self::Output {
        self - rps.clone()
    }
}

impl<R: UnitalRing + Clone + Eq> Sub<LinearTerm<R>> for &LinearTerm<R> {
    type Output = LinearCombination<R>;

    fn sub(self, rps: LinearTerm<R>) -> Self::Output {
        self.clone() - rps
    }
}

impl<R: UnitalRing + Clone + Eq> Sub for &LinearTerm<R> {
    type Output = LinearCombination<R>;

    fn sub(self, rps: Self) -> Self::Output {
        self.clone() - rps.clone()
    }
}

impl<R: UnitalSemiring + Eq> Mul for LinearTerm<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, rps: Self) -> Self::Output {
        let lps: LinearCombination<R> = self.into();
        let rps: LinearCombination<R> = rps.into();
        [lps, rps].into()
    }
}

impl<R: UnitalSemiring + Clone + Eq> Mul<&Self> for LinearTerm<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, rps: &Self) -> Self::Output {
        self * rps.clone()
    }
}

impl<R: UnitalSemiring + Clone + Eq> Mul<LinearTerm<R>> for &LinearTerm<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, rps: LinearTerm<R>) -> Self::Output {
        self.clone() * rps
    }
}

impl<R: UnitalSemiring + Clone + Eq> Mul for &LinearTerm<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, rps: Self) -> Self::Output {
        self.clone() * rps.clone()
    }
}

impl<R: UnitalSemiring + Clone + Eq> Square for LinearTerm<R> {
    type Output = LinearMonoid<R>;

    fn square(self) -> Self::Output {
        let lc: LinearCombination<R> = self.into();
        [lc.clone(), lc].into()
    }
}

impl<R: UnitalSemiring + Clone + Eq> Square for &LinearTerm<R> {
    type Output = LinearMonoid<R>;

    fn square(self) -> Self::Output {
        self.clone().square()
    }
}

impl<R: UnitalSemiring + Eq> Add<Constant<R>> for LinearTerm<R> {
    type Output = LinearCombination<R>;

    fn add(self, rps: Constant<R>) -> Self::Output {
        let mut lps: LinearCombination<R> = self.into();
        lps += rps;
        lps
    }
}

impl<R: UnitalSemiring + Clone + Eq> Add<&Constant<R>> for LinearTerm<R> {
    type Output = LinearCombination<R>;

    fn add(self, rps: &Constant<R>) -> Self::Output {
        self + rps.clone()
    }
}

impl<R: UnitalSemiring + Clone + Eq> Add<Constant<R>> for &LinearTerm<R> {
    type Output = LinearCombination<R>;

    fn add(self, rps: Constant<R>) -> Self::Output {
        self.clone() + rps
    }
}

impl<R: UnitalSemiring + Clone + Eq> Add<&Constant<R>> for &LinearTerm<R> {
    type Output = LinearCombination<R>;

    fn add(self, rps: &Constant<R>) -> Self::Output {
        self.clone() + rps.clone()
    }
}

impl<R: UnitalRing + Eq> Sub<Constant<R>> for LinearTerm<R> {
    type Output = LinearCombination<R>;

    fn sub(self, rps: Constant<R>) -> Self::Output {
        let mut lps: LinearCombination<R> = self.into();
        lps -= rps;
        lps
    }
}

impl<R: UnitalRing + Clone + Eq> Sub<&Constant<R>> for LinearTerm<R> {
    type Output = LinearCombination<R>;

    fn sub(self, rps: &Constant<R>) -> Self::Output {
        self - rps.clone()
    }
}

impl<R: UnitalRing + Clone + Eq> Sub<Constant<R>> for &LinearTerm<R> {
    type Output = LinearCombination<R>;

    fn sub(self, rps: Constant<R>) -> Self::Output {
        self.clone() - rps
    }
}

impl<R: UnitalRing + Clone + Eq> Sub<&Constant<R>> for &LinearTerm<R> {
    type Output = LinearCombination<R>;

    fn sub(self, rps: &Constant<R>) -> Self::Output {
        self.clone() - rps.clone()
    }
}

impl<R: UnitalSemiring> Mul<Constant<R>> for LinearTerm<R> {
    type Output = LinearTerm<R>;

    fn mul(self, rps: Constant<R>) -> Self::Output {
        Self {
            variable: self.variable,
            coefficient: self.coefficient * rps,
        }
    }
}

impl<R: UnitalSemiring> Mul<&Constant<R>> for LinearTerm<R> {
    type Output = LinearTerm<R>;

    fn mul(self, rps: &Constant<R>) -> Self::Output {
        Self {
            variable: self.variable,
            coefficient: self.coefficient * rps,
        }
    }
}

impl<R: UnitalSemiring> Mul<Constant<R>> for &LinearTerm<R>
where
    for<'a> &'a R: SemiringOps<R>,
{
    type Output = LinearTerm<R>;

    fn mul(self, rps: Constant<R>) -> Self::Output {
        Self::Output {
            variable: self.variable,
            coefficient: &self.coefficient * rps,
        }
    }
}

impl<R: UnitalSemiring> Mul<&Constant<R>> for &LinearTerm<R>
where
    for<'a> &'a R: SemiringOps<R>,
{
    type Output = LinearTerm<R>;

    fn mul(self, rps: &Constant<R>) -> Self::Output {
        Self::Output {
            variable: self.variable,
            coefficient: &self.coefficient * rps,
        }
    }
}

impl<R: UnitalSemiring> MulAssign<Constant<R>> for LinearTerm<R> {
    fn mul_assign(&mut self, rps: Constant<R>) {
        self.coefficient *= rps
    }
}

impl<R: UnitalSemiring> MulAssign<&Constant<R>> for LinearTerm<R> {
    fn mul_assign(&mut self, rps: &Constant<R>) {
        self.coefficient *= rps
    }
}

impl<R: UnitalSemiring + Eq> Add<Variable<R>> for LinearTerm<R> {
    type Output = LinearCombination<R>;

    fn add(self, rps: Variable<R>) -> Self::Output {
        LinearCombination::with_terms([self, (rps, Constant::ONE).into()])
    }
}

impl<R: UnitalSemiring + Eq> Add<&Variable<R>> for LinearTerm<R> {
    type Output = LinearCombination<R>;

    fn add(self, rps: &Variable<R>) -> Self::Output {
        self + *rps
    }
}

impl<R: UnitalSemiring + Clone + Eq> Add<Variable<R>> for &LinearTerm<R> {
    type Output = LinearCombination<R>;

    fn add(self, rps: Variable<R>) -> Self::Output {
        self.clone() + rps
    }
}

impl<R: UnitalSemiring + Clone + Eq> Add<&Variable<R>> for &LinearTerm<R> {
    type Output = LinearCombination<R>;

    fn add(self, rps: &Variable<R>) -> Self::Output {
        self.clone() + *rps
    }
}

impl<R: UnitalRing + Eq> Sub<Variable<R>> for LinearTerm<R> {
    type Output = LinearCombination<R>;

    fn sub(self, rps: Variable<R>) -> Self::Output {
        LinearCombination::with_terms([self, (rps, -Constant::ONE).into()])
    }
}

impl<R: UnitalRing + Eq> Sub<&Variable<R>> for LinearTerm<R> {
    type Output = LinearCombination<R>;

    fn sub(self, rps: &Variable<R>) -> Self::Output {
        self - *rps
    }
}

impl<R: UnitalRing + Clone + Eq> Sub<Variable<R>> for &LinearTerm<R> {
    type Output = LinearCombination<R>;

    fn sub(self, rps: Variable<R>) -> Self::Output {
        self.clone() - rps
    }
}

impl<R: UnitalRing + Clone + Eq> Sub<&Variable<R>> for &LinearTerm<R> {
    type Output = LinearCombination<R>;

    fn sub(self, rps: &Variable<R>) -> Self::Output {
        self.clone() - *rps
    }
}

impl<R: UnitalSemiring + Eq> Mul<Variable<R>> for LinearTerm<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, rps: Variable<R>) -> Self::Output {
        [self.into(), rps.into()].into()
    }
}

impl<R: UnitalSemiring + Eq> Mul<&Variable<R>> for LinearTerm<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, rps: &Variable<R>) -> Self::Output {
        self * *rps
    }
}

impl<R: UnitalSemiring + Clone + Eq> Mul<Variable<R>> for &LinearTerm<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, rps: Variable<R>) -> Self::Output {
        self.clone() * rps
    }
}

impl<R: UnitalSemiring + Clone + Eq> Mul<&Variable<R>> for &LinearTerm<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, rps: &Variable<R>) -> Self::Output {
        self.clone() * *rps
    }
}

impl<R: UnitalSemiring + Clone + Eq> Add<LinearCombination<R>> for LinearTerm<R>
where
    for<'a> &'a R: SemiringOps<R>,
{
    type Output = LinearCombination<R>;

    fn add(self, rps: LinearCombination<R>) -> Self::Output {
        let mut lps: LinearCombination<R> = self.into();
        lps += rps;
        lps
    }
}

impl<R: UnitalSemiring + Clone + Eq> Add<&LinearCombination<R>> for LinearTerm<R>
where
    for<'a> &'a R: SemiringOps<R>,
{
    type Output = LinearCombination<R>;

    fn add(self, rps: &LinearCombination<R>) -> Self::Output {
        let mut lps: LinearCombination<R> = self.into();
        lps += rps;
        lps
    }
}

impl<R: UnitalSemiring + Clone + Eq> Add<LinearCombination<R>> for &LinearTerm<R>
where
    for<'a> &'a R: SemiringOps<R>,
{
    type Output = LinearCombination<R>;

    fn add(self, rps: LinearCombination<R>) -> Self::Output {
        self.clone() + rps
    }
}

impl<R: UnitalSemiring + Clone + Eq> Add<&LinearCombination<R>> for &LinearTerm<R>
where
    for<'a> &'a R: SemiringOps<R>,
{
    type Output = LinearCombination<R>;

    fn add(self, rps: &LinearCombination<R>) -> Self::Output {
        self.clone() + rps
    }
}

impl<R: UnitalRing + Clone + Eq> Sub<LinearCombination<R>> for LinearTerm<R>
where
    for<'a> &'a R: RingOps<R>,
{
    type Output = LinearCombination<R>;

    fn sub(self, rps: LinearCombination<R>) -> Self::Output {
        let mut lps: LinearCombination<R> = self.into();
        lps -= rps;
        lps
    }
}

impl<R: UnitalRing + Clone + Eq> Sub<&LinearCombination<R>> for LinearTerm<R>
where
    for<'a> &'a R: RingOps<R>,
{
    type Output = LinearCombination<R>;

    fn sub(self, rps: &LinearCombination<R>) -> Self::Output {
        let mut lps: LinearCombination<R> = self.into();
        lps -= rps;
        lps
    }
}

impl<R: UnitalRing + Clone + Eq> Sub<LinearCombination<R>> for &LinearTerm<R>
where
    for<'a> &'a R: RingOps<R>,
{
    type Output = LinearCombination<R>;

    fn sub(self, rps: LinearCombination<R>) -> Self::Output {
        self.clone() - rps
    }
}

impl<R: UnitalRing + Clone + Eq> Sub<&LinearCombination<R>> for &LinearTerm<R>
where
    for<'a> &'a R: RingOps<R>,
{
    type Output = LinearCombination<R>;

    fn sub(self, rps: &LinearCombination<R>) -> Self::Output {
        self.clone() - rps
    }
}

impl<R: UnitalSemiring + Eq> Mul<LinearCombination<R>> for LinearTerm<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, rps: LinearCombination<R>) -> Self::Output {
        [self.into(), rps].into()
    }
}

impl<R: UnitalSemiring + Clone + Eq> Mul<&LinearCombination<R>> for LinearTerm<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, rps: &LinearCombination<R>) -> Self::Output {
        [self.into(), rps.clone()].into()
    }
}

impl<R: UnitalSemiring + Clone + Eq> Mul<LinearCombination<R>> for &LinearTerm<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, rps: LinearCombination<R>) -> Self::Output {
        self.clone() * rps
    }
}

impl<R: UnitalSemiring + Clone + Eq> Mul<&LinearCombination<R>> for &LinearTerm<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, rps: &LinearCombination<R>) -> Self::Output {
        self.clone() * rps
    }
}

impl<R: UnitalSemiring + Eq> Mul<LinearMonoid<R>> for LinearTerm<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, mut rps: LinearMonoid<R>) -> Self::Output {
        rps.factors.push_front(self.into());
        rps
    }
}

impl<R: UnitalSemiring + Clone + Eq> Mul<&LinearMonoid<R>> for LinearTerm<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, rps: &LinearMonoid<R>) -> Self::Output {
        self * rps.clone()
    }
}

impl<R: UnitalSemiring + Clone + Eq> Mul<LinearMonoid<R>> for &LinearTerm<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, rps: LinearMonoid<R>) -> Self::Output {
        self.clone() * rps
    }
}

impl<R: UnitalSemiring + Clone + Eq> Mul<&LinearMonoid<R>> for &LinearTerm<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, rps: &LinearMonoid<R>) -> Self::Output {
        self.clone() * rps
    }
}
