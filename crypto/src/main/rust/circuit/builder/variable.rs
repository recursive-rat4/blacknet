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

use crate::algebra::{Double, Square, UnitalRing, UnitalSemiring};
use crate::circuit::builder::{
    Constant, Expression, LinearCombination, LinearMonoid, LinearSpan, LinearTerm,
};
use alloc::vec;
use core::cmp::Ordering;
use core::fmt::{Debug, Display, Formatter, Result};
use core::marker::PhantomData;
use core::ops::{Add, Mul, Neg, Sub};

/// Kind of variables in assigment.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum VariableKind {
    /// Before public.
    Constant,
    /// Public input.
    Public,
    /// Secret input.
    Private,
    /// After private.
    Auxiliary,
}

/// An allocated variable.
pub struct Variable<R: UnitalSemiring> {
    data: u32,
    phantom: PhantomData<R>,
}

impl<R: UnitalSemiring> Variable<R> {
    pub(super) const fn new(kind: VariableKind, number: u32) -> Self {
        let data = (kind as u32) << 30 | number;
        Self {
            data,
            phantom: PhantomData,
        }
    }

    /// The kind of variable.
    pub const fn kind(&self) -> VariableKind {
        unsafe { core::mem::transmute((self.data >> 30) as u8) }
    }

    /// The number of variable within its kind.
    pub const fn number(&self) -> u32 {
        self.data & 0x3FFFFFFF
    }

    pub(super) const CONSTANT: Self = Self::new(VariableKind::Constant, 0);
}

impl<R: UnitalSemiring> Expression<R> for Variable<R> {
    fn span(self) -> LinearSpan<R> {
        vec![self.into()].into()
    }

    fn degree(&self) -> u32 {
        1
    }
}

impl<R: UnitalSemiring> Clone for Variable<R> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<R: UnitalSemiring> Copy for Variable<R> {}

impl<R: UnitalSemiring> Debug for Variable<R> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("Variable")
            .field("data", &format_args!("{:08X}", self.data))
            .finish()
    }
}

impl<R: UnitalSemiring> Display for Variable<R> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("Variable")
            .field("kind", &self.kind())
            .field("number", &self.number())
            .finish()
    }
}

impl<R: UnitalSemiring> PartialEq for Variable<R> {
    fn eq(&self, rps: &Self) -> bool {
        self.data.eq(&rps.data)
    }
}

impl<R: UnitalSemiring> Eq for Variable<R> {}

impl<R: UnitalSemiring> Ord for Variable<R> {
    fn cmp(&self, rps: &Self) -> Ordering {
        self.data.cmp(&rps.data)
    }
}

impl<R: UnitalSemiring> PartialOrd for Variable<R> {
    fn partial_cmp(&self, rps: &Self) -> Option<Ordering> {
        Some(self.cmp(rps))
    }
}

impl<R: UnitalSemiring> Add for Variable<R> {
    type Output = LinearCombination<R>;

    fn add(self, rps: Self) -> Self::Output {
        LinearCombination::with_terms([(self, Constant::ONE).into(), (rps, Constant::ONE).into()])
    }
}

impl<R: UnitalSemiring> Double for Variable<R> {
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
        LinearCombination::with_terms([(self, Constant::ONE).into(), (rps, -Constant::ONE).into()])
    }
}

impl<R: UnitalSemiring> Mul for Variable<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, rps: Self) -> Self::Output {
        [self.into(), rps.into()].into()
    }
}

impl<R: UnitalSemiring> Square for Variable<R> {
    type Output = LinearMonoid<R>;

    fn square(self) -> Self::Output {
        [self.into(), self.into()].into()
    }
}

impl<R: UnitalSemiring> Add<Constant<R>> for Variable<R> {
    type Output = LinearCombination<R>;

    fn add(self, rps: Constant<R>) -> Self::Output {
        LinearCombination::with_terms([
            (self, Constant::ONE).into(),
            (Variable::CONSTANT, rps).into(),
        ])
    }
}

impl<R: UnitalRing> Sub<Constant<R>> for Variable<R> {
    type Output = LinearCombination<R>;

    fn sub(self, rps: Constant<R>) -> Self::Output {
        LinearCombination::with_terms([
            (self, Constant::ONE).into(),
            (Variable::CONSTANT, -rps).into(),
        ])
    }
}

impl<R: UnitalSemiring> Mul<Constant<R>> for Variable<R> {
    type Output = LinearTerm<R>;

    fn mul(self, rps: Constant<R>) -> Self::Output {
        (self, rps).into()
    }
}

impl<R: UnitalSemiring> Add<LinearTerm<R>> for Variable<R> {
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

impl<R: UnitalSemiring> Mul<LinearTerm<R>> for Variable<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, rps: LinearTerm<R>) -> Self::Output {
        [LinearTerm::new(self, Constant::ONE).into(), rps.into()].into()
    }
}

impl<R: UnitalSemiring> Add<LinearCombination<R>> for Variable<R> {
    type Output = LinearCombination<R>;

    fn add(self, mut rps: LinearCombination<R>) -> Self::Output {
        rps += LinearTerm::new(self, Constant::ONE);
        rps
    }
}

impl<R: UnitalSemiring + Clone> Add<&LinearCombination<R>> for Variable<R> {
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

impl<R: UnitalRing + Clone> Sub<&LinearCombination<R>> for Variable<R> {
    type Output = LinearCombination<R>;

    fn sub(self, rps: &LinearCombination<R>) -> Self::Output {
        self - rps.clone()
    }
}

impl<R: UnitalSemiring> Mul<LinearCombination<R>> for Variable<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, rps: LinearCombination<R>) -> Self::Output {
        [LinearTerm::new(self, Constant::ONE).into(), rps].into()
    }
}

impl<R: UnitalSemiring + Clone> Mul<&LinearCombination<R>> for Variable<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, rps: &LinearCombination<R>) -> Self::Output {
        self * rps.clone()
    }
}

impl<R: UnitalSemiring> Mul<LinearMonoid<R>> for Variable<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, mut rps: LinearMonoid<R>) -> Self::Output {
        rps.factors.push_front(self.into());
        rps
    }
}

impl<R: UnitalSemiring + Clone> Mul<&LinearMonoid<R>> for Variable<R> {
    type Output = LinearMonoid<R>;

    fn mul(self, rps: &LinearMonoid<R>) -> Self::Output {
        self * rps.clone()
    }
}
