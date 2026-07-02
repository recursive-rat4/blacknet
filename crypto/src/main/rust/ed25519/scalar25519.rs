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
use core::fmt;
use core::iter::{Product, Sum};
use core::mem::transmute;
use core::ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub, SubAssign};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use zeroize::DefaultIsZeroes;

/// The prime field `2²⁵² + 27742317777372353535851937790883648493`.
#[derive(Clone, Copy, Default, Eq, PartialEq)]
pub struct Scalar25519 {
    n: UInt256,
}

impl Scalar25519 {
    /// # Panics
    /// On inappropriate string.
    pub fn with_hex(hex: &str) -> Self {
        Self::new(UInt256::from_hex(hex))
    }

    /// Construct an element.
    /// # Safety
    /// `n` requires spare bit and Montgomery form.
    pub const unsafe fn from_unchecked(n: UInt256) -> Self {
        Self { n }
    }

    /// Construct an element.
    pub fn with_512(bytes: [u8; 64]) -> Self {
        let bytes: [[u8; 32]; 2] = unsafe { transmute(bytes) };
        let x = UInt256::from_le_bytes(bytes[0]);
        let l = Scalar25519::new(x);
        let x = UInt256::from_le_bytes(bytes[1]);
        let x = Self::to_form(x);
        let r = Scalar25519::new(x);
        l + r
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

    fn reduce_add(n: UInt256) -> UInt256 {
        let (x, o) = n.overflowing_sub(Self::MODULUS);
        x.bl_select(n, o)
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
        let mut b = Scalar25519::MODULUS;
        let mut c = rps;
        let mut d = Scalar25519::ZERO;
        for _ in 0..504 {
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
        // addchain: cost: 278
        let b1 = self;
        let b10 = b1.square();
        let b11 = b1 * b10;
        let b110 = b11.square();
        let b1000 = b10 * b110;
        let b1011 = b11 * b1000;
        let b1100 = b1 * b1011;
        let b10100 = b1000 * b1100;
        let b11010 = b110 * b10100;
        let b11101 = b11 * b11010;
        let b100011 = b110 * b11101;
        let b101111 = b1100 * b100011;
        let b1001001 = b11010 * b101111;
        let b1010001 = b1000 * b1001001;
        let b1010011 = b10 * b1010001;
        let b1100111 = b10100 * b1010011;
        let b1101001 = b10 * b1100111;
        let b1101011 = b10 * b1101001;
        let b1110011 = b1000 * b1101011;
        let b1111011 = b1000 * b1110011;
        let b10000000 = b101111 * b1010001;
        let i164 =
            ((b10000000.square_n::<127>() * b1010011).square_n::<8>() * b1111011).square_n::<7>();
        let i181 = ((b1110011 * i164).square_n::<6>() * b101111).square_n::<8>() * b1010001;
        let i207 = ((i181.square_n::<8>() * b1111011).square_n::<7>() * b1100111).square_n::<9>();
        let i229 = ((b1101011 * i207).square_n::<6>() * b1011).square_n::<13>() * b1001001;
        let i255 = ((i229.square_n::<6>() * b100011).square_n::<10>() * b1101001).square_n::<8>();
        let i272 = ((b1110011 * i255).square_n::<7>() * b1101011).square_n::<7>() * b1010011;
        i272.square_n::<5>() * b11101
    }

    const R2: UInt256 =
        UInt256::from_hex("0399411B7C309A3DCEEC73D217F5BE65D00E1BA768859347A40611E3449C0F01");
    const RN: u64 = 0xD2B51DA312547E1B;
    const P_MINUS_1_HALF: UInt256 =
        UInt256::from_hex("080000000000000000000000000000000A6F7CEF517BCE6B2C09318D2E7AE9F6");
}

impl From<i8> for Scalar25519 {
    fn from(n: i8) -> Self {
        Self::from(n as i64)
    }
}

impl From<i16> for Scalar25519 {
    fn from(n: i16) -> Self {
        Self::from(n as i64)
    }
}

impl From<i32> for Scalar25519 {
    fn from(n: i32) -> Self {
        Self::from(n as i64)
    }
}

impl From<i64> for Scalar25519 {
    fn from(n: i64) -> Self {
        let s = n < 0;
        let mut n = UInt256::from(n.bl_unsigned_abs());
        let x = Self::MODULUS - n;
        n.bl_assign(x, s);
        Self::new(n)
    }
}

impl From<u8> for Scalar25519 {
    fn from(n: u8) -> Self {
        Self::from(n as u64)
    }
}

impl From<u16> for Scalar25519 {
    fn from(n: u16) -> Self {
        Self::from(n as u64)
    }
}

impl From<u32> for Scalar25519 {
    fn from(n: u32) -> Self {
        Self::from(n as u64)
    }
}

impl From<u64> for Scalar25519 {
    fn from(n: u64) -> Self {
        Self::new(n.into())
    }
}

impl fmt::Debug for Scalar25519 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.canonical())
    }
}

impl Add for Scalar25519 {
    type Output = Self;

    fn add(self, rps: Self) -> Self::Output {
        Self {
            n: Self::reduce_add(self.n + rps.n),
        }
    }
}

impl Add<&Self> for Scalar25519 {
    type Output = Self;

    fn add(self, rps: &Self) -> Self::Output {
        Self {
            n: Self::reduce_add(self.n + rps.n),
        }
    }
}

impl Add<Scalar25519> for &Scalar25519 {
    type Output = Scalar25519;

    fn add(self, rps: Scalar25519) -> Self::Output {
        Self::Output {
            n: Self::Output::reduce_add(self.n + rps.n),
        }
    }
}

impl<'a> Add<&'a Scalar25519> for &Scalar25519 {
    type Output = Scalar25519;

    fn add(self, rps: &'a Scalar25519) -> Self::Output {
        Self::Output {
            n: Self::Output::reduce_add(self.n + rps.n),
        }
    }
}

impl AddAssign for Scalar25519 {
    #[inline]
    fn add_assign(&mut self, rps: Self) {
        *self = *self + rps
    }
}

impl AddAssign<&Self> for Scalar25519 {
    #[inline]
    fn add_assign(&mut self, rps: &Self) {
        *self = *self + rps
    }
}

impl Double for Scalar25519 {
    type Output = Self;

    fn double(self) -> Self {
        Self {
            n: Self::reduce_add(self.n << 1),
        }
    }
}

impl Double for &Scalar25519 {
    type Output = Scalar25519;

    fn double(self) -> Self::Output {
        Self::Output {
            n: Self::Output::reduce_add(self.n << 1),
        }
    }
}

impl Neg for Scalar25519 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let z = self.n.bl_eq(&UInt256::ZERO);
        let x = Self::MODULUS - self.n;
        let n = x.bl_select(UInt256::ZERO, z);
        Self { n }
    }
}

impl Neg for &Scalar25519 {
    type Output = Scalar25519;

    fn neg(self) -> Self::Output {
        let z = self.n.bl_eq(&UInt256::ZERO);
        let x = Self::Output::MODULUS - self.n;
        let n = x.bl_select(UInt256::ZERO, z);
        Self::Output { n }
    }
}

impl Sub for Scalar25519 {
    type Output = Self;

    fn sub(self, rps: Self) -> Self::Output {
        let (mut n, o) = self.n.overflowing_sub(rps.n);
        #[allow(clippy::suspicious_arithmetic_impl)]
        n.bl_assign(n + Self::MODULUS, o);
        Self { n }
    }
}

impl Sub<&Self> for Scalar25519 {
    type Output = Self;

    fn sub(self, rps: &Self) -> Self::Output {
        let (mut n, o) = self.n.overflowing_sub(rps.n);
        #[allow(clippy::suspicious_arithmetic_impl)]
        n.bl_assign(n + Self::MODULUS, o);
        Self { n }
    }
}

impl Sub<Scalar25519> for &Scalar25519 {
    type Output = Scalar25519;

    fn sub(self, rps: Scalar25519) -> Self::Output {
        let (mut n, o) = self.n.overflowing_sub(rps.n);
        #[allow(clippy::suspicious_arithmetic_impl)]
        n.bl_assign(n + Self::Output::MODULUS, o);
        Self::Output { n }
    }
}

impl<'a> Sub<&'a Scalar25519> for &Scalar25519 {
    type Output = Scalar25519;

    fn sub(self, rps: &'a Scalar25519) -> Self::Output {
        let (mut n, o) = self.n.overflowing_sub(rps.n);
        #[allow(clippy::suspicious_arithmetic_impl)]
        n.bl_assign(n + Self::Output::MODULUS, o);
        Self::Output { n }
    }
}

impl SubAssign for Scalar25519 {
    #[inline]
    fn sub_assign(&mut self, rps: Self) {
        *self = *self - rps
    }
}

impl SubAssign<&Self> for Scalar25519 {
    #[inline]
    fn sub_assign(&mut self, rps: &Self) {
        *self = *self - rps
    }
}

impl Mul for Scalar25519 {
    type Output = Self;

    fn mul(self, rps: Self) -> Self::Output {
        Self {
            n: Self::reduce_mul(self.n.widening_mul(rps.n)),
        }
    }
}

impl Mul<&Self> for Scalar25519 {
    type Output = Self;

    fn mul(self, rps: &Self) -> Self::Output {
        Self {
            n: Self::reduce_mul(self.n.widening_mul(rps.n)),
        }
    }
}

impl Mul<Scalar25519> for &Scalar25519 {
    type Output = Scalar25519;

    fn mul(self, rps: Scalar25519) -> Self::Output {
        Self::Output {
            n: Self::Output::reduce_mul(self.n.widening_mul(rps.n)),
        }
    }
}

impl<'a> Mul<&'a Scalar25519> for &Scalar25519 {
    type Output = Scalar25519;

    fn mul(self, rps: &'a Scalar25519) -> Self::Output {
        Self::Output {
            n: Self::Output::reduce_mul(self.n.widening_mul(rps.n)),
        }
    }
}

impl MulAssign for Scalar25519 {
    #[inline]
    fn mul_assign(&mut self, rps: Self) {
        *self = *self * rps
    }
}

impl MulAssign<&Self> for Scalar25519 {
    #[inline]
    fn mul_assign(&mut self, rps: &Self) {
        *self = *self * rps
    }
}

impl Square for Scalar25519 {
    type Output = Self;

    fn square(self) -> Self {
        Self {
            n: Self::reduce_mul(self.n.widening_square()),
        }
    }
}

impl Square for &Scalar25519 {
    type Output = Scalar25519;

    fn square(self) -> Self::Output {
        Self::Output {
            n: Self::Output::reduce_mul(self.n.widening_square()),
        }
    }
}

impl Inv for Scalar25519 {
    type Output = BlOption<Self>;

    fn inv(self) -> Self::Output {
        Scalar25519::egcd(self, Scalar25519::ONE)
    }
}

impl Inv for &Scalar25519 {
    type Output = BlOption<Scalar25519>;

    fn inv(self) -> Self::Output {
        Scalar25519::egcd(*self, Scalar25519::ONE)
    }
}

impl Div for Scalar25519 {
    type Output = BlOption<Self>;

    fn div(self, rps: Self) -> Self::Output {
        Scalar25519::egcd(rps, self)
    }
}

impl Div<&Self> for Scalar25519 {
    type Output = BlOption<Self>;

    fn div(self, rps: &Self) -> Self::Output {
        Scalar25519::egcd(*rps, self)
    }
}

impl Div<Scalar25519> for &Scalar25519 {
    type Output = BlOption<Scalar25519>;

    fn div(self, rps: Scalar25519) -> Self::Output {
        Scalar25519::egcd(rps, *self)
    }
}

impl<'a> Div<&'a Scalar25519> for &Scalar25519 {
    type Output = BlOption<Scalar25519>;

    fn div(self, rps: &'a Scalar25519) -> Self::Output {
        Scalar25519::egcd(*rps, *self)
    }
}

impl Sqrt for Scalar25519 {
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

impl Sum for Scalar25519 {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|lps, rps| lps + rps).unwrap_or(Self::ZERO)
    }
}

impl<'a> Sum<&'a Self> for Scalar25519 {
    #[inline]
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.copied().sum()
    }
}

impl Product for Scalar25519 {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|lps, rps| lps * rps).unwrap_or(Self::ONE)
    }
}

impl<'a> Product<&'a Self> for Scalar25519 {
    #[inline]
    fn product<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.copied().product()
    }
}

impl LeftZero for Scalar25519 {
    const LEFT_ZERO: Self = Self { n: UInt256::ZERO };
}

impl RightZero for Scalar25519 {
    const RIGHT_ZERO: Self = Self { n: UInt256::ZERO };
}

impl Zero for Scalar25519 {
    const ZERO: Self = Self { n: UInt256::ZERO };
}

impl LeftOne for Scalar25519 {
    const LEFT_ONE: Self = Self {
        n: UInt256::from_hex("0FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEC6EF5BF4737DCF70D6EC31748D98951D"),
    };
}

impl RightOne for Scalar25519 {
    const RIGHT_ONE: Self = Self {
        n: UInt256::from_hex("0FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEC6EF5BF4737DCF70D6EC31748D98951D"),
    };
}

impl One for Scalar25519 {
    const ONE: Self = Self {
        n: UInt256::from_hex("0FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEC6EF5BF4737DCF70D6EC31748D98951D"),
    };
}

impl Set for Scalar25519 {}

impl AdditiveCommutativeMagma for Scalar25519 {}

impl AdditiveSemigroup for Scalar25519 {}

impl MultiplicativeCommutativeMagma for Scalar25519 {}

impl MultiplicativeSemigroup for Scalar25519 {}

impl Semifield for Scalar25519 {}

impl IntegerModRing for Scalar25519 {
    type Int = UInt256;
    type Modulus = UInt256;

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

    fn canonical(&self) -> UInt256 {
        Self::from_form(self.n)
    }
    fn absolute(&self) -> UInt256 {
        let n = self.canonical();
        let x = Self::MODULUS - n;
        let g = n.bl_gt(&Self::P_MINUS_1_HALF);
        n.bl_select(x, g)
    }

    const BITS: u32 = 253;
    const MODULUS: Self::Modulus =
        UInt256::from_hex("1000000000000000000000000000000014DEF9DEA2F79CD65812631A5CF5D3ED");
}

impl BlSelect for Scalar25519 {
    type Output = Self;

    fn bl_select(self, rps: Self, condition: bool) -> Self {
        let n = self.n.bl_select(rps.n, condition);
        Self { n }
    }
}

impl BlSelect<&Self> for Scalar25519 {
    type Output = Self;

    fn bl_select(self, rps: &Self, condition: bool) -> Self {
        let n = self.n.bl_select(&rps.n, condition);
        Self { n }
    }
}

impl BlSelect<Scalar25519> for &Scalar25519 {
    type Output = Scalar25519;

    fn bl_select(self, rps: Scalar25519, condition: bool) -> Self::Output {
        let n = (&self.n).bl_select(rps.n, condition);
        Self::Output { n }
    }
}

impl BlSelect for &Scalar25519 {
    type Output = Scalar25519;

    fn bl_select(self, rps: Self, condition: bool) -> Self::Output {
        let n = (&self.n).bl_select(&rps.n, condition);
        Self::Output { n }
    }
}

impl BlSwap for Scalar25519 {
    fn bl_swap(&mut self, rps: &mut Self, condition: bool) {
        self.n.bl_swap(&mut rps.n, condition)
    }
}

impl Serialize for Scalar25519 {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let bytes: [u8; 32] = self.canonical().to_le_bytes();
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Scalar25519 {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let bytes = <[u8; 32]>::deserialize(deserializer)?;
        let n = UInt256::from_le_bytes(bytes);
        Ok(Self::new(n))
    }
}

impl DefaultIsZeroes for Scalar25519 {}
