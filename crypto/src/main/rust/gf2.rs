/*
 * Copyright (c) 2025-2026 Pavel Vasin
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
#![allow(clippy::suspicious_op_assign_impl)]

use crate::algebra::{
    AdditiveCommutativeMagma, AdditiveMonoid, AdditiveSemigroup, BalancedRepresentative, Double,
    IntegerRing, Inv, LeftOne, LeftZero, MultiplicativeCommutativeMagma, MultiplicativeMonoid,
    MultiplicativeSemigroup, One, RightOne, RightZero, Semifield, Set, Sqrt, Square, Zero,
};
use crate::branchless::{BlAssign, BlEq, BlSelect};
use crate::integer::Integer;
use core::fmt::{Debug, Formatter, Result};
use core::iter::{Product, Sum};
use core::ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub, SubAssign};
use serde::{Deserialize, Serialize};
use zeroize::DefaultIsZeroes;

/// The quotient ring `ℤ/2ℤ`.
#[derive(Clone, Copy, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct GF2 {
    n: bool,
}

impl Debug for GF2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.n as i8)
    }
}

impl From<bool> for GF2 {
    fn from(n: bool) -> Self {
        Self { n }
    }
}

impl Add for GF2 {
    type Output = Self;

    fn add(self, rps: Self) -> Self::Output {
        Self { n: self.n ^ rps.n }
    }
}

impl Add<&Self> for GF2 {
    type Output = Self;

    fn add(self, rps: &Self) -> Self::Output {
        Self { n: self.n ^ rps.n }
    }
}

impl Add<GF2> for &GF2 {
    type Output = GF2;

    fn add(self, rps: GF2) -> Self::Output {
        Self::Output { n: self.n ^ rps.n }
    }
}

impl<'a> Add<&'a GF2> for &GF2 {
    type Output = GF2;

    fn add(self, rps: &'a GF2) -> Self::Output {
        Self::Output { n: self.n ^ rps.n }
    }
}

impl AddAssign for GF2 {
    fn add_assign(&mut self, rps: Self) {
        self.n ^= rps.n
    }
}

impl AddAssign<&Self> for GF2 {
    fn add_assign(&mut self, rps: &Self) {
        self.n ^= rps.n
    }
}

impl Double for GF2 {
    type Output = Self;

    #[inline]
    fn double(self) -> Self {
        Self::ZERO
    }
}

impl Double for &GF2 {
    type Output = GF2;

    #[inline]
    fn double(self) -> Self::Output {
        Self::Output::ZERO
    }
}

impl Neg for GF2 {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        self
    }
}

impl Neg for &GF2 {
    type Output = GF2;

    #[inline]
    fn neg(self) -> Self::Output {
        *self
    }
}

impl Sub for GF2 {
    type Output = Self;

    fn sub(self, rps: Self) -> Self::Output {
        Self { n: self.n ^ rps.n }
    }
}

impl Sub<&Self> for GF2 {
    type Output = Self;

    fn sub(self, rps: &Self) -> Self::Output {
        Self { n: self.n ^ rps.n }
    }
}

impl Sub<GF2> for &GF2 {
    type Output = GF2;

    fn sub(self, rps: GF2) -> Self::Output {
        Self::Output { n: self.n ^ rps.n }
    }
}

impl<'a> Sub<&'a GF2> for &GF2 {
    type Output = GF2;

    fn sub(self, rps: &'a GF2) -> Self::Output {
        Self::Output { n: self.n ^ rps.n }
    }
}

impl SubAssign for GF2 {
    fn sub_assign(&mut self, rps: Self) {
        self.n ^= rps.n
    }
}

impl SubAssign<&Self> for GF2 {
    fn sub_assign(&mut self, rps: &Self) {
        self.n ^= rps.n
    }
}

impl Mul for GF2 {
    type Output = Self;

    fn mul(self, rps: Self) -> Self::Output {
        Self { n: self.n & rps.n }
    }
}

impl Mul<&Self> for GF2 {
    type Output = Self;

    fn mul(self, rps: &Self) -> Self::Output {
        Self { n: self.n & rps.n }
    }
}

impl Mul<GF2> for &GF2 {
    type Output = GF2;

    fn mul(self, rps: GF2) -> Self::Output {
        Self::Output { n: self.n & rps.n }
    }
}

impl<'a> Mul<&'a GF2> for &GF2 {
    type Output = GF2;

    fn mul(self, rps: &'a GF2) -> Self::Output {
        Self::Output { n: self.n & rps.n }
    }
}

impl MulAssign for GF2 {
    fn mul_assign(&mut self, rps: Self) {
        self.n &= rps.n
    }
}

impl MulAssign<&Self> for GF2 {
    fn mul_assign(&mut self, rps: &Self) {
        self.n &= rps.n
    }
}

impl Square for GF2 {
    type Output = Self;

    #[inline]
    fn square(self) -> Self {
        self
    }
}

impl Square for &GF2 {
    type Output = GF2;

    #[inline]
    fn square(self) -> Self::Output {
        *self
    }
}

impl Inv for GF2 {
    type Output = Option<Self>;

    fn inv(self) -> Self::Output {
        match self.n {
            true => Some(Self::ONE),
            false => None,
        }
    }
}

impl Inv for &GF2 {
    type Output = Option<GF2>;

    fn inv(self) -> Self::Output {
        match self.n {
            true => Some(GF2::ONE),
            false => None,
        }
    }
}

impl Div for GF2 {
    type Output = Option<Self>;

    fn div(self, rps: Self) -> Self::Output {
        if rps.n { Some(self) } else { None }
    }
}

impl Div<&Self> for GF2 {
    type Output = Option<Self>;

    fn div(self, rps: &Self) -> Self::Output {
        if rps.n { Some(self) } else { None }
    }
}

impl Div<GF2> for &GF2 {
    type Output = Option<GF2>;

    fn div(self, rps: GF2) -> Self::Output {
        if rps.n { Some(*self) } else { None }
    }
}

impl<'a> Div<&'a GF2> for &GF2 {
    type Output = Option<GF2>;

    fn div(self, rps: &'a GF2) -> Self::Output {
        if rps.n { Some(*self) } else { None }
    }
}

impl Sqrt for GF2 {
    type Output = Self;

    #[inline]
    fn sqrt(self) -> Self {
        self
    }
}

impl Sqrt for &GF2 {
    type Output = GF2;

    #[inline]
    fn sqrt(self) -> Self::Output {
        *self
    }
}

impl Sum for GF2 {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|lps, rps| lps + rps).unwrap_or(Self::ZERO)
    }
}

impl<'a> Sum<&'a Self> for GF2 {
    #[inline]
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.copied().sum()
    }
}

impl Product for GF2 {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|lps, rps| lps * rps).unwrap_or(Self::ONE)
    }
}

impl<'a> Product<&'a Self> for GF2 {
    #[inline]
    fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.copied().product()
    }
}

impl LeftZero for GF2 {
    const LEFT_ZERO: Self = Self { n: false };
}

impl RightZero for GF2 {
    const RIGHT_ZERO: Self = Self { n: false };
}

impl Zero for GF2 {
    const ZERO: Self = Self { n: false };
}

impl LeftOne for GF2 {
    const LEFT_ONE: Self = Self { n: true };
}

impl RightOne for GF2 {
    const RIGHT_ONE: Self = Self { n: true };
}

impl One for GF2 {
    const ONE: Self = Self { n: true };
}

impl Set for GF2 {}

impl AdditiveCommutativeMagma for GF2 {}

impl AdditiveSemigroup for GF2 {}

impl AdditiveMonoid for GF2 {}

impl MultiplicativeCommutativeMagma for GF2 {}

impl MultiplicativeSemigroup for GF2 {}

impl MultiplicativeMonoid for GF2 {}

impl Semifield for GF2 {}

impl IntegerRing for GF2 {
    type Int = i8;

    fn new(n: Self::Int) -> Self {
        Self { n: (n & 1) == 1 }
    }
    fn with_limb(n: <Self::Int as Integer>::Limb) -> Self {
        Self::new(n)
    }

    fn canonical(&self) -> Self::Int {
        self.n.into()
    }
    fn absolute(&self) -> Self::Int {
        self.n.into()
    }

    const BITS: u32 = 1;
    const MODULUS: Self::Int = 2;
}

impl BalancedRepresentative for GF2 {
    type Output = i8;

    fn balanced(&self) -> Self::Output {
        self.n.into()
    }
}

impl BlAssign for GF2 {
    fn bl_assign(&mut self, rps: Self, condition: bool) {
        self.n.bl_assign(rps.n, condition)
    }
}

impl BlAssign<&Self> for GF2 {
    fn bl_assign(&mut self, rps: &Self, condition: bool) {
        self.n.bl_assign(&rps.n, condition)
    }
}

impl BlSelect for GF2 {
    type Output = Self;

    fn bl_select(self, rps: Self, condition: bool) -> Self {
        let n = self.n.bl_select(rps.n, condition);
        Self { n }
    }
}

impl BlSelect<&Self> for GF2 {
    type Output = Self;

    fn bl_select(self, rps: &Self, condition: bool) -> Self {
        let n = self.n.bl_select(&rps.n, condition);
        Self { n }
    }
}

impl BlSelect<GF2> for &GF2 {
    type Output = GF2;

    fn bl_select(self, rps: GF2, condition: bool) -> Self::Output {
        let n = (&self.n).bl_select(rps.n, condition);
        Self::Output { n }
    }
}

impl BlSelect for &GF2 {
    type Output = GF2;

    fn bl_select(self, rps: Self, condition: bool) -> Self::Output {
        let n = (&self.n).bl_select(&rps.n, condition);
        Self::Output { n }
    }
}

impl BlEq for GF2 {
    fn bl_eq(&self, rps: &Self) -> bool {
        self.n.bl_eq(&rps.n)
    }

    fn bl_ne(&self, rps: &Self) -> bool {
        self.n.bl_ne(&rps.n)
    }
}

impl DefaultIsZeroes for GF2 {}
