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

use crate::algebra::DivisionAlgebra;
use crate::convolution::Negacyclic;
use crate::field::{AlgebraicExtension, Field, PrimeField};
use crate::magma::{AdditiveMagma, Inv, MultiplicativeMagma};
use crate::monoid::{AdditiveMonoid, MultiplicativeMonoid};
use crate::polynomialringmonomial::PolynomialRingMonomial;
use crate::ring::{CommutativeRing, IntegerRing, PolynomialRing, Ring, UnitalRing};
use crate::semigroup::MultiplicativeSemigroup;
use core::fmt::{Debug, Formatter, Result};
use core::iter::{Product, Sum};
use core::ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub, SubAssign};

// 2⁶¹ - 1

#[derive(Clone, Copy, Default, Eq)]
pub struct PervushinField {
    n: i64,
}

impl PervushinField {
    pub const fn new(n: i64) -> Self {
        Self {
            n: Self::reduce_add(n),
        }
    }

    const fn reduce_add(x: i64) -> i64 {
        (x & 0x1FFFFFFFFFFFFFFF) + (x >> 61)
    }

    const fn reduce_mul(x: i128) -> i64 {
        Self::reduce_add(((x & 0x1FFFFFFFFFFFFFFF) + (x >> 61)) as i64)
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

    const P_MINUS_2: [bool; 61] = Self::bits(0x1FFFFFFFFFFFFFFD);
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

impl AddAssign for PervushinField {
    #[inline]
    fn add_assign(&mut self, rps: Self) {
        *self = *self + rps
    }
}

impl Neg for PervushinField {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self { n: -self.n }
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

impl SubAssign for PervushinField {
    #[inline]
    fn sub_assign(&mut self, rps: Self) {
        *self = *self - rps
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

impl MulAssign for PervushinField {
    #[inline]
    fn mul_assign(&mut self, rps: Self) {
        *self = *self * rps
    }
}

impl Inv for PervushinField {
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

impl Div for PervushinField {
    type Output = Option<Self>;

    fn div(self, rps: Self) -> Self::Output {
        rps.inv().map(|v| self * v)
    }
}

impl Sum for PervushinField {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|lps, rps| lps + rps).unwrap_or(Self::ZERO)
    }
}

impl Product for PervushinField {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|lps, rps| lps * rps).unwrap_or(Self::UNITY)
    }
}

impl AdditiveMagma for PervushinField {
    fn double(self) -> Self {
        Self {
            n: Self::reduce_add(self.n << 1),
        }
    }
}

impl AdditiveMonoid for PervushinField {
    const IDENTITY: Self = Self { n: 0 };
}

impl MultiplicativeMagma for PervushinField {
    #[inline]
    fn square(self) -> Self {
        self * self
    }
}

impl MultiplicativeSemigroup for PervushinField {
    const LEFT_IDENTITY: Self = Self { n: 1 };
    const RIGHT_IDENTITY: Self = Self { n: 1 };
}

impl MultiplicativeMonoid for PervushinField {
    const IDENTITY: Self = Self { n: 1 };
}

impl Ring for PervushinField {
    type BaseRing = Self;
    type Int = i64;
}

impl CommutativeRing for PervushinField {}

impl IntegerRing for PervushinField {
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

    const BITS: usize = 61;
    const MODULUS: Self::Int = 2305843009213693951;
}

impl PrimeField for PervushinField {}

// (2⁶¹ - 1) / (x² + 1)

pub type PervushinField2 = PolynomialRingMonomial<PervushinField, 2, Negacyclic>;

impl PervushinField2 {
    const R1: [bool; 61] = PervushinField::bits(0x1FFFFFFFFFFFFFFF);
}

impl Inv for PervushinField2 {
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

impl Div for PervushinField2 {
    type Output = Option<Self>;

    fn div(self, rps: Self) -> Self::Output {
        rps.inv().map(|v| self * v)
    }
}

impl Div<PervushinField> for PervushinField2 {
    type Output = Option<Self>;

    fn div(self, rps: PervushinField) -> Self::Output {
        rps.inv().map(|v| self * v)
    }
}

impl Field for PervushinField2 {}

impl DivisionAlgebra<PervushinField, 2> for PervushinField2 {}

impl AlgebraicExtension<PervushinField, 2, Negacyclic> for PervushinField2 {}
