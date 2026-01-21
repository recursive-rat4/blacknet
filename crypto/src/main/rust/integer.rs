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

use crate::bigint::BigInt;
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
    type CastUnsigned: UnsignedInteger;

    fn cast_unsigned(self) -> Self::CastUnsigned;
    fn count_ones(self) -> u32;
    fn leading_zeros(self) -> u32;

    const BITS: u32;
    const MAX: Self;

    const ZERO: Self;
    const ONE: Self;

    const LIMB_ONE: Self::Limb;
    const LIMB_TWO: Self::Limb;
    const LIMB_THREE: Self::Limb;
}

pub trait SignedInteger: Integer {}
pub trait UnsignedInteger: Integer {}

impl Integer for i8 {
    type Limb = Self;
    type CastUnsigned = u8;

    #[inline]
    fn cast_unsigned(self) -> Self::CastUnsigned {
        self.cast_unsigned()
    }
    #[inline]
    fn count_ones(self) -> u32 {
        self.count_ones()
    }
    #[inline]
    fn leading_zeros(self) -> u32 {
        self.leading_zeros()
    }

    const BITS: u32 = Self::BITS;
    const MAX: Self = Self::MAX;

    const ZERO: Self = 0;
    const ONE: Self = 1;

    const LIMB_ONE: Self::Limb = 1;
    const LIMB_TWO: Self::Limb = 2;
    const LIMB_THREE: Self::Limb = 3;
}

impl SignedInteger for i8 {}

impl Integer for i16 {
    type Limb = Self;
    type CastUnsigned = u16;

    #[inline]
    fn cast_unsigned(self) -> Self::CastUnsigned {
        self.cast_unsigned()
    }
    #[inline]
    fn count_ones(self) -> u32 {
        self.count_ones()
    }
    #[inline]
    fn leading_zeros(self) -> u32 {
        self.leading_zeros()
    }

    const BITS: u32 = Self::BITS;
    const MAX: Self = Self::MAX;

    const ZERO: Self = 0;
    const ONE: Self = 1;

    const LIMB_ONE: Self::Limb = 1;
    const LIMB_TWO: Self::Limb = 2;
    const LIMB_THREE: Self::Limb = 3;
}

impl SignedInteger for i16 {}

impl Integer for i32 {
    type Limb = Self;
    type CastUnsigned = u32;

    #[inline]
    fn cast_unsigned(self) -> Self::CastUnsigned {
        self.cast_unsigned()
    }
    #[inline]
    fn count_ones(self) -> u32 {
        self.count_ones()
    }
    #[inline]
    fn leading_zeros(self) -> u32 {
        self.leading_zeros()
    }

    const BITS: u32 = Self::BITS;
    const MAX: Self = Self::MAX;

    const ZERO: Self = 0;
    const ONE: Self = 1;

    const LIMB_ONE: Self::Limb = 1;
    const LIMB_TWO: Self::Limb = 2;
    const LIMB_THREE: Self::Limb = 3;
}

impl SignedInteger for i32 {}

impl Integer for i64 {
    type Limb = Self;
    type CastUnsigned = u64;

    #[inline]
    fn cast_unsigned(self) -> Self::CastUnsigned {
        self.cast_unsigned()
    }
    #[inline]
    fn count_ones(self) -> u32 {
        self.count_ones()
    }
    #[inline]
    fn leading_zeros(self) -> u32 {
        self.leading_zeros()
    }

    const BITS: u32 = Self::BITS;
    const MAX: Self = Self::MAX;

    const ZERO: Self = 0;
    const ONE: Self = 1;

    const LIMB_ONE: Self::Limb = 1;
    const LIMB_TWO: Self::Limb = 2;
    const LIMB_THREE: Self::Limb = 3;
}

impl SignedInteger for i64 {}

impl Integer for isize {
    type Limb = Self;
    type CastUnsigned = usize;

    #[inline]
    fn cast_unsigned(self) -> Self::CastUnsigned {
        self.cast_unsigned()
    }
    #[inline]
    fn count_ones(self) -> u32 {
        self.count_ones()
    }
    #[inline]
    fn leading_zeros(self) -> u32 {
        self.leading_zeros()
    }

    const BITS: u32 = Self::BITS;
    const MAX: Self = Self::MAX;

    const ZERO: Self = 0;
    const ONE: Self = 1;

    const LIMB_ONE: Self::Limb = 1;
    const LIMB_TWO: Self::Limb = 2;
    const LIMB_THREE: Self::Limb = 3;
}

impl SignedInteger for isize {}

impl Integer for u8 {
    type Limb = Self;
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
    fn leading_zeros(self) -> u32 {
        self.leading_zeros()
    }

    const BITS: u32 = Self::BITS;
    const MAX: Self = Self::MAX;

    const ZERO: Self = 0;
    const ONE: Self = 1;

    const LIMB_ONE: Self::Limb = 1;
    const LIMB_TWO: Self::Limb = 2;
    const LIMB_THREE: Self::Limb = 3;
}

impl UnsignedInteger for u8 {}

impl Integer for u16 {
    type Limb = Self;
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
    fn leading_zeros(self) -> u32 {
        self.leading_zeros()
    }

    const BITS: u32 = Self::BITS;
    const MAX: Self = Self::MAX;

    const ZERO: Self = 0;
    const ONE: Self = 1;

    const LIMB_ONE: Self::Limb = 1;
    const LIMB_TWO: Self::Limb = 2;
    const LIMB_THREE: Self::Limb = 3;
}

impl UnsignedInteger for u16 {}

impl Integer for u32 {
    type Limb = Self;
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
    fn leading_zeros(self) -> u32 {
        self.leading_zeros()
    }

    const BITS: u32 = Self::BITS;
    const MAX: Self = Self::MAX;

    const ZERO: Self = 0;
    const ONE: Self = 1;

    const LIMB_ONE: Self::Limb = 1;
    const LIMB_TWO: Self::Limb = 2;
    const LIMB_THREE: Self::Limb = 3;
}

impl UnsignedInteger for u32 {}

impl Integer for u64 {
    type Limb = Self;
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
    fn leading_zeros(self) -> u32 {
        self.leading_zeros()
    }

    const BITS: u32 = Self::BITS;
    const MAX: Self = Self::MAX;

    const ZERO: Self = 0;
    const ONE: Self = 1;

    const LIMB_ONE: Self::Limb = 1;
    const LIMB_TWO: Self::Limb = 2;
    const LIMB_THREE: Self::Limb = 3;
}

impl UnsignedInteger for u64 {}

impl Integer for usize {
    type Limb = Self;
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
    fn leading_zeros(self) -> u32 {
        self.leading_zeros()
    }

    const BITS: u32 = Self::BITS;
    const MAX: Self = Self::MAX;

    const ZERO: Self = 0;
    const ONE: Self = 1;

    const LIMB_ONE: Self::Limb = 1;
    const LIMB_TWO: Self::Limb = 2;
    const LIMB_THREE: Self::Limb = 3;
}

impl UnsignedInteger for usize {}

impl<const N: usize> Integer for BigInt<N> {
    type Limb = u64;
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
    fn leading_zeros(self) -> u32 {
        self.leading_zeros()
    }

    const BITS: u32 = Self::BITS;
    const MAX: Self = Self::MAX;

    const ZERO: Self = Self::ZERO;
    const ONE: Self = Self::ONE;

    const LIMB_ONE: Self::Limb = 1;
    const LIMB_TWO: Self::Limb = 2;
    const LIMB_THREE: Self::Limb = 3;
}

impl<const N: usize> UnsignedInteger for BigInt<N> {}

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
