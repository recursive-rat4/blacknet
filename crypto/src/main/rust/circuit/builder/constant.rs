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

use crate::algebra::{Double, Semiring, Square, UnitalRing};
use crate::circuit::builder::{
    Expression, LinearCombination, LinearMonoid, LinearSpan, LinearTerm, Variable,
};
use alloc::vec;
use core::iter::{Product, Sum};
use core::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// A constant coefficient.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Constant<R: Semiring> {
    pub(super) value: R,
}

impl<R: Semiring> Constant<R> {
    pub const ZERO: Self = Self::new(R::ZERO);
    pub const ONE: Self = Self::new(R::ONE);

    pub const fn new(value: R) -> Self {
        Self { value }
    }
}

impl<'a, R: Semiring + 'a> Expression<'a, R> for Constant<R> {
    fn span(&self) -> LinearSpan<R> {
        vec![(*self).into()].into()
    }

    fn degree(&self) -> usize {
        0
    }
}

impl<R: Semiring> From<R> for Constant<R> {
    fn from(value: R) -> Self {
        Self { value }
    }
}

impl<R: Semiring> Add for Constant<R> {
    type Output = Self;

    fn add(self, rps: Self) -> Self::Output {
        Self {
            value: self.value + rps.value,
        }
    }
}

impl<R: Semiring> Add<&Self> for Constant<R> {
    type Output = Self;

    fn add(self, rps: &Self) -> Self::Output {
        Self {
            value: self.value + rps.value,
        }
    }
}

impl<R: Semiring> Add<Constant<R>> for &Constant<R> {
    type Output = Constant<R>;

    fn add(self, rps: Constant<R>) -> Self::Output {
        Self::Output {
            value: self.value + rps.value,
        }
    }
}

impl<R: Semiring> Add for &Constant<R> {
    type Output = Constant<R>;

    fn add(self, rps: Self) -> Self::Output {
        Self::Output {
            value: self.value + rps.value,
        }
    }
}

impl<R: Semiring> AddAssign for Constant<R> {
    fn add_assign(&mut self, rps: Self) {
        self.value += rps.value
    }
}

impl<R: Semiring> AddAssign<&Self> for Constant<R> {
    fn add_assign(&mut self, rps: &Self) {
        self.value += rps.value
    }
}

impl<R: Semiring> Double for Constant<R> {
    type Output = Self;

    fn double(self) -> Self::Output {
        Self {
            value: self.value.double(),
        }
    }
}

impl<R: Semiring> Double for &Constant<R> {
    type Output = Constant<R>;

    fn double(self) -> Self::Output {
        Self::Output {
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

impl<R: UnitalRing> Neg for &Constant<R> {
    type Output = Constant<R>;

    fn neg(self) -> Self::Output {
        Self::Output { value: -self.value }
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
            value: self.value - rps.value,
        }
    }
}

impl<R: UnitalRing> Sub<Constant<R>> for &Constant<R> {
    type Output = Constant<R>;

    fn sub(self, rps: Constant<R>) -> Self::Output {
        Self::Output {
            value: self.value - rps.value,
        }
    }
}

impl<R: UnitalRing> Sub for &Constant<R> {
    type Output = Constant<R>;

    fn sub(self, rps: Self) -> Self::Output {
        Self::Output {
            value: self.value - rps.value,
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
        self.value -= rps.value
    }
}

impl<R: Semiring> Mul for Constant<R> {
    type Output = Self;

    fn mul(self, rps: Constant<R>) -> Self::Output {
        Self {
            value: self.value * rps.value,
        }
    }
}

impl<R: Semiring> Mul<&Constant<R>> for Constant<R> {
    type Output = Self;

    fn mul(self, rps: &Constant<R>) -> Self::Output {
        Self {
            value: self.value * rps.value,
        }
    }
}

impl<R: Semiring> Mul<Constant<R>> for &Constant<R> {
    type Output = Constant<R>;

    fn mul(self, rps: Constant<R>) -> Self::Output {
        Self::Output {
            value: self.value * rps.value,
        }
    }
}

impl<R: Semiring> Mul for &Constant<R> {
    type Output = Constant<R>;

    fn mul(self, rps: Self) -> Self::Output {
        Self::Output {
            value: self.value * rps.value,
        }
    }
}

impl<R: Semiring> MulAssign for Constant<R> {
    fn mul_assign(&mut self, rps: Self) {
        self.value *= rps.value
    }
}

impl<R: Semiring> MulAssign<&Self> for Constant<R> {
    fn mul_assign(&mut self, rps: &Self) {
        self.value *= rps.value
    }
}

impl<R: Semiring> Square for Constant<R> {
    type Output = Self;

    fn square(self) -> Self::Output {
        Self {
            value: self.value.square(),
        }
    }
}

impl<R: Semiring> Square for &Constant<R> {
    type Output = Constant<R>;

    fn square(self) -> Self::Output {
        Self::Output {
            value: self.value.square(),
        }
    }
}

impl<R: Semiring> Add<Variable<R>> for Constant<R> {
    type Output = LinearCombination<R>;

    fn add(self, rps: Variable<R>) -> Self::Output {
        [
            (Variable::CONSTANT, self).into(),
            (rps, Constant::ONE).into(),
        ]
        .into()
    }
}

impl<R: Semiring> Add<Variable<R>> for &Constant<R> {
    type Output = LinearCombination<R>;

    fn add(self, rps: Variable<R>) -> Self::Output {
        *self + rps
    }
}

impl<R: UnitalRing> Sub<Variable<R>> for Constant<R> {
    type Output = LinearCombination<R>;

    fn sub(self, rps: Variable<R>) -> Self::Output {
        [
            (Variable::CONSTANT, self).into(),
            (rps, -Constant::ONE).into(),
        ]
        .into()
    }
}

impl<R: UnitalRing> Sub<Variable<R>> for &Constant<R> {
    type Output = LinearCombination<R>;

    fn sub(self, rps: Variable<R>) -> Self::Output {
        *self - rps
    }
}

impl<R: Semiring> Mul<Variable<R>> for Constant<R> {
    type Output = LinearTerm<R>;

    fn mul(self, rps: Variable<R>) -> Self::Output {
        (rps, self).into()
    }
}

impl<R: Semiring> Mul<Variable<R>> for &Constant<R> {
    type Output = LinearTerm<R>;

    fn mul(self, rps: Variable<R>) -> Self::Output {
        *self * rps
    }
}

impl<R: Semiring> Add<LinearTerm<R>> for Constant<R> {
    type Output = LinearCombination<R>;

    fn add(self, rps: LinearTerm<R>) -> Self::Output {
        [(Variable::CONSTANT, self).into(), rps].into()
    }
}

impl<R: Semiring> Add<LinearTerm<R>> for &Constant<R> {
    type Output = LinearCombination<R>;

    fn add(self, rps: LinearTerm<R>) -> Self::Output {
        *self + rps
    }
}

impl<R: Semiring> Add<&LinearTerm<R>> for Constant<R> {
    type Output = LinearCombination<R>;

    fn add(self, rps: &LinearTerm<R>) -> Self::Output {
        self + *rps
    }
}

impl<R: Semiring> Add<&LinearTerm<R>> for &Constant<R> {
    type Output = LinearCombination<R>;

    fn add(self, rps: &LinearTerm<R>) -> Self::Output {
        *self + *rps
    }
}

impl<R: UnitalRing> Sub<LinearTerm<R>> for Constant<R> {
    type Output = LinearCombination<R>;

    fn sub(self, rps: LinearTerm<R>) -> Self::Output {
        [(Variable::CONSTANT, self).into(), -rps].into()
    }
}

impl<R: UnitalRing> Sub<LinearTerm<R>> for &Constant<R> {
    type Output = LinearCombination<R>;

    fn sub(self, rps: LinearTerm<R>) -> Self::Output {
        *self - rps
    }
}

impl<R: UnitalRing> Sub<&LinearTerm<R>> for Constant<R> {
    type Output = LinearCombination<R>;

    fn sub(self, rps: &LinearTerm<R>) -> Self::Output {
        self - *rps
    }
}

impl<R: UnitalRing> Sub<&LinearTerm<R>> for &Constant<R> {
    type Output = LinearCombination<R>;

    fn sub(self, rps: &LinearTerm<R>) -> Self::Output {
        *self - *rps
    }
}

impl<R: Semiring> Mul<LinearTerm<R>> for Constant<R> {
    type Output = LinearTerm<R>;

    fn mul(self, rps: LinearTerm<R>) -> Self::Output {
        (rps.variable, self * rps.coefficient).into()
    }
}

impl<R: Semiring> Mul<LinearTerm<R>> for &Constant<R> {
    type Output = LinearTerm<R>;

    fn mul(self, rps: LinearTerm<R>) -> Self::Output {
        *self * rps
    }
}

impl<R: Semiring> Mul<&LinearTerm<R>> for Constant<R> {
    type Output = LinearTerm<R>;

    fn mul(self, rps: &LinearTerm<R>) -> Self::Output {
        self * *rps
    }
}

impl<R: Semiring> Mul<&LinearTerm<R>> for &Constant<R> {
    type Output = LinearTerm<R>;

    fn mul(self, rps: &LinearTerm<R>) -> Self::Output {
        *self * *rps
    }
}

impl<R: Semiring> Add<LinearCombination<R>> for Constant<R> {
    type Output = LinearCombination<R>;

    fn add(self, mut rps: LinearCombination<R>) -> Self::Output {
        rps += LinearTerm::new(Variable::CONSTANT, self);
        rps
    }
}

impl<R: Semiring> Add<LinearCombination<R>> for &Constant<R> {
    type Output = LinearCombination<R>;

    fn add(self, rps: LinearCombination<R>) -> Self::Output {
        *self + rps
    }
}

impl<R: Semiring> Add<&LinearCombination<R>> for Constant<R> {
    type Output = LinearCombination<R>;

    fn add(self, rps: &LinearCombination<R>) -> Self::Output {
        self + rps.clone()
    }
}

impl<R: Semiring> Add<&LinearCombination<R>> for &Constant<R> {
    type Output = LinearCombination<R>;

    fn add(self, rps: &LinearCombination<R>) -> Self::Output {
        *self + rps.clone()
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

impl<R: UnitalRing> Sub<LinearCombination<R>> for &Constant<R> {
    type Output = LinearCombination<R>;

    fn sub(self, rps: LinearCombination<R>) -> Self::Output {
        *self - rps
    }
}

impl<R: UnitalRing> Sub<&LinearCombination<R>> for Constant<R> {
    type Output = LinearCombination<R>;

    fn sub(self, rps: &LinearCombination<R>) -> Self::Output {
        self - rps.clone()
    }
}

impl<R: UnitalRing> Sub<&LinearCombination<R>> for &Constant<R> {
    type Output = LinearCombination<R>;

    fn sub(self, rps: &LinearCombination<R>) -> Self::Output {
        *self - rps.clone()
    }
}

impl<R: Semiring> Mul<LinearCombination<R>> for Constant<R> {
    type Output = LinearCombination<R>;

    fn mul(self, mut rps: LinearCombination<R>) -> Self::Output {
        for coefficient in rps.terms.values_mut() {
            *coefficient = self * *coefficient
        }
        rps
    }
}

impl<R: Semiring> Mul<LinearCombination<R>> for &Constant<R> {
    type Output = LinearCombination<R>;

    fn mul(self, rps: LinearCombination<R>) -> Self::Output {
        *self * rps
    }
}

impl<R: Semiring> Mul<&LinearCombination<R>> for Constant<R> {
    type Output = LinearCombination<R>;

    fn mul(self, rps: &LinearCombination<R>) -> Self::Output {
        let mut lc = LinearCombination::new();
        for (&variable, coefficient) in &rps.terms {
            lc += LinearTerm::new(variable, self * coefficient)
        }
        lc
    }
}

impl<R: Semiring> Mul<&LinearCombination<R>> for &Constant<R> {
    type Output = LinearCombination<R>;

    fn mul(self, rps: &LinearCombination<R>) -> Self::Output {
        *self * rps
    }
}

impl<R: Semiring> Mul<LinearMonoid<R>> for Constant<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, mut rps: LinearMonoid<R>) -> Self::Output {
        match rps.factors.front_mut() {
            Some(factor) => *factor = self * &*factor,
            None => rps.factors.push_front(self.into()),
        }
        rps
    }
}

impl<R: Semiring> Mul<LinearMonoid<R>> for &Constant<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, rps: LinearMonoid<R>) -> Self::Output {
        *self * rps
    }
}

impl<R: Semiring> Mul<&LinearMonoid<R>> for Constant<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, rps: &LinearMonoid<R>) -> Self::Output {
        self * rps.clone()
    }
}

impl<R: Semiring> Mul<&LinearMonoid<R>> for &Constant<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, rps: &LinearMonoid<R>) -> Self::Output {
        *self * rps.clone()
    }
}

impl<R: Semiring> Sum for Constant<R> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        Self {
            value: iter.map(|constant| constant.value).sum(),
        }
    }
}

impl<'a, R: Semiring> Sum<&'a Self> for Constant<R> {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        Self {
            value: iter.map(|constant| constant.value).sum(),
        }
    }
}

impl<R: Semiring> Product for Constant<R> {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        Self {
            value: iter.map(|constant| constant.value).product(),
        }
    }
}

impl<'a, R: Semiring> Product<&'a Self> for Constant<R> {
    fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        Self {
            value: iter.map(|constant| constant.value).product(),
        }
    }
}
