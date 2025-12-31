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
use crate::algebra::UnitalRing;
use crate::algebra::{Double, Square};
use crate::circuit::builder::{
    Constant, Expression, LinearCombination, LinearMonoid, LinearSpan, LinearTerm,
};
use alloc::vec;
use core::cmp::Ordering;
use core::fmt::{Debug, Formatter, Result};
use core::marker::PhantomData;
use core::ops::{Add, Mul, Neg, Sub};

/// Layout of variables in assigment.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum VariableKind {
    Constant,
    PublicInput,
    PublicOutput,
    PrivateInput,
    PrivateOutput,
    Auxiliary,
}

/// An allocated variable.
#[derive(Clone, Copy)]
pub struct Variable<R: Semiring> {
    pub(super) kind: VariableKind,
    pub(super) number: usize,
    phantom: PhantomData<R>,
}

impl<R: Semiring> Variable<R> {
    pub(super) const fn new(kind: VariableKind, number: usize) -> Self {
        Self {
            kind,
            number,
            phantom: PhantomData,
        }
    }

    pub(super) const CONSTANT: Self = Self::new(VariableKind::Constant, 0);
}

impl<'a, R: Semiring + 'a> Expression<'a, R> for Variable<R> {
    fn span(&self) -> LinearSpan<R> {
        vec![(*self).into()].into()
    }

    fn degree(&self) -> usize {
        1
    }
}

impl<R: Semiring> Debug for Variable<R> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "Variable {{ kind: {:?}, number: {:?} }}",
            self.kind, self.number
        )
    }
}

impl<R: Semiring> PartialEq for Variable<R> {
    fn eq(&self, rps: &Self) -> bool {
        self.kind == rps.kind && self.number == rps.number
    }
}

impl<R: Semiring> Eq for Variable<R> {}

impl<R: Semiring> Ord for Variable<R> {
    fn cmp(&self, rps: &Self) -> Ordering {
        match self.kind.cmp(&rps.kind) {
            Ordering::Equal => self.number.cmp(&rps.number),
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
        }
    }
}

impl<R: Semiring> PartialOrd for Variable<R> {
    fn partial_cmp(&self, rps: &Self) -> Option<Ordering> {
        Some(self.cmp(rps))
    }
}

impl<R: Semiring> Add for Variable<R> {
    type Output = LinearCombination<R>;

    fn add(self, rps: Self) -> Self::Output {
        if self != rps {
            [(self, Constant::ONE).into(), (rps, Constant::ONE).into()].into()
        } else {
            [(self, Constant::ONE.double()).into()].into()
        }
    }
}

impl<R: Semiring> Double for Variable<R> {
    type Output = LinearTerm<R>;

    fn double(self) -> Self::Output {
        (self, Constant::ONE.double()).into()
    }
}

impl<R: UnitalRing> Neg for Variable<R> {
    type Output = LinearTerm<R>;

    fn neg(self) -> Self::Output {
        (self, -Constant::ONE).into()
    }
}

impl<R: UnitalRing> Sub for Variable<R> {
    type Output = LinearCombination<R>;

    fn sub(self, rps: Self) -> Self::Output {
        if self != rps {
            [(self, Constant::ONE).into(), (rps, -Constant::ONE).into()].into()
        } else {
            [].into()
        }
    }
}

impl<R: Semiring> Mul for Variable<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, rps: Self) -> Self::Output {
        [self.into(), rps.into()].into()
    }
}

impl<R: Semiring> Square for Variable<R> {
    type Output = LinearMonoid<R>;

    fn square(self) -> Self::Output {
        [self.into(), self.into()].into()
    }
}

impl<R: Semiring> Add<Constant<R>> for Variable<R> {
    type Output = LinearCombination<R>;

    fn add(self, rps: Constant<R>) -> Self::Output {
        [
            (self, Constant::ONE).into(),
            (Variable::CONSTANT, rps).into(),
        ]
        .into()
    }
}

impl<R: UnitalRing> Sub<Constant<R>> for Variable<R> {
    type Output = LinearCombination<R>;

    fn sub(self, rps: Constant<R>) -> Self::Output {
        [
            (self, Constant::ONE).into(),
            (Variable::CONSTANT, -rps).into(),
        ]
        .into()
    }
}

impl<R: Semiring> Mul<Constant<R>> for Variable<R> {
    type Output = LinearTerm<R>;

    fn mul(self, rps: Constant<R>) -> Self::Output {
        (self, rps).into()
    }
}

impl<R: Semiring> Add<LinearTerm<R>> for Variable<R> {
    type Output = LinearCombination<R>;

    fn add(self, rps: LinearTerm<R>) -> Self::Output {
        let mut lc: LinearCombination<R> = self.into();
        lc += rps;
        lc
    }
}

impl<R: UnitalRing> Sub<LinearTerm<R>> for Variable<R> {
    type Output = LinearCombination<R>;

    fn sub(self, rps: LinearTerm<R>) -> Self::Output {
        let mut lc: LinearCombination<R> = self.into();
        lc -= rps;
        lc
    }
}

impl<R: Semiring> Mul<LinearTerm<R>> for Variable<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, rps: LinearTerm<R>) -> Self::Output {
        [LinearTerm::new(self, Constant::ONE).into(), rps.into()].into()
    }
}

impl<R: Semiring> Add<LinearCombination<R>> for Variable<R> {
    type Output = LinearCombination<R>;

    fn add(self, mut rps: LinearCombination<R>) -> Self::Output {
        rps += LinearTerm::new(self, Constant::ONE);
        rps
    }
}

impl<R: Semiring> Add<&LinearCombination<R>> for Variable<R> {
    type Output = LinearCombination<R>;

    fn add(self, rps: &LinearCombination<R>) -> Self::Output {
        self + rps.clone()
    }
}

impl<R: UnitalRing> Sub<LinearCombination<R>> for Variable<R> {
    type Output = LinearCombination<R>;

    fn sub(self, mut rps: LinearCombination<R>) -> Self::Output {
        rps = -rps;
        rps += LinearTerm::new(self, Constant::ONE);
        rps
    }
}

impl<R: UnitalRing> Sub<&LinearCombination<R>> for Variable<R> {
    type Output = LinearCombination<R>;

    fn sub(self, rps: &LinearCombination<R>) -> Self::Output {
        self - rps.clone()
    }
}

impl<R: Semiring> Mul<LinearCombination<R>> for Variable<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, rps: LinearCombination<R>) -> Self::Output {
        [LinearTerm::new(self, Constant::ONE).into(), rps].into()
    }
}

impl<R: Semiring> Mul<&LinearCombination<R>> for Variable<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, rps: &LinearCombination<R>) -> Self::Output {
        self * rps.clone()
    }
}

impl<R: Semiring> Mul<LinearMonoid<R>> for Variable<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, mut rps: LinearMonoid<R>) -> Self::Output {
        rps.factors.push_front(self.into());
        rps
    }
}

impl<R: Semiring> Mul<&LinearMonoid<R>> for Variable<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, rps: &LinearMonoid<R>) -> Self::Output {
        self * rps.clone()
    }
}
