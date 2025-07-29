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

use crate::ring::Ring;
use core::cmp::Ordering;
use core::marker::PhantomData;
use core::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use std::collections::BTreeMap;

#[derive(Copy, Clone)]
pub struct Constant<R: Ring> {
    value: R,
}

impl<R: Ring> Constant<R> {
    pub const UNITY: Self = Self { value: R::UNITY };
    pub const ZERO: Self = Self { value: R::ZERO };
}

impl<R: Ring> From<R> for Constant<R> {
    fn from(value: R) -> Self {
        Self { value }
    }
}

impl<R: Ring> Add for Constant<R> {
    type Output = Constant<R>;

    fn add(self, rps: Constant<R>) -> Self::Output {
        Self {
            value: self.value + rps.value,
        }
    }
}

impl<R: Ring> AddAssign for Constant<R> {
    fn add_assign(&mut self, rps: Self) {
        *self = *self + rps
    }
}

impl<R: Ring> Neg for Constant<R> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self { value: -self.value }
    }
}

impl<R: Ring> Sub for Constant<R> {
    type Output = Constant<R>;

    fn sub(self, rps: Constant<R>) -> Self::Output {
        Self {
            value: self.value - rps.value,
        }
    }
}

impl<R: Ring> SubAssign for Constant<R> {
    fn sub_assign(&mut self, rps: Self) {
        *self = *self - rps
    }
}

impl<R: Ring> Mul for Constant<R> {
    type Output = Constant<R>;

    fn mul(self, rps: Constant<R>) -> Self::Output {
        Self {
            value: self.value * rps.value,
        }
    }
}

impl<R: Ring> MulAssign for Constant<R> {
    fn mul_assign(&mut self, rps: Self) {
        *self = *self * rps
    }
}

#[derive(Eq, Ord, PartialEq, PartialOrd)]
pub enum VariableKind {
    Constant,
    PublicInput,
    PublicOutput,
    PrivateInput,
    PrivateOutput,
    Auxiliary,
}

#[derive(Eq, PartialEq)]
pub struct Variable<R: Ring> {
    kind: VariableKind,
    number: usize,
    phantom: PhantomData<R>,
}

impl<R: Ring> Variable<R> {
    const CONSTANT: Self = Self {
        kind: VariableKind::Constant,
        number: 0,
        phantom: PhantomData,
    };
}

impl<R: Ring> Ord for Variable<R> {
    fn cmp(&self, rps: &Self) -> Ordering {
        match self.kind.cmp(&rps.kind) {
            Ordering::Equal => self.number.cmp(&rps.number),
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
        }
    }
}

impl<R: Ring> PartialOrd for Variable<R> {
    fn partial_cmp(&self, rps: &Self) -> Option<Ordering> {
        Some(self.cmp(rps))
    }
}

impl<R: Ring> Add<Constant<R>> for Variable<R> {
    type Output = LinearCombination<R>;

    fn add(self, rps: Constant<R>) -> Self::Output {
        [(self, Constant::UNITY), (Variable::CONSTANT, rps)].into()
    }
}

impl<R: Ring> Add<Variable<R>> for Constant<R> {
    type Output = LinearCombination<R>;

    fn add(self, rps: Variable<R>) -> Self::Output {
        [(Variable::CONSTANT, self), (rps, Constant::UNITY)].into()
    }
}

impl<R: Ring> Neg for Variable<R> {
    type Output = LinearCombination<R>;

    fn neg(self) -> Self::Output {
        [(self, -Constant::UNITY)].into()
    }
}

impl<R: Ring> Sub<Constant<R>> for Variable<R> {
    type Output = LinearCombination<R>;

    fn sub(self, rps: Constant<R>) -> Self::Output {
        [(self, Constant::UNITY), (Variable::CONSTANT, -rps)].into()
    }
}

impl<R: Ring> Sub<Variable<R>> for Constant<R> {
    type Output = LinearCombination<R>;

    fn sub(self, rps: Variable<R>) -> Self::Output {
        [(Variable::CONSTANT, self), (rps, -Constant::UNITY)].into()
    }
}

impl<R: Ring> Mul<Constant<R>> for Variable<R> {
    type Output = LinearCombination<R>;

    fn mul(self, rps: Constant<R>) -> Self::Output {
        [(self, rps)].into()
    }
}

impl<R: Ring> Mul<Variable<R>> for Constant<R> {
    type Output = LinearCombination<R>;

    fn mul(self, rps: Variable<R>) -> Self::Output {
        [(rps, self)].into()
    }
}

type Term<R> = (Variable<R>, Constant<R>);

pub struct LinearCombination<R: Ring> {
    terms: BTreeMap<Variable<R>, Constant<R>>,
}

impl<R: Ring> From<Term<R>> for LinearCombination<R> {
    fn from(term: Term<R>) -> Self {
        Self {
            terms: [term].into(),
        }
    }
}

impl<R: Ring, const N: usize> From<[Term<R>; N]> for LinearCombination<R> {
    fn from(terms: [Term<R>; N]) -> Self {
        Self {
            terms: terms.into(),
        }
    }
}

impl<R: Ring> AddAssign<Term<R>> for LinearCombination<R> {
    fn add_assign(&mut self, rps: Term<R>) {
        let (variable, coefficient) = rps;
        self.terms
            .entry(variable)
            .and_modify(|value| *value += coefficient)
            .or_insert(coefficient);
    }
}

impl<R: Ring> Add<Constant<R>> for LinearCombination<R> {
    type Output = Self;

    fn add(mut self, rps: Constant<R>) -> Self::Output {
        self += rps;
        self
    }
}

impl<R: Ring> AddAssign<Constant<R>> for LinearCombination<R> {
    fn add_assign(&mut self, rps: Constant<R>) {
        *self += (Variable::CONSTANT, rps)
    }
}

impl<R: Ring> Add<LinearCombination<R>> for Constant<R> {
    type Output = LinearCombination<R>;

    fn add(self, mut rps: LinearCombination<R>) -> Self::Output {
        rps += (Variable::CONSTANT, self);
        rps
    }
}

impl<R: Ring> Add<Variable<R>> for LinearCombination<R> {
    type Output = Self;

    fn add(mut self, rps: Variable<R>) -> Self::Output {
        self += rps;
        self
    }
}

impl<R: Ring> AddAssign<Variable<R>> for LinearCombination<R> {
    fn add_assign(&mut self, rps: Variable<R>) {
        *self += (rps, Constant::UNITY)
    }
}

impl<R: Ring> Add<LinearCombination<R>> for Variable<R> {
    type Output = LinearCombination<R>;

    fn add(self, mut rps: LinearCombination<R>) -> Self::Output {
        rps += (self, Constant::UNITY);
        rps
    }
}

impl<R: Ring> Add for LinearCombination<R> {
    type Output = Self;

    fn add(mut self, rps: Self) -> Self::Output {
        self += rps;
        self
    }
}

impl<R: Ring> AddAssign for LinearCombination<R> {
    fn add_assign(&mut self, rps: Self) {
        for term in rps.terms {
            *self += term
        }
    }
}

impl<R: Ring> Neg for LinearCombination<R> {
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        for coefficient in self.terms.values_mut() {
            *coefficient = -*coefficient;
        }
        self
    }
}

impl<R: Ring> SubAssign<Term<R>> for LinearCombination<R> {
    fn sub_assign(&mut self, rps: Term<R>) {
        let (variable, coefficient) = rps;
        self.terms
            .entry(variable)
            .and_modify(|value| *value -= coefficient)
            .or_insert(coefficient);
    }
}

impl<R: Ring> Sub<Constant<R>> for LinearCombination<R> {
    type Output = Self;

    fn sub(mut self, rps: Constant<R>) -> Self::Output {
        self -= rps;
        self
    }
}

impl<R: Ring> SubAssign<Constant<R>> for LinearCombination<R> {
    fn sub_assign(&mut self, rps: Constant<R>) {
        *self -= (Variable::CONSTANT, rps)
    }
}

impl<R: Ring> Sub<LinearCombination<R>> for Constant<R> {
    type Output = LinearCombination<R>;

    fn sub(self, mut rps: LinearCombination<R>) -> Self::Output {
        rps -= (Variable::CONSTANT, self);
        rps
    }
}

impl<R: Ring> Sub<Variable<R>> for LinearCombination<R> {
    type Output = Self;

    fn sub(mut self, rps: Variable<R>) -> Self::Output {
        self -= rps;
        self
    }
}

impl<R: Ring> SubAssign<Variable<R>> for LinearCombination<R> {
    fn sub_assign(&mut self, rps: Variable<R>) {
        *self -= (rps, Constant::UNITY)
    }
}

impl<R: Ring> Sub<LinearCombination<R>> for Variable<R> {
    type Output = LinearCombination<R>;

    fn sub(self, mut rps: LinearCombination<R>) -> Self::Output {
        rps -= (self, Constant::UNITY);
        rps
    }
}

impl<R: Ring> Sub for LinearCombination<R> {
    type Output = Self;

    fn sub(mut self, rps: Self) -> Self::Output {
        self -= rps;
        self
    }
}

impl<R: Ring> SubAssign for LinearCombination<R> {
    fn sub_assign(&mut self, rps: Self) {
        for term in rps.terms {
            *self -= term
        }
    }
}

impl<R: Ring> Mul<Constant<R>> for LinearCombination<R> {
    type Output = Self;

    fn mul(mut self, rps: Constant<R>) -> Self::Output {
        self *= rps;
        self
    }
}

impl<R: Ring> MulAssign<Constant<R>> for LinearCombination<R> {
    fn mul_assign(&mut self, rps: Constant<R>) {
        for coefficient in self.terms.values_mut() {
            *coefficient *= rps;
        }
    }
}

impl<R: Ring> Mul<LinearCombination<R>> for Constant<R> {
    type Output = LinearCombination<R>;

    fn mul(self, mut rps: LinearCombination<R>) -> Self::Output {
        for coefficient in rps.terms.values_mut() {
            *coefficient = self * *coefficient;
        }
        rps
    }
}

impl<R: Ring> Mul<Variable<R>> for LinearCombination<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, rps: Variable<R>) -> Self::Output {
        [self, (rps, Constant::UNITY).into()].into()
    }
}

impl<R: Ring> Mul<LinearCombination<R>> for Variable<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, rps: LinearCombination<R>) -> Self::Output {
        [(self, Constant::UNITY).into(), rps].into()
    }
}

impl<R: Ring> Mul for LinearCombination<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, rps: Self) -> Self::Output {
        [self, rps].into()
    }
}

pub struct LinearMonoid<R: Ring> {
    factors: Vec<LinearCombination<R>>,
}

impl<R: Ring, const N: usize> From<[LinearCombination<R>; N]> for LinearMonoid<R> {
    fn from(factors: [LinearCombination<R>; N]) -> Self {
        Self {
            factors: factors.into(),
        }
    }
}
