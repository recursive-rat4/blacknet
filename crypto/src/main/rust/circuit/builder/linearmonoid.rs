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

use crate::algebra::Semiring;
use crate::algebra::Square;
use crate::circuit::builder::{
    Constant, Expression, LinearCombination, LinearSpan, LinearTerm, Variable,
};
use alloc::collections::VecDeque;
use core::ops::{Mul, MulAssign};

/// Linear monoid is a product of linear combinations.
#[derive(Clone)]
pub struct LinearMonoid<R: Semiring> {
    pub(super) factors: VecDeque<LinearCombination<R>>,
}

impl<'a, R: Semiring + Eq + 'a> Expression<'a, R> for LinearMonoid<R> {
    fn span(&self) -> LinearSpan<R> {
        self.factors.clone().into()
    }

    fn degree(&self) -> usize {
        self.factors.iter().map(Expression::degree).sum()
    }
}

impl<R: Semiring, const N: usize> From<[LinearCombination<R>; N]> for LinearMonoid<R> {
    fn from(factors: [LinearCombination<R>; N]) -> Self {
        Self {
            factors: factors.into(),
        }
    }
}

impl<R: Semiring> Mul<Constant<R>> for LinearMonoid<R> {
    type Output = Self;

    fn mul(mut self, rps: Constant<R>) -> Self::Output {
        self *= rps;
        self
    }
}

impl<R: Semiring> MulAssign<Constant<R>> for LinearMonoid<R> {
    fn mul_assign(&mut self, rps: Constant<R>) {
        match self.factors.back_mut() {
            Some(factor) => *factor *= rps,
            None => self.factors.push_back(rps.into()),
        }
    }
}

impl<R: Semiring> Mul<Constant<R>> for &LinearMonoid<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, rps: Constant<R>) -> Self::Output {
        self.clone() * rps
    }
}

impl<R: Semiring> Mul<Variable<R>> for LinearMonoid<R> {
    type Output = Self;

    fn mul(mut self, rps: Variable<R>) -> Self::Output {
        self *= rps;
        self
    }
}

impl<R: Semiring> MulAssign<Variable<R>> for LinearMonoid<R> {
    fn mul_assign(&mut self, rps: Variable<R>) {
        self.factors.push_back(rps.into())
    }
}

impl<R: Semiring> Mul<Variable<R>> for &LinearMonoid<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, rps: Variable<R>) -> Self::Output {
        self.clone() * rps
    }
}

impl<R: Semiring> Mul<LinearTerm<R>> for LinearMonoid<R> {
    type Output = Self;

    fn mul(mut self, rps: LinearTerm<R>) -> Self::Output {
        self *= rps;
        self
    }
}

impl<R: Semiring> MulAssign<LinearTerm<R>> for LinearMonoid<R> {
    fn mul_assign(&mut self, rps: LinearTerm<R>) {
        self.factors.push_back(rps.into())
    }
}

impl<R: Semiring> Mul<LinearTerm<R>> for &LinearMonoid<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, rps: LinearTerm<R>) -> Self::Output {
        self.clone() * rps
    }
}

impl<R: Semiring> Mul<LinearCombination<R>> for LinearMonoid<R> {
    type Output = Self;

    fn mul(mut self, rps: LinearCombination<R>) -> Self::Output {
        self *= rps;
        self
    }
}

impl<R: Semiring> MulAssign<LinearCombination<R>> for LinearMonoid<R> {
    fn mul_assign(&mut self, rps: LinearCombination<R>) {
        self.factors.push_back(rps)
    }
}

impl<R: Semiring> Mul<LinearCombination<R>> for &LinearMonoid<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, rps: LinearCombination<R>) -> Self::Output {
        self.clone() * rps
    }
}

impl<R: Semiring> Mul<&LinearCombination<R>> for LinearMonoid<R> {
    type Output = Self;

    fn mul(mut self, rps: &LinearCombination<R>) -> Self::Output {
        self *= rps;
        self
    }
}

impl<R: Semiring> MulAssign<&LinearCombination<R>> for LinearMonoid<R> {
    fn mul_assign(&mut self, rps: &LinearCombination<R>) {
        self.factors.push_back(rps.clone())
    }
}

impl<R: Semiring> Mul for LinearMonoid<R> {
    type Output = Self;

    fn mul(mut self, rps: Self) -> Self::Output {
        self *= rps;
        self
    }
}

impl<R: Semiring> MulAssign for LinearMonoid<R> {
    fn mul_assign(&mut self, mut rps: Self) {
        self.factors.append(&mut rps.factors)
    }
}

impl<R: Semiring> Mul<&Self> for LinearMonoid<R> {
    type Output = Self;

    fn mul(mut self, rps: &Self) -> Self::Output {
        self *= rps.clone();
        self
    }
}

impl<R: Semiring> MulAssign<&Self> for LinearMonoid<R> {
    fn mul_assign(&mut self, rps: &Self) {
        self.factors.extend(rps.factors.iter().cloned())
    }
}

impl<R: Semiring> Mul<LinearMonoid<R>> for &LinearMonoid<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, rps: LinearMonoid<R>) -> Self::Output {
        self.clone() * rps
    }
}

impl<R: Semiring> Mul for &LinearMonoid<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, rps: Self) -> Self::Output {
        self.clone() * rps.clone()
    }
}

impl<R: Semiring> Square for LinearMonoid<R> {
    type Output = LinearMonoid<R>;

    fn square(self) -> Self::Output {
        self.clone() * self
    }
}

impl<R: Semiring> Square for &LinearMonoid<R> {
    type Output = LinearMonoid<R>;

    fn square(self) -> Self::Output {
        self * self
    }
}
