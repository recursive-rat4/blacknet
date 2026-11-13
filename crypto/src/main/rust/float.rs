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

use core::ops::{Add, Div, Mul, Sub};

#[rustfmt::skip]
pub trait Float
    : Copy
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
{
    type Bits;

    fn from_bits(v: Self::Bits) -> Self;
    fn recip(self) -> Self;
    fn to_bits(self) -> Self::Bits;

    const MANTISSA_DIGITS: u32;
}

impl Float for f32 {
    type Bits = u32;

    #[inline]
    fn from_bits(v: Self::Bits) -> Self {
        Self::from_bits(v)
    }
    #[inline]
    fn recip(self) -> Self {
        self.recip()
    }
    #[inline]
    fn to_bits(self) -> Self::Bits {
        self.to_bits()
    }

    const MANTISSA_DIGITS: u32 = Self::MANTISSA_DIGITS;
}

impl Float for f64 {
    type Bits = u64;

    #[inline]
    fn from_bits(v: Self::Bits) -> Self {
        Self::from_bits(v)
    }
    #[inline]
    fn recip(self) -> Self {
        self.recip()
    }
    #[inline]
    fn to_bits(self) -> Self::Bits {
        self.to_bits()
    }

    const MANTISSA_DIGITS: u32 = Self::MANTISSA_DIGITS;
}

pub trait CastFloat<T> {
    fn cast_float(self) -> T;
}

macro_rules! impl_cast_float {
    ( $x:ty, $($y:ty),+ ) => {
        $(
            impl CastFloat<$x> for $y {
                #[inline(always)]
                fn cast_float(self) -> $x {
                    self as $x
                }
            }
            impl CastFloat<$y> for $x {
                #[inline(always)]
                fn cast_float(self) -> $y {
                    self as $y
                }
            }
        )+
    };
}

impl_cast_float!(f32, i8, i16, i32, i64, u8, u16, u32, u64);
impl_cast_float!(f64, i8, i16, i32, i64, u8, u16, u32, u64);
