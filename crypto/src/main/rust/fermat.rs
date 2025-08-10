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

use crate::field::PrimeField;
use crate::magma::{AdditiveMagma, Inv, MultiplicativeMagma};
use crate::monoid::{AdditiveMonoid, MultiplicativeMonoid};
use crate::ring::{IntegerRing, Ring, UnitalRing};
use crate::semigroup::MultiplicativeSemigroup;
use core::fmt::{Debug, Formatter, Result};
use core::iter::{Product, Sum};
use core::ops::{Add, AddAssign, Div, Mul, MulAssign, Neg, Sub, SubAssign};

// 2¹⁶ + 1

#[derive(Clone, Copy, Default, Eq)]
pub struct FermatField {
    n: i32,
}

impl FermatField {
    pub const fn new(n: i32) -> Self {
        Self {
            n: Self::reduce_add(n),
        }
    }

    // Lazy reduction
    pub const fn reduce(self) -> Self {
        Self::new(self.n)
    }

    const fn reduce_add(x: i32) -> i32 {
        (x & 0xFFFF) - (x >> 16)
    }

    const fn reduce_mul(x: i64) -> i32 {
        ((x & 0xFFFF) - (x >> 16)) as i32
    }

    pub const fn balanced(self) -> i32 {
        let x = Self::reduce_add(self.n);
        if x > Self::MODULUS / 2 {
            x - Self::MODULUS
        } else if x < -Self::MODULUS / 2 {
            x + Self::MODULUS
        } else {
            x
        }
    }

    const fn bits<const N: usize>(n: u32) -> [bool; N] {
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

    const P_MINUS_2: [bool; 16] = Self::bits(0xFFFF);
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

    fn add(self, rps: Self) -> Self::Output {
        Self { n: self.n + rps.n }
    }
}

impl AddAssign for FermatField {
    #[inline]
    fn add_assign(&mut self, rps: Self) {
        *self = *self + rps
    }
}

impl Neg for FermatField {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self { n: -self.n }
    }
}

impl Sub for FermatField {
    type Output = Self;

    fn sub(self, rps: Self) -> Self::Output {
        Self { n: self.n - rps.n }
    }
}

impl SubAssign for FermatField {
    #[inline]
    fn sub_assign(&mut self, rps: Self) {
        *self = *self - rps
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

impl MulAssign for FermatField {
    #[inline]
    fn mul_assign(&mut self, rps: Self) {
        *self = *self * rps
    }
}

impl Inv for FermatField {
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

impl Div for FermatField {
    type Output = Option<Self>;

    fn div(self, rps: Self) -> Self::Output {
        rps.inv().map(|v| self * v)
    }
}

impl Sum for FermatField {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|lps, rps| lps + rps).unwrap_or(Self::ZERO)
    }
}

impl Product for FermatField {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|lps, rps| lps * rps).unwrap_or(Self::UNITY)
    }
}

impl AdditiveMagma for FermatField {
    fn double(self) -> Self {
        Self { n: self.n << 1 }
    }
}

impl AdditiveMonoid for FermatField {
    const IDENTITY: Self = Self { n: 0 };
}

impl MultiplicativeMagma for FermatField {
    #[inline]
    fn square(self) -> Self {
        self * self
    }
}

impl MultiplicativeSemigroup for FermatField {
    const LEFT_IDENTITY: Self = Self { n: 1 };
    const RIGHT_IDENTITY: Self = Self { n: 1 };
}

impl MultiplicativeMonoid for FermatField {
    const IDENTITY: Self = Self { n: 1 };
}

impl Ring for FermatField {
    type BaseRing = Self;
    type Int = i32;
}

impl IntegerRing for FermatField {
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

    const BITS: usize = 17;
    const MODULUS: Self::Int = 65537;
}

impl PrimeField for FermatField {}
