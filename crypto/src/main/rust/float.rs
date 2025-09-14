/*
 * Copyright (c) 2025 Pavel Vasin
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

use crate::integer::Integer;
use core::ops::{Add, Div, Mul, Sub};

#[rustfmt::skip]
pub trait Float
    : Copy
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
{
    const MANTISSA_DIGITS: u32;

    const ONE: Self;
}

impl Float for f32 {
    const MANTISSA_DIGITS: u32 = Self::MANTISSA_DIGITS;

    const ONE: Self = 1.0;
}

impl Float for f64 {
    const MANTISSA_DIGITS: u32 = Self::MANTISSA_DIGITS;

    const ONE: Self = 1.0;
}

pub trait FloatOn<F: Float>: Integer {
    fn float_on(self) -> F;
}

impl FloatOn<f32> for i8 {
    #[inline]
    fn float_on(self) -> f32 {
        self as f32
    }
}

impl FloatOn<f64> for i8 {
    #[inline]
    fn float_on(self) -> f64 {
        self as f64
    }
}

impl FloatOn<f32> for i16 {
    #[inline]
    fn float_on(self) -> f32 {
        self as f32
    }
}

impl FloatOn<f64> for i16 {
    #[inline]
    fn float_on(self) -> f64 {
        self as f64
    }
}

impl FloatOn<f32> for i32 {
    #[inline]
    fn float_on(self) -> f32 {
        self as f32
    }
}

impl FloatOn<f64> for i32 {
    #[inline]
    fn float_on(self) -> f64 {
        self as f64
    }
}

impl FloatOn<f32> for i64 {
    #[inline]
    fn float_on(self) -> f32 {
        self as f32
    }
}

impl FloatOn<f64> for i64 {
    #[inline]
    fn float_on(self) -> f64 {
        self as f64
    }
}

impl FloatOn<f32> for u8 {
    #[inline]
    fn float_on(self) -> f32 {
        self as f32
    }
}

impl FloatOn<f64> for u8 {
    #[inline]
    fn float_on(self) -> f64 {
        self as f64
    }
}

impl FloatOn<f32> for u16 {
    #[inline]
    fn float_on(self) -> f32 {
        self as f32
    }
}

impl FloatOn<f64> for u16 {
    #[inline]
    fn float_on(self) -> f64 {
        self as f64
    }
}

impl FloatOn<f32> for u32 {
    #[inline]
    fn float_on(self) -> f32 {
        self as f32
    }
}

impl FloatOn<f64> for u32 {
    #[inline]
    fn float_on(self) -> f64 {
        self as f64
    }
}

impl FloatOn<f32> for u64 {
    #[inline]
    fn float_on(self) -> f32 {
        self as f32
    }
}

impl FloatOn<f64> for u64 {
    #[inline]
    fn float_on(self) -> f64 {
        self as f64
    }
}
