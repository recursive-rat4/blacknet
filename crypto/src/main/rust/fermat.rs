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
    DivisionRing, Double, IntegerRing, Inv, LeftOne, LeftZero, MultiplicativeCommutativeMagma,
    MultiplicativeMonoid, MultiplicativeSemigroup, NTTRing, One, RightOne, RightZero, Set, Square,
    UnivariateRing, Zero,
};
use crate::convolution::Negacyclic;
use crate::integer::Integer;
use core::fmt::{Debug, Formatter, Result};
use core::iter::{Product, Sum};
use core::ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub, SubAssign};

/// The prime field of Fermat number `2¹⁶ + 1`.
#[derive(Clone, Copy, Default, Eq)]
pub struct FermatField {
    n: i32,
}

impl FermatField {
    /// Construct an element.
    /// # Safety
    /// `n` requires spare bits.
    pub const unsafe fn from_unchecked(n: i32) -> Self {
        Self { n }
    }

    const fn reduce_add(x: i32) -> i32 {
        (x & 0xFFFF) - (x >> 16)
    }

    const fn reduce_mul(x: i64) -> i32 {
        ((x & 0xFFFF) - (x >> 16)) as i32
    }

    const fn halve(mut self) -> Self {
        if self.n & 1 == 1 {
            self.n += Self::MODULUS;
        }
        self.n >>= 1;
        self
    }
}

impl Debug for FermatField {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.balanced())
    }
}

impl From<i8> for FermatField {
    fn from(n: i8) -> Self {
        Self { n: n.into() }
    }
}

impl From<u8> for FermatField {
    fn from(n: u8) -> Self {
        Self { n: n.into() }
    }
}

impl PartialEq for FermatField {
    fn eq(&self, rps: &Self) -> bool {
        self.balanced() == rps.balanced()
    }
}

impl Add for FermatField {
    type Output = Self;

    fn add(self, rps: Self) -> Self {
        Self {
            n: Self::reduce_add(self.n + rps.n),
        }
    }
}

impl Add<&Self> for FermatField {
    type Output = Self;

    fn add(self, rps: &Self) -> Self {
        Self {
            n: Self::reduce_add(self.n + rps.n),
        }
    }
}

impl Add<FermatField> for &FermatField {
    type Output = FermatField;

    fn add(self, rps: FermatField) -> Self::Output {
        Self::Output {
            n: Self::Output::reduce_add(self.n + rps.n),
        }
    }
}

impl<'a> Add<&'a FermatField> for &FermatField {
    type Output = FermatField;

    fn add(self, rps: &'a FermatField) -> Self::Output {
        Self::Output {
            n: Self::Output::reduce_add(self.n + rps.n),
        }
    }
}

impl AddAssign for FermatField {
    fn add_assign(&mut self, rps: Self) {
        self.n = Self::reduce_add(self.n + rps.n)
    }
}

impl AddAssign<&Self> for FermatField {
    fn add_assign(&mut self, rps: &Self) {
        self.n = Self::reduce_add(self.n + rps.n)
    }
}

impl Double for FermatField {
    type Output = Self;

    fn double(self) -> Self {
        Self {
            n: Self::reduce_add(self.n << 1),
        }
    }
}

impl Double for &FermatField {
    type Output = FermatField;

    fn double(self) -> Self::Output {
        Self::Output {
            n: Self::Output::reduce_add(self.n << 1),
        }
    }
}

impl Neg for FermatField {
    type Output = Self;

    fn neg(self) -> Self {
        Self { n: -self.n }
    }
}

impl Neg for &FermatField {
    type Output = FermatField;

    fn neg(self) -> Self::Output {
        Self::Output { n: -self.n }
    }
}

impl Sub for FermatField {
    type Output = Self;

    fn sub(self, rps: Self) -> Self {
        Self {
            n: Self::reduce_add(self.n - rps.n),
        }
    }
}

impl Sub<&Self> for FermatField {
    type Output = Self;

    fn sub(self, rps: &Self) -> Self {
        Self {
            n: Self::reduce_add(self.n - rps.n),
        }
    }
}

impl Sub<FermatField> for &FermatField {
    type Output = FermatField;

    fn sub(self, rps: FermatField) -> Self::Output {
        Self::Output {
            n: Self::Output::reduce_add(self.n - rps.n),
        }
    }
}

impl<'a> Sub<&'a FermatField> for &FermatField {
    type Output = FermatField;

    fn sub(self, rps: &'a FermatField) -> Self::Output {
        Self::Output {
            n: Self::Output::reduce_add(self.n - rps.n),
        }
    }
}

impl SubAssign for FermatField {
    fn sub_assign(&mut self, rps: Self) {
        self.n = Self::reduce_add(self.n - rps.n)
    }
}

impl SubAssign<&Self> for FermatField {
    fn sub_assign(&mut self, rps: &Self) {
        self.n = Self::reduce_add(self.n - rps.n)
    }
}

impl Mul for FermatField {
    type Output = Self;

    fn mul(self, rps: Self) -> Self::Output {
        Self {
            n: Self::reduce_mul(self.n as i64 * rps.n as i64),
        }
    }
}

impl Mul<&Self> for FermatField {
    type Output = Self;

    fn mul(self, rps: &Self) -> Self::Output {
        Self {
            n: Self::reduce_mul(self.n as i64 * rps.n as i64),
        }
    }
}

impl Mul<FermatField> for &FermatField {
    type Output = FermatField;

    fn mul(self, rps: FermatField) -> Self::Output {
        Self::Output {
            n: Self::Output::reduce_mul(self.n as i64 * rps.n as i64),
        }
    }
}

impl<'a> Mul<&'a FermatField> for &FermatField {
    type Output = FermatField;

    fn mul(self, rps: &'a FermatField) -> Self::Output {
        Self::Output {
            n: Self::Output::reduce_mul(self.n as i64 * rps.n as i64),
        }
    }
}

impl MulAssign for FermatField {
    fn mul_assign(&mut self, rps: Self) {
        self.n = Self::reduce_mul(self.n as i64 * rps.n as i64)
    }
}

impl MulAssign<&Self> for FermatField {
    fn mul_assign(&mut self, rps: &Self) {
        self.n = Self::reduce_mul(self.n as i64 * rps.n as i64)
    }
}

impl Square for FermatField {
    type Output = Self;

    fn square(self) -> Self {
        Self {
            n: Self::reduce_mul(self.n as i64 * self.n as i64),
        }
    }
}

impl Square for &FermatField {
    type Output = FermatField;

    fn square(self) -> Self::Output {
        Self::Output {
            n: Self::Output::reduce_mul(self.n as i64 * self.n as i64),
        }
    }
}

impl Inv for FermatField {
    type Output = Option<Self>;

    fn inv(self) -> Self::Output {
        // Extended Binary GCD (classic algorithm)
        // https://eprint.iacr.org/2020/972
        let mut a = self.canonical();
        let mut b = Self::MODULUS;
        let mut c = Self::ONE;
        let mut d = Self::ZERO;
        while a != 0 {
            if a & 1 == 0 {
                a >>= 1;
                c = c.halve();
            } else {
                if a < b {
                    (a, b) = (b, a);
                    (c, d) = (d, c);
                }
                a -= b;
                a >>= 1;
                c -= d;
                c = c.halve();
            }
        }
        if b != 1 {
            return None;
        }
        Some(d)
    }
}

impl Div for FermatField {
    type Output = Option<Self>;

    fn div(self, rps: Self) -> Self::Output {
        rps.inv().map(|v| self * v)
    }
}

impl Div<&Self> for FermatField {
    type Output = Option<Self>;

    fn div(self, rps: &Self) -> Self::Output {
        rps.inv().map(|v| self * v)
    }
}

impl Sum for FermatField {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|lps, rps| lps + rps).unwrap_or(Self::ZERO)
    }
}

impl<'a> Sum<&'a Self> for FermatField {
    #[inline]
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.copied().sum()
    }
}

impl Product for FermatField {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|lps, rps| lps * rps).unwrap_or(Self::ONE)
    }
}

impl<'a> Product<&'a Self> for FermatField {
    #[inline]
    fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.copied().product()
    }
}

impl LeftZero for FermatField {
    const LEFT_ZERO: Self = Self { n: 0 };
}

impl RightZero for FermatField {
    const RIGHT_ZERO: Self = Self { n: 0 };
}

impl Zero for FermatField {
    const ZERO: Self = Self { n: 0 };
}

impl LeftOne for FermatField {
    const LEFT_ONE: Self = Self { n: 1 };
}

impl RightOne for FermatField {
    const RIGHT_ONE: Self = Self { n: 1 };
}

impl One for FermatField {
    const ONE: Self = Self { n: 1 };
}

impl Set for FermatField {}

impl AdditiveCommutativeMagma for FermatField {}

impl AdditiveSemigroup for FermatField {}

impl AdditiveMonoid for FermatField {}

impl MultiplicativeCommutativeMagma for FermatField {}

impl MultiplicativeSemigroup for FermatField {}

impl MultiplicativeMonoid for FermatField {}

impl DivisionRing for FermatField {}

impl IntegerRing for FermatField {
    type Int = i32;

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

    const BITS: u32 = 17;
    const MODULUS: Self::Int = 65537;
}

impl BalancedRepresentative for FermatField {
    type Output = i32;

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

// (2¹⁶ + 1) / (x¹⁰²⁴ + 1)

pub type FermatRing1024 = UnivariateRing<FermatField, 1024, Negacyclic>;

pub type FermatNTT1024 = NTTRing<FermatField, 1024, 1024>;
