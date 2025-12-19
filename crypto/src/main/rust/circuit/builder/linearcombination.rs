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

use crate::circuit::builder::{
    Constant, Expression, LinearMonoid, LinearSpan, LinearTerm, Variable,
};
use crate::operation::{Double, Square};
use crate::ring::UnitalRing;
use crate::semiring::Semiring;
use alloc::collections::BTreeMap;
use alloc::vec;
use core::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// Linear combination is a sum of linear terms.
#[derive(Clone, Default)]
pub struct LinearCombination<R: Semiring> {
    pub(super) terms: BTreeMap<Variable<R>, Constant<R>>,
}

impl<R: Semiring> LinearCombination<R> {
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
}

impl<'a, R: Semiring + Eq + 'a> Expression<'a, R> for LinearCombination<R> {
    fn span(&self) -> LinearSpan<R> {
        vec![self.clone()].into()
    }

    fn degree(&self) -> usize {
        if self
            .terms
            .values()
            .any(|&coefficient| coefficient != Constant::ZERO)
        {
            1
        } else {
            0
        }
    }
}

impl<R: Semiring> From<Constant<R>> for LinearCombination<R> {
    fn from(constant: Constant<R>) -> Self {
        Self {
            terms: [(Variable::CONSTANT, constant)].into(),
        }
    }
}

impl<R: Semiring> From<Variable<R>> for LinearCombination<R> {
    fn from(variable: Variable<R>) -> Self {
        Self {
            terms: [(variable, Constant::ONE)].into(),
        }
    }
}

impl<R: Semiring> From<LinearTerm<R>> for LinearCombination<R> {
    fn from(term: LinearTerm<R>) -> Self {
        let mut terms = BTreeMap::new();
        terms.insert(term.variable, term.coefficient);
        Self { terms }
    }
}

impl<R: Semiring, const N: usize> From<[LinearTerm<R>; N]> for LinearCombination<R> {
    fn from(terms: [LinearTerm<R>; N]) -> Self {
        let iter = terms
            .into_iter()
            .map(|term| (term.variable, term.coefficient));
        Self {
            terms: BTreeMap::from_iter(iter),
        }
    }
}

impl<R: Semiring> Add<LinearTerm<R>> for LinearCombination<R> {
    type Output = Self;

    fn add(mut self, rps: LinearTerm<R>) -> Self::Output {
        self += rps;
        self
    }
}

impl<R: Semiring> AddAssign<LinearTerm<R>> for LinearCombination<R> {
    fn add_assign(&mut self, rps: LinearTerm<R>) {
        self.terms
            .entry(rps.variable)
            .and_modify(|value| *value += rps.coefficient)
            .or_insert(rps.coefficient);
    }
}

impl<R: Semiring> Add<LinearTerm<R>> for &LinearCombination<R> {
    type Output = LinearCombination<R>;

    fn add(self, rps: LinearTerm<R>) -> Self::Output {
        let mut lps = self.clone();
        lps += rps;
        lps
    }
}

impl<R: Semiring> Add<Constant<R>> for LinearCombination<R> {
    type Output = Self;

    fn add(mut self, rps: Constant<R>) -> Self::Output {
        self += rps;
        self
    }
}

impl<R: Semiring> AddAssign<Constant<R>> for LinearCombination<R> {
    fn add_assign(&mut self, rps: Constant<R>) {
        *self += LinearTerm::new(Variable::CONSTANT, rps)
    }
}

impl<R: Semiring> Add<Constant<R>> for &LinearCombination<R> {
    type Output = LinearCombination<R>;

    fn add(self, rps: Constant<R>) -> Self::Output {
        self.clone() + rps
    }
}

impl<R: Semiring> Add<Variable<R>> for LinearCombination<R> {
    type Output = Self;

    fn add(mut self, rps: Variable<R>) -> Self::Output {
        self += rps;
        self
    }
}

impl<R: Semiring> AddAssign<Variable<R>> for LinearCombination<R> {
    fn add_assign(&mut self, rps: Variable<R>) {
        *self += LinearTerm::new(rps, Constant::ONE)
    }
}

impl<R: Semiring> Add<Variable<R>> for &LinearCombination<R> {
    type Output = LinearCombination<R>;

    fn add(self, rps: Variable<R>) -> Self::Output {
        self.clone() + rps
    }
}

impl<R: Semiring> Add for LinearCombination<R> {
    type Output = Self;

    fn add(mut self, rps: Self) -> Self::Output {
        self += rps;
        self
    }
}

impl<R: Semiring> AddAssign for LinearCombination<R> {
    fn add_assign(&mut self, rps: Self) {
        for (variable, coefficient) in rps.terms {
            *self += LinearTerm::new(variable, coefficient)
        }
    }
}

impl<R: Semiring> Add<&Self> for LinearCombination<R> {
    type Output = Self;

    fn add(mut self, rps: &Self) -> Self::Output {
        self += rps;
        self
    }
}

impl<R: Semiring> AddAssign<&Self> for LinearCombination<R> {
    fn add_assign(&mut self, rps: &Self) {
        for (&variable, &coefficient) in &rps.terms {
            *self += LinearTerm::new(variable, coefficient)
        }
    }
}

impl<R: Semiring> Add<LinearCombination<R>> for &LinearCombination<R> {
    type Output = LinearCombination<R>;

    fn add(self, rps: LinearCombination<R>) -> Self::Output {
        self.clone() + rps
    }
}

impl<R: Semiring> Add for &LinearCombination<R> {
    type Output = LinearCombination<R>;

    fn add(self, rps: Self) -> Self::Output {
        self.clone() + rps
    }
}

impl<R: Semiring> Double for LinearCombination<R> {
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

impl<R: Semiring> Double for &LinearCombination<R> {
    type Output = LinearCombination<R>;

    fn double(self) -> Self::Output {
        LinearCombination::<R> {
            terms: self
                .terms
                .iter()
                .map(|(&var, &val)| (var, val.double()))
                .collect(),
        }
    }
}

impl<R: UnitalRing> Neg for LinearCombination<R> {
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        for coefficient in self.terms.values_mut() {
            *coefficient = -*coefficient;
        }
        self
    }
}

impl<R: UnitalRing> Neg for &LinearCombination<R> {
    type Output = LinearCombination<R>;

    fn neg(self) -> Self::Output {
        let mut lc = LinearCombination::new();
        for (&variable, &coefficient) in &self.terms {
            lc -= LinearTerm::new(variable, coefficient);
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
            .and_modify(|value| *value -= rps.coefficient)
            .or_insert(-rps.coefficient);
    }
}

impl<R: UnitalRing> Sub<LinearTerm<R>> for &LinearCombination<R> {
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

impl<R: UnitalRing> SubAssign<Constant<R>> for LinearCombination<R> {
    fn sub_assign(&mut self, rps: Constant<R>) {
        *self -= LinearTerm::new(Variable::CONSTANT, rps)
    }
}

impl<R: UnitalRing> Sub<Constant<R>> for &LinearCombination<R> {
    type Output = LinearCombination<R>;

    fn sub(self, rps: Constant<R>) -> Self::Output {
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

impl<R: UnitalRing> Sub<Variable<R>> for &LinearCombination<R> {
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

impl<R: UnitalRing> Sub<&Self> for LinearCombination<R> {
    type Output = Self;

    fn sub(mut self, rps: &Self) -> Self::Output {
        self -= rps;
        self
    }
}

impl<R: UnitalRing> SubAssign<&Self> for LinearCombination<R> {
    fn sub_assign(&mut self, rps: &Self) {
        for (&variable, &coefficient) in &rps.terms {
            *self -= LinearTerm::new(variable, coefficient)
        }
    }
}

impl<R: UnitalRing> Sub<LinearCombination<R>> for &LinearCombination<R> {
    type Output = LinearCombination<R>;

    fn sub(self, rps: LinearCombination<R>) -> Self::Output {
        self.clone() - rps
    }
}

impl<R: UnitalRing> Sub for &LinearCombination<R> {
    type Output = LinearCombination<R>;

    fn sub(self, rps: Self) -> Self::Output {
        self.clone() - rps
    }
}

impl<R: Semiring> Mul<LinearTerm<R>> for LinearCombination<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, rps: LinearTerm<R>) -> Self::Output {
        [self, rps.into()].into()
    }
}

impl<R: Semiring> Mul<LinearTerm<R>> for &LinearCombination<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, rps: LinearTerm<R>) -> Self::Output {
        [self.clone(), rps.into()].into()
    }
}

impl<R: Semiring> Mul<Constant<R>> for LinearCombination<R> {
    type Output = Self;

    fn mul(mut self, rps: Constant<R>) -> Self::Output {
        self *= rps;
        self
    }
}

impl<R: Semiring> MulAssign<Constant<R>> for LinearCombination<R> {
    fn mul_assign(&mut self, rps: Constant<R>) {
        for coefficient in self.terms.values_mut() {
            *coefficient *= rps;
        }
    }
}

impl<R: Semiring> Mul<Constant<R>> for &LinearCombination<R> {
    type Output = LinearCombination<R>;

    fn mul(self, rps: Constant<R>) -> Self::Output {
        let mut lc = LinearCombination::new();
        for (&variable, &coefficient) in &self.terms {
            lc += LinearTerm::new(variable, coefficient * rps);
        }
        lc
    }
}

impl<R: Semiring> Mul<Variable<R>> for LinearCombination<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, rps: Variable<R>) -> Self::Output {
        [self, LinearTerm::new(rps, Constant::ONE).into()].into()
    }
}

impl<R: Semiring> Mul<Variable<R>> for &LinearCombination<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, rps: Variable<R>) -> Self::Output {
        self.clone() * rps
    }
}

impl<R: Semiring> Mul for LinearCombination<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, rps: Self) -> Self::Output {
        [self, rps].into()
    }
}

impl<R: Semiring> Mul<&Self> for LinearCombination<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, rps: &Self) -> Self::Output {
        [self, rps.clone()].into()
    }
}

impl<R: Semiring> Mul<LinearCombination<R>> for &LinearCombination<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, rps: LinearCombination<R>) -> Self::Output {
        [self.clone(), rps].into()
    }
}

impl<R: Semiring> Mul for &LinearCombination<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, rps: Self) -> Self::Output {
        [self.clone(), rps.clone()].into()
    }
}

impl<R: Semiring> Square for LinearCombination<R> {
    type Output = LinearMonoid<R>;

    fn square(self) -> Self::Output {
        [self.clone(), self].into()
    }
}

impl<R: Semiring> Square for &LinearCombination<R> {
    type Output = LinearMonoid<R>;

    fn square(self) -> Self::Output {
        [self.clone(), self.clone()].into()
    }
}

impl<R: Semiring> Mul<LinearMonoid<R>> for LinearCombination<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, mut rps: LinearMonoid<R>) -> Self::Output {
        rps.factors.push_front(self);
        rps
    }
}

impl<R: Semiring> Mul<LinearMonoid<R>> for &LinearCombination<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, mut rps: LinearMonoid<R>) -> Self::Output {
        rps.factors.push_front(self.clone());
        rps
    }
}

impl<R: Semiring> Mul<&LinearMonoid<R>> for LinearCombination<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, rps: &LinearMonoid<R>) -> Self::Output {
        self * rps.clone()
    }
}

impl<R: Semiring> Mul<&LinearMonoid<R>> for &LinearCombination<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, rps: &LinearMonoid<R>) -> Self::Output {
        self.clone() * rps.clone()
    }
}
