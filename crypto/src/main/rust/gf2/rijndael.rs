/*
 * Copyright (c) 2026 Pavel Vasin
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

use crate::algebra::{
    AdditiveCommutativeMagma, AdditiveSemigroup, Algebra, DivisionAlgebra, Double, Inv, LeftOne,
    LeftZero, MultiplicativeCommutativeMagma, MultiplicativeSemigroup, One, RightOne, RightZero,
    Semifield, Semimodule, Set, Square, Zero, square_and_multiply,
};
use crate::branchless::{BlAssign, BlEq, BlOption, BlSelect};
use crate::gf2::GF2;
use core::fmt::{Debug, Formatter, Result};
use core::iter::{Product, Sum};
use core::ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub, SubAssign};
use serde::{Deserialize, Serialize};
use zeroize::DefaultIsZeroes;

/// The quotient ring `ℤ/(2, x⁸ + x⁴ + x³ + x + 1)`.
#[derive(Clone, Copy, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct RijndaelField {
    coefficients: u8,
}

impl RijndaelField {
    pub const fn new(coefficients: u8) -> Self {
        Self { coefficients }
    }

    const fn reduce(x: u16) -> u8 {
        let l = x & 0xFF;
        let h = x >> 8;
        let t = l ^ h ^ h << 1 ^ h << 3 ^ h << 4;
        let l = t & 0xFF;
        let h = t >> 8;
        let t = l ^ h ^ h << 1 ^ h << 3 ^ h << 4;
        t as u8
    }
}

impl Debug for RijndaelField {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:02X}", self.coefficients)
    }
}

impl From<GF2> for RijndaelField {
    fn from(scalar: GF2) -> Self {
        let coefficients = bool::from(scalar) as u8;
        Self { coefficients }
    }
}

impl Add for RijndaelField {
    type Output = Self;

    fn add(self, rps: Self) -> Self::Output {
        let coefficients = self.coefficients ^ rps.coefficients;
        Self { coefficients }
    }
}

impl Add<&Self> for RijndaelField {
    type Output = Self;

    fn add(self, rps: &Self) -> Self::Output {
        let coefficients = self.coefficients ^ rps.coefficients;
        Self { coefficients }
    }
}

impl Add<RijndaelField> for &RijndaelField {
    type Output = RijndaelField;

    fn add(self, rps: RijndaelField) -> Self::Output {
        let coefficients = self.coefficients ^ rps.coefficients;
        Self::Output { coefficients }
    }
}

impl<'a> Add<&'a RijndaelField> for &RijndaelField {
    type Output = RijndaelField;

    fn add(self, rps: &'a RijndaelField) -> Self::Output {
        let coefficients = self.coefficients ^ rps.coefficients;
        Self::Output { coefficients }
    }
}

impl AddAssign for RijndaelField {
    fn add_assign(&mut self, rps: Self) {
        self.coefficients ^= rps.coefficients
    }
}

impl AddAssign<&Self> for RijndaelField {
    fn add_assign(&mut self, rps: &Self) {
        self.coefficients ^= rps.coefficients
    }
}

impl Double for RijndaelField {
    type Output = Self;

    #[inline]
    fn double(self) -> Self {
        Self::ZERO
    }
}

impl Double for &RijndaelField {
    type Output = RijndaelField;

    #[inline]
    fn double(self) -> Self::Output {
        Self::Output::ZERO
    }
}

impl Neg for RijndaelField {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        self
    }
}

impl Neg for &RijndaelField {
    type Output = RijndaelField;

    #[inline]
    fn neg(self) -> Self::Output {
        *self
    }
}

impl Sub for RijndaelField {
    type Output = Self;

    fn sub(self, rps: Self) -> Self::Output {
        let coefficients = self.coefficients ^ rps.coefficients;
        Self { coefficients }
    }
}

impl Sub<&Self> for RijndaelField {
    type Output = Self;

    fn sub(self, rps: &Self) -> Self::Output {
        let coefficients = self.coefficients ^ rps.coefficients;
        Self { coefficients }
    }
}

impl Sub<RijndaelField> for &RijndaelField {
    type Output = RijndaelField;

    fn sub(self, rps: RijndaelField) -> Self::Output {
        let coefficients = self.coefficients ^ rps.coefficients;
        Self::Output { coefficients }
    }
}

impl<'a> Sub<&'a RijndaelField> for &RijndaelField {
    type Output = RijndaelField;

    fn sub(self, rps: &'a RijndaelField) -> Self::Output {
        let coefficients = self.coefficients ^ rps.coefficients;
        Self::Output { coefficients }
    }
}

impl SubAssign for RijndaelField {
    fn sub_assign(&mut self, rps: Self) {
        self.coefficients ^= rps.coefficients
    }
}

impl SubAssign<&Self> for RijndaelField {
    fn sub_assign(&mut self, rps: &Self) {
        self.coefficients ^= rps.coefficients
    }
}

impl Mul for RijndaelField {
    type Output = Self;

    fn mul(self, rps: Self) -> Self::Output {
        let coefficients = Self::reduce(clmul(self.coefficients, rps.coefficients));
        Self { coefficients }
    }
}

impl Mul<&Self> for RijndaelField {
    type Output = Self;

    #[inline]
    fn mul(self, rps: &Self) -> Self::Output {
        self * *rps
    }
}

impl Mul<RijndaelField> for &RijndaelField {
    type Output = RijndaelField;

    #[inline]
    fn mul(self, rps: RijndaelField) -> Self::Output {
        *self * rps
    }
}

impl<'a> Mul<&'a RijndaelField> for &RijndaelField {
    type Output = RijndaelField;

    #[inline]
    fn mul(self, rps: &'a RijndaelField) -> Self::Output {
        *self * *rps
    }
}

impl MulAssign for RijndaelField {
    #[inline]
    fn mul_assign(&mut self, rps: Self) {
        *self = *self * rps
    }
}

impl MulAssign<&Self> for RijndaelField {
    #[inline]
    fn mul_assign(&mut self, rps: &Self) {
        *self = *self * *rps
    }
}

impl Square for RijndaelField {
    type Output = Self;

    fn square(self) -> Self {
        let coefficients = Self::reduce(clsqr(self.coefficients));
        Self { coefficients }
    }
}

impl Square for &RijndaelField {
    type Output = RijndaelField;

    #[inline]
    fn square(self) -> Self::Output {
        (*self).square()
    }
}

impl Mul<GF2> for RijndaelField {
    type Output = Self;

    fn mul(self, rps: GF2) -> Self::Output {
        let mask = (bool::from(rps) as u8).wrapping_neg();
        let coefficients = self.coefficients & mask;
        Self { coefficients }
    }
}

impl Mul<&GF2> for RijndaelField {
    type Output = Self;

    #[inline]
    fn mul(self, rps: &GF2) -> Self::Output {
        self * *rps
    }
}

impl Mul<GF2> for &RijndaelField {
    type Output = RijndaelField;

    #[inline]
    fn mul(self, rps: GF2) -> Self::Output {
        *self * rps
    }
}

impl<'a> Mul<&'a GF2> for &RijndaelField {
    type Output = RijndaelField;

    #[inline]
    fn mul(self, rps: &'a GF2) -> Self::Output {
        *self * *rps
    }
}

impl MulAssign<GF2> for RijndaelField {
    #[inline]
    fn mul_assign(&mut self, rps: GF2) {
        *self = *self * rps
    }
}

impl MulAssign<&GF2> for RijndaelField {
    #[inline]
    fn mul_assign(&mut self, rps: &GF2) {
        *self = *self * *rps
    }
}

impl Inv for RijndaelField {
    type Output = BlOption<Self>;

    fn inv(self) -> Self::Output {
        // Feng and Itoh-Tsujii algorithm
        const R1: [bool; 8] = [false, true, true, true, true, true, true, true];
        let r1 = square_and_multiply(self, R1);
        BlOption::new(r1, self.coefficients != 0)
    }
}

impl Inv for &RijndaelField {
    type Output = BlOption<RijndaelField>;

    #[inline]
    fn inv(self) -> Self::Output {
        (*self).inv()
    }
}

impl Div for RijndaelField {
    type Output = BlOption<Self>;

    fn div(self, rps: Self) -> Self::Output {
        rps.inv().map(|v| self * v)
    }
}

impl Div<&Self> for RijndaelField {
    type Output = BlOption<Self>;

    #[inline]
    fn div(self, rps: &Self) -> Self::Output {
        self / *rps
    }
}

impl Div<RijndaelField> for &RijndaelField {
    type Output = BlOption<RijndaelField>;

    #[inline]
    fn div(self, rps: RijndaelField) -> Self::Output {
        *self / rps
    }
}

impl<'a> Div<&'a RijndaelField> for &RijndaelField {
    type Output = BlOption<RijndaelField>;

    #[inline]
    fn div(self, rps: &'a RijndaelField) -> Self::Output {
        *self / *rps
    }
}

impl Div<GF2> for RijndaelField {
    type Output = BlOption<Self>;

    fn div(self, rps: GF2) -> Self::Output {
        rps.inv().map(|v| self * v)
    }
}

impl Div<&GF2> for RijndaelField {
    type Output = BlOption<Self>;

    #[inline]
    fn div(self, rps: &GF2) -> Self::Output {
        self / *rps
    }
}

impl Div<GF2> for &RijndaelField {
    type Output = BlOption<RijndaelField>;

    #[inline]
    fn div(self, rps: GF2) -> Self::Output {
        *self / rps
    }
}

impl<'a> Div<&'a GF2> for &RijndaelField {
    type Output = BlOption<RijndaelField>;

    #[inline]
    fn div(self, rps: &'a GF2) -> Self::Output {
        *self / *rps
    }
}

impl Sum for RijndaelField {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|lps, rps| lps + rps).unwrap_or(Self::ZERO)
    }
}

impl<'a> Sum<&'a Self> for RijndaelField {
    #[inline]
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.copied().sum()
    }
}

impl Product for RijndaelField {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|lps, rps| lps * rps).unwrap_or(Self::ONE)
    }
}

impl<'a> Product<&'a Self> for RijndaelField {
    #[inline]
    fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.copied().product()
    }
}

impl LeftZero for RijndaelField {
    const LEFT_ZERO: Self = Self { coefficients: 0 };
}

impl RightZero for RijndaelField {
    const RIGHT_ZERO: Self = Self { coefficients: 0 };
}

impl Zero for RijndaelField {
    const ZERO: Self = Self { coefficients: 0 };
}

impl LeftOne for RijndaelField {
    const LEFT_ONE: Self = Self { coefficients: 1 };
}

impl RightOne for RijndaelField {
    const RIGHT_ONE: Self = Self { coefficients: 1 };
}

impl One for RijndaelField {
    const ONE: Self = Self { coefficients: 1 };
}

impl Set for RijndaelField {}

impl AdditiveCommutativeMagma for RijndaelField {}

impl AdditiveSemigroup for RijndaelField {}

impl MultiplicativeCommutativeMagma for RijndaelField {}

impl MultiplicativeSemigroup for RijndaelField {}

impl Semifield for RijndaelField {}

impl Semimodule<GF2> for RijndaelField {}

impl Algebra<GF2> for RijndaelField {}

impl DivisionAlgebra<GF2> for RijndaelField {}

impl BlAssign for RijndaelField {
    fn bl_assign(&mut self, rps: Self, condition: bool) {
        self.coefficients.bl_assign(rps.coefficients, condition)
    }
}

impl BlAssign<&Self> for RijndaelField {
    fn bl_assign(&mut self, rps: &Self, condition: bool) {
        self.coefficients.bl_assign(&rps.coefficients, condition)
    }
}

impl BlSelect for RijndaelField {
    type Output = Self;

    fn bl_select(self, rps: Self, condition: bool) -> Self {
        let coefficients = self.coefficients.bl_select(rps.coefficients, condition);
        Self { coefficients }
    }
}

impl BlSelect<&Self> for RijndaelField {
    type Output = Self;

    fn bl_select(self, rps: &Self, condition: bool) -> Self {
        let coefficients = self.coefficients.bl_select(&rps.coefficients, condition);
        Self { coefficients }
    }
}

impl BlSelect<RijndaelField> for &RijndaelField {
    type Output = RijndaelField;

    fn bl_select(self, rps: RijndaelField, condition: bool) -> Self::Output {
        let coefficients = (&self.coefficients).bl_select(rps.coefficients, condition);
        Self::Output { coefficients }
    }
}

impl BlSelect for &RijndaelField {
    type Output = RijndaelField;

    fn bl_select(self, rps: Self, condition: bool) -> Self::Output {
        let coefficients = (&self.coefficients).bl_select(&rps.coefficients, condition);
        Self::Output { coefficients }
    }
}

impl BlEq for RijndaelField {
    fn bl_eq(&self, rps: &Self) -> bool {
        self.coefficients.bl_eq(&rps.coefficients)
    }

    fn bl_ne(&self, rps: &Self) -> bool {
        self.coefficients.bl_ne(&rps.coefficients)
    }
}

impl DefaultIsZeroes for RijndaelField {}

#[inline(always)]
fn clmul(a: u8, b: u8) -> u16 {
    cfg_select! {
        target_feature = "pclmulqdq" => {
            unsafe {
                #[cfg(target_arch = "x86")]
                use core::arch::x86::*;
                #[cfg(target_arch = "x86_64")]
                use core::arch::x86_64::*;

                let c: u128 = core::mem::transmute(_mm_clmulepi64_si128(
                    _mm_cvtsi32_si128(a as i32),
                    _mm_cvtsi32_si128(b as i32),
                    0,
                ));
                c as u16
            }
        }
        _ => {
            let mut a = a as u16;
            let mut b = b as u16;
            let mut c = 0;
            for _ in 0..u8::BITS {
                let mask = (a & 1).wrapping_neg();
                c ^= b & mask;
                a >>= 1;
                b <<= 1;
            }
            c
        }
    }
}

#[inline(always)]
fn clsqr(a: u8) -> u16 {
    cfg_select! {
        target_feature = "pclmulqdq" => {
            unsafe {
                #[cfg(target_arch = "x86")]
                use core::arch::x86::*;
                #[cfg(target_arch = "x86_64")]
                use core::arch::x86_64::*;

                let c: u128 = core::mem::transmute(_mm_clmulepi64_si128(
                    _mm_cvtsi32_si128(a as i32),
                    _mm_cvtsi32_si128(a as i32),
                    0,
                ));
                c as u16
            }
        }
        _ => {
            let mut a = a as u16;
            let mut c = 0;
            for i in 0..u8::BITS {
                let b = a & 1;
                c |= b << (i << 1);
                a >>= 1;
            }
            c
        }
    }
}
