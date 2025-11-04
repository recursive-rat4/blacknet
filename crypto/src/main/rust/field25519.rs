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

use crate::bigint::{UInt256, UInt512};
use crate::field::{Field, PrimeField};
use crate::integer::Integer;
use crate::magma::{
    AdditiveCommutativeMagma, AdditiveMagma, Inv, MultiplicativeCommutativeMagma,
    MultiplicativeMagma,
};
use crate::monoid::{AdditiveMonoid, MultiplicativeMonoid};
use crate::ring::{IntegerRing, Ring};
use crate::semigroup::{AdditiveSemigroup, MultiplicativeSemigroup};
use core::fmt::{Debug, Formatter, Result};
use core::iter::{Product, Sum};
use core::ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub, SubAssign};

// 2²⁵⁵ - 19

#[derive(Clone, Copy, Default, Eq, PartialEq)]
pub struct Field25519 {
    n: UInt256,
}

impl Field25519 {
    pub fn from_hex(hex: &str) -> Self {
        Self::new(UInt256::from_hex(hex))
    }

    pub fn sqrt(self) -> Option<Self> {
        // Tonelli–Shanks algorithm
        let ls = self.legendre_symbol();
        if ls == Self::ONE {
            let mut m = Self::S;
            let mut c = Self::Z_IN_Q;
            let mut t = self.square_and_multiply(Self::Q);
            let mut r = self.square_and_multiply(Self::Q_PLUS_1_HALVED);
            loop {
                if t == Self::ZERO {
                    return Some(Self::ZERO);
                } else if t == Self::ONE {
                    return Some(r);
                } else {
                    let mut i = Self::ONE;
                    while t.power(Self::TWO.power(i)) != Self::ONE {
                        i += Self::ONE;
                    }
                    let b = c.power(Self::TWO.power(m - i - Self::ONE));
                    m = i;
                    c = b.square();
                    t *= c;
                    r *= b;
                }
            }
        } else if ls == Self::ZERO {
            Some(Self::ZERO)
        } else {
            None
        }
    }

    fn legendre_symbol(self) -> Self {
        self.square_and_multiply(Self::P_MINUS_1_HALVED_BITS)
    }

    fn power(self, rps: Self) -> Self {
        self.square_and_multiply(rps.canonical().bits::<{ Self::BITS as usize }>())
    }

    fn to_form(x: UInt256) -> UInt256 {
        Self::reduce_mul(x.mul(Self::R2))
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

    const R2: UInt256 =
        UInt256::from_hex("00000000000000000000000000000000000000000000000000000000000005A4");
    const RN: u64 = 0x86BCA1AF286BCA1B;
    const TWO: Self = Self {
        n: UInt256::from_hex("000000000000000000000000000000000000000000000000000000000000004C"),
    };
    const TWO_INVERTED: Self = Self {
        n: UInt256::from_hex("0000000000000000000000000000000000000000000000000000000000000013"),
    };
    const P_MINUS_1_HALVED_NUM: UInt256 =
        UInt256::from_hex("3FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF6");
    const P_MINUS_1_HALVED_BITS: [bool; 254] = Self::P_MINUS_1_HALVED_NUM.bits();
    const S: Self = Self::TWO;
    const Q: [bool; 253] =
        UInt256::from_hex("1FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFB")
            .bits();
    const Q_PLUS_1_HALVED: [bool; 252] =
        UInt256::from_hex("0FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFE")
            .bits();
    const Z_IN_Q: Self = Self {
        n: UInt256::from_hex("75776B0BD6C71BA86D6E16BF336202D103F590FDB51BE9ED3B5807D4FE2BDB04"),
    };
}

impl From<i8> for Field25519 {
    fn from(n: i8) -> Self {
        Self::from(n as i32)
    }
}

impl From<i16> for Field25519 {
    fn from(n: i16) -> Self {
        Self::from(n as i32)
    }
}

impl From<i32> for Field25519 {
    fn from(n: i32) -> Self {
        if n >= 0 {
            Self::new((n as u64).into())
        } else {
            Self::new(Self::MODULUS - ((-n) as u64).into())
        }
    }
}

impl From<u8> for Field25519 {
    fn from(n: u8) -> Self {
        Self::from(n as u32)
    }
}

impl From<u16> for Field25519 {
    fn from(n: u16) -> Self {
        Self::from(n as u32)
    }
}

impl From<u32> for Field25519 {
    fn from(n: u32) -> Self {
        Self::new((n as u64).into())
    }
}

impl Debug for Field25519 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
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

impl AddAssign for Field25519 {
    #[inline]
    fn add_assign(&mut self, rps: Self) {
        *self = *self + rps
    }
}

impl Neg for Field25519 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::ZERO - self
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

impl SubAssign for Field25519 {
    #[inline]
    fn sub_assign(&mut self, rps: Self) {
        *self = *self - rps
    }
}

impl Mul for Field25519 {
    type Output = Self;

    fn mul(self, rps: Self) -> Self::Output {
        Self {
            n: Self::reduce_mul(self.n.mul(rps.n)),
        }
    }
}

impl MulAssign for Field25519 {
    #[inline]
    fn mul_assign(&mut self, rps: Self) {
        *self = *self * rps
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
                c *= Self::TWO_INVERTED;
            } else {
                if a < b {
                    (a, b) = (b, a);
                    (c, d) = (d, c);
                }
                a -= b;
                a >>= 1;
                c -= d;
                c *= Self::TWO_INVERTED;
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

impl Sum for Field25519 {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|lps, rps| lps + rps).unwrap_or(Self::ZERO)
    }
}

impl Product for Field25519 {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|lps, rps| lps * rps).unwrap_or(Self::ONE)
    }
}

impl AdditiveMagma for Field25519 {
    fn double(self) -> Self {
        Self {
            n: Self::reduce_add(self.n << 1),
        }
    }
}

impl AdditiveCommutativeMagma for Field25519 {}

impl AdditiveSemigroup for Field25519 {
    const LEFT_IDENTITY: Self = Self { n: UInt256::ZERO };
    const RIGHT_IDENTITY: Self = Self { n: UInt256::ZERO };
}

impl AdditiveMonoid for Field25519 {
    const IDENTITY: Self = Self { n: UInt256::ZERO };
}

impl MultiplicativeMagma for Field25519 {
    fn square(self) -> Self {
        Self {
            n: Self::reduce_mul(self.n.square()),
        }
    }
}

impl MultiplicativeCommutativeMagma for Field25519 {}

impl MultiplicativeSemigroup for Field25519 {
    const LEFT_IDENTITY: Self = Self {
        n: UInt256::from_hex("0000000000000000000000000000000000000000000000000000000000000026"),
    };
    const RIGHT_IDENTITY: Self = Self {
        n: UInt256::from_hex("0000000000000000000000000000000000000000000000000000000000000026"),
    };
}

impl MultiplicativeMonoid for Field25519 {
    const IDENTITY: Self = Self {
        n: UInt256::from_hex("0000000000000000000000000000000000000000000000000000000000000026"),
    };
}

impl Ring for Field25519 {
    type Int = UInt256;
}

impl IntegerRing for Field25519 {
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
        if n <= Self::P_MINUS_1_HALVED_NUM {
            n
        } else {
            Self::MODULUS - n
        }
    }

    const BITS: u32 = 255;
    const MODULUS: UInt256 =
        UInt256::from_hex("7FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFED");
}

impl PrimeField for Field25519 {}
