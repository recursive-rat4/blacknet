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
use core::array;
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
    /// # Panics
    /// On inappropriate string.
    pub fn with_hex(hex: &str) -> Self {
        Self::new(UInt256::from_hex(hex))
    }

    /// Construct an element.
    /// # Safety
    /// `n` is canonical representative.
    pub const unsafe fn from_unchecked(n: UInt256) -> Self {
        Self { n }
    }

    fn reduce_256(mut x: UInt256) -> UInt256 {
        if x >= Self::MODULUS {
            x -= Self::MODULUS
        }
        x
    }

    fn reduce_512(x: UInt512) -> UInt256 {
        let mut c: bool = false;
        let mut x = x.limbs();
        let mut y: [u64; 4] = array::from_fn(|i| {
            let ll = x[i + 4] as u128 * 38;
            (x[i], c) = x[i].carrying_add(ll as u64, c);
            (ll >> 64) as u64
        });
        (y[3], _) = y[3].carrying_add(0, c);

        y[3] = (y[3] << 1) | (x[3] >> 63);
        x[3] &= 0x7FFFFFFFFFFFFFFF;
        (x[0], c) = x[0].carrying_add(y[3] * 19, false);
        (x[1], c) = x[1].carrying_add(y[0], c);
        (x[2], c) = x[2].carrying_add(y[1], c);
        (x[3], _) = x[3].carrying_add(y[2], c);

        let n = UInt256::from([x[0], x[1], x[2], x[3]]);
        Self::reduce_256(n)
    }

    fn halve(mut self) -> Self {
        if self.n.is_odd() {
            self.n += Self::MODULUS;
        }
        self.n >>= 1;
        self
    }

    fn egcd(self, rps: Self) -> Option<Self> {
        // Extended Binary GCD (classic algorithm)
        // https://eprint.iacr.org/2020/972
        let mut a = self.canonical();
        let mut b = Self::MODULUS;
        let mut c = rps;
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
            Self {
                n: (n as u64).into(),
            }
        } else {
            Self {
                n: Self::MODULUS - n.unsigned_abs().into(),
            }
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
        Self { n: n.into() }
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
            n: Self::reduce_256(self.n + rps.n),
        }
    }
}

impl Add<&Self> for Field25519 {
    type Output = Self;

    fn add(self, rps: &Self) -> Self::Output {
        Self {
            n: Self::reduce_256(self.n + rps.n),
        }
    }
}

impl Add<Field25519> for &Field25519 {
    type Output = Field25519;

    fn add(self, rps: Field25519) -> Self::Output {
        Self::Output {
            n: Self::Output::reduce_256(self.n + rps.n),
        }
    }
}

impl<'a> Add<&'a Field25519> for &Field25519 {
    type Output = Field25519;

    fn add(self, rps: &'a Field25519) -> Self::Output {
        Self::Output {
            n: Self::Output::reduce_256(self.n + rps.n),
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
        *self = *self + rps
    }
}

impl Double for Field25519 {
    type Output = Self;

    fn double(self) -> Self {
        Self {
            n: Self::reduce_256(self.n << 1),
        }
    }
}

impl Double for &Field25519 {
    type Output = Field25519;

    fn double(self) -> Self::Output {
        Self::Output {
            n: Self::Output::reduce_256(self.n << 1),
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
        *self = *self - rps
    }
}

impl Mul for Field25519 {
    type Output = Self;

    fn mul(self, rps: Self) -> Self::Output {
        Self {
            n: Self::reduce_512(self.n.widening_mul(rps.n)),
        }
    }
}

impl Mul<&Self> for Field25519 {
    type Output = Self;

    fn mul(self, rps: &Self) -> Self::Output {
        Self {
            n: Self::reduce_512(self.n.widening_mul(rps.n)),
        }
    }
}

impl Mul<Field25519> for &Field25519 {
    type Output = Field25519;

    fn mul(self, rps: Field25519) -> Self::Output {
        Self::Output {
            n: Self::Output::reduce_512(self.n.widening_mul(rps.n)),
        }
    }
}

impl<'a> Mul<&'a Field25519> for &Field25519 {
    type Output = Field25519;

    fn mul(self, rps: &'a Field25519) -> Self::Output {
        Self::Output {
            n: Self::Output::reduce_512(self.n.widening_mul(rps.n)),
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
        *self = *self * rps
    }
}

impl Square for Field25519 {
    type Output = Self;

    fn square(self) -> Self {
        Self {
            n: Self::reduce_512(self.n.widening_square()),
        }
    }
}

impl Square for &Field25519 {
    type Output = Field25519;

    fn square(self) -> Self::Output {
        Self::Output {
            n: Self::Output::reduce_512(self.n.widening_square()),
        }
    }
}

impl Inv for Field25519 {
    type Output = Option<Self>;

    fn inv(self) -> Self::Output {
        Field25519::egcd(self, Field25519::ONE)
    }
}

impl Inv for &Field25519 {
    type Output = Option<Field25519>;

    fn inv(self) -> Self::Output {
        Field25519::egcd(*self, Field25519::ONE)
    }
}

impl Div for Field25519 {
    type Output = Option<Self>;

    fn div(self, rps: Self) -> Self::Output {
        Field25519::egcd(rps, self)
    }
}

impl Div<&Self> for Field25519 {
    type Output = Option<Self>;

    fn div(self, rps: &Self) -> Self::Output {
        Field25519::egcd(*rps, self)
    }
}

impl Div<Field25519> for &Field25519 {
    type Output = Option<Field25519>;

    fn div(self, rps: Field25519) -> Self::Output {
        Field25519::egcd(rps, *self)
    }
}

impl Div for &Field25519 {
    type Output = Option<Field25519>;

    fn div(self, rps: Self) -> Self::Output {
        Field25519::egcd(*rps, *self)
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
    const LEFT_ONE: Self = Self { n: UInt256::ONE };
}

impl RightOne for Field25519 {
    const RIGHT_ONE: Self = Self { n: UInt256::ONE };
}

impl One for Field25519 {
    const ONE: Self = Self { n: UInt256::ONE };
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
            n: Self::reduce_256(n),
        }
    }
    fn with_limb(n: <Self::Int as Integer>::Limb) -> Self {
        Self { n: n.into() }
    }

    fn canonical(&self) -> UInt256 {
        self.n
    }
    fn absolute(&self) -> UInt256 {
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
