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
use crate::gf2::{GF2, clmul192, clsqr192};
use crate::symmetric::sponge::{Absorb, Sponge, Squeeze};
use bytemuck::Zeroable;
use core::array;
use core::fmt::{Debug, Formatter, Result};
use core::iter::{Product, Sum, zip};
use core::ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub, SubAssign};

/// The quotient ring `ℤ/(2, Φ₂₄₃(x))`.
#[derive(Clone, Copy, Default, Eq, PartialEq, Zeroable)]
pub struct LabField {
    coefficients: [u64; 3],
}

impl LabField {
    pub const fn new(coefficients: [u64; 3]) -> Option<Self> {
        if coefficients[2] & 0xFFFFFFFC00000000 == 0 {
            Some(Self { coefficients })
        } else {
            None
        }
    }

    /// Construct a polynomial.
    /// # Safety
    /// `coefficients` are reduced.
    pub const unsafe fn from_unchecked(coefficients: [u64; 3]) -> Self {
        Self { coefficients }
    }

    const fn reduce(x: [u64; 6]) -> [u64; 3] {
        let [ll, lh, ml, mh, hl, hh] = x;
        let tll = ml >> 34 | mh << 30;
        let tlh = mh >> 34 & 0x1FFFF;
        let thl = mh >> 51 | hl << 13;
        let thh = hl >> 51 | hh << 13;
        [
            ll ^ tll ^ thl,
            lh ^ tlh ^ thh ^ tll << 17,
            ml & 0x3FFFFFFFF ^ tlh << 17 ^ tll >> 47,
        ]
    }

    fn square_n<const N: usize>(mut self) -> Self {
        for _ in 0..N {
            self = self.square()
        }
        self
    }
}

impl Debug for LabField {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for chunk in self.coefficients.iter().rev() {
            write!(f, "{:016X}", chunk)?
        }
        Ok(())
    }
}

impl From<GF2> for LabField {
    fn from(scalar: GF2) -> Self {
        let coefficients = [bool::from(scalar) as u64, 0, 0];
        Self { coefficients }
    }
}

impl Add for LabField {
    type Output = Self;

    fn add(self, rps: Self) -> Self::Output {
        let coefficients = array::from_fn(|i| self.coefficients[i] ^ rps.coefficients[i]);
        Self { coefficients }
    }
}

impl Add<&Self> for LabField {
    type Output = Self;

    fn add(self, rps: &Self) -> Self::Output {
        let coefficients = array::from_fn(|i| self.coefficients[i] ^ rps.coefficients[i]);
        Self { coefficients }
    }
}

impl Add<LabField> for &LabField {
    type Output = LabField;

    fn add(self, rps: LabField) -> Self::Output {
        let coefficients = array::from_fn(|i| self.coefficients[i] ^ rps.coefficients[i]);
        Self::Output { coefficients }
    }
}

impl<'a> Add<&'a LabField> for &LabField {
    type Output = LabField;

    fn add(self, rps: &'a LabField) -> Self::Output {
        let coefficients = array::from_fn(|i| self.coefficients[i] ^ rps.coefficients[i]);
        Self::Output { coefficients }
    }
}

impl AddAssign for LabField {
    fn add_assign(&mut self, rps: Self) {
        for (l, r) in zip(&mut self.coefficients, rps.coefficients) {
            *l ^= r
        }
    }
}

impl AddAssign<&Self> for LabField {
    fn add_assign(&mut self, rps: &Self) {
        for (l, r) in zip(&mut self.coefficients, rps.coefficients) {
            *l ^= r
        }
    }
}

impl Double for LabField {
    type Output = Self;

    #[inline]
    fn double(self) -> Self {
        Self::ZERO
    }
}

impl Double for &LabField {
    type Output = LabField;

    #[inline]
    fn double(self) -> Self::Output {
        Self::Output::ZERO
    }
}

impl Neg for LabField {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        self
    }
}

impl Neg for &LabField {
    type Output = LabField;

    #[inline]
    fn neg(self) -> Self::Output {
        *self
    }
}

impl Sub for LabField {
    type Output = Self;

    #[inline]
    fn sub(self, rps: Self) -> Self::Output {
        self + rps
    }
}

impl Sub<&Self> for LabField {
    type Output = Self;

    #[inline]
    fn sub(self, rps: &Self) -> Self::Output {
        self + rps
    }
}

impl Sub<LabField> for &LabField {
    type Output = LabField;

    #[inline]
    fn sub(self, rps: LabField) -> Self::Output {
        self + rps
    }
}

impl<'a> Sub<&'a LabField> for &LabField {
    type Output = LabField;

    #[inline]
    fn sub(self, rps: &'a LabField) -> Self::Output {
        self + rps
    }
}

impl SubAssign for LabField {
    #[inline]
    fn sub_assign(&mut self, rps: Self) {
        *self += rps
    }
}

impl SubAssign<&Self> for LabField {
    #[inline]
    fn sub_assign(&mut self, rps: &Self) {
        *self += rps
    }
}

impl Mul for LabField {
    type Output = Self;

    fn mul(self, rps: Self) -> Self::Output {
        let coefficients = Self::reduce(clmul192(self.coefficients, rps.coefficients));
        Self { coefficients }
    }
}

impl Mul<&Self> for LabField {
    type Output = Self;

    #[inline]
    fn mul(self, rps: &Self) -> Self::Output {
        self * *rps
    }
}

impl Mul<LabField> for &LabField {
    type Output = LabField;

    #[inline]
    fn mul(self, rps: LabField) -> Self::Output {
        *self * rps
    }
}

impl<'a> Mul<&'a LabField> for &LabField {
    type Output = LabField;

    #[inline]
    fn mul(self, rps: &'a LabField) -> Self::Output {
        *self * *rps
    }
}

impl MulAssign for LabField {
    #[inline]
    fn mul_assign(&mut self, rps: Self) {
        *self = *self * rps
    }
}

impl MulAssign<&Self> for LabField {
    #[inline]
    fn mul_assign(&mut self, rps: &Self) {
        *self = *self * *rps
    }
}

impl Square for LabField {
    type Output = Self;

    fn square(self) -> Self {
        let coefficients = Self::reduce(clsqr192(self.coefficients));
        Self { coefficients }
    }
}

impl Square for &LabField {
    type Output = LabField;

    #[inline]
    fn square(self) -> Self::Output {
        (*self).square()
    }
}

impl Mul<GF2> for LabField {
    type Output = Self;

    fn mul(self, rps: GF2) -> Self::Output {
        let mask = 0.bl_select(u64::MAX, rps != GF2::ZERO);
        let coefficients = array::from_fn(|i| self.coefficients[i] & mask);
        Self { coefficients }
    }
}

impl Mul<&GF2> for LabField {
    type Output = Self;

    #[inline]
    fn mul(self, rps: &GF2) -> Self::Output {
        self * *rps
    }
}

impl Mul<GF2> for &LabField {
    type Output = LabField;

    #[inline]
    fn mul(self, rps: GF2) -> Self::Output {
        *self * rps
    }
}

impl<'a> Mul<&'a GF2> for &LabField {
    type Output = LabField;

    #[inline]
    fn mul(self, rps: &'a GF2) -> Self::Output {
        *self * *rps
    }
}

impl MulAssign<GF2> for LabField {
    #[inline]
    fn mul_assign(&mut self, rps: GF2) {
        *self = *self * rps
    }
}

impl MulAssign<&GF2> for LabField {
    #[inline]
    fn mul_assign(&mut self, rps: &GF2) {
        *self = *self * *rps
    }
}

impl Inv for LabField {
    type Output = BlOption<Self>;

    fn inv(self) -> Self::Output {
        // Feng and Itoh-Tsujii algorithm
        // addchain: cost: 170
        let b1 = self;
        let b10 = b1.square();
        let b11 = b1 * b10;
        let b110 = b11.square();
        let b111 = b1 * b110;
        let b11100 = b111.square_n::<2>();
        let b11111 = b11 * b11100;
        let x10 = b11111.square_n::<5>() * b11111;
        let x20 = x10.square_n::<10>() * x10;
        let x40 = x20.square_n::<20>() * x20;
        let x80 = x40.square_n::<40>() * x40;
        let x81 = x80.square() * b1;
        let x161 = x81.square_n::<80>() * x80;
        let r1 = x161.square();
        BlOption::new(r1, self.bl_ne(&Self::ZERO))
    }
}

impl Inv for &LabField {
    type Output = BlOption<LabField>;

    #[inline]
    fn inv(self) -> Self::Output {
        (*self).inv()
    }
}

impl Div for LabField {
    type Output = BlOption<Self>;

    fn div(self, rps: Self) -> Self::Output {
        rps.inv().map(|v| self * v)
    }
}

impl Div<&Self> for LabField {
    type Output = BlOption<Self>;

    #[inline]
    fn div(self, rps: &Self) -> Self::Output {
        self / *rps
    }
}

impl Div<LabField> for &LabField {
    type Output = BlOption<LabField>;

    #[inline]
    fn div(self, rps: LabField) -> Self::Output {
        *self / rps
    }
}

impl<'a> Div<&'a LabField> for &LabField {
    type Output = BlOption<LabField>;

    #[inline]
    fn div(self, rps: &'a LabField) -> Self::Output {
        *self / *rps
    }
}

impl Div<GF2> for LabField {
    type Output = BlOption<Self>;

    fn div(self, rps: GF2) -> Self::Output {
        rps.inv().map(|v| self * v)
    }
}

impl Div<&GF2> for LabField {
    type Output = BlOption<Self>;

    #[inline]
    fn div(self, rps: &GF2) -> Self::Output {
        self / *rps
    }
}

impl Div<GF2> for &LabField {
    type Output = BlOption<LabField>;

    #[inline]
    fn div(self, rps: GF2) -> Self::Output {
        *self / rps
    }
}

impl<'a> Div<&'a GF2> for &LabField {
    type Output = BlOption<LabField>;

    #[inline]
    fn div(self, rps: &'a GF2) -> Self::Output {
        *self / *rps
    }
}

impl Sum for LabField {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|lps, rps| lps + rps).unwrap_or(Self::ZERO)
    }
}

impl<'a> Sum<&'a Self> for LabField {
    #[inline]
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.copied().sum()
    }
}

impl Product for LabField {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|lps, rps| lps * rps).unwrap_or(Self::ONE)
    }
}

impl<'a> Product<&'a Self> for LabField {
    #[inline]
    fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.copied().product()
    }
}

impl LeftZero for LabField {
    const LEFT_ZERO: Self = Self::ZERO;
}

impl RightZero for LabField {
    const RIGHT_ZERO: Self = Self::ZERO;
}

impl Zero for LabField {
    const ZERO: Self = Self {
        coefficients: [0; 3],
    };
}

impl LeftOne for LabField {
    const LEFT_ONE: Self = Self::ONE;
}

impl RightOne for LabField {
    const RIGHT_ONE: Self = Self::ONE;
}

impl One for LabField {
    const ONE: Self = Self {
        coefficients: [1, 0, 0],
    };
}

impl Set for LabField {}

impl AdditiveCommutativeMagma for LabField {}

impl AdditiveSemigroup for LabField {}

impl MultiplicativeCommutativeMagma for LabField {}

impl MultiplicativeSemigroup for LabField {}

impl Semifield for LabField {}

impl Semimodule<GF2> for LabField {}

impl Algebra<GF2> for LabField {}

impl BlAssign for LabField {
    fn bl_assign(&mut self, rps: Self, condition: bool) {
        self.coefficients.bl_assign(rps.coefficients, condition)
    }
}

impl BlAssign<&Self> for LabField {
    fn bl_assign(&mut self, rps: &Self, condition: bool) {
        self.coefficients.bl_assign(&rps.coefficients, condition)
    }
}

impl BlSelect for LabField {
    type Output = Self;

    fn bl_select(self, rps: Self, condition: bool) -> Self {
        let coefficients = self.coefficients.bl_select(rps.coefficients, condition);
        Self { coefficients }
    }
}

impl BlSelect<&Self> for LabField {
    type Output = Self;

    fn bl_select(self, rps: &Self, condition: bool) -> Self {
        let coefficients = self.coefficients.bl_select(&rps.coefficients, condition);
        Self { coefficients }
    }
}

impl BlSelect<LabField> for &LabField {
    type Output = LabField;

    fn bl_select(self, rps: LabField, condition: bool) -> Self::Output {
        let coefficients = (&self.coefficients).bl_select(rps.coefficients, condition);
        Self::Output { coefficients }
    }
}

impl BlSelect for &LabField {
    type Output = LabField;

    fn bl_select(self, rps: Self, condition: bool) -> Self::Output {
        let coefficients = (&self.coefficients).bl_select(&rps.coefficients, condition);
        Self::Output { coefficients }
    }
}

impl BlEq for LabField {
    fn bl_eq(&self, rps: &Self) -> bool {
        self.coefficients.bl_eq(&rps.coefficients)
    }

    fn bl_ne(&self, rps: &Self) -> bool {
        self.coefficients.bl_ne(&rps.coefficients)
    }
}

impl Absorb<u8> for LabField {
    fn absorb_into<S: Sponge<Msg = u8>>(self, sponge: &mut S) {
        let bytes = self.coefficients.map(u64::to_le_bytes);
        let bytes: [u8; 24] = unsafe { core::mem::transmute(bytes) };
        sponge.absorb_iter(bytes.into_iter().take(21))
    }
}

impl Squeeze<u8> for LabField {
    fn squeeze_from<S: Sponge<Msg = u8>>(sponge: &mut S) -> Self {
        let mut bytes = [0u8; 24];
        for b in &mut bytes[..21] {
            *b = sponge.squeeze_msg();
        }
        bytes[20] &= 3;
        let bytes: [[u8; 8]; 3] = unsafe { core::mem::transmute(bytes) };
        let coefficients = bytes.map(u64::from_le_bytes);
        Self { coefficients }
    }
}
