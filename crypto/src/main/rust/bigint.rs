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
    pub const fn from_hex(mut hex: &str) -> Self {
        let mut limbs = [0; N];
        let mut i = 0;
        loop {
            let chunk: &str;
            (hex, chunk) = hex.split_at(hex.len() - 16);
            match u64::from_str_radix(chunk, 16) {
                Ok(v) => limbs[i] = v,
                Err(_) => panic!("Can't parse a chunk of hex"),
            }
            i += 1;
            if i == N {
                break;
            }
        }
        Self { limbs }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn mul<const M: usize, const NM: usize>(self, rps: BigInt<M>) -> BigInt<NM> {
        let mut c: u128 = 0;
        let mut n = BigInt::<NM>::default();
        for i in 0..N {
            for j in 0..M {
                c += self.limbs[i] as u128 * rps.limbs[j] as u128 + n.limbs[i + j] as u128;
                n.limbs[i + j] = c as u64;
                c >>= u64::BITS;
            }
            n.limbs[i + M] = c as u64;
            c = 0;
        }
        n
    }

    #[inline]
    pub fn square<const NN: usize>(self) -> BigInt<NN> {
        const SQUARE_THRESHOLD: usize = 4;

        if N <= SQUARE_THRESHOLD {
            self.mul(self)
        } else {
            self.square_impl()
        }
    }

    fn square_impl<const NN: usize>(self) -> BigInt<NN> {
        let mut c: u64 = 0;
        let mut n = BigInt::<NN>::default();
        let mut j = N * 2;
        for i in (0..N).rev() {
            let p = self.limbs[i] as u128 * self.limbs[i] as u128;
            j -= 1;
            n.limbs[j] = ((c as u128) << (u64::BITS - 1) | p >> (u64::BITS + 1)) as u64;
            j -= 1;
            n.limbs[j] = (p >> 1) as u64;
            c = p as u64;
        }

        j = 2;
        let mut b: u128 = 0;
        for i in 1..N {
            let mut d: u128 = 0;
            for k in 0..i {
                d += self.limbs[i] as u128 * self.limbs[k] as u128 + n.limbs[i + k] as u128;
                n.limbs[i + k] = d as u64;
                d >>= u64::BITS;
            }
            b += d;
            b += n.limbs[j] as u128;
            n.limbs[j] = b as u64;
            j += 1;
            b >>= u64::BITS;
            b += n.limbs[j] as u128;
            n.limbs[j] = b as u64;
            j += 1;
            b >>= u64::BITS;
        }

        c = self.limbs[0] << (u64::BITS - 1);
        for i in 0..N * 2 {
            let d = n.limbs[i];
            n.limbs[i] = d << 1 | c >> (u64::BITS - 1);
            c = d;
        }

        n
    }

    pub const fn bits<const M: usize>(self) -> [bool; M] {
        let mut bits = [false; M];
        let mut i = 0;
        let mut j = 0;
        let mut k = 0;
        loop {
            bits[i] = self.limbs[j] >> k & 1 == 1;
            i += 1;
            if i == M {
                break;
            }
            k += 1;
            if k == u64::BITS {
                k = 0;
                j += 1;
            }
        }
        bits
    }

    pub const fn is_even(self) -> bool {
        self.limbs[0] & 1 == 0
    }

    pub const fn limbs(self) -> [u64; N] {
        self.limbs
    }

    pub const fn count_ones(self) -> u32 {
        let mut ones = 0;
        let mut i = 0;
        loop {
            if i != N {
                ones += self.limbs[i].count_ones();
                i += 1;
            } else {
                break;
            }
        }
        ones
    }

    pub const ZERO: Self = Self { limbs: [0; N] };
    pub const ONE: Self = {
        let mut limbs = [0; N];
        limbs[0] = 1;
        Self { limbs }
    };
}

impl<const N: usize> fmt::Debug for BigInt<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for limb in self.limbs.iter().rev() {
            write!(f, "{limb:016X}")?;
        }
        Ok(())
    }
}

impl<const N: usize> Default for BigInt<N> {
    #[inline]
    fn default() -> Self {
        Self::ZERO
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
        for i in (0..N).rev() {
            if self.limbs[i] < rps.limbs[i] {
                return Ordering::Less;
            } else if self.limbs[i] > rps.limbs[i] {
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

impl<const N: usize> BitAnd<u64> for BigInt<N> {
    type Output = u64;

    fn bitand(self, rps: u64) -> Self::Output {
        self.limbs[0] & rps
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

impl<const N: usize> Shl<u64> for BigInt<N> {
    type Output = Self;

    fn shl(self, rps: u64) -> Self::Output {
        let mut c = 0;
        Self {
            limbs: array::from_fn(|i| {
                let n = (self.limbs[i] << rps) | c;
                c = self.limbs[i] >> (u64::BITS as u64 - rps);
                n
            }),
        }
    }
}

impl<const N: usize> ShlAssign<u64> for BigInt<N> {
    #[inline]
    fn shl_assign(&mut self, rps: u64) {
        *self = *self << rps
    }
}

impl<const N: usize> Shr<u64> for BigInt<N> {
    type Output = Self;

    fn shr(self, rps: u64) -> Self::Output {
        let mut n = Self::default();
        let mut c = 0;
        for i in (0..N).rev() {
            n.limbs[i] = (self.limbs[i] >> rps) | (c << (u64::BITS as u64 - rps));
            c = self.limbs[i] & ((1 << rps) - 1);
        }
        n
    }
}

impl<const N: usize> ShrAssign<u64> for BigInt<N> {
    #[inline]
    fn shr_assign(&mut self, rps: u64) {
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
