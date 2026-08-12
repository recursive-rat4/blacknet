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
    Expression, LinearCombination, LinearMonoid, LinearSpan, LinearTerm, Variable,
};
use alloc::vec;
use core::iter::{Product, Sum};
use core::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// A constant coefficient.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Constant<R: UnitalSemiring> {
    pub(super) value: R,
}

impl<R: UnitalSemiring> Constant<R> {
    pub const ZERO: Self = Self { value: R::ZERO };
    pub const ONE: Self = Self { value: R::ONE };

    pub const fn new(value: R) -> Self {
        Self { value }
    }
}

impl<R: UnitalSemiring> Expression<R> for Constant<R> {
    fn span(self) -> LinearSpan<R> {
        vec![self.into()].into()
    }

    fn degree(&self) -> usize {
        0
    }
}

impl<R: UnitalSemiring> From<R> for Constant<R> {
    fn from(value: R) -> Self {
        Self { value }
    }
}

impl<R: UnitalSemiring> Add for Constant<R> {
    type Output = Self;

    fn add(self, rps: Self) -> Self::Output {
        Self {
            value: self.value + rps.value,
        }
    }
}

impl<R: UnitalSemiring> Add<&Self> for Constant<R> {
    type Output = Self;

    fn add(self, rps: &Self) -> Self::Output {
        Self {
            value: self.value + &rps.value,
        }
    }
}

impl<R: UnitalSemiring> Add<Constant<R>> for &Constant<R>
where
    for<'a> &'a R: SemiringOps<R>,
{
    type Output = Constant<R>;

    fn add(self, rps: Constant<R>) -> Self::Output {
        Self::Output {
            value: &self.value + rps.value,
        }
    }
}

impl<R: UnitalSemiring> Add for &Constant<R>
where
    for<'a> &'a R: SemiringOps<R>,
{
    type Output = Constant<R>;

    fn add(self, rps: Self) -> Self::Output {
        Self::Output {
            value: &self.value + &rps.value,
        }
    }
}

impl<R: UnitalSemiring> AddAssign for Constant<R> {
    fn add_assign(&mut self, rps: Self) {
        self.value += rps.value
    }
}

impl<R: UnitalSemiring> AddAssign<&Self> for Constant<R> {
    fn add_assign(&mut self, rps: &Self) {
        self.value += &rps.value
    }
}

impl<R: UnitalSemiring> Double for Constant<R> {
    type Output = Self;

    fn double(self) -> Self::Output {
        Self {
            value: self.value.double(),
        }
    }
}

impl<R: UnitalSemiring> Double for &Constant<R>
where
    for<'a> &'a R: SemiringOps<R>,
{
    type Output = Constant<R>;

    fn double(self) -> Self::Output {
        Self::Output {
            value: (&self.value).double(),
        }
    }
}

impl<R: UnitalRing> Neg for Constant<R> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self { value: -self.value }
    }
}

impl<R: UnitalRing> Neg for &Constant<R>
where
    for<'a> &'a R: RingOps<R>,
{
    type Output = Constant<R>;

    fn neg(self) -> Self::Output {
        Self::Output {
            value: -&self.value,
        }
    }
}

impl<R: UnitalRing> Sub for Constant<R> {
    type Output = Self;

    fn sub(self, rps: Constant<R>) -> Self::Output {
        Self {
            value: self.value - rps.value,
        }
    }
}

impl<R: UnitalRing> Sub<&Self> for Constant<R> {
    type Output = Self;

    fn sub(self, rps: &Self) -> Self::Output {
        Self {
            value: self.value - &rps.value,
        }
    }
}

impl<R: UnitalRing> Sub<Constant<R>> for &Constant<R>
where
    for<'a> &'a R: RingOps<R>,
{
    type Output = Constant<R>;

    fn sub(self, rps: Constant<R>) -> Self::Output {
        Self::Output {
            value: &self.value - rps.value,
        }
    }
}

impl<R: UnitalRing> Sub for &Constant<R>
where
    for<'a> &'a R: RingOps<R>,
{
    type Output = Constant<R>;

    fn sub(self, rps: Self) -> Self::Output {
        Self::Output {
            value: &self.value - &rps.value,
        }
    }
}

impl<R: UnitalRing> SubAssign for Constant<R> {
    fn sub_assign(&mut self, rps: Self) {
        self.value -= rps.value
    }
}

impl<R: UnitalRing> SubAssign<&Self> for Constant<R> {
    fn sub_assign(&mut self, rps: &Self) {
        self.value -= &rps.value
    }
}

impl<R: UnitalSemiring> Mul for Constant<R> {
    type Output = Self;

    fn mul(self, rps: Constant<R>) -> Self::Output {
        Self {
            value: self.value * rps.value,
        }
    }
}

impl<R: UnitalSemiring> Mul<&Constant<R>> for Constant<R> {
    type Output = Self;

    fn mul(self, rps: &Constant<R>) -> Self::Output {
        Self {
            value: self.value * &rps.value,
        }
    }
}

impl<R: UnitalSemiring> Mul<Constant<R>> for &Constant<R>
where
    for<'a> &'a R: SemiringOps<R>,
{
    type Output = Constant<R>;

    fn mul(self, rps: Constant<R>) -> Self::Output {
        Self::Output {
            value: &self.value * rps.value,
        }
    }
}

impl<R: UnitalSemiring> Mul for &Constant<R>
where
    for<'a> &'a R: SemiringOps<R>,
{
    type Output = Constant<R>;

    fn mul(self, rps: Self) -> Self::Output {
        Self::Output {
            value: &self.value * &rps.value,
        }
    }
}

impl<R: UnitalSemiring> MulAssign for Constant<R> {
    fn mul_assign(&mut self, rps: Self) {
        self.value *= rps.value
    }
}

impl<R: UnitalSemiring> MulAssign<&Self> for Constant<R> {
    fn mul_assign(&mut self, rps: &Self) {
        self.value *= &rps.value
    }
}

impl<R: UnitalSemiring> Square for Constant<R> {
    type Output = Self;

    fn square(self) -> Self::Output {
        Self {
            value: self.value.square(),
        }
    }
}

impl<R: UnitalSemiring> Square for &Constant<R>
where
    for<'a> &'a R: SemiringOps<R>,
{
    type Output = Constant<R>;

    fn square(self) -> Self::Output {
        Self::Output {
            value: (&self.value).square(),
        }
    }
}

impl<R: UnitalSemiring> Add<Variable<R>> for Constant<R> {
    type Output = LinearCombination<R>;

    fn add(self, rps: Variable<R>) -> Self::Output {
        LinearCombination::with_terms([
            (Variable::CONSTANT, self).into(),
            (rps, Constant::ONE).into(),
        ])
    }
}

impl<R: UnitalSemiring + Clone> Add<Variable<R>> for &Constant<R> {
    type Output = LinearCombination<R>;

    fn add(self, rps: Variable<R>) -> Self::Output {
        self.clone() + rps
    }
}

impl<R: UnitalRing> Sub<Variable<R>> for Constant<R> {
    type Output = LinearCombination<R>;

    fn sub(self, rps: Variable<R>) -> Self::Output {
        LinearCombination::with_terms([
            (Variable::CONSTANT, self).into(),
            (rps, -Constant::ONE).into(),
        ])
    }
}

impl<R: UnitalRing + Clone> Sub<Variable<R>> for &Constant<R> {
    type Output = LinearCombination<R>;

    fn sub(self, rps: Variable<R>) -> Self::Output {
        self.clone() - rps
    }
}

impl<R: UnitalSemiring> Mul<Variable<R>> for Constant<R> {
    type Output = LinearTerm<R>;

    fn mul(self, rps: Variable<R>) -> Self::Output {
        (rps, self).into()
    }
}

impl<R: UnitalSemiring + Clone> Mul<Variable<R>> for &Constant<R> {
    type Output = LinearTerm<R>;

    fn mul(self, rps: Variable<R>) -> Self::Output {
        self.clone() * rps
    }
}

impl<R: UnitalSemiring> Add<LinearTerm<R>> for Constant<R> {
    type Output = LinearCombination<R>;

    fn add(self, rps: LinearTerm<R>) -> Self::Output {
        LinearCombination::with_terms([(Variable::CONSTANT, self).into(), rps])
    }
}

impl<R: UnitalSemiring + Clone> Add<LinearTerm<R>> for &Constant<R> {
    type Output = LinearCombination<R>;

    fn add(self, rps: LinearTerm<R>) -> Self::Output {
        self.clone() + rps
    }
}

impl<R: UnitalSemiring + Clone> Add<&LinearTerm<R>> for Constant<R> {
    type Output = LinearCombination<R>;

    fn add(self, rps: &LinearTerm<R>) -> Self::Output {
        self + rps.clone()
    }
}

impl<R: UnitalSemiring + Clone> Add<&LinearTerm<R>> for &Constant<R> {
    type Output = LinearCombination<R>;

    fn add(self, rps: &LinearTerm<R>) -> Self::Output {
        self.clone() + rps.clone()
    }
}

impl<R: UnitalRing> Sub<LinearTerm<R>> for Constant<R> {
    type Output = LinearCombination<R>;

    fn sub(self, rps: LinearTerm<R>) -> Self::Output {
        LinearCombination::with_terms([(Variable::CONSTANT, self).into(), -rps])
    }
}

impl<R: UnitalRing + Clone> Sub<LinearTerm<R>> for &Constant<R> {
    type Output = LinearCombination<R>;

    fn sub(self, rps: LinearTerm<R>) -> Self::Output {
        self.clone() - rps
    }
}

impl<R: UnitalRing + Clone> Sub<&LinearTerm<R>> for Constant<R> {
    type Output = LinearCombination<R>;

    fn sub(self, rps: &LinearTerm<R>) -> Self::Output {
        self - rps.clone()
    }
}

impl<R: UnitalRing + Clone> Sub<&LinearTerm<R>> for &Constant<R> {
    type Output = LinearCombination<R>;

    fn sub(self, rps: &LinearTerm<R>) -> Self::Output {
        self.clone() - rps.clone()
    }
}

impl<R: UnitalSemiring> Mul<LinearTerm<R>> for Constant<R> {
    type Output = LinearTerm<R>;

    fn mul(self, rps: LinearTerm<R>) -> Self::Output {
        (rps.variable, self * rps.coefficient).into()
    }
}

impl<R: UnitalSemiring> Mul<LinearTerm<R>> for &Constant<R>
where
    for<'a> &'a R: SemiringOps<R>,
{
    type Output = LinearTerm<R>;

    fn mul(self, rps: LinearTerm<R>) -> Self::Output {
        (rps.variable, self * rps.coefficient).into()
    }
}

impl<R: UnitalSemiring + Clone> Mul<&LinearTerm<R>> for Constant<R> {
    type Output = LinearTerm<R>;

    fn mul(self, rps: &LinearTerm<R>) -> Self::Output {
        self * rps.clone()
    }
}

impl<R: UnitalSemiring + Clone> Mul<&LinearTerm<R>> for &Constant<R>
where
    for<'a> &'a R: SemiringOps<R>,
{
    type Output = LinearTerm<R>;

    fn mul(self, rps: &LinearTerm<R>) -> Self::Output {
        self * rps.clone()
    }
}

impl<R: UnitalSemiring> Add<LinearCombination<R>> for Constant<R> {
    type Output = LinearCombination<R>;

    fn add(self, mut rps: LinearCombination<R>) -> Self::Output {
        rps += LinearTerm::new(Variable::CONSTANT, self);
        rps
    }
}

impl<R: UnitalSemiring + Clone> Add<LinearCombination<R>> for &Constant<R> {
    type Output = LinearCombination<R>;

    fn add(self, rps: LinearCombination<R>) -> Self::Output {
        self.clone() + rps
    }
}

impl<R: UnitalSemiring + Clone> Add<&LinearCombination<R>> for Constant<R> {
    type Output = LinearCombination<R>;

    fn add(self, rps: &LinearCombination<R>) -> Self::Output {
        self + rps.clone()
    }
}

impl<R: UnitalSemiring + Clone> Add<&LinearCombination<R>> for &Constant<R> {
    type Output = LinearCombination<R>;

    fn add(self, rps: &LinearCombination<R>) -> Self::Output {
        self.clone() + rps.clone()
    }
}

impl<R: UnitalRing> Sub<LinearCombination<R>> for Constant<R> {
    type Output = LinearCombination<R>;

    fn sub(self, mut rps: LinearCombination<R>) -> Self::Output {
        rps = -rps;
        rps += LinearTerm::new(Variable::CONSTANT, self);
        rps
    }
}

impl<R: UnitalRing + Clone> Sub<LinearCombination<R>> for &Constant<R> {
    type Output = LinearCombination<R>;

    fn sub(self, rps: LinearCombination<R>) -> Self::Output {
        self.clone() - rps
    }
}

impl<R: UnitalRing + Clone> Sub<&LinearCombination<R>> for Constant<R> {
    type Output = LinearCombination<R>;

    fn sub(self, rps: &LinearCombination<R>) -> Self::Output {
        self - rps.clone()
    }
}

impl<R: UnitalRing + Clone> Sub<&LinearCombination<R>> for &Constant<R> {
    type Output = LinearCombination<R>;

    fn sub(self, rps: &LinearCombination<R>) -> Self::Output {
        self.clone() - rps.clone()
    }
}

impl<R: UnitalSemiring> Mul<LinearCombination<R>> for Constant<R>
where
    for<'a> &'a R: SemiringOps<R>,
{
    type Output = LinearCombination<R>;

    fn mul(self, rps: LinearCombination<R>) -> Self::Output {
        &self * rps
    }
}

impl<R: UnitalSemiring> Mul<LinearCombination<R>> for &Constant<R>
where
    for<'a> &'a R: SemiringOps<R>,
{
    type Output = LinearCombination<R>;

    fn mul(self, rps: LinearCombination<R>) -> Self::Output {
        LinearCombination::<R> {
            terms: rps.terms.into_iter().map(|r| self * r).collect(),
        }
    }
}

impl<R: UnitalSemiring + Clone> Mul<&LinearCombination<R>> for Constant<R>
where
    for<'a> &'a R: SemiringOps<R>,
{
    type Output = LinearCombination<R>;

    fn mul(self, rps: &LinearCombination<R>) -> Self::Output {
        &self * rps.clone()
    }
}

impl<R: UnitalSemiring + Clone> Mul<&LinearCombination<R>> for &Constant<R>
where
    for<'a> &'a R: SemiringOps<R>,
{
    type Output = LinearCombination<R>;

    fn mul(self, rps: &LinearCombination<R>) -> Self::Output {
        self * rps.clone()
    }
}

impl<R: UnitalSemiring + Clone> Mul<LinearMonoid<R>> for Constant<R>
where
    for<'a> &'a R: SemiringOps<R>,
{
    type Output = LinearMonoid<R>;

    fn mul(self, mut rps: LinearMonoid<R>) -> Self::Output {
        match rps.factors.front_mut() {
            Some(factor) => *factor = self * &*factor,
            None => rps.factors.push_front(self.into()),
        }
        rps
    }
}

impl<R: UnitalSemiring + Clone> Mul<LinearMonoid<R>> for &Constant<R>
where
    for<'a> &'a R: SemiringOps<R>,
{
    type Output = LinearMonoid<R>;

    fn mul(self, mut rps: LinearMonoid<R>) -> Self::Output {
        match rps.factors.front_mut() {
            Some(factor) => *factor = self * &*factor,
            None => rps.factors.push_front(self.clone().into()),
        }
        rps
    }
}

impl<R: UnitalSemiring + Clone> Mul<&LinearMonoid<R>> for Constant<R>
where
    for<'a> &'a R: SemiringOps<R>,
{
    type Output = LinearMonoid<R>;

    fn mul(self, rps: &LinearMonoid<R>) -> Self::Output {
        self * rps.clone()
    }
}

impl<R: UnitalSemiring + Clone> Mul<&LinearMonoid<R>> for &Constant<R>
where
    for<'a> &'a R: SemiringOps<R>,
{
    type Output = LinearMonoid<R>;

    fn mul(self, rps: &LinearMonoid<R>) -> Self::Output {
        self * rps.clone()
    }
}

impl<R: UnitalSemiring> Sum for Constant<R> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        Self {
            value: iter.map(|constant| constant.value).sum(),
        }
    }
}

impl<'a, R: UnitalSemiring> Sum<&'a Self> for Constant<R> {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        Self {
            value: iter.map(|constant| &constant.value).sum(),
        }
    }
}

impl<R: UnitalSemiring> Product for Constant<R> {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        Self {
            value: iter.map(|constant| constant.value).product(),
        }
    }
}

impl<'a, R: UnitalSemiring> Product<&'a Self> for Constant<R> {
    fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        Self {
            value: iter.map(|constant| &constant.value).product(),
        }
    }
}
