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

use crate::algebra::DivisionAlgebra;
use crate::convolution::{Binomial, Convolution, Negacyclic};
use crate::integer::Integer;
use crate::interpolation::InterpolationConsts;
use crate::magma::{
    AdditiveCommutativeMagma, AdditiveMagma, MultiplicativeCommutativeMagma, MultiplicativeMagma,
};
use crate::monoid::{AdditiveMonoid, MultiplicativeMonoid};
use crate::nttring::NTTRing;
use crate::operation::{Double, Inv, Square};
use crate::ring::{DivisionRing, IntegerRing, PolynomialRing, Ring};
use crate::semigroup::{AdditiveSemigroup, MultiplicativeSemigroup};
use crate::semiring::{Presemiring, Semiring};
use crate::univariatering::UnivariateRing;
use core::fmt::{Debug, Formatter, Result};
use core::iter::{Product, Sum};
use core::ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub, SubAssign};

// 2⁶⁰ + 2⁵ + 1

#[derive(Clone, Copy, Default, Eq)]
pub struct LMField {
    n: i64,
}

impl LMField {
    pub const unsafe fn from_unchecked(n: i64) -> Self {
        Self { n }
    }

    const fn reduce_add(x: i64) -> i64 {
        (x & 0xFFFFFFFFFFFFFFF) - 33 * (x >> 60)
    }

    const fn reduce_mul(x: i128) -> i64 {
        let t = (x & 0xFFFFFFFFFFFFFFF) - 33 * (x >> 60);
        ((t & 0xFFFFFFFFFFFFFFF) - 33 * (t >> 60)) as i64
    }

    pub const fn balanced(self) -> i64 {
        let x = Self::reduce_add(self.n);
        if x > Self::MODULUS / 2 {
            x - Self::MODULUS
        } else if x < -Self::MODULUS / 2 {
            x + Self::MODULUS
        } else {
            x
        }
    }

    const fn bits<const N: usize>(n: u64) -> [bool; N] {
        let mut bits = [false; N];
        let mut i = 0;
        loop {
            bits[i] = n >> i & 1 == 1;
            i += 1;
            if i == N {
                break;
            }
        }
        bits
    }

    const P_MINUS_2: [bool; 61] = Self::bits(0x100000000000001F);
}

impl Debug for LMField {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.balanced())
    }
}

impl From<i8> for LMField {
    fn from(n: i8) -> Self {
        Self { n: n.into() }
    }
}

impl From<i16> for LMField {
    fn from(n: i16) -> Self {
        Self { n: n.into() }
    }
}

impl From<i32> for LMField {
    fn from(n: i32) -> Self {
        Self { n: n.into() }
    }
}

impl From<u8> for LMField {
    fn from(n: u8) -> Self {
        Self { n: n.into() }
    }
}

impl From<u16> for LMField {
    fn from(n: u16) -> Self {
        Self { n: n.into() }
    }
}

impl From<u32> for LMField {
    fn from(n: u32) -> Self {
        Self { n: n.into() }
    }
}

impl PartialEq for LMField {
    fn eq(&self, rps: &Self) -> bool {
        self.balanced() == rps.balanced()
    }
}

impl Add for LMField {
    type Output = Self;

    fn add(self, rps: Self) -> Self::Output {
        Self {
            n: Self::reduce_add(self.n + rps.n),
        }
    }
}

impl AddAssign for LMField {
    #[inline]
    fn add_assign(&mut self, rps: Self) {
        *self = *self + rps
    }
}

impl Double for LMField {
    type Output = Self;

    fn double(self) -> Self {
        Self {
            n: Self::reduce_add(self.n << 1),
        }
    }
}

impl Neg for LMField {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self { n: -self.n }
    }
}

impl Sub for LMField {
    type Output = Self;

    fn sub(self, rps: Self) -> Self::Output {
        Self {
            n: Self::reduce_add(self.n - rps.n),
        }
    }
}

impl SubAssign for LMField {
    #[inline]
    fn sub_assign(&mut self, rps: Self) {
        *self = *self - rps
    }
}

impl Mul for LMField {
    type Output = Self;

    fn mul(self, rps: Self) -> Self::Output {
        Self {
            n: Self::reduce_mul(self.n as i128 * rps.n as i128),
        }
    }
}

impl MulAssign for LMField {
    #[inline]
    fn mul_assign(&mut self, rps: Self) {
        *self = *self * rps
    }
}

impl Square for LMField {
    type Output = Self;

    #[inline]
    fn square(self) -> Self {
        self * self
    }
}

impl Inv for LMField {
    type Output = Option<Self>;

    fn inv(self) -> Self::Output {
        if self != Self::ZERO {
            // Fermat little theorem
            Some(self.square_and_multiply(Self::P_MINUS_2))
        } else {
            None
        }
    }
}

impl Div for LMField {
    type Output = Option<Self>;

    fn div(self, rps: Self) -> Self::Output {
        rps.inv().map(|v| self * v)
    }
}

impl Sum for LMField {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|lps, rps| lps + rps).unwrap_or(Self::ZERO)
    }
}

impl Product for LMField {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|lps, rps| lps * rps).unwrap_or(Self::ONE)
    }
}

impl AdditiveMagma for LMField {}

impl AdditiveCommutativeMagma for LMField {}

impl AdditiveSemigroup for LMField {
    const LEFT_IDENTITY: Self = Self { n: 0 };
    const RIGHT_IDENTITY: Self = Self { n: 0 };
}

impl AdditiveMonoid for LMField {
    const IDENTITY: Self = Self { n: 0 };
}

impl MultiplicativeMagma for LMField {}

impl MultiplicativeCommutativeMagma for LMField {}

impl MultiplicativeSemigroup for LMField {
    const LEFT_IDENTITY: Self = Self { n: 1 };
    const RIGHT_IDENTITY: Self = Self { n: 1 };
}

impl MultiplicativeMonoid for LMField {
    const IDENTITY: Self = Self { n: 1 };
}

impl Ring for LMField {
    type Int = i64;
}

impl DivisionRing for LMField {}

impl IntegerRing for LMField {
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
    const MODULUS: Self::Int = 1152921504606847009;
}

impl InterpolationConsts for LMField {
    const INV2: Self = Self {
        n: 576460752303423505,
    };
    const INV3: Self = Self {
        n: 768614336404564673,
    };
    const INV4: Self = Self {
        n: 864691128455135257,
    };
    const INV6: Self = Self {
        n: 960767920505705841,
    };
    const INV12: Self = Self {
        n: 1056844712556276425,
    };
    const INV20: Self = Self {
        n: 634106827533765855,
    };
    const INV24: Self = Self {
        n: 1104883108581561717,
    };
    const INV30: Self = Self {
        n: 422737885022510570,
    };
    const INV120: Self = Self {
        n: 682145223559051147,
    };
    const INV3_MUL2: Self = Self {
        n: 384307168202282337,
    };
    const INV4_MUL5: Self = Self {
        n: 864691128455135258,
    };
    const INV12_MUL5: Self = Self {
        n: 672537544353994089,
    };
    const INV12_MUL7: Self = Self {
        n: 480383960252852921,
    };
    const INV24_MUL7: Self = Self {
        n: 816652732429849965,
    };
}

// (2⁶⁰ + 2⁵ + 1) / (x² - ³²√1)

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct LMField2Convolution {}

impl Convolution<LMField, 2> for LMField2Convolution {
    fn convolute(a: [LMField; 2], b: [LMField; 2]) -> [LMField; 2] {
        <Self as Binomial<LMField, 2>>::convolute(a, b)
    }
}

impl Binomial<LMField, 2> for LMField2Convolution {
    const ZETA: LMField = LMField {
        n: -14367867355629317,
    };
}

pub type LMField2 = UnivariateRing<LMField, 2, LMField2Convolution>;

impl LMField2 {
    const R1: [bool; 61] = LMField::bits(0x1000000000000021);
}

impl Inv for LMField2 {
    type Output = Option<Self>;

    fn inv(self) -> Self::Output {
        if self != Self::ZERO {
            // Feng and Itoh-Tsujii algorithm
            let r1 = self.square_and_multiply(Self::R1);
            let r0 = (r1 * self).constant_term();
            Some((r1 / r0).expect("multiplicative group of subfield"))
        } else {
            None
        }
    }
}

impl Div for LMField2 {
    type Output = Option<Self>;

    fn div(self, rps: Self) -> Self::Output {
        rps.inv().map(|v| self * v)
    }
}

impl Div<LMField> for LMField2 {
    type Output = Option<Self>;

    fn div(self, rps: LMField) -> Self::Output {
        rps.inv().map(|v| self * v)
    }
}

impl DivisionRing for LMField2 {}

impl DivisionAlgebra<LMField> for LMField2 {}

// (2⁶⁰ + 2⁵ + 1) / (x⁶⁴ + 1)

pub type LMRing64 = UnivariateRing<LMField, 64, Negacyclic>;

pub type LMNTT64 = NTTRing<LMField, 16, 64>;
