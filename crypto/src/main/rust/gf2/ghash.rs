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
    AdditiveCommutativeMagma, AdditiveSemigroup, Algebra, Double, Inv, LeftOne, LeftZero,
    MultiplicativeCommutativeMagma, MultiplicativeSemigroup, One, RightOne, RightZero, Semifield,
    Semimodule, Set, Square, Zero,
};
use crate::branchless::{BlAssign, BlEq, BlOption, BlSelect};
use crate::gf2::GF2;
use crate::symmetric::{Absorb, Duplexer, Squeeze};
use core::array;
use core::fmt::{Debug, Formatter, Result};
use core::iter::{Product, Sum, zip};
use core::ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub, SubAssign};
use serde::{Deserialize, Serialize};
use zeroize::DefaultIsZeroes;

/// The quotient ring `ℤ/(2, x¹²⁸ + x⁷ + x² + x + 1)`.
#[derive(Clone, Copy, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct GHashField {
    coefficients: [u64; 2],
}

impl GHashField {
    pub const fn new(coefficients: [u64; 2]) -> Self {
        Self { coefficients }
    }

    pub const fn with_u128(coefficients: u128) -> Self {
        let coefficients = [coefficients as u64, (coefficients >> 64) as u64];
        Self { coefficients }
    }

    const fn reduce(x: [u64; 4]) -> [u64; 2] {
        let [ll, lh, hl, hh] = x;
        let tl = hl ^ hl << 1 ^ hl << 2 ^ hl << 7;
        let th = (hh << 1 | hl >> 63) ^ (hh << 2 | hl >> 62) ^ (hh << 7 | hl >> 57);
        let oh = hh >> 63 ^ hh >> 62 ^ hh >> 57;
        let ol = oh ^ oh << 1 ^ oh << 2 ^ oh << 7;
        [ll ^ tl ^ ol, lh ^ th ^ hh]
    }

    fn square_n<const N: usize>(mut self) -> Self {
        for _ in 0..N {
            self = self.square()
        }
        self
    }
}

impl Debug for GHashField {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for chunk in self.coefficients.iter().rev() {
            write!(f, "{:016X}", chunk)?
        }
        Ok(())
    }
}

impl From<GF2> for GHashField {
    fn from(scalar: GF2) -> Self {
        let coefficients = [bool::from(scalar) as u64, 0];
        Self { coefficients }
    }
}

impl Add for GHashField {
    type Output = Self;

    fn add(self, rps: Self) -> Self::Output {
        let coefficients = array::from_fn(|i| self.coefficients[i] ^ rps.coefficients[i]);
        Self { coefficients }
    }
}

impl Add<&Self> for GHashField {
    type Output = Self;

    fn add(self, rps: &Self) -> Self::Output {
        let coefficients = array::from_fn(|i| self.coefficients[i] ^ rps.coefficients[i]);
        Self { coefficients }
    }
}

impl Add<GHashField> for &GHashField {
    type Output = GHashField;

    fn add(self, rps: GHashField) -> Self::Output {
        let coefficients = array::from_fn(|i| self.coefficients[i] ^ rps.coefficients[i]);
        Self::Output { coefficients }
    }
}

impl<'a> Add<&'a GHashField> for &GHashField {
    type Output = GHashField;

    fn add(self, rps: &'a GHashField) -> Self::Output {
        let coefficients = array::from_fn(|i| self.coefficients[i] ^ rps.coefficients[i]);
        Self::Output { coefficients }
    }
}

impl AddAssign for GHashField {
    fn add_assign(&mut self, rps: Self) {
        for (l, r) in zip(&mut self.coefficients, rps.coefficients) {
            *l ^= r
        }
    }
}

impl AddAssign<&Self> for GHashField {
    fn add_assign(&mut self, rps: &Self) {
        for (l, r) in zip(&mut self.coefficients, rps.coefficients) {
            *l ^= r
        }
    }
}

impl Double for GHashField {
    type Output = Self;

    #[inline]
    fn double(self) -> Self {
        Self::ZERO
    }
}

impl Double for &GHashField {
    type Output = GHashField;

    #[inline]
    fn double(self) -> Self::Output {
        Self::Output::ZERO
    }
}

impl Neg for GHashField {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        self
    }
}

impl Neg for &GHashField {
    type Output = GHashField;

    #[inline]
    fn neg(self) -> Self::Output {
        *self
    }
}

impl Sub for GHashField {
    type Output = Self;

    #[inline]
    fn sub(self, rps: Self) -> Self::Output {
        self + rps
    }
}

impl Sub<&Self> for GHashField {
    type Output = Self;

    #[inline]
    fn sub(self, rps: &Self) -> Self::Output {
        self + rps
    }
}

impl Sub<GHashField> for &GHashField {
    type Output = GHashField;

    #[inline]
    fn sub(self, rps: GHashField) -> Self::Output {
        self + rps
    }
}

impl<'a> Sub<&'a GHashField> for &GHashField {
    type Output = GHashField;

    #[inline]
    fn sub(self, rps: &'a GHashField) -> Self::Output {
        self + rps
    }
}

impl SubAssign for GHashField {
    #[inline]
    fn sub_assign(&mut self, rps: Self) {
        *self += rps
    }
}

impl SubAssign<&Self> for GHashField {
    #[inline]
    fn sub_assign(&mut self, rps: &Self) {
        *self += rps
    }
}

impl Mul for GHashField {
    type Output = Self;

    fn mul(self, rps: Self) -> Self::Output {
        let coefficients = Self::reduce(clmul(self.coefficients, rps.coefficients));
        Self { coefficients }
    }
}

impl Mul<&Self> for GHashField {
    type Output = Self;

    #[inline]
    fn mul(self, rps: &Self) -> Self::Output {
        self * *rps
    }
}

impl Mul<GHashField> for &GHashField {
    type Output = GHashField;

    #[inline]
    fn mul(self, rps: GHashField) -> Self::Output {
        *self * rps
    }
}

impl<'a> Mul<&'a GHashField> for &GHashField {
    type Output = GHashField;

    #[inline]
    fn mul(self, rps: &'a GHashField) -> Self::Output {
        *self * *rps
    }
}

impl MulAssign for GHashField {
    #[inline]
    fn mul_assign(&mut self, rps: Self) {
        *self = *self * rps
    }
}

impl MulAssign<&Self> for GHashField {
    #[inline]
    fn mul_assign(&mut self, rps: &Self) {
        *self = *self * *rps
    }
}

impl Square for GHashField {
    type Output = Self;

    fn square(self) -> Self {
        let coefficients = Self::reduce(clsqr(self.coefficients));
        Self { coefficients }
    }
}

impl Square for &GHashField {
    type Output = GHashField;

    #[inline]
    fn square(self) -> Self::Output {
        (*self).square()
    }
}

impl Mul<GF2> for GHashField {
    type Output = Self;

    fn mul(self, rps: GF2) -> Self::Output {
        let mask = 0.bl_select(u64::MAX, rps != GF2::ZERO);
        let coefficients = array::from_fn(|i| self.coefficients[i] & mask);
        Self { coefficients }
    }
}

impl Mul<&GF2> for GHashField {
    type Output = Self;

    #[inline]
    fn mul(self, rps: &GF2) -> Self::Output {
        self * *rps
    }
}

impl Mul<GF2> for &GHashField {
    type Output = GHashField;

    #[inline]
    fn mul(self, rps: GF2) -> Self::Output {
        *self * rps
    }
}

impl<'a> Mul<&'a GF2> for &GHashField {
    type Output = GHashField;

    #[inline]
    fn mul(self, rps: &'a GF2) -> Self::Output {
        *self * *rps
    }
}

impl MulAssign<GF2> for GHashField {
    #[inline]
    fn mul_assign(&mut self, rps: GF2) {
        *self = *self * rps
    }
}

impl MulAssign<&GF2> for GHashField {
    #[inline]
    fn mul_assign(&mut self, rps: &GF2) {
        *self = *self * *rps
    }
}

impl Inv for GHashField {
    type Output = BlOption<Self>;

    fn inv(self) -> Self::Output {
        // Feng and Itoh-Tsujii algorithm
        // addchain: cost: 137
        let b1 = self;
        let b10 = b1.square();
        let b11 = b1 * b10;
        let b110 = b11.square();
        let b111 = b1 * b110;
        let b111000 = b111.square_n::<3>();
        let b111111 = b111 * b111000;
        let b1111110 = b111111.square();
        let b1111111 = b1 * b1111110;
        let x12 = b1111110.square_n::<5>() * b111111;
        let x24 = x12.square_n::<12>() * x12;
        let i36 = x24.square_n::<7>();
        let x31 = b1111111 * i36;
        let x48 = i36.square_n::<17>() * x24;
        let x96 = x48.square_n::<48>() * x48;
        let x127 = x96.square_n::<31>() * x31;
        let r1 = x127.square();
        BlOption::new(r1, self.bl_ne(&Self::ZERO))
    }
}

impl Inv for &GHashField {
    type Output = BlOption<GHashField>;

    #[inline]
    fn inv(self) -> Self::Output {
        (*self).inv()
    }
}

impl Div for GHashField {
    type Output = BlOption<Self>;

    fn div(self, rps: Self) -> Self::Output {
        rps.inv().map(|v| self * v)
    }
}

impl Div<&Self> for GHashField {
    type Output = BlOption<Self>;

    #[inline]
    fn div(self, rps: &Self) -> Self::Output {
        self / *rps
    }
}

impl Div<GHashField> for &GHashField {
    type Output = BlOption<GHashField>;

    #[inline]
    fn div(self, rps: GHashField) -> Self::Output {
        *self / rps
    }
}

impl<'a> Div<&'a GHashField> for &GHashField {
    type Output = BlOption<GHashField>;

    #[inline]
    fn div(self, rps: &'a GHashField) -> Self::Output {
        *self / *rps
    }
}

impl Div<GF2> for GHashField {
    type Output = BlOption<Self>;

    fn div(self, rps: GF2) -> Self::Output {
        rps.inv().map(|v| self * v)
    }
}

impl Div<&GF2> for GHashField {
    type Output = BlOption<Self>;

    #[inline]
    fn div(self, rps: &GF2) -> Self::Output {
        self / *rps
    }
}

impl Div<GF2> for &GHashField {
    type Output = BlOption<GHashField>;

    #[inline]
    fn div(self, rps: GF2) -> Self::Output {
        *self / rps
    }
}

impl<'a> Div<&'a GF2> for &GHashField {
    type Output = BlOption<GHashField>;

    #[inline]
    fn div(self, rps: &'a GF2) -> Self::Output {
        *self / *rps
    }
}

impl Sum for GHashField {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|lps, rps| lps + rps).unwrap_or(Self::ZERO)
    }
}

impl<'a> Sum<&'a Self> for GHashField {
    #[inline]
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.copied().sum()
    }
}

impl Product for GHashField {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|lps, rps| lps * rps).unwrap_or(Self::ONE)
    }
}

impl<'a> Product<&'a Self> for GHashField {
    #[inline]
    fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.copied().product()
    }
}

impl LeftZero for GHashField {
    const LEFT_ZERO: Self = Self {
        coefficients: [0, 0],
    };
}

impl RightZero for GHashField {
    const RIGHT_ZERO: Self = Self {
        coefficients: [0, 0],
    };
}

impl Zero for GHashField {
    const ZERO: Self = Self {
        coefficients: [0, 0],
    };
}

impl LeftOne for GHashField {
    const LEFT_ONE: Self = Self {
        coefficients: [1, 0],
    };
}

impl RightOne for GHashField {
    const RIGHT_ONE: Self = Self {
        coefficients: [1, 0],
    };
}

impl One for GHashField {
    const ONE: Self = Self {
        coefficients: [1, 0],
    };
}

impl Set for GHashField {}

impl AdditiveCommutativeMagma for GHashField {}

impl AdditiveSemigroup for GHashField {}

impl MultiplicativeCommutativeMagma for GHashField {}

impl MultiplicativeSemigroup for GHashField {}

impl Semifield for GHashField {}

impl Semimodule<GF2> for GHashField {}

impl Algebra<GF2> for GHashField {}

impl BlAssign for GHashField {
    fn bl_assign(&mut self, rps: Self, condition: bool) {
        self.coefficients.bl_assign(rps.coefficients, condition)
    }
}

impl BlAssign<&Self> for GHashField {
    fn bl_assign(&mut self, rps: &Self, condition: bool) {
        self.coefficients.bl_assign(&rps.coefficients, condition)
    }
}

impl BlSelect for GHashField {
    type Output = Self;

    fn bl_select(self, rps: Self, condition: bool) -> Self {
        let coefficients = self.coefficients.bl_select(rps.coefficients, condition);
        Self { coefficients }
    }
}

impl BlSelect<&Self> for GHashField {
    type Output = Self;

    fn bl_select(self, rps: &Self, condition: bool) -> Self {
        let coefficients = self.coefficients.bl_select(&rps.coefficients, condition);
        Self { coefficients }
    }
}

impl BlSelect<GHashField> for &GHashField {
    type Output = GHashField;

    fn bl_select(self, rps: GHashField, condition: bool) -> Self::Output {
        let coefficients = (&self.coefficients).bl_select(rps.coefficients, condition);
        Self::Output { coefficients }
    }
}

impl BlSelect for &GHashField {
    type Output = GHashField;

    fn bl_select(self, rps: Self, condition: bool) -> Self::Output {
        let coefficients = (&self.coefficients).bl_select(&rps.coefficients, condition);
        Self::Output { coefficients }
    }
}

impl BlEq for GHashField {
    fn bl_eq(&self, rps: &Self) -> bool {
        self.coefficients.bl_eq(&rps.coefficients)
    }

    fn bl_ne(&self, rps: &Self) -> bool {
        self.coefficients.bl_ne(&rps.coefficients)
    }
}

impl Absorb<u8> for GHashField {
    fn absorb_into<D: Duplexer<Msg = u8>>(self, duplex: &mut D) {
        duplex.absorb_iter(self.coefficients.into_iter().flat_map(u64::to_le_bytes))
    }
}

impl Squeeze<u8> for GHashField {
    fn squeeze_from<D: Duplexer<Msg = u8>>(duplex: &mut D) -> Self {
        let coefficients = array::from_fn(|_| {
            let bytes: [u8; 8] = array::from_fn(|_| duplex.squeeze_msg());
            u64::from_le_bytes(bytes)
        });
        Self { coefficients }
    }
}

impl DefaultIsZeroes for GHashField {}

#[inline(always)]
fn clmul(a: [u64; 2], b: [u64; 2]) -> [u64; 4] {
    cfg_select! {
        target_feature = "pclmulqdq" => {
            unsafe {
                #[cfg(target_arch = "x86")]
                use core::arch::x86::*;
                #[cfg(target_arch = "x86_64")]
                use core::arch::x86_64::*;

                // Long method
                let a = _mm_loadu_si128(a.as_ptr() as *const __m128i);
                let b = _mm_loadu_si128(b.as_ptr() as *const __m128i);
                let ll = _mm_clmulepi64_si128(a, b, 0);
                let lh = _mm_clmulepi64_si128(a, b, 1);
                let hl = _mm_clmulepi64_si128(a, b, 16);
                let hh = _mm_clmulepi64_si128(a, b, 17);
                let [lll, llh]: [u64; 2] = core::mem::transmute(ll);
                let [lhl, lhh]: [u64; 2] = core::mem::transmute(lh);
                let [hll, hlh]: [u64; 2] = core::mem::transmute(hl);
                let [hhl, hhh]: [u64; 2] = core::mem::transmute(hh);
                [
                    lll,
                    llh ^ lhl ^ hll,
                    hhl ^ lhh ^ hlh,
                    hhh,
                ]
            }
        }
        _ => {
            fn clmul64(a: u64, b: u64) -> [u64; 2] {
                let [mut l, mut h] = [0, 0];
                let mask = (a & 1).wrapping_neg();
                l ^= mask & b;
                for i in 1..64 {
                    let mask = (a >> i & 1).wrapping_neg();
                    l ^= mask & b << i;
                    h ^= mask & b >> (64 - i);
                }
                [l, h]
            }

            // Karatsuba method
            let [al, ah] = a;
            let [bl, bh] = b;
            let [ta, tb] = [al ^ ah, bl ^ bh];
            let [ll, lh] = clmul64(al, bl);
            let [hl, hh] = clmul64(ah, bh);
            let [tl, th] = clmul64(ta, tb);
            [
                ll,
                lh ^ ll ^ hl ^ tl,
                hl ^ lh ^ hh ^ th,
                hh,
            ]
        }
    }
}

#[inline(always)]
fn clsqr(a: [u64; 2]) -> [u64; 4] {
    cfg_select! {
        target_feature = "pclmulqdq" => {
            unsafe {
                #[cfg(target_arch = "x86")]
                use core::arch::x86::*;
                #[cfg(target_arch = "x86_64")]
                use core::arch::x86_64::*;

                let a = _mm_loadu_si128(a.as_ptr() as *const __m128i);
                let cl = _mm_clmulepi64_si128(a, a, 0);
                let ch = _mm_clmulepi64_si128(a, a, 17);
                core::mem::transmute([cl, ch])
            }
        }
        _ => {
            let mut a = a;
            let mut c = [0; 4];
            for i in 0..2 {
                for j in 0..32 {
                    let b = a[i] & 1;
                    c[i * 2] |= b << (j << 1);
                    a[i] >>= 1;
                }
                for j in 0..32 {
                    let b = a[i] & 1;
                    c[i * 2 + 1] |= b << (j << 1);
                    a[i] >>= 1;
                }
            }
            c
        }
    }
}
