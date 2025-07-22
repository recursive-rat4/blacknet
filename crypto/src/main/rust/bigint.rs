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

use core::array;
use core::cmp::Ordering;
use core::fmt;
use core::iter::zip;
use core::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign,
};

pub type UInt256 = BigInt<4>;
pub type UInt512 = BigInt<8>;

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct BigInt<const N: usize> {
    limbs: [u64; N],
}

impl<const N: usize> BigInt<N> {
    pub fn from_hex(hex: &str) -> Self {
        Self {
            limbs: array::from_fn(|i| {
                u64::from_str_radix(&hex[(N - i - 1) * 16..(N - i) * 16], 16).unwrap()
            }),
        }
    }
}

impl<const N: usize> fmt::Debug for BigInt<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for limb in self.limbs.iter().rev() {
            write!(f, "{limb:X}")?;
        }
        Ok(())
    }
}

impl<const N: usize> Default for BigInt<N> {
    fn default() -> Self {
        Self { limbs: [0; N] }
    }
}

impl<const N: usize> From<u64> for BigInt<N> {
    fn from(n: u64) -> Self {
        let mut limbs = [0; N];
        limbs[0] = n;
        Self { limbs }
    }
}

impl<const N: usize> From<[u64; N]> for BigInt<N> {
    #[inline]
    fn from(limbs: [u64; N]) -> Self {
        Self { limbs }
    }
}

impl<const N: usize> Ord for BigInt<N> {
    fn cmp(&self, rps: &Self) -> Ordering {
        for (l, r) in zip(self.limbs, rps.limbs).rev() {
            if l < r {
                return Ordering::Less;
            } else if l > r {
                return Ordering::Greater;
            }
        }
        Ordering::Equal
    }
}

impl<const N: usize> PartialOrd for BigInt<N> {
    #[inline]
    fn partial_cmp(&self, rps: &Self) -> Option<Ordering> {
        Some(self.cmp(rps))
    }
}

impl<const N: usize> BitAnd for BigInt<N> {
    type Output = Self;

    fn bitand(self, rps: Self) -> Self::Output {
        Self {
            limbs: array::from_fn(|i| self.limbs[i] & rps.limbs[i]),
        }
    }
}

impl<const N: usize> BitAndAssign for BigInt<N> {
    #[inline]
    fn bitand_assign(&mut self, rps: Self) {
        *self = *self & rps
    }
}

impl<const N: usize> Add for BigInt<N> {
    type Output = Self;

    fn add(self, rps: Self) -> Self::Output {
        let mut c: u128 = 0;
        Self {
            limbs: array::from_fn(|i| {
                c += self.limbs[i] as u128 + rps.limbs[i] as u128;
                let n = c as u64;
                c >>= u64::BITS;
                n
            }),
        }
    }
}

impl<const N: usize> AddAssign for BigInt<N> {
    #[inline]
    fn add_assign(&mut self, rps: Self) {
        *self = *self + rps
    }
}

impl<const N: usize> Shl<u32> for BigInt<N> {
    type Output = Self;

    fn shl(self, rps: u32) -> Self::Output {
        let mut c = 0;
        Self {
            limbs: array::from_fn(|i| {
                let n = (self.limbs[i] << rps) | c;
                c = self.limbs[i] >> (u64::BITS - rps);
                n
            }),
        }
    }
}

impl<const N: usize> ShlAssign<u32> for BigInt<N> {
    #[inline]
    fn shl_assign(&mut self, rps: u32) {
        *self = *self << rps
    }
}

impl<const N: usize> Shr<u32> for BigInt<N> {
    type Output = Self;

    fn shr(self, rps: u32) -> Self::Output {
        let mut n = Self::default();
        let mut c = 0;
        for (n, l) in zip(n.limbs.iter_mut(), self.limbs).rev() {
            *n = (l >> rps) | (c << (u64::BITS - rps));
            c = l & ((1 << rps) - 1);
        }
        n
    }
}

impl<const N: usize> ShrAssign<u32> for BigInt<N> {
    #[inline]
    fn shr_assign(&mut self, rps: u32) {
        *self = *self >> rps
    }
}

impl<const N: usize> Sub for BigInt<N> {
    type Output = Self;

    fn sub(self, rps: Self) -> Self::Output {
        let mut c: i128 = 0;
        Self {
            limbs: array::from_fn(|i| {
                c += self.limbs[i] as i128 - rps.limbs[i] as i128;
                let n = c as u64;
                c >>= u64::BITS;
                n
            }),
        }
    }
}

impl<const N: usize> SubAssign for BigInt<N> {
    #[inline]
    fn sub_assign(&mut self, rps: Self) {
        *self = *self - rps
    }
}
