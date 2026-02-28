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
    AdditiveCommutativeMagma, AdditiveMonoid, AdditiveSemigroup, DivisionRing, Double, IntegerRing,
    Inv, LeftOne, LeftZero, MultiplicativeCommutativeMagma, MultiplicativeMonoid,
    MultiplicativeSemigroup, One, RightOne, RightZero, Set, Sqrt, Square, Zero,
    square_and_multiply,
};
use crate::bigint::{UInt256, UInt512};
use crate::integer::Integer;
use core::fmt;
use core::iter::{Product, Sum};
use core::ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub, SubAssign};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// The prime field `2²⁵⁵ - 19`.
#[derive(Clone, Copy, Default, Eq, PartialEq)]
pub struct Field25519 {
    n: UInt256,
}

impl Field25519 {
    pub fn from_hex(hex: &str) -> Self {
        Self::new(UInt256::from_hex(hex))
    }

    /// Construct an element.
    /// # Safety
    /// `n` requires spare bit and Montgomery form.
    pub const unsafe fn from_unchecked(n: UInt256) -> Self {
        Self { n }
    }

    fn to_form(x: UInt256) -> UInt256 {
        Self::reduce_mul(x.widening_mul(Self::R2))
    }

    fn from_form(x: UInt256) -> UInt256 {
        let limbs = x.limbs();
        Self::reduce_mul(UInt512::from([
            limbs[0], limbs[1], limbs[2], limbs[3], 0, 0, 0, 0,
        ]))
    }

    fn reduce_add(mut x: UInt256) -> UInt256 {
        if x >= Self::MODULUS {
            x -= Self::MODULUS
        }
        x
    }

    fn reduce_mul(x: UInt512) -> UInt256 {
        let mut limbs = x.limbs();
        // Montgomery reduction
        let mut c: u128 = 0;
        for i in 0..4 {
            let mut ll: u128 = 0;
            let l = limbs[i].wrapping_mul(Self::RN);
            for j in 0..4 {
                ll += l as u128 * Self::MODULUS.limbs()[j] as u128 + limbs[i + j] as u128;
                limbs[i + j] = ll as u64;
                ll >>= u64::BITS;
            }
            c += limbs[i + 4] as u128 + ll;
            limbs[i + 4] = c as u64;
            c >>= u64::BITS;
        }
        let n = UInt256::from([limbs[4], limbs[5], limbs[6], limbs[7]]);
        Self::reduce_add(n)
    }

    fn halve(mut self) -> Self {
        if self.n.is_odd() {
            self.n += Self::MODULUS;
        }
        self.n >>= 1;
        self
    }

    const R2: UInt256 =
        UInt256::from_hex("00000000000000000000000000000000000000000000000000000000000005A4");
    const RN: u64 = 0x86BCA1AF286BCA1B;
    const P_MINUS_5_EIGHTH: [bool; 252] =
        UInt256::from_hex("0FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFD")
            .bits();
    const P_MINUS_1_HALF: UInt256 =
        UInt256::from_hex("3FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF6");
}

impl From<i8> for Field25519 {
    fn from(n: i8) -> Self {
        Self::from(n as i64)
    }
}

impl From<i16> for Field25519 {
    fn from(n: i16) -> Self {
        Self::from(n as i64)
    }
}

impl From<i32> for Field25519 {
    fn from(n: i32) -> Self {
        Self::from(n as i64)
    }
}

impl From<i64> for Field25519 {
    fn from(n: i64) -> Self {
        if n >= 0 {
            Self::new((n as u64).into())
        } else {
            Self::new(Self::MODULUS - n.unsigned_abs().into())
        }
    }
}

impl From<u8> for Field25519 {
    fn from(n: u8) -> Self {
        Self::from(n as u64)
    }
}

impl From<u16> for Field25519 {
    fn from(n: u16) -> Self {
        Self::from(n as u64)
    }
}

impl From<u32> for Field25519 {
    fn from(n: u32) -> Self {
        Self::from(n as u64)
    }
}

impl From<u64> for Field25519 {
    fn from(n: u64) -> Self {
        Self::new(n.into())
    }
}

impl fmt::Debug for Field25519 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.canonical())
    }
}

impl Add for Field25519 {
    type Output = Self;

    fn add(self, rps: Self) -> Self::Output {
        Self {
            n: Self::reduce_add(self.n + rps.n),
        }
    }
}

impl Add<&Self> for Field25519 {
    type Output = Self;

    fn add(self, rps: &Self) -> Self::Output {
        Self {
            n: Self::reduce_add(self.n + rps.n),
        }
    }
}

impl Add<Field25519> for &Field25519 {
    type Output = Field25519;

    fn add(self, rps: Field25519) -> Self::Output {
        Self::Output {
            n: Self::Output::reduce_add(self.n + rps.n),
        }
    }
}

impl<'a> Add<&'a Field25519> for &Field25519 {
    type Output = Field25519;

    fn add(self, rps: &'a Field25519) -> Self::Output {
        Self::Output {
            n: Self::Output::reduce_add(self.n + rps.n),
        }
    }
}

impl AddAssign for Field25519 {
    #[inline]
    fn add_assign(&mut self, rps: Self) {
        *self = *self + rps
    }
}

impl AddAssign<&Self> for Field25519 {
    #[inline]
    fn add_assign(&mut self, rps: &Self) {
        *self = *self + *rps
    }
}

impl Double for Field25519 {
    type Output = Self;

    fn double(self) -> Self {
        Self {
            n: Self::reduce_add(self.n << 1),
        }
    }
}

impl Double for &Field25519 {
    type Output = Field25519;

    fn double(self) -> Self::Output {
        Self::Output {
            n: Self::Output::reduce_add(self.n << 1),
        }
    }
}

impl Neg for Field25519 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        if self.n != UInt256::ZERO {
            Self {
                n: Self::MODULUS - self.n,
            }
        } else {
            Self::ZERO
        }
    }
}

impl Neg for &Field25519 {
    type Output = Field25519;

    fn neg(self) -> Self::Output {
        if self.n != UInt256::ZERO {
            Self::Output {
                n: Self::Output::MODULUS - self.n,
            }
        } else {
            Self::Output::ZERO
        }
    }
}

impl Sub for Field25519 {
    type Output = Self;

    fn sub(self, rps: Self) -> Self::Output {
        let mut n = self.n - rps.n;
        if n >= Self::MODULUS {
            n += Self::MODULUS
        }
        Self { n }
    }
}

impl Sub<&Self> for Field25519 {
    type Output = Self;

    fn sub(self, rps: &Self) -> Self::Output {
        let mut n = self.n - rps.n;
        if n >= Self::MODULUS {
            n += Self::MODULUS
        }
        Self { n }
    }
}

impl Sub<Field25519> for &Field25519 {
    type Output = Field25519;

    fn sub(self, rps: Field25519) -> Self::Output {
        let mut n = self.n - rps.n;
        if n >= Self::Output::MODULUS {
            n += Self::Output::MODULUS
        }
        Self::Output { n }
    }
}

impl<'a> Sub<&'a Field25519> for &Field25519 {
    type Output = Field25519;

    fn sub(self, rps: &'a Field25519) -> Self::Output {
        let mut n = self.n - rps.n;
        if n >= Self::Output::MODULUS {
            n += Self::Output::MODULUS
        }
        Self::Output { n }
    }
}

impl SubAssign for Field25519 {
    #[inline]
    fn sub_assign(&mut self, rps: Self) {
        *self = *self - rps
    }
}

impl SubAssign<&Self> for Field25519 {
    #[inline]
    fn sub_assign(&mut self, rps: &Self) {
        *self = *self - *rps
    }
}

impl Mul for Field25519 {
    type Output = Self;

    fn mul(self, rps: Self) -> Self::Output {
        Self {
            n: Self::reduce_mul(self.n.widening_mul(rps.n)),
        }
    }
}

impl Mul<&Self> for Field25519 {
    type Output = Self;

    fn mul(self, rps: &Self) -> Self::Output {
        Self {
            n: Self::reduce_mul(self.n.widening_mul(rps.n)),
        }
    }
}

impl Mul<Field25519> for &Field25519 {
    type Output = Field25519;

    fn mul(self, rps: Field25519) -> Self::Output {
        Self::Output {
            n: Self::Output::reduce_mul(self.n.widening_mul(rps.n)),
        }
    }
}

impl<'a> Mul<&'a Field25519> for &Field25519 {
    type Output = Field25519;

    fn mul(self, rps: &'a Field25519) -> Self::Output {
        Self::Output {
            n: Self::Output::reduce_mul(self.n.widening_mul(rps.n)),
        }
    }
}

impl MulAssign for Field25519 {
    #[inline]
    fn mul_assign(&mut self, rps: Self) {
        *self = *self * rps
    }
}

impl MulAssign<&Self> for Field25519 {
    #[inline]
    fn mul_assign(&mut self, rps: &Self) {
        *self = *self * *rps
    }
}

impl Square for Field25519 {
    type Output = Self;

    fn square(self) -> Self {
        Self {
            n: Self::reduce_mul(self.n.widening_square()),
        }
    }
}

impl Square for &Field25519 {
    type Output = Field25519;

    fn square(self) -> Self::Output {
        Self::Output {
            n: Self::Output::reduce_mul(self.n.widening_square()),
        }
    }
}

impl Inv for Field25519 {
    type Output = Option<Self>;

    fn inv(self) -> Self::Output {
        // Extended Binary GCD (classic algorithm)
        // https://eprint.iacr.org/2020/972
        let mut a = self.canonical();
        let mut b = Self::MODULUS;
        let mut c = Self::ONE;
        let mut d = Self::ZERO;
        while a != UInt256::ZERO {
            if a.is_even() {
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
        if b != UInt256::ONE {
            return None;
        }
        Some(d)
    }
}

impl Div for Field25519 {
    type Output = Option<Self>;

    fn div(self, rps: Self) -> Self::Output {
        rps.inv().map(|v| self * v)
    }
}

impl Div<&Self> for Field25519 {
    type Output = Option<Self>;

    #[inline]
    fn div(self, rps: &Self) -> Self::Output {
        self / *rps
    }
}

impl Sqrt for Field25519 {
    type Output = Option<Self>;

    fn sqrt(self) -> Option<Self> {
        // p = 5 mod 8
        let a = self.double();
        let b = square_and_multiply(a, Self::P_MINUS_5_EIGHTH);
        let c = a * b.square();
        let d = self * b * (c - Self::ONE);
        if d.square() == self { Some(d) } else { None }
    }
}

impl Sum for Field25519 {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|lps, rps| lps + rps).unwrap_or(Self::ZERO)
    }
}

impl<'a> Sum<&'a Self> for Field25519 {
    #[inline]
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.copied().sum()
    }
}

impl Product for Field25519 {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|lps, rps| lps * rps).unwrap_or(Self::ONE)
    }
}

impl<'a> Product<&'a Self> for Field25519 {
    #[inline]
    fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.copied().product()
    }
}

impl LeftZero for Field25519 {
    const LEFT_ZERO: Self = Self { n: UInt256::ZERO };
}

impl RightZero for Field25519 {
    const RIGHT_ZERO: Self = Self { n: UInt256::ZERO };
}

impl Zero for Field25519 {
    const ZERO: Self = Self { n: UInt256::ZERO };
}

impl LeftOne for Field25519 {
    const LEFT_ONE: Self = Self {
        n: UInt256::from_hex("0000000000000000000000000000000000000000000000000000000000000026"),
    };
}

impl RightOne for Field25519 {
    const RIGHT_ONE: Self = Self {
        n: UInt256::from_hex("0000000000000000000000000000000000000000000000000000000000000026"),
    };
}

impl One for Field25519 {
    const ONE: Self = Self {
        n: UInt256::from_hex("0000000000000000000000000000000000000000000000000000000000000026"),
    };
}

impl Set for Field25519 {}

impl AdditiveCommutativeMagma for Field25519 {}

impl AdditiveSemigroup for Field25519 {}

impl AdditiveMonoid for Field25519 {}

impl MultiplicativeCommutativeMagma for Field25519 {}

impl MultiplicativeSemigroup for Field25519 {}

impl MultiplicativeMonoid for Field25519 {}

impl DivisionRing for Field25519 {}

impl IntegerRing for Field25519 {
    type Int = UInt256;

    fn new(n: UInt256) -> Self {
        Self {
            n: Self::to_form(n),
        }
    }
    fn with_limb(n: <Self::Int as Integer>::Limb) -> Self {
        Self {
            n: Self::to_form(n.into()),
        }
    }

    fn canonical(self) -> UInt256 {
        Self::from_form(self.n)
    }
    fn absolute(self) -> UInt256 {
        let n = self.canonical();
        if n <= Self::P_MINUS_1_HALF {
            n
        } else {
            Self::MODULUS - n
        }
    }

    const BITS: u32 = 255;
    const MODULUS: UInt256 =
        UInt256::from_hex("7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFED");
}

impl Serialize for Field25519 {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let bytes: [u8; 32] = self.canonical().to_le_bytes();
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Field25519 {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let bytes = <[u8; 32]>::deserialize(deserializer)?;
        let n = UInt256::from_le_bytes(bytes);
        Ok(Self::new(n))
    }
}
