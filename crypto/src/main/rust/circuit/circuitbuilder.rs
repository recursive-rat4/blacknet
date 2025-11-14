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

use crate::customizableconstraintsystem::CustomizableConstraintSystem;
use crate::matrixsparse::MatrixSparseBuilder;
use crate::operation::Double;
use crate::r1cs::R1CS;
use crate::ring::UnitalRing;
use alloc::boxed::Box;
use alloc::collections::{BTreeMap, VecDeque};
use alloc::vec;
use alloc::vec::Vec;
use core::cell::{Cell, RefCell};
use core::cmp::{Ordering, max};
use core::fmt::{Display, Formatter, Result};
use core::marker::PhantomData;
use core::ops::{Add, AddAssign, Index, Mul, MulAssign, Neg, Sub, SubAssign};
use orx_tree::{Dyn, DynTree, NodeIdx, NodeRef};

pub trait Expression<R: UnitalRing>: 'static {
    fn span(&self) -> LinearSpan<R>;
    fn degree(&self) -> usize;
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Constant<R: UnitalRing> {
    value: R,
}

impl<R: UnitalRing> Constant<R> {
    pub const UNITY: Self = Self { value: R::UNITY };
    pub const ZERO: Self = Self { value: R::ZERO };
}

impl<R: UnitalRing> Expression<R> for Constant<R> {
    fn span(&self) -> LinearSpan<R> {
        vec![(*self).into()].into()
    }

    fn degree(&self) -> usize {
        0
    }
}

impl<R: UnitalRing> From<R> for Constant<R> {
    fn from(value: R) -> Self {
        Self { value }
    }
}

impl<R: UnitalRing> Add for Constant<R> {
    type Output = Constant<R>;

    fn add(self, rps: Constant<R>) -> Self::Output {
        Self {
            value: self.value + rps.value,
        }
    }
}

impl<R: UnitalRing> AddAssign for Constant<R> {
    fn add_assign(&mut self, rps: Self) {
        *self = *self + rps
    }
}

impl<R: UnitalRing> Double for Constant<R> {
    type Output = Self;

    fn double(self) -> Self::Output {
        Self {
            value: self.value.double(),
        }
    }
}

impl<R: UnitalRing> Neg for Constant<R> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self { value: -self.value }
    }
}

impl<R: UnitalRing> Sub for Constant<R> {
    type Output = Constant<R>;

    fn sub(self, rps: Constant<R>) -> Self::Output {
        Self {
            value: self.value - rps.value,
        }
    }
}

impl<R: UnitalRing> SubAssign for Constant<R> {
    fn sub_assign(&mut self, rps: Self) {
        *self = *self - rps
    }
}

impl<R: UnitalRing> Mul for Constant<R> {
    type Output = Constant<R>;

    fn mul(self, rps: Constant<R>) -> Self::Output {
        Self {
            value: self.value * rps.value,
        }
    }
}

impl<R: UnitalRing> MulAssign for Constant<R> {
    fn mul_assign(&mut self, rps: Self) {
        *self = *self * rps
    }
}

#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
pub enum VariableKind {
    Constant,
    PublicInput,
    PublicOutput,
    PrivateInput,
    PrivateOutput,
    Auxiliary,
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Variable<R: UnitalRing> {
    kind: VariableKind,
    number: usize,
    phantom: PhantomData<R>,
}

impl<R: UnitalRing> Variable<R> {
    const fn new(kind: VariableKind, number: usize) -> Self {
        Self {
            kind,
            number,
            phantom: PhantomData,
        }
    }

    const CONSTANT: Self = Self::new(VariableKind::Constant, 0);
}

impl<R: UnitalRing> Expression<R> for Variable<R> {
    fn span(&self) -> LinearSpan<R> {
        vec![(*self).into()].into()
    }

    fn degree(&self) -> usize {
        1
    }
}

impl<R: UnitalRing> Ord for Variable<R> {
    fn cmp(&self, rps: &Self) -> Ordering {
        match self.kind.cmp(&rps.kind) {
            Ordering::Equal => self.number.cmp(&rps.number),
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
        }
    }
}

impl<R: UnitalRing> PartialOrd for Variable<R> {
    fn partial_cmp(&self, rps: &Self) -> Option<Ordering> {
        Some(self.cmp(rps))
    }
}

impl<R: UnitalRing> Add<Constant<R>> for Variable<R> {
    type Output = LinearCombination<R>;

    fn add(self, rps: Constant<R>) -> Self::Output {
        [(self, Constant::UNITY), (Variable::CONSTANT, rps)].into()
    }
}

impl<R: UnitalRing> Add<Variable<R>> for Constant<R> {
    type Output = LinearCombination<R>;

    fn add(self, rps: Variable<R>) -> Self::Output {
        [(Variable::CONSTANT, self), (rps, Constant::UNITY)].into()
    }
}

impl<R: UnitalRing> Add for Variable<R> {
    type Output = LinearCombination<R>;

    fn add(self, rps: Self) -> Self::Output {
        if self != rps {
            [(self, Constant::UNITY), (rps, Constant::UNITY)].into()
        } else {
            [(self, Constant::UNITY.double())].into()
        }
    }
}

impl<R: UnitalRing> Double for Variable<R> {
    type Output = LinearCombination<R>;

    fn double(self) -> Self::Output {
        [(self, Constant::UNITY.double())].into()
    }
}

impl<R: UnitalRing> Neg for Variable<R> {
    type Output = LinearCombination<R>;

    fn neg(self) -> Self::Output {
        [(self, -Constant::UNITY)].into()
    }
}

impl<R: UnitalRing> Sub<Constant<R>> for Variable<R> {
    type Output = LinearCombination<R>;

    fn sub(self, rps: Constant<R>) -> Self::Output {
        [(self, Constant::UNITY), (Variable::CONSTANT, -rps)].into()
    }
}

impl<R: UnitalRing> Sub<Variable<R>> for Constant<R> {
    type Output = LinearCombination<R>;

    fn sub(self, rps: Variable<R>) -> Self::Output {
        [(Variable::CONSTANT, self), (rps, -Constant::UNITY)].into()
    }
}

impl<R: UnitalRing> Sub for Variable<R> {
    type Output = LinearCombination<R>;

    fn sub(self, rps: Self) -> Self::Output {
        if self != rps {
            [(self, Constant::UNITY), (rps, -Constant::UNITY)].into()
        } else {
            [].into()
        }
    }
}

impl<R: UnitalRing> Mul<Constant<R>> for Variable<R> {
    type Output = LinearCombination<R>;

    fn mul(self, rps: Constant<R>) -> Self::Output {
        [(self, rps)].into()
    }
}

impl<R: UnitalRing> Mul<Variable<R>> for Constant<R> {
    type Output = LinearCombination<R>;

    fn mul(self, rps: Variable<R>) -> Self::Output {
        [(rps, self)].into()
    }
}

impl<R: UnitalRing> Mul for Variable<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, rps: Self) -> Self::Output {
        [self.into(), rps.into()].into()
    }
}

pub type Term<R> = (Variable<R>, Constant<R>);

#[derive(Clone)]
pub struct LinearCombination<R: UnitalRing> {
    terms: BTreeMap<Variable<R>, Constant<R>>,
}

impl<R: UnitalRing> Expression<R> for LinearCombination<R> {
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

impl<R: UnitalRing> Default for LinearCombination<R> {
    fn default() -> Self {
        Self {
            terms: BTreeMap::new(),
        }
    }
}

impl<R: UnitalRing> From<Constant<R>> for LinearCombination<R> {
    fn from(constant: Constant<R>) -> Self {
        Self {
            terms: [(Variable::CONSTANT, constant)].into(),
        }
    }
}

impl<R: UnitalRing> From<Variable<R>> for LinearCombination<R> {
    fn from(variable: Variable<R>) -> Self {
        Self {
            terms: [(variable, Constant::UNITY)].into(),
        }
    }
}

impl<R: UnitalRing> From<Term<R>> for LinearCombination<R> {
    fn from(term: Term<R>) -> Self {
        Self {
            terms: [term].into(),
        }
    }
}

impl<R: UnitalRing, const N: usize> From<[Term<R>; N]> for LinearCombination<R> {
    fn from(terms: [Term<R>; N]) -> Self {
        Self {
            terms: terms.into(),
        }
    }
}

impl<R: UnitalRing> AddAssign<Term<R>> for LinearCombination<R> {
    fn add_assign(&mut self, rps: Term<R>) {
        let (variable, coefficient) = rps;
        self.terms
            .entry(variable)
            .and_modify(|value| *value += coefficient)
            .or_insert(coefficient);
    }
}

impl<R: UnitalRing> Add<Constant<R>> for LinearCombination<R> {
    type Output = Self;

    fn add(mut self, rps: Constant<R>) -> Self::Output {
        self += rps;
        self
    }
}

impl<R: UnitalRing> AddAssign<Constant<R>> for LinearCombination<R> {
    fn add_assign(&mut self, rps: Constant<R>) {
        *self += (Variable::CONSTANT, rps)
    }
}

impl<R: UnitalRing> Add<Constant<R>> for &LinearCombination<R> {
    type Output = LinearCombination<R>;

    fn add(self, rps: Constant<R>) -> Self::Output {
        self.clone() + rps
    }
}

impl<R: UnitalRing> Add<LinearCombination<R>> for Constant<R> {
    type Output = LinearCombination<R>;

    fn add(self, mut rps: LinearCombination<R>) -> Self::Output {
        rps += (Variable::CONSTANT, self);
        rps
    }
}

impl<R: UnitalRing> Add<&LinearCombination<R>> for Constant<R> {
    type Output = LinearCombination<R>;

    fn add(self, rps: &LinearCombination<R>) -> Self::Output {
        self + rps.clone()
    }
}

impl<R: UnitalRing> Add<Variable<R>> for LinearCombination<R> {
    type Output = Self;

    fn add(mut self, rps: Variable<R>) -> Self::Output {
        self += rps;
        self
    }
}

impl<R: UnitalRing> AddAssign<Variable<R>> for LinearCombination<R> {
    fn add_assign(&mut self, rps: Variable<R>) {
        *self += (rps, Constant::UNITY)
    }
}

impl<R: UnitalRing> Add<Variable<R>> for &LinearCombination<R> {
    type Output = LinearCombination<R>;

    fn add(self, rps: Variable<R>) -> Self::Output {
        self.clone() + rps
    }
}

impl<R: UnitalRing> Add<LinearCombination<R>> for Variable<R> {
    type Output = LinearCombination<R>;

    fn add(self, mut rps: LinearCombination<R>) -> Self::Output {
        rps += (self, Constant::UNITY);
        rps
    }
}

impl<R: UnitalRing> Add<&LinearCombination<R>> for Variable<R> {
    type Output = LinearCombination<R>;

    fn add(self, rps: &LinearCombination<R>) -> Self::Output {
        self + rps.clone()
    }
}

impl<R: UnitalRing> Add for LinearCombination<R> {
    type Output = Self;

    fn add(mut self, rps: Self) -> Self::Output {
        self += rps;
        self
    }
}

impl<R: UnitalRing> AddAssign for LinearCombination<R> {
    fn add_assign(&mut self, rps: Self) {
        for term in rps.terms {
            *self += term
        }
    }
}

impl<R: UnitalRing> Add<&Self> for LinearCombination<R> {
    type Output = Self;

    fn add(mut self, rps: &Self) -> Self::Output {
        self += rps;
        self
    }
}

impl<R: UnitalRing> AddAssign<&Self> for LinearCombination<R> {
    fn add_assign(&mut self, rps: &Self) {
        for (&variable, &coefficient) in &rps.terms {
            *self += (variable, coefficient)
        }
    }
}

impl<R: UnitalRing> Add for &LinearCombination<R> {
    type Output = LinearCombination<R>;

    fn add(self, rps: Self) -> Self::Output {
        self.clone() + rps
    }
}

impl<R: UnitalRing> Double for &LinearCombination<R> {
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
        let mut lc = LinearCombination::default();
        for (&variable, &coefficient) in &self.terms {
            lc -= (variable, coefficient);
        }
        lc
    }
}

impl<R: UnitalRing> SubAssign<Term<R>> for LinearCombination<R> {
    fn sub_assign(&mut self, rps: Term<R>) {
        let (variable, coefficient) = rps;
        self.terms
            .entry(variable)
            .and_modify(|value| *value -= coefficient)
            .or_insert(-coefficient);
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
        *self -= (Variable::CONSTANT, rps)
    }
}

impl<R: UnitalRing> Sub<Constant<R>> for &LinearCombination<R> {
    type Output = LinearCombination<R>;

    fn sub(self, rps: Constant<R>) -> Self::Output {
        self.clone() - rps
    }
}

impl<R: UnitalRing> Sub<LinearCombination<R>> for Constant<R> {
    type Output = LinearCombination<R>;

    fn sub(self, mut rps: LinearCombination<R>) -> Self::Output {
        rps = -rps;
        rps += (Variable::CONSTANT, self);
        rps
    }
}

impl<R: UnitalRing> Sub<&LinearCombination<R>> for Constant<R> {
    type Output = LinearCombination<R>;

    fn sub(self, rps: &LinearCombination<R>) -> Self::Output {
        self - rps.clone()
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
        *self -= (rps, Constant::UNITY)
    }
}

impl<R: UnitalRing> Sub<Variable<R>> for &LinearCombination<R> {
    type Output = LinearCombination<R>;

    fn sub(self, rps: Variable<R>) -> Self::Output {
        self.clone() - rps
    }
}

impl<R: UnitalRing> Sub<LinearCombination<R>> for Variable<R> {
    type Output = LinearCombination<R>;

    fn sub(self, mut rps: LinearCombination<R>) -> Self::Output {
        rps = -rps;
        rps += (self, Constant::UNITY);
        rps
    }
}

impl<R: UnitalRing> Sub<&LinearCombination<R>> for Variable<R> {
    type Output = LinearCombination<R>;

    fn sub(self, rps: &LinearCombination<R>) -> Self::Output {
        self - rps.clone()
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
        for term in rps.terms {
            *self -= term
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
            *self -= (variable, coefficient)
        }
    }
}

impl<R: UnitalRing> Sub for &LinearCombination<R> {
    type Output = LinearCombination<R>;

    fn sub(self, rps: Self) -> Self::Output {
        self.clone() - rps
    }
}

impl<R: UnitalRing> Mul<Constant<R>> for LinearCombination<R> {
    type Output = Self;

    fn mul(mut self, rps: Constant<R>) -> Self::Output {
        self *= rps;
        self
    }
}

impl<R: UnitalRing> MulAssign<Constant<R>> for LinearCombination<R> {
    fn mul_assign(&mut self, rps: Constant<R>) {
        for coefficient in self.terms.values_mut() {
            *coefficient *= rps;
        }
    }
}

impl<R: UnitalRing> Mul<Constant<R>> for &LinearCombination<R> {
    type Output = LinearCombination<R>;

    fn mul(self, rps: Constant<R>) -> Self::Output {
        let mut lc = LinearCombination::default();
        for (&variable, &coefficient) in &self.terms {
            lc += (variable, coefficient * rps);
        }
        lc
    }
}

impl<R: UnitalRing> Mul<LinearCombination<R>> for Constant<R> {
    type Output = LinearCombination<R>;

    fn mul(self, mut rps: LinearCombination<R>) -> Self::Output {
        for coefficient in rps.terms.values_mut() {
            *coefficient = self * *coefficient;
        }
        rps
    }
}

impl<R: UnitalRing> Mul<&LinearCombination<R>> for Constant<R> {
    type Output = LinearCombination<R>;

    fn mul(self, rps: &LinearCombination<R>) -> Self::Output {
        let mut lc = LinearCombination::default();
        for (&variable, &coefficient) in &rps.terms {
            lc += (variable, self * coefficient);
        }
        lc
    }
}

impl<R: UnitalRing> Mul<Variable<R>> for LinearCombination<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, rps: Variable<R>) -> Self::Output {
        [self, (rps, Constant::UNITY).into()].into()
    }
}

impl<R: UnitalRing> Mul<Variable<R>> for &LinearCombination<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, rps: Variable<R>) -> Self::Output {
        self.clone() * rps
    }
}

impl<R: UnitalRing> Mul<LinearCombination<R>> for Variable<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, rps: LinearCombination<R>) -> Self::Output {
        [(self, Constant::UNITY).into(), rps].into()
    }
}

impl<R: UnitalRing> Mul<&LinearCombination<R>> for Variable<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, rps: &LinearCombination<R>) -> Self::Output {
        self * rps.clone()
    }
}

impl<R: UnitalRing> Mul for LinearCombination<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, rps: Self) -> Self::Output {
        [self, rps].into()
    }
}

impl<R: UnitalRing> Mul<&Self> for LinearCombination<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, rps: &Self) -> Self::Output {
        [self, rps.clone()].into()
    }
}

impl<R: UnitalRing> Mul<LinearCombination<R>> for &LinearCombination<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, rps: LinearCombination<R>) -> Self::Output {
        [self.clone(), rps].into()
    }
}

impl<R: UnitalRing> Mul for &LinearCombination<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, rps: Self) -> Self::Output {
        [self.clone(), rps.clone()].into()
    }
}

#[derive(Clone)]
pub struct LinearMonoid<R: UnitalRing> {
    factors: VecDeque<LinearCombination<R>>,
}

impl<R: UnitalRing> Expression<R> for LinearMonoid<R> {
    fn span(&self) -> LinearSpan<R> {
        self.factors.clone().into()
    }

    fn degree(&self) -> usize {
        self.factors.iter().map(Expression::degree).sum()
    }
}

impl<R: UnitalRing, const N: usize> From<[LinearCombination<R>; N]> for LinearMonoid<R> {
    fn from(factors: [LinearCombination<R>; N]) -> Self {
        Self {
            factors: factors.into(),
        }
    }
}

impl<R: UnitalRing> Mul<Constant<R>> for LinearMonoid<R> {
    type Output = Self;

    fn mul(mut self, rps: Constant<R>) -> Self::Output {
        self *= rps;
        self
    }
}

impl<R: UnitalRing> MulAssign<Constant<R>> for LinearMonoid<R> {
    fn mul_assign(&mut self, rps: Constant<R>) {
        match self.factors.back_mut() {
            Some(factor) => *factor *= rps,
            None => self.factors.push_back(rps.into()),
        }
    }
}

impl<R: UnitalRing> Mul<Constant<R>> for &LinearMonoid<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, rps: Constant<R>) -> Self::Output {
        self.clone() * rps
    }
}

impl<R: UnitalRing> Mul<LinearMonoid<R>> for Constant<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, mut rps: LinearMonoid<R>) -> Self::Output {
        match rps.factors.front_mut() {
            Some(factor) => *factor = self * &*factor,
            None => rps.factors.push_front(self.into()),
        }
        rps
    }
}

impl<R: UnitalRing> Mul<&LinearMonoid<R>> for Constant<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, rps: &LinearMonoid<R>) -> Self::Output {
        self * rps.clone()
    }
}

impl<R: UnitalRing> Mul<Variable<R>> for LinearMonoid<R> {
    type Output = Self;

    fn mul(mut self, rps: Variable<R>) -> Self::Output {
        self *= rps;
        self
    }
}

impl<R: UnitalRing> MulAssign<Variable<R>> for LinearMonoid<R> {
    fn mul_assign(&mut self, rps: Variable<R>) {
        self.factors.push_back(rps.into())
    }
}

impl<R: UnitalRing> Mul<Variable<R>> for &LinearMonoid<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, rps: Variable<R>) -> Self::Output {
        self.clone() * rps
    }
}

impl<R: UnitalRing> Mul<LinearMonoid<R>> for Variable<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, mut rps: LinearMonoid<R>) -> Self::Output {
        rps.factors.push_front(self.into());
        rps
    }
}

impl<R: UnitalRing> Mul<&LinearMonoid<R>> for Variable<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, rps: &LinearMonoid<R>) -> Self::Output {
        self * rps.clone()
    }
}

impl<R: UnitalRing> Mul<LinearCombination<R>> for LinearMonoid<R> {
    type Output = Self;

    fn mul(mut self, rps: LinearCombination<R>) -> Self::Output {
        self *= rps;
        self
    }
}

impl<R: UnitalRing> MulAssign<LinearCombination<R>> for LinearMonoid<R> {
    fn mul_assign(&mut self, rps: LinearCombination<R>) {
        self.factors.push_back(rps)
    }
}

impl<R: UnitalRing> Mul<LinearCombination<R>> for &LinearMonoid<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, rps: LinearCombination<R>) -> Self::Output {
        self.clone() * rps
    }
}

impl<R: UnitalRing> Mul<&LinearCombination<R>> for LinearMonoid<R> {
    type Output = Self;

    fn mul(mut self, rps: &LinearCombination<R>) -> Self::Output {
        self *= rps;
        self
    }
}

impl<R: UnitalRing> MulAssign<&LinearCombination<R>> for LinearMonoid<R> {
    fn mul_assign(&mut self, rps: &LinearCombination<R>) {
        self.factors.push_back(rps.clone())
    }
}

impl<R: UnitalRing> Mul<LinearMonoid<R>> for LinearCombination<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, mut rps: LinearMonoid<R>) -> Self::Output {
        rps.factors.push_front(self);
        rps
    }
}

impl<R: UnitalRing> Mul<LinearMonoid<R>> for &LinearCombination<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, mut rps: LinearMonoid<R>) -> Self::Output {
        rps.factors.push_front(self.clone());
        rps
    }
}

impl<R: UnitalRing> Mul<&LinearMonoid<R>> for LinearCombination<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, rps: &LinearMonoid<R>) -> Self::Output {
        self * rps.clone()
    }
}

impl<R: UnitalRing> Mul<&LinearMonoid<R>> for &LinearCombination<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, rps: &LinearMonoid<R>) -> Self::Output {
        self.clone() * rps.clone()
    }
}

impl<R: UnitalRing> Mul for LinearMonoid<R> {
    type Output = Self;

    fn mul(mut self, rps: Self) -> Self::Output {
        self *= rps;
        self
    }
}

impl<R: UnitalRing> MulAssign for LinearMonoid<R> {
    fn mul_assign(&mut self, mut rps: Self) {
        self.factors.append(&mut rps.factors)
    }
}

impl<R: UnitalRing> Mul<&Self> for LinearMonoid<R> {
    type Output = Self;

    fn mul(mut self, rps: &Self) -> Self::Output {
        self *= rps.clone();
        self
    }
}

impl<R: UnitalRing> MulAssign<&Self> for LinearMonoid<R> {
    fn mul_assign(&mut self, rps: &Self) {
        self.factors.extend(rps.factors.iter().cloned())
    }
}

impl<R: UnitalRing> Mul<LinearMonoid<R>> for &LinearMonoid<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, rps: LinearMonoid<R>) -> Self::Output {
        self.clone() * rps
    }
}

impl<R: UnitalRing> Mul for &LinearMonoid<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, rps: Self) -> Self::Output {
        self.clone() * rps.clone()
    }
}

pub struct LinearSpan<R: UnitalRing> {
    vectors: Vec<LinearCombination<R>>,
}

impl<R: UnitalRing> LinearSpan<R> {
    pub const fn dimension(&self) -> usize {
        self.vectors.len()
    }
}

impl<R: UnitalRing> From<Vec<LinearCombination<R>>> for LinearSpan<R> {
    fn from(vectors: Vec<LinearCombination<R>>) -> Self {
        Self { vectors }
    }
}

impl<R: UnitalRing> From<VecDeque<LinearCombination<R>>> for LinearSpan<R> {
    fn from(vectors: VecDeque<LinearCombination<R>>) -> Self {
        Self {
            vectors: vectors.into(),
        }
    }
}

impl<R: UnitalRing> Index<usize> for LinearSpan<R> {
    type Output = LinearCombination<R>;

    fn index(&self, dimension: usize) -> &Self::Output {
        &self.vectors[dimension]
    }
}

pub struct Constraint<R: UnitalRing> {
    lps: Box<dyn Expression<R>>,
    rps: Box<dyn Expression<R>>,
}

pub struct CircuitBuilder<R: UnitalRing> {
    degree: usize,
    public_inputs: Cell<usize>,
    public_outputs: Cell<usize>,
    private_inputs: Cell<usize>,
    private_outputs: Cell<usize>,
    auxiliaries: Cell<usize>,
    constraints: RefCell<Vec<Constraint<R>>>,
    scopes: RefCell<DynTree<ScopeInfo>>,
    current_scope: RefCell<NodeIdx<Dyn<ScopeInfo>>>,
}

impl<R: UnitalRing> CircuitBuilder<R> {
    pub fn new(degree: usize) -> Self {
        let mut tree = DynTree::empty();
        let root = tree.push_root(ScopeInfo::root());
        Self {
            degree,
            public_inputs: Cell::new(0),
            public_outputs: Cell::new(0),
            private_inputs: Cell::new(0),
            private_outputs: Cell::new(0),
            auxiliaries: Cell::new(0),
            constraints: RefCell::new(Vec::new()),
            scopes: RefCell::new(tree),
            current_scope: RefCell::new(root),
        }
    }

    pub const fn degree(&self) -> usize {
        self.degree
    }

    pub fn constraints(&self) -> usize {
        self.constraints.borrow().len()
    }

    pub const fn variables(&self) -> usize {
        1 + self.public_inputs.get()
            + self.public_outputs.get()
            + self.private_inputs.get()
            + self.private_outputs.get()
            + self.auxiliaries.get()
    }

    pub fn scope<'a>(&'a self, name: &'static str) -> Scope<'a, R> {
        let mut scopes = self.scopes.borrow_mut();
        let mut current_scope = self.current_scope.borrow_mut();
        let info = ScopeInfo::new(name);
        let mut node = scopes.get_node_mut(&*current_scope).expect("Scope");
        *current_scope = node.push_child(info);
        Scope { builder: self }
    }

    pub fn r1cs(self) -> R1CS<R> {
        let (constraints_num, variables_num) = (self.constraints(), self.variables());
        let constraints = self.constraints.take();
        let (lps_degree, rps_degree) = constraints
            .iter()
            .map(|c| (c.lps.degree(), c.rps.degree()))
            .fold((0, 0), |acc, x| (max(acc.0, x.0), max(acc.1, x.1)));
        assert!(
            lps_degree <= 2 && rps_degree <= 1,
            "Shape [{lps_degree}, {rps_degree}] is not compatible with [2, 1]"
        );
        let mut a = MatrixSparseBuilder::<R>::new(constraints_num, variables_num);
        let mut b = MatrixSparseBuilder::<R>::new(constraints_num, variables_num);
        let mut c = MatrixSparseBuilder::<R>::new(constraints_num, variables_num);

        self.lay_out();
        for constraint in constraints {
            let (lps_span, rps_span) = (constraint.lps.span(), constraint.rps.span());
            match lps_span.dimension() {
                2 => {
                    self.put(&mut a, &lps_span[0]);
                    self.put(&mut b, &lps_span[1]);
                }
                1 => {
                    self.put(&mut a, &lps_span[0]);
                    self.pad(&mut b);
                }
                0 => {
                    self.pad(&mut a);
                    self.pad(&mut b);
                }
                _ => unreachable!(),
            }
            match rps_span.dimension() {
                1 => {
                    self.put(&mut c, &rps_span[0]);
                }
                0 => {
                    self.pad(&mut c);
                }
                _ => unreachable!(),
            }
        }

        R1CS::new(a.build(), b.build(), c.build())
    }

    pub fn ccs(self) -> CustomizableConstraintSystem<R> {
        let (constraints_num, variables_num) = (self.constraints(), self.variables());
        let constraints = self.constraints.take();
        let (lps_degree, rps_degree) = constraints
            .iter()
            .map(|c| (c.lps.degree(), c.rps.degree()))
            .fold((0, 0), |acc, x| (max(acc.0, x.0), max(acc.1, x.1)));
        let (mut lps_matrices, mut rps_matrices) = (Vec::new(), Vec::new());
        lps_matrices.resize_with(lps_degree, || {
            MatrixSparseBuilder::<R>::new(constraints_num, variables_num)
        });
        rps_matrices.resize_with(rps_degree, || {
            MatrixSparseBuilder::<R>::new(constraints_num, variables_num)
        });

        self.lay_out();
        #[allow(clippy::needless_range_loop)]
        for constraint in constraints {
            let (lps_span, rps_span) = (constraint.lps.span(), constraint.rps.span());
            for i in 0..lps_span.dimension() {
                self.put(&mut lps_matrices[i], &lps_span[i])
            }
            for i in lps_span.dimension()..lps_degree {
                self.pad(&mut lps_matrices[i]);
            }
            for i in 0..rps_span.dimension() {
                self.put(&mut rps_matrices[i], &rps_span[i])
            }
            for i in rps_span.dimension()..rps_degree {
                self.pad(&mut rps_matrices[i]);
            }
        }

        let mut matrices = Vec::with_capacity(lps_degree + rps_degree);
        lps_matrices
            .into_iter()
            .for_each(|b| matrices.push(b.build()));
        rps_matrices
            .into_iter()
            .for_each(|b| matrices.push(b.build()));

        let multisets = vec![(0..matrices.len() - 1).collect(), vec![matrices.len() - 1]];

        let constants = vec![R::UNITY, -R::UNITY];

        CustomizableConstraintSystem::new(matrices, multisets, constants)
    }

    #[must_use = "Circuit variable should be constrained"]
    fn allocate(&self, kind: VariableKind) -> Variable<R> {
        let mut scopes = self.scopes.borrow_mut();
        let current_scope = self.current_scope.borrow();
        let mut scope = scopes.get_node_mut(&*current_scope).expect("Scope");
        let info = scope.data_mut();
        info.variables += 1;

        let n = match kind {
            VariableKind::PublicInput => {
                let n = self.public_inputs.get();
                self.public_inputs.update(|n| n + 1);
                n
            }
            VariableKind::PublicOutput => {
                let n = self.public_outputs.get();
                self.public_outputs.update(|n| n + 1);
                n
            }
            VariableKind::PrivateInput => {
                let n = self.private_inputs.get();
                self.private_inputs.update(|n| n + 1);
                n
            }
            VariableKind::PrivateOutput => {
                let n = self.private_outputs.get();
                self.private_outputs.update(|n| n + 1);
                n
            }
            VariableKind::Auxiliary => {
                let n = self.auxiliaries.get();
                self.auxiliaries.update(|n| n + 1);
                n
            }
            VariableKind::Constant => panic!("New constant variable requested"),
        };
        Variable::new(kind, n)
    }

    fn constrain(&self, constraint: Constraint<R>) {
        let mut scopes = self.scopes.borrow_mut();
        let current_scope = self.current_scope.borrow();
        let mut scope = scopes.get_node_mut(&*current_scope).expect("Scope");
        let info = scope.data_mut();

        assert!(
            self.degree >= constraint.lps.degree(),
            "In scope {} constraint left degree {} is higher than circuit degree {}",
            info.name,
            constraint.lps.degree(),
            self.degree
        );
        assert!(
            self.degree >= constraint.rps.degree(),
            "In scope {} constraint right degree {} is higher than circuit degree {}",
            info.name,
            constraint.lps.degree(),
            self.degree
        );

        info.constraints += 1;
        let mut constraints = self.constraints.borrow_mut();
        constraints.push(constraint)
    }

    fn put(&self, m: &mut MatrixSparseBuilder<R>, lc: &LinearCombination<R>) {
        for (variable, coefficient) in &lc.terms {
            let column: usize = match variable.kind {
                VariableKind::Constant => 0,
                VariableKind::PublicInput => self.public_inputs.get() + variable.number,
                VariableKind::PublicOutput => self.public_outputs.get() + variable.number,
                VariableKind::PrivateInput => self.private_inputs.get() + variable.number,
                VariableKind::PrivateOutput => self.private_outputs.get() + variable.number,
                VariableKind::Auxiliary => self.auxiliaries.get() + variable.number,
            };
            m.column(column, coefficient.value);
        }
        m.row();
    }

    fn pad(&self, m: &mut MatrixSparseBuilder<R>) {
        m.column(0, R::UNITY);
        m.row();
    }

    fn lay_out(&self) {
        let mut n;
        let mut offset = 1;

        n = self.public_inputs.get();
        self.public_inputs.set(offset);
        offset += n;

        n = self.public_outputs.get();
        self.public_outputs.set(offset);
        offset += n;

        n = self.private_inputs.get();
        self.private_inputs.set(offset);
        offset += n;

        n = self.private_outputs.get();
        self.private_outputs.set(offset);
        offset += n;

        self.auxiliaries.set(offset);
    }
}

impl<R: UnitalRing> Display for CircuitBuilder<R> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "Circuit degree {} constraints {} variables {}\n{}",
            self.degree,
            self.constraints(),
            self.variables(),
            self.scopes.borrow()
        )
    }
}

pub struct Scope<'a, R: UnitalRing> {
    builder: &'a CircuitBuilder<R>,
}

impl<'a, R: UnitalRing> Scope<'a, R> {
    pub fn constrain<LPS: Expression<R>, RPS: Expression<R>>(&self, lps: LPS, rps: RPS) {
        self.builder.constrain(Constraint {
            lps: Box::new(lps),
            rps: Box::new(rps),
        })
    }

    #[must_use = "Circuit variable should be constrained"]
    pub fn public_input(&self) -> Variable<R> {
        self.builder.allocate(VariableKind::PublicInput)
    }

    #[must_use = "Circuit variable should be constrained"]
    pub fn public_output(&self) -> Variable<R> {
        self.builder.allocate(VariableKind::PublicOutput)
    }

    #[must_use = "Circuit variable should be constrained"]
    pub fn private_input(&self) -> Variable<R> {
        self.builder.allocate(VariableKind::PrivateInput)
    }

    #[must_use = "Circuit variable should be constrained"]
    pub fn private_output(&self) -> Variable<R> {
        self.builder.allocate(VariableKind::PrivateOutput)
    }

    #[must_use = "Circuit variable should be constrained"]
    pub fn auxiliary(&self) -> Variable<R> {
        self.builder.allocate(VariableKind::Auxiliary)
    }

    #[must_use = "Circuit variable should be constrained"]
    pub fn variable(&self, kind: VariableKind) -> Variable<R> {
        self.builder.allocate(kind)
    }
}

impl<'a, R: UnitalRing> Drop for Scope<'a, R> {
    fn drop(&mut self) {
        let scopes = self.builder.scopes.borrow();
        let mut current_scope = self.builder.current_scope.borrow_mut();
        let node = scopes.get_node(&*current_scope).expect("Tree");
        *current_scope = node.parent().expect("Scope").idx();
    }
}

struct ScopeInfo {
    name: &'static str,
    constraints: usize,
    variables: usize,
}

impl ScopeInfo {
    const fn new(name: &'static str) -> Self {
        Self {
            name,
            constraints: 0,
            variables: 0,
        }
    }

    const fn root() -> Self {
        Self {
            name: "Root",
            constraints: 0,
            variables: 1,
        }
    }
}

impl Display for ScopeInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} {}x{}", self.name, self.constraints, self.variables)
    }
}
