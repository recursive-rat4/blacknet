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
    Constant, Expression, LinearMonoid, LinearSpan, LinearTerm, Variable,
};
use crate::symmetric::{Absorb, Duplexer, Squeeze};
use alloc::collections::BTreeMap;
use alloc::vec;
use core::iter::Sum;
use core::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// Linear combination is a sum of linear terms.
#[derive(Clone, Default)]
pub struct LinearCombination<R: UnitalSemiring> {
    pub(super) terms: BTreeMap<Variable<R>, Constant<R>>,
}

impl<R: UnitalSemiring> LinearCombination<R> {
    /// Construct an empty linear combination.
    pub const fn new() -> Self {
        Self {
            terms: BTreeMap::new(),
        }
    }

    /// Remove all terms.
    pub fn clear(&mut self) {
        self.terms.clear()
    }

    /// Construct a new linear combination given some terms.
    pub fn with_terms<const N: usize>(terms: [LinearTerm<R>; N]) -> Self {
        let mut lc = Self::new();
        terms.into_iter().for_each(|term| lc += term);
        lc
    }

    /// Empty linear combination.
    pub const ZERO: Self = Self::new();
}

impl<R: UnitalSemiring + Eq> Expression<R> for LinearCombination<R> {
    fn span(self) -> LinearSpan<R> {
        vec![self].into()
    }

    fn degree(&self) -> usize {
        if self
            .terms
            .values()
            .any(|coefficient| *coefficient != Constant::ZERO)
        {
            1
        } else {
            0
        }
    }
}

impl<R: UnitalSemiring> From<Constant<R>> for LinearCombination<R> {
    fn from(constant: Constant<R>) -> Self {
        Self {
            terms: [(Variable::CONSTANT, constant)].into(),
        }
    }
}

impl<R: UnitalSemiring> From<Variable<R>> for LinearCombination<R> {
    fn from(variable: Variable<R>) -> Self {
        Self {
            terms: [(variable, Constant::ONE)].into(),
        }
    }
}

impl<R: UnitalSemiring> From<LinearTerm<R>> for LinearCombination<R> {
    fn from(term: LinearTerm<R>) -> Self {
        let mut terms = BTreeMap::new();
        terms.insert(term.variable, term.coefficient);
        Self { terms }
    }
}

impl<R: UnitalSemiring> Add<LinearTerm<R>> for LinearCombination<R> {
    type Output = Self;

    fn add(mut self, rps: LinearTerm<R>) -> Self::Output {
        self += rps;
        self
    }
}

impl<R: UnitalSemiring> AddAssign<LinearTerm<R>> for LinearCombination<R> {
    fn add_assign(&mut self, rps: LinearTerm<R>) {
        self.terms
            .entry(rps.variable)
            .and_modify(|value| *value += &rps.coefficient)
            .or_insert(rps.coefficient);
    }
}

impl<R: UnitalSemiring + Clone> AddAssign<&LinearTerm<R>> for LinearCombination<R> {
    fn add_assign(&mut self, rps: &LinearTerm<R>) {
        self.terms
            .entry(rps.variable)
            .and_modify(|value| *value += &rps.coefficient)
            .or_insert_with(|| rps.coefficient.clone());
    }
}

impl<R: UnitalSemiring + Clone> Add<LinearTerm<R>> for &LinearCombination<R> {
    type Output = LinearCombination<R>;

    fn add(self, rps: LinearTerm<R>) -> Self::Output {
        let mut lps = self.clone();
        lps += rps;
        lps
    }
}

impl<R: UnitalSemiring> Add<Constant<R>> for LinearCombination<R> {
    type Output = Self;

    fn add(mut self, rps: Constant<R>) -> Self::Output {
        self += rps;
        self
    }
}

impl<R: UnitalSemiring + Clone> Add<&Constant<R>> for LinearCombination<R> {
    type Output = Self;

    fn add(mut self, rps: &Constant<R>) -> Self::Output {
        self += rps;
        self
    }
}

impl<R: UnitalSemiring> AddAssign<Constant<R>> for LinearCombination<R> {
    fn add_assign(&mut self, rps: Constant<R>) {
        *self += LinearTerm::new(Variable::CONSTANT, rps)
    }
}

impl<R: UnitalSemiring + Clone> AddAssign<&Constant<R>> for LinearCombination<R> {
    fn add_assign(&mut self, rps: &Constant<R>) {
        *self += LinearTerm::new(Variable::CONSTANT, rps.clone())
    }
}

impl<R: UnitalSemiring + Clone> Add<Constant<R>> for &LinearCombination<R> {
    type Output = LinearCombination<R>;

    fn add(self, rps: Constant<R>) -> Self::Output {
        self.clone() + rps
    }
}

impl<R: UnitalSemiring + Clone> Add<&Constant<R>> for &LinearCombination<R> {
    type Output = LinearCombination<R>;

    fn add(self, rps: &Constant<R>) -> Self::Output {
        self.clone() + rps
    }
}

impl<R: UnitalSemiring> Add<Variable<R>> for LinearCombination<R> {
    type Output = Self;

    fn add(mut self, rps: Variable<R>) -> Self::Output {
        self += rps;
        self
    }
}

impl<R: UnitalSemiring> AddAssign<Variable<R>> for LinearCombination<R> {
    fn add_assign(&mut self, rps: Variable<R>) {
        *self += LinearTerm::new(rps, Constant::ONE)
    }
}

impl<R: UnitalSemiring> AddAssign<&Variable<R>> for LinearCombination<R> {
    fn add_assign(&mut self, rps: &Variable<R>) {
        *self += LinearTerm::new(*rps, Constant::ONE)
    }
}

impl<R: UnitalSemiring + Clone> Add<Variable<R>> for &LinearCombination<R> {
    type Output = LinearCombination<R>;

    fn add(self, rps: Variable<R>) -> Self::Output {
        self.clone() + rps
    }
}

impl<R: UnitalSemiring> Add for LinearCombination<R> {
    type Output = Self;

    fn add(mut self, rps: Self) -> Self::Output {
        self += rps;
        self
    }
}

impl<R: UnitalSemiring> AddAssign for LinearCombination<R> {
    fn add_assign(&mut self, rps: Self) {
        for (variable, coefficient) in rps.terms {
            *self += LinearTerm::new(variable, coefficient)
        }
    }
}

impl<R: UnitalSemiring + Clone> Add<&Self> for LinearCombination<R> {
    type Output = Self;

    fn add(mut self, rps: &Self) -> Self::Output {
        self += rps;
        self
    }
}

impl<R: UnitalSemiring + Clone> AddAssign<&Self> for LinearCombination<R> {
    fn add_assign(&mut self, rps: &Self) {
        for (&variable, coefficient) in &rps.terms {
            *self += LinearTerm::new(variable, coefficient.clone())
        }
    }
}

impl<R: UnitalSemiring + Clone> Add<LinearCombination<R>> for &LinearCombination<R> {
    type Output = LinearCombination<R>;

    fn add(self, rps: LinearCombination<R>) -> Self::Output {
        self.clone() + rps
    }
}

impl<R: UnitalSemiring + Clone> Add for &LinearCombination<R> {
    type Output = LinearCombination<R>;

    fn add(self, rps: Self) -> Self::Output {
        self.clone() + rps
    }
}

impl<R: UnitalSemiring> Double for LinearCombination<R> {
    type Output = Self;

    fn double(self) -> Self::Output {
        Self {
            terms: self
                .terms
                .into_iter()
                .map(|(var, val)| (var, val.double()))
                .collect(),
        }
    }
}

impl<R: UnitalSemiring> Double for &LinearCombination<R>
where
    for<'a> &'a R: SemiringOps<R>,
{
    type Output = LinearCombination<R>;

    fn double(self) -> Self::Output {
        LinearCombination::<R> {
            terms: self
                .terms
                .iter()
                .map(|(&var, val)| (var, val.double()))
                .collect(),
        }
    }
}

impl<R: UnitalRing> Neg for LinearCombination<R> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        LinearCombination::<R> {
            terms: self
                .terms
                .into_iter()
                .map(|(var, val)| (var, -val))
                .collect(),
        }
    }
}

impl<R: UnitalRing> Neg for &LinearCombination<R>
where
    for<'a> &'a R: RingOps<R>,
{
    type Output = LinearCombination<R>;

    fn neg(self) -> Self::Output {
        let mut lc = LinearCombination::new();
        for (&variable, coefficient) in &self.terms {
            lc += LinearTerm::new(variable, -coefficient);
        }
        lc
    }
}

impl<R: UnitalRing> Sub<LinearTerm<R>> for LinearCombination<R> {
    type Output = Self;

    fn sub(mut self, rps: LinearTerm<R>) -> Self::Output {
        self -= rps;
        self
    }
}

impl<R: UnitalRing> SubAssign<LinearTerm<R>> for LinearCombination<R> {
    fn sub_assign(&mut self, rps: LinearTerm<R>) {
        self.terms
            .entry(rps.variable)
            .and_modify(|value| *value -= &rps.coefficient)
            .or_insert(-rps.coefficient);
    }
}

impl<R: UnitalRing + Clone> Sub<LinearTerm<R>> for &LinearCombination<R> {
    type Output = LinearCombination<R>;

    fn sub(self, rps: LinearTerm<R>) -> Self::Output {
        let mut lps = self.clone();
        lps -= rps;
        lps
    }
}

impl<R: UnitalRing> Sub<Constant<R>> for LinearCombination<R> {
    type Output = Self;

    fn sub(mut self, rps: Constant<R>) -> Self::Output {
        self -= rps;
        self
    }
}

impl<R: UnitalRing + Clone> Sub<&Constant<R>> for LinearCombination<R> {
    type Output = Self;

    fn sub(mut self, rps: &Constant<R>) -> Self::Output {
        self -= rps;
        self
    }
}

impl<R: UnitalRing> SubAssign<Constant<R>> for LinearCombination<R> {
    fn sub_assign(&mut self, rps: Constant<R>) {
        *self -= LinearTerm::new(Variable::CONSTANT, rps)
    }
}

impl<R: UnitalRing + Clone> SubAssign<&Constant<R>> for LinearCombination<R> {
    fn sub_assign(&mut self, rps: &Constant<R>) {
        *self -= LinearTerm::new(Variable::CONSTANT, rps.clone())
    }
}

impl<R: UnitalRing + Clone> Sub<Constant<R>> for &LinearCombination<R> {
    type Output = LinearCombination<R>;

    fn sub(self, rps: Constant<R>) -> Self::Output {
        self.clone() - rps
    }
}

impl<R: UnitalRing + Clone> Sub<&Constant<R>> for &LinearCombination<R> {
    type Output = LinearCombination<R>;

    fn sub(self, rps: &Constant<R>) -> Self::Output {
        self.clone() - rps
    }
}

impl<R: UnitalRing> Sub<Variable<R>> for LinearCombination<R> {
    type Output = Self;

    fn sub(mut self, rps: Variable<R>) -> Self::Output {
        self -= rps;
        self
    }
}

impl<R: UnitalRing> SubAssign<Variable<R>> for LinearCombination<R> {
    fn sub_assign(&mut self, rps: Variable<R>) {
        *self -= LinearTerm::new(rps, Constant::ONE)
    }
}

impl<R: UnitalRing + Clone> Sub<Variable<R>> for &LinearCombination<R> {
    type Output = LinearCombination<R>;

    fn sub(self, rps: Variable<R>) -> Self::Output {
        self.clone() - rps
    }
}

impl<R: UnitalRing> Sub for LinearCombination<R> {
    type Output = Self;

    fn sub(mut self, rps: Self) -> Self::Output {
        self -= rps;
        self
    }
}

impl<R: UnitalRing> SubAssign for LinearCombination<R> {
    fn sub_assign(&mut self, rps: Self) {
        for (variable, coefficient) in rps.terms {
            *self -= LinearTerm::new(variable, coefficient)
        }
    }
}

impl<R: UnitalRing> Sub<&Self> for LinearCombination<R>
where
    for<'a> &'a R: RingOps<R>,
{
    type Output = Self;

    fn sub(mut self, rps: &Self) -> Self::Output {
        self -= rps;
        self
    }
}

impl<R: UnitalRing> SubAssign<&Self> for LinearCombination<R>
where
    for<'a> &'a R: RingOps<R>,
{
    fn sub_assign(&mut self, rps: &Self) {
        for (&variable, coefficient) in &rps.terms {
            *self += LinearTerm::new(variable, -coefficient)
        }
    }
}

impl<R: UnitalRing + Clone> Sub<LinearCombination<R>> for &LinearCombination<R> {
    type Output = LinearCombination<R>;

    fn sub(self, rps: LinearCombination<R>) -> Self::Output {
        self.clone() - rps
    }
}

impl<R: UnitalRing + Clone> Sub for &LinearCombination<R>
where
    for<'a> &'a R: RingOps<R>,
{
    type Output = LinearCombination<R>;

    fn sub(self, rps: Self) -> Self::Output {
        self.clone() - rps
    }
}

impl<R: UnitalSemiring> Mul<LinearTerm<R>> for LinearCombination<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, rps: LinearTerm<R>) -> Self::Output {
        [self, rps.into()].into()
    }
}

impl<R: UnitalSemiring + Clone> Mul<LinearTerm<R>> for &LinearCombination<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, rps: LinearTerm<R>) -> Self::Output {
        [self.clone(), rps.into()].into()
    }
}

impl<R: UnitalSemiring> Mul<Constant<R>> for LinearCombination<R> {
    type Output = Self;

    fn mul(mut self, rps: Constant<R>) -> Self::Output {
        self *= rps;
        self
    }
}

impl<R: UnitalSemiring> Mul<&Constant<R>> for LinearCombination<R> {
    type Output = Self;

    fn mul(mut self, rps: &Constant<R>) -> Self::Output {
        self *= rps;
        self
    }
}

impl<R: UnitalSemiring> MulAssign<Constant<R>> for LinearCombination<R> {
    fn mul_assign(&mut self, rps: Constant<R>) {
        *self *= &rps
    }
}

impl<R: UnitalSemiring> MulAssign<&Constant<R>> for LinearCombination<R> {
    fn mul_assign(&mut self, rps: &Constant<R>) {
        for coefficient in self.terms.values_mut() {
            *coefficient *= rps
        }
    }
}

impl<R: UnitalSemiring> Mul<Constant<R>> for &LinearCombination<R>
where
    for<'a> &'a R: SemiringOps<R>,
{
    type Output = LinearCombination<R>;

    fn mul(self, rps: Constant<R>) -> Self::Output {
        self * &rps
    }
}

impl<R: UnitalSemiring> Mul<&Constant<R>> for &LinearCombination<R>
where
    for<'a> &'a R: SemiringOps<R>,
{
    type Output = LinearCombination<R>;

    fn mul(self, rps: &Constant<R>) -> Self::Output {
        let mut lc = LinearCombination::new();
        for (&variable, coefficient) in &self.terms {
            lc += LinearTerm::new(variable, coefficient * rps);
        }
        lc
    }
}

impl<R: UnitalSemiring> Mul<Variable<R>> for LinearCombination<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, rps: Variable<R>) -> Self::Output {
        [self, LinearTerm::new(rps, Constant::ONE).into()].into()
    }
}

impl<R: UnitalSemiring + Clone> Mul<Variable<R>> for &LinearCombination<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, rps: Variable<R>) -> Self::Output {
        self.clone() * rps
    }
}

impl<R: UnitalSemiring> Mul for LinearCombination<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, rps: Self) -> Self::Output {
        [self, rps].into()
    }
}

impl<R: UnitalSemiring + Clone> Mul<&Self> for LinearCombination<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, rps: &Self) -> Self::Output {
        [self, rps.clone()].into()
    }
}

impl<R: UnitalSemiring + Clone> Mul<LinearCombination<R>> for &LinearCombination<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, rps: LinearCombination<R>) -> Self::Output {
        [self.clone(), rps].into()
    }
}

impl<R: UnitalSemiring + Clone> Mul for &LinearCombination<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, rps: Self) -> Self::Output {
        [self.clone(), rps.clone()].into()
    }
}

impl<R: UnitalSemiring + Clone> Square for LinearCombination<R> {
    type Output = LinearMonoid<R>;

    fn square(self) -> Self::Output {
        [self.clone(), self].into()
    }
}

impl<R: UnitalSemiring + Clone> Square for &LinearCombination<R> {
    type Output = LinearMonoid<R>;

    fn square(self) -> Self::Output {
        [self.clone(), self.clone()].into()
    }
}

impl<R: UnitalSemiring> Mul<LinearMonoid<R>> for LinearCombination<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, mut rps: LinearMonoid<R>) -> Self::Output {
        rps.factors.push_front(self);
        rps
    }
}

impl<R: UnitalSemiring + Clone> Mul<LinearMonoid<R>> for &LinearCombination<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, mut rps: LinearMonoid<R>) -> Self::Output {
        rps.factors.push_front(self.clone());
        rps
    }
}

impl<R: UnitalSemiring + Clone> Mul<&LinearMonoid<R>> for LinearCombination<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, rps: &LinearMonoid<R>) -> Self::Output {
        self * rps.clone()
    }
}

impl<R: UnitalSemiring + Clone> Mul<&LinearMonoid<R>> for &LinearCombination<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, rps: &LinearMonoid<R>) -> Self::Output {
        self.clone() * rps.clone()
    }
}

impl<R: UnitalSemiring> Sum<LinearTerm<R>> for LinearCombination<R> {
    fn sum<I: Iterator<Item = LinearTerm<R>>>(iter: I) -> Self {
        let mut lc = Self::new();
        for i in iter {
            lc += i
        }
        lc
    }
}

impl<'a, R: UnitalSemiring + Clone> Sum<&'a LinearTerm<R>> for LinearCombination<R> {
    fn sum<I: Iterator<Item = &'a LinearTerm<R>>>(iter: I) -> Self {
        let mut lc = Self::new();
        for i in iter {
            lc += i
        }
        lc
    }
}

impl<R: UnitalSemiring> Sum<Variable<R>> for LinearCombination<R> {
    fn sum<I: Iterator<Item = Variable<R>>>(iter: I) -> Self {
        let mut lc = Self::new();
        for i in iter {
            lc += i
        }
        lc
    }
}

impl<'a, R: UnitalSemiring> Sum<&'a Variable<R>> for LinearCombination<R> {
    fn sum<I: Iterator<Item = &'a Variable<R>>>(iter: I) -> Self {
        let mut lc = Self::new();
        for i in iter {
            lc += i
        }
        lc
    }
}

impl<R: UnitalSemiring> Sum for LinearCombination<R> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut lc = Self::new();
        for i in iter {
            lc += i
        }
        lc
    }
}

impl<'a, R: UnitalSemiring + Clone> Sum<&'a Self> for LinearCombination<R> {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        let mut lc = Self::new();
        for i in iter {
            lc += i
        }
        lc
    }
}

impl<R: UnitalSemiring> Absorb<Self> for LinearCombination<R> {
    fn absorb_into<D: Duplexer<Msg = Self>>(self, duplex: &mut D) {
        duplex.absorb_msg(self)
    }
}

impl<R: UnitalSemiring> Squeeze<Self> for LinearCombination<R> {
    fn squeeze_from<D: Duplexer<Msg = Self>>(duplex: &mut D) -> Self {
        duplex.squeeze_msg()
    }
}
