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

use crate::algebra::{
    AdditiveCommutativeMagma, AdditiveMonoid, AdditiveSemigroup, BalancedRepresentative,
    DivisionAlgebra, DivisionRing, Double, IntegerRing, Inv, LeftOne, LeftZero,
    MultiplicativeCommutativeMagma, MultiplicativeMonoid, MultiplicativeSemigroup, One,
    PolynomialRing, RightOne, RightZero, Set, Square, UnivariateRing, Zero, square_and_multiply,
};
use crate::convolution::Negacyclic;
use crate::integer::{Integer, bits_u64};
use crate::polynomial::interpolation::InterpolationConsts;
use core::fmt::{Debug, Formatter, Result};
use core::iter::{Product, Sum};
use core::ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub, SubAssign};

/// The prime field of Pervushin number `2⁶¹ - 1`.
#[derive(Clone, Copy, Default, Eq)]
pub struct PervushinField {
    n: i64,
}

impl PervushinField {
    /// Construct an element.
    /// # Safety
    /// `n` requires spare bits.
    pub const unsafe fn from_unchecked(n: i64) -> Self {
        Self { n }
    }

    const fn reduce_add(x: i64) -> i64 {
        (x & 0x1FFFFFFFFFFFFFFF) + (x >> 61)
    }

    const fn reduce_mul(x: i128) -> i64 {
        Self::reduce_add(((x & 0x1FFFFFFFFFFFFFFF) + (x >> 61)) as i64)
    }

    const P_MINUS_2: [bool; 61] = bits_u64(0x1FFFFFFFFFFFFFFD);
}

impl Debug for PervushinField {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.balanced())
    }
}

impl From<i8> for PervushinField {
    fn from(n: i8) -> Self {
        Self { n: n.into() }
    }
}

impl From<i16> for PervushinField {
    fn from(n: i16) -> Self {
        Self { n: n.into() }
    }
}

impl From<i32> for PervushinField {
    fn from(n: i32) -> Self {
        Self { n: n.into() }
    }
}

impl From<u8> for PervushinField {
    fn from(n: u8) -> Self {
        Self { n: n.into() }
    }
}

impl From<u16> for PervushinField {
    fn from(n: u16) -> Self {
        Self { n: n.into() }
    }
}

impl From<u32> for PervushinField {
    fn from(n: u32) -> Self {
        Self { n: n.into() }
    }
}

impl PartialEq for PervushinField {
    fn eq(&self, rps: &Self) -> bool {
        self.balanced() == rps.balanced()
    }
}

impl Add for PervushinField {
    type Output = Self;

    fn add(self, rps: Self) -> Self::Output {
        Self {
            n: Self::reduce_add(self.n + rps.n),
        }
    }
}

impl Add<&Self> for PervushinField {
    type Output = Self;

    #[inline]
    fn add(self, rps: &Self) -> Self::Output {
        self + *rps
    }
}

impl Add for &PervushinField {
    type Output = PervushinField;

    #[inline]
    fn add(self, rps: Self) -> Self::Output {
        *self + *rps
    }
}

impl AddAssign for PervushinField {
    #[inline]
    fn add_assign(&mut self, rps: Self) {
        *self = *self + rps
    }
}

impl AddAssign<&Self> for PervushinField {
    #[inline]
    fn add_assign(&mut self, rps: &Self) {
        *self = *self + *rps
    }
}

impl Double for PervushinField {
    type Output = Self;

    fn double(self) -> Self {
        Self {
            n: Self::reduce_add(self.n << 1),
        }
    }
}

impl Neg for PervushinField {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self { n: -self.n }
    }
}

impl Neg for &PervushinField {
    type Output = PervushinField;

    #[inline]
    fn neg(self) -> Self::Output {
        -(*self)
    }
}

impl Sub for PervushinField {
    type Output = Self;

    fn sub(self, rps: Self) -> Self::Output {
        Self {
            n: Self::reduce_add(self.n - rps.n),
        }
    }
}

impl Sub<&Self> for PervushinField {
    type Output = Self;

    #[inline]
    fn sub(self, rps: &Self) -> Self::Output {
        self - *rps
    }
}

impl Sub for &PervushinField {
    type Output = PervushinField;

    #[inline]
    fn sub(self, rps: Self) -> Self::Output {
        *self - *rps
    }
}

impl SubAssign for PervushinField {
    #[inline]
    fn sub_assign(&mut self, rps: Self) {
        *self = *self - rps
    }
}

impl SubAssign<&Self> for PervushinField {
    #[inline]
    fn sub_assign(&mut self, rps: &Self) {
        *self = *self - *rps
    }
}

impl Mul for PervushinField {
    type Output = Self;

    fn mul(self, rps: Self) -> Self::Output {
        Self {
            n: Self::reduce_mul(self.n as i128 * rps.n as i128),
        }
    }
}

impl Mul<&Self> for PervushinField {
    type Output = Self;

    #[inline]
    fn mul(self, rps: &Self) -> Self::Output {
        self * *rps
    }
}

impl Mul for &PervushinField {
    type Output = PervushinField;

    #[inline]
    fn mul(self, rps: Self) -> Self::Output {
        *self * *rps
    }
}

impl MulAssign for PervushinField {
    #[inline]
    fn mul_assign(&mut self, rps: Self) {
        *self = *self * rps
    }
}

impl MulAssign<&Self> for PervushinField {
    #[inline]
    fn mul_assign(&mut self, rps: &Self) {
        *self = *self * *rps
    }
}

impl Square for PervushinField {
    type Output = Self;

    #[inline]
    fn square(self) -> Self {
        self * self
    }
}

impl Inv for PervushinField {
    type Output = Option<Self>;

    fn inv(self) -> Self::Output {
        if self != Self::ZERO {
            // Fermat little theorem
            Some(square_and_multiply(self, Self::P_MINUS_2))
        } else {
            None
        }
    }
}

impl Div for PervushinField {
    type Output = Option<Self>;

    fn div(self, rps: Self) -> Self::Output {
        rps.inv().map(|v| self * v)
    }
}

impl Div<&Self> for PervushinField {
    type Output = Option<Self>;

    #[inline]
    fn div(self, rps: &Self) -> Self::Output {
        self / *rps
    }
}

impl Sum for PervushinField {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|lps, rps| lps + rps).unwrap_or(Self::ZERO)
    }
}

impl<'a> Sum<&'a Self> for PervushinField {
    #[inline]
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.copied().sum()
    }
}

impl Product for PervushinField {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|lps, rps| lps * rps).unwrap_or(Self::ONE)
    }
}

impl<'a> Product<&'a Self> for PervushinField {
    #[inline]
    fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.copied().product()
    }
}

impl LeftZero for PervushinField {
    const LEFT_ZERO: Self = Self { n: 0 };
}

impl RightZero for PervushinField {
    const RIGHT_ZERO: Self = Self { n: 0 };
}

impl Zero for PervushinField {
    const ZERO: Self = Self { n: 0 };
}

impl LeftOne for PervushinField {
    const LEFT_ONE: Self = Self { n: 1 };
}

impl RightOne for PervushinField {
    const RIGHT_ONE: Self = Self { n: 1 };
}

impl One for PervushinField {
    const ONE: Self = Self { n: 1 };
}

impl Set for PervushinField {}

impl AdditiveCommutativeMagma for PervushinField {}

impl AdditiveSemigroup for PervushinField {}

impl AdditiveMonoid for PervushinField {}

impl MultiplicativeCommutativeMagma for PervushinField {}

impl MultiplicativeSemigroup for PervushinField {}

impl MultiplicativeMonoid for PervushinField {}

impl DivisionRing for PervushinField {}

impl IntegerRing for PervushinField {
    type Int = i64;

    fn new(n: Self::Int) -> Self {
        Self {
            n: Self::reduce_add(n),
        }
    }
    fn with_limb(n: <Self::Int as Integer>::Limb) -> Self {
        Self::new(n)
    }

    fn canonical(self) -> Self::Int {
        let x = Self::reduce_add(self.n);
        if x >= Self::MODULUS {
            x - Self::MODULUS
        } else if x < 0 {
            x + Self::MODULUS
        } else {
            x
        }
    }
    fn absolute(self) -> Self::Int {
        self.balanced().abs()
    }

    const BITS: u32 = 61;
    const MODULUS: Self::Int = 2305843009213693951;
}

impl InterpolationConsts for PervushinField {
    const INV2: Self = Self {
        n: -1152921504606846975,
    };
    const INV3: Self = Self {
        n: -768614336404564650,
    };
    const INV4: Self = Self {
        n: 576460752303423488,
    };
    const INV6: Self = Self {
        n: -384307168202282325,
    };
    const INV12: Self = Self {
        n: 960767920505705813,
    };
    const INV20: Self = Self {
        n: 1037629354146162278,
    };
    const INV24: Self = Self {
        n: -672537544353994069,
    };
    const INV30: Self = Self {
        n: -76861433640456465,
    };
    const INV120: Self = Self {
        n: -595676110713537604,
    };
    const INV3_MUL2: Self = Self {
        n: 768614336404564651,
    };
    const INV4_MUL5: Self = Self {
        n: 576460752303423489,
    };
    const INV12_MUL5: Self = Self {
        n: 192153584101141163,
    };
    const INV12_MUL7: Self = Self {
        n: -192153584101141162,
    };
    const INV24_MUL7: Self = Self {
        n: -96076792050570581,
    };
}

impl BalancedRepresentative for PervushinField {
    type Output = i64;

    fn balanced(self) -> Self::Output {
        let x = Self::reduce_add(self.n);
        if x > Self::MODULUS / 2 {
            x - Self::MODULUS
        } else if x < -Self::MODULUS / 2 {
            x + Self::MODULUS
        } else {
            x
        }
    }
}

// (2⁶¹ - 1) / (x² + 1)

pub type PervushinField2 = UnivariateRing<PervushinField, 2, Negacyclic>;

impl PervushinField2 {
    fn frobenius(mut self) -> Self {
        self[1] = -self[1];
        self
    }
}

impl Inv for PervushinField2 {
    type Output = Option<Self>;

    fn inv(self) -> Self::Output {
        if self != Self::ZERO {
            // Feng and Itoh-Tsujii algorithm
            let r1 = self.frobenius();
            let r0 = (r1 * self).constant_term();
            Some((r1 / r0).expect("multiplicative group of subfield"))
        } else {
            None
        }
    }
}

impl Div for PervushinField2 {
    type Output = Option<Self>;

    fn div(self, rps: Self) -> Self::Output {
        rps.inv().map(|v| self * v)
    }
}

impl Div<&Self> for PervushinField2 {
    type Output = Option<Self>;

    #[inline]
    fn div(self, rps: &Self) -> Self::Output {
        self / *rps
    }
}

impl DivisionRing for PervushinField2 {}

impl DivisionAlgebra<PervushinField> for PervushinField2 {}
