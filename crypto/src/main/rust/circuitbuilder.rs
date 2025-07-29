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
use core::ops::{Add, Mul, Neg, Sub};
use std::collections::BTreeMap;

pub struct Constant<R: Ring> {
    value: R,
}

impl<R: Ring> Constant<R> {
    pub const UNITY: Self = Self { value: R::UNITY };
    pub const ZERO: Self = Self { value: R::ZERO };
}

impl<R: Ring> Neg for Constant<R> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self { value: -self.value }
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

pub struct LinearCombination<R: Ring> {
    terms: BTreeMap<Variable<R>, Constant<R>>,
}

impl<R: Ring, const N: usize> From<[(Variable<R>, Constant<R>); N]> for LinearCombination<R> {
    fn from(terms: [(Variable<R>, Constant<R>); N]) -> Self {
        Self {
            terms: BTreeMap::from(terms),
        }
    }
}
