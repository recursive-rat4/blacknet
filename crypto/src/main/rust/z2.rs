/*
 * Copyright (c) 2025 Pavel Vasin
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

#![allow(clippy::suspicious_arithmetic_impl)]

use crate::integer::Integer;
use crate::magma::{
    AdditiveCommutativeMagma, AdditiveMagma, MultiplicativeCommutativeMagma, MultiplicativeMagma,
};
use crate::monoid::{AdditiveMonoid, MultiplicativeMonoid};
use crate::operation::{Double, Inv, Square};
use crate::ring::{DivisionRing, IntegerRing, Ring};
use crate::semigroup::{AdditiveSemigroup, MultiplicativeSemigroup};
use crate::semiring::{Presemiring, Semiring};
use core::fmt::{Debug, Formatter, Result};
use core::iter::{Product, Sum};
use core::ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub, SubAssign};

/// The quotient ring `ℤ/2ℤ`.
#[derive(Clone, Copy, Default, Eq, PartialEq)]
pub struct Z2 {
    n: bool,
}

impl Debug for Z2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.n as i8)
    }
}

impl From<bool> for Z2 {
    fn from(n: bool) -> Self {
        Self { n }
    }
}

impl Add for Z2 {
    type Output = Self;

    fn add(self, rps: Self) -> Self::Output {
        Self { n: self.n ^ rps.n }
    }
}

impl Add<&Self> for Z2 {
    type Output = Self;

    #[inline]
    fn add(self, rps: &Self) -> Self::Output {
        self + *rps
    }
}

impl AddAssign for Z2 {
    #[inline]
    fn add_assign(&mut self, rps: Self) {
        *self = *self + rps
    }
}

impl AddAssign<&Self> for Z2 {
    #[inline]
    fn add_assign(&mut self, rps: &Self) {
        *self = *self + *rps
    }
}

impl Double for Z2 {
    type Output = Self;

    #[inline]
    fn double(self) -> Self {
        Self::ZERO
    }
}

impl Neg for Z2 {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        self
    }
}

impl Sub for Z2 {
    type Output = Self;

    fn sub(self, rps: Self) -> Self::Output {
        Self { n: self.n ^ rps.n }
    }
}

impl Sub<&Self> for Z2 {
    type Output = Self;

    #[inline]
    fn sub(self, rps: &Self) -> Self::Output {
        self - *rps
    }
}

impl SubAssign for Z2 {
    #[inline]
    fn sub_assign(&mut self, rps: Self) {
        *self = *self - rps
    }
}

impl SubAssign<&Self> for Z2 {
    #[inline]
    fn sub_assign(&mut self, rps: &Self) {
        *self = *self - *rps
    }
}

impl Mul for Z2 {
    type Output = Self;

    fn mul(self, rps: Self) -> Self::Output {
        Self { n: self.n & rps.n }
    }
}

impl Mul<&Self> for Z2 {
    type Output = Self;

    #[inline]
    fn mul(self, rps: &Self) -> Self::Output {
        self * *rps
    }
}

impl MulAssign for Z2 {
    #[inline]
    fn mul_assign(&mut self, rps: Self) {
        *self = *self * rps
    }
}

impl MulAssign<&Self> for Z2 {
    #[inline]
    fn mul_assign(&mut self, rps: &Self) {
        *self = *self * *rps
    }
}

impl Square for Z2 {
    type Output = Self;

    #[inline]
    fn square(self) -> Self {
        self
    }
}

impl Inv for Z2 {
    type Output = Option<Self>;

    fn inv(self) -> Self::Output {
        match self.n {
            true => Some(Self::ONE),
            false => None,
        }
    }
}

impl Div for Z2 {
    type Output = Option<Self>;

    fn div(self, rps: Self) -> Self::Output {
        rps.inv().map(|v| self * v)
    }
}

impl Div<&Self> for Z2 {
    type Output = Option<Self>;

    #[inline]
    fn div(self, rps: &Self) -> Self::Output {
        self / *rps
    }
}

impl Sum for Z2 {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|lps, rps| lps + rps).unwrap_or(Self::ZERO)
    }
}

impl<'a> Sum<&'a Self> for Z2 {
    #[inline]
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.copied().sum()
    }
}

impl Product for Z2 {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|lps, rps| lps * rps).unwrap_or(Self::ONE)
    }
}

impl<'a> Product<&'a Self> for Z2 {
    #[inline]
    fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.copied().product()
    }
}

impl AdditiveMagma for Z2 {}

impl AdditiveCommutativeMagma for Z2 {}

impl AdditiveSemigroup for Z2 {
    const LEFT_IDENTITY: Self = Self { n: false };
    const RIGHT_IDENTITY: Self = Self { n: false };
}

impl AdditiveMonoid for Z2 {
    const IDENTITY: Self = Self { n: false };
}

impl MultiplicativeMagma for Z2 {}

impl MultiplicativeCommutativeMagma for Z2 {}

impl MultiplicativeSemigroup for Z2 {
    const LEFT_IDENTITY: Self = Self { n: true };
    const RIGHT_IDENTITY: Self = Self { n: true };
}

impl MultiplicativeMonoid for Z2 {
    const IDENTITY: Self = Self { n: true };
}

impl Ring for Z2 {
    type Int = i8;
}

impl DivisionRing for Z2 {}

impl IntegerRing for Z2 {
    fn new(n: Self::Int) -> Self {
        Self { n: (n & 1) == 1 }
    }
    fn with_limb(n: <Self::Int as Integer>::Limb) -> Self {
        Self::new(n)
    }

    fn canonical(self) -> Self::Int {
        self.n.into()
    }
    fn absolute(self) -> Self::Int {
        self.n.into()
    }

    const BITS: u32 = 1;
    const MODULUS: Self::Int = 2;
}
