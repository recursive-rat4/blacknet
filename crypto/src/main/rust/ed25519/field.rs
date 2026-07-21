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
    AdditiveCommutativeMagma, AdditiveSemigroup, Double, IntegerModRing, Inv, LeftOne, LeftZero,
    MultiplicativeCommutativeMagma, MultiplicativeSemigroup, One, RightOne, RightZero, Semifield,
    Set, Sqrt, Square, Zero,
};
use crate::bigint::{UInt256, UInt512};
use crate::branchless::{BlAbs, BlAssign, BlEq, BlOption, BlOrd, BlSelect, BlSwap};
use crate::integer::Integer;
use core::array;
use core::fmt;
use core::iter::{Product, Sum};
use core::ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub, SubAssign};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use zeroize::DefaultIsZeroes;

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

    pub(crate) fn add_lazy(self, rps: Self) -> Self {
        let n = self.n + rps.n;
        Self { n }
    }

    pub(crate) fn double_lazy(self) -> Self {
        let n = self.n + self.n;
        Self { n }
    }

    pub(crate) fn neg_lazy(self) -> Self {
        let n = Self::MODULUS - self.n;
        Self { n }
    }

    pub(crate) fn sub_lazy(self, rps: Self) -> Self {
        let n = Self::MODULUS - rps.n + self.n;
        Self { n }
    }

    fn reduce_256(n: UInt256) -> UInt256 {
        let (x, o) = n.overflowing_sub(Self::MODULUS);
        x.bl_select(n, o)
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

    fn halve(self) -> Self {
        let mut n = self.n;
        n.bl_assign(n + Self::MODULUS, n.is_odd());
        n >>= 1;
        Self { n }
    }

    fn egcd(self, rps: Self) -> BlOption<Self> {
        // Extended Binary GCD (classic algorithm)
        // https://eprint.iacr.org/2020/972
        let mut a = self.canonical();
        let mut b = Self::MODULUS;
        let mut c = rps;
        let mut d = Self::ZERO;
        for _ in 0..508 {
            let a_is_odd = a.is_odd();
            let a_is_less = a.bl_lt(&b);
            a.bl_swap(&mut b, a_is_odd & a_is_less);
            c.bl_swap(&mut d, a_is_odd & a_is_less);
            a -= UInt256::ZERO.bl_select(b, a_is_odd);
            c -= Self::ZERO.bl_select(d, a_is_odd);
            a >>= 1;
            c = c.halve();
        }
        BlOption::new(d, b.bl_eq(&UInt256::ONE))
    }

    fn square_n<const N: usize>(mut self) -> Self {
        for _ in 0..N {
            self = self.square()
        }
        self
    }

    fn pow_p_minus_5_eighth(self) -> Self {
        let b1 = self;
        let b10 = b1.square();
        let b100 = b10.square();
        let b1000 = b100.square();
        let b1001 = b1 * b1000;
        let b1011 = b10 * b1001;
        let b10110 = b1011.square();
        let b11111 = b1001 * b10110;
        let b111110 = b11111.square();
        let x10 = b111110.square_n::<4>() * b11111;
        let x20 = x10.square_n::<10>() * x10;
        let x40 = x20.square_n::<20>() * x20;
        let x50 = x40.square_n::<10>() * x10;
        let x100 = x50.square_n::<50>() * x50;
        let x200 = x100.square_n::<100>() * x100;
        let x250 = x200.square_n::<50>() * x50;
        x250.square_n::<2>() * b1
    }

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
        let s = n < 0;
        let mut n = UInt256::from(n.bl_unsigned_abs());
        let x = Self::MODULUS - n;
        n.bl_assign(x, s);
        Self { n }
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
            n: Self::reduce_256(self.n + self.n),
        }
    }
}

impl Double for &Field25519 {
    type Output = Field25519;

    fn double(self) -> Self::Output {
        Self::Output {
            n: Self::Output::reduce_256(self.n + self.n),
        }
    }
}

impl Neg for Field25519 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let z = self.n.bl_eq(&UInt256::ZERO);
        let x = Self::MODULUS - self.n;
        let n = x.bl_select(UInt256::ZERO, z);
        Self { n }
    }
}

impl Neg for &Field25519 {
    type Output = Field25519;

    fn neg(self) -> Self::Output {
        let z = self.n.bl_eq(&UInt256::ZERO);
        let x = Self::Output::MODULUS - self.n;
        let n = x.bl_select(UInt256::ZERO, z);
        Self::Output { n }
    }
}

impl Sub for Field25519 {
    type Output = Self;

    fn sub(self, rps: Self) -> Self::Output {
        let (mut n, o) = self.n.overflowing_sub(rps.n);
        #[allow(clippy::suspicious_arithmetic_impl)]
        n.bl_assign(n + Self::MODULUS, o);
        Self { n }
    }
}

impl Sub<&Self> for Field25519 {
    type Output = Self;

    fn sub(self, rps: &Self) -> Self::Output {
        let (mut n, o) = self.n.overflowing_sub(rps.n);
        #[allow(clippy::suspicious_arithmetic_impl)]
        n.bl_assign(n + Self::MODULUS, o);
        Self { n }
    }
}

impl Sub<Field25519> for &Field25519 {
    type Output = Field25519;

    fn sub(self, rps: Field25519) -> Self::Output {
        let (mut n, o) = self.n.overflowing_sub(rps.n);
        #[allow(clippy::suspicious_arithmetic_impl)]
        n.bl_assign(n + Self::Output::MODULUS, o);
        Self::Output { n }
    }
}

impl<'a> Sub<&'a Field25519> for &Field25519 {
    type Output = Field25519;

    fn sub(self, rps: &'a Field25519) -> Self::Output {
        let (mut n, o) = self.n.overflowing_sub(rps.n);
        #[allow(clippy::suspicious_arithmetic_impl)]
        n.bl_assign(n + Self::Output::MODULUS, o);
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
    type Output = BlOption<Self>;

    fn inv(self) -> Self::Output {
        Field25519::egcd(self, Field25519::ONE)
    }
}

impl Inv for &Field25519 {
    type Output = BlOption<Field25519>;

    fn inv(self) -> Self::Output {
        Field25519::egcd(*self, Field25519::ONE)
    }
}

impl Div for Field25519 {
    type Output = BlOption<Self>;

    fn div(self, rps: Self) -> Self::Output {
        Field25519::egcd(rps, self)
    }
}

impl Div<&Self> for Field25519 {
    type Output = BlOption<Self>;

    fn div(self, rps: &Self) -> Self::Output {
        Field25519::egcd(*rps, self)
    }
}

impl Div<Field25519> for &Field25519 {
    type Output = BlOption<Field25519>;

    fn div(self, rps: Field25519) -> Self::Output {
        Field25519::egcd(rps, *self)
    }
}

impl<'a> Div<&'a Field25519> for &Field25519 {
    type Output = BlOption<Field25519>;

    fn div(self, rps: &'a Field25519) -> Self::Output {
        Field25519::egcd(*rps, *self)
    }
}

impl Sqrt for Field25519 {
    type Output = Option<Self>;

    fn sqrt(self) -> Option<Self> {
        // p = 5 mod 8
        let a = self.double();
        let b = a.pow_p_minus_5_eighth();
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

impl MultiplicativeCommutativeMagma for Field25519 {}

impl MultiplicativeSemigroup for Field25519 {}

impl Semifield for Field25519 {}

impl IntegerModRing for Field25519 {
    type Int = UInt256;
    type Modulus = UInt256;

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
        let x = Self::MODULUS - n;
        let g = n.bl_gt(&Self::P_MINUS_1_HALF);
        n.bl_select(x, g)
    }

    const BITS: u32 = 255;
    const MODULUS: Self::Modulus =
        UInt256::from_hex("7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFED");
}

impl BlSelect for Field25519 {
    type Output = Self;

    fn bl_select(self, rps: Self, condition: bool) -> Self {
        let n = self.n.bl_select(rps.n, condition);
        Self { n }
    }
}

impl BlSelect<&Self> for Field25519 {
    type Output = Self;

    fn bl_select(self, rps: &Self, condition: bool) -> Self {
        let n = self.n.bl_select(&rps.n, condition);
        Self { n }
    }
}

impl BlSelect<Field25519> for &Field25519 {
    type Output = Field25519;

    fn bl_select(self, rps: Field25519, condition: bool) -> Self::Output {
        let n = (&self.n).bl_select(rps.n, condition);
        Self::Output { n }
    }
}

impl BlSelect for &Field25519 {
    type Output = Field25519;

    fn bl_select(self, rps: Self, condition: bool) -> Self::Output {
        let n = (&self.n).bl_select(&rps.n, condition);
        Self::Output { n }
    }
}

impl BlSwap for Field25519 {
    fn bl_swap(&mut self, rps: &mut Self, condition: bool) {
        self.n.bl_swap(&mut rps.n, condition)
    }
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

impl DefaultIsZeroes for Field25519 {}
