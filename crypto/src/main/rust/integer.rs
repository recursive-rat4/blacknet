/*
 * Copyright (c) 2025-2026 Pavel Vasin
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

use crate::bigint::UInt256;
use core::borrow::BorrowMut;
use core::ops::{Add, BitAnd, BitOrAssign, Shl, ShrAssign, Sub};

#[rustfmt::skip]
pub trait Integer
    : Copy
    + Default
    + From<Self::Limb>
    + Ord
    + BitAnd<Self, Output = Self>
    + BitAnd<Self::Limb, Output = Self::Limb>
    + BitOrAssign<Self>
    + Shl<u32, Output = Self>
    + ShrAssign<Self::Limb>
    + ShrAssign<u32>
    + Add<Output = Self>
    + Sub<Output = Self>
{
    type Limb
        : Copy
        + Ord
        + Sub<Output = Self::Limb>
        ;
    type Bytes: BorrowMut<[u8]> + Default;
    type CastUnsigned: UnsignedInteger<Bytes = Self::Bytes>;

    fn cast_unsigned(self) -> Self::CastUnsigned;
    fn count_ones(self) -> u32;
    fn from_le_bytes(bytes: Self::Bytes) -> Self;
    fn leading_zeros(self) -> u32;
    fn to_le_bytes(self) -> Self::Bytes;
    fn wrapping_add(self, rps: Self) -> Self;
    fn wrapping_sub(self, rps: Self) -> Self;

    const BITS: u32;
    const BYTES: usize;
    const MAX: Self;
    const MIN: Self;

    const ZERO: Self;
    const ONE: Self;

    const LIMB_ONE: Self::Limb;
    const LIMB_TWO: Self::Limb;
    const LIMB_THREE: Self::Limb;
}

pub trait SignedInteger: Integer {}
pub trait UnsignedInteger: Integer {}

macro_rules! impl_integer {
    ( $($x:ty, $y:ty),+ ) => {
        $(
            impl Integer for $x {
                type Limb = Self;
                type Bytes = [u8; Self::BYTES];
                type CastUnsigned = Self;

                #[inline]
                fn cast_unsigned(self) -> Self::CastUnsigned {
                    self
                }
                #[inline]
                fn count_ones(self) -> u32 {
                    self.count_ones()
                }
                #[inline]
                fn from_le_bytes(bytes: Self::Bytes) -> Self {
                    Self::from_le_bytes(bytes)
                }
                #[inline]
                fn leading_zeros(self) -> u32 {
                    self.leading_zeros()
                }
                #[inline]
                fn to_le_bytes(self) -> Self::Bytes {
                    self.to_le_bytes()
                }
                #[inline]
                fn wrapping_add(self, rps: Self) -> Self {
                    self.wrapping_add(rps)
                }
                #[inline]
                fn wrapping_sub(self, rps: Self) -> Self {
                    self.wrapping_sub(rps)
                }

                const BITS: u32 = Self::BITS;
                const BYTES: usize = Self::BITS as usize / 8;
                const MAX: Self = Self::MAX;
                const MIN: Self = Self::MIN;

                const ZERO: Self = 0;
                const ONE: Self = 1;

                const LIMB_ONE: Self::Limb = 1;
                const LIMB_TWO: Self::Limb = 2;
                const LIMB_THREE: Self::Limb = 3;
            }

            impl UnsignedInteger for $x {}

            impl Integer for $y {
                type Limb = Self;
                type Bytes = [u8; Self::BYTES];
                type CastUnsigned = $x;

                #[inline]
                fn cast_unsigned(self) -> Self::CastUnsigned {
                    self.cast_unsigned()
                }
                #[inline]
                fn count_ones(self) -> u32 {
                    self.count_ones()
                }
                #[inline]
                fn from_le_bytes(bytes: Self::Bytes) -> Self {
                    Self::from_le_bytes(bytes)
                }
                #[inline]
                fn leading_zeros(self) -> u32 {
                    self.leading_zeros()
                }
                #[inline]
                fn to_le_bytes(self) -> Self::Bytes {
                    self.to_le_bytes()
                }
                #[inline]
                fn wrapping_add(self, rps: Self) -> Self {
                    self.wrapping_add(rps)
                }
                #[inline]
                fn wrapping_sub(self, rps: Self) -> Self {
                    self.wrapping_sub(rps)
                }

                const BITS: u32 = Self::BITS;
                const BYTES: usize = Self::BITS as usize / 8;
                const MAX: Self = Self::MAX;
                const MIN: Self = Self::MIN;

                const ZERO: Self = 0;
                const ONE: Self = 1;

                const LIMB_ONE: Self::Limb = 1;
                const LIMB_TWO: Self::Limb = 2;
                const LIMB_THREE: Self::Limb = 3;
            }

            impl SignedInteger for $y {}
        )+
    };
}

impl_integer!(u8, i8, u16, i16, u32, i32, u64, i64, usize, isize);

impl Integer for UInt256 {
    type Limb = u64;
    type Bytes = [u8; Self::BYTES];
    type CastUnsigned = Self;

    #[inline]
    fn cast_unsigned(self) -> Self::CastUnsigned {
        self
    }
    #[inline]
    fn count_ones(self) -> u32 {
        self.count_ones()
    }
    #[inline]
    fn from_le_bytes(bytes: Self::Bytes) -> Self {
        Self::from_le_bytes(bytes)
    }
    #[inline]
    fn leading_zeros(self) -> u32 {
        self.leading_zeros()
    }
    #[inline]
    fn to_le_bytes(self) -> Self::Bytes {
        self.to_le_bytes()
    }
    #[inline]
    fn wrapping_add(self, rps: Self) -> Self {
        self.add(rps)
    }
    #[inline]
    fn wrapping_sub(self, rps: Self) -> Self {
        self.sub(rps)
    }

    const BITS: u32 = Self::BITS;
    const BYTES: usize = Self::BITS as usize / 8;
    const MAX: Self = Self::MAX;
    const MIN: Self = Self::MIN;

    const ZERO: Self = Self::ZERO;
    const ONE: Self = Self::ONE;

    const LIMB_ONE: Self::Limb = 1;
    const LIMB_TWO: Self::Limb = 2;
    const LIMB_THREE: Self::Limb = 3;
}

impl UnsignedInteger for UInt256 {}

macro_rules! impl_bits {
    ( $x:ty, $y:ident ) => {
        pub const fn $y<const N: usize>(n: $x) -> [bool; N] {
            let mut bits = [false; N];
            let mut i = 0;
            while i < N {
                bits[i] = n >> i & 1 == 1;
                i += 1;
            }
            bits
        }
    };
}

impl_bits!(u8, bits_u8);
impl_bits!(u16, bits_u16);
impl_bits!(u32, bits_u32);
impl_bits!(u64, bits_u64);
impl_bits!(u128, bits_u128);
