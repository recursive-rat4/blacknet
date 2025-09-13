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

use crate::bigint::BigInt;
use core::ops::{BitAnd, ShrAssign, Sub};

#[rustfmt::skip]
pub trait Integer
    : Copy
    + Default
    + From<Self::Limb>
    + Ord
    + FloatOn
    + BitAnd<Self::Limb, Output = Self::Limb>
    + ShrAssign<Self::Limb>
{
    type Limb
        : Copy
        + Ord
        + Sub<Output = Self::Limb>
        ;

    fn count_ones(self) -> u32;
    fn leading_zeros(self) -> u32;

    const BITS: u32;
    const LIMB_ONE: Self::Limb;
    const LIMB_TWO: Self::Limb;
    const LIMB_THREE: Self::Limb;
}

pub trait FloatOn {
    fn float_on(self) -> f64;
}

impl FloatOn for i8 {
    fn float_on(self) -> f64 {
        self as f64
    }
}

impl FloatOn for i16 {
    fn float_on(self) -> f64 {
        self as f64
    }
}

impl FloatOn for i32 {
    fn float_on(self) -> f64 {
        self as f64
    }
}

impl FloatOn for i64 {
    fn float_on(self) -> f64 {
        self as f64
    }
}

impl<const N: usize> FloatOn for BigInt<N> {
    fn float_on(self) -> f64 {
        unimplemented!("BigInt::float_on");
    }
}

impl Integer for i8 {
    type Limb = Self;

    #[inline]
    fn count_ones(self) -> u32 {
        self.count_ones()
    }
    #[inline]
    fn leading_zeros(self) -> u32 {
        self.leading_zeros()
    }

    const BITS: u32 = Self::BITS;
    const LIMB_ONE: Self::Limb = 1;
    const LIMB_TWO: Self::Limb = 2;
    const LIMB_THREE: Self::Limb = 3;
}

impl Integer for i16 {
    type Limb = Self;

    #[inline]
    fn count_ones(self) -> u32 {
        self.count_ones()
    }
    #[inline]
    fn leading_zeros(self) -> u32 {
        self.leading_zeros()
    }

    const BITS: u32 = Self::BITS;
    const LIMB_ONE: Self::Limb = 1;
    const LIMB_TWO: Self::Limb = 2;
    const LIMB_THREE: Self::Limb = 3;
}

impl Integer for i32 {
    type Limb = Self;

    #[inline]
    fn count_ones(self) -> u32 {
        self.count_ones()
    }
    #[inline]
    fn leading_zeros(self) -> u32 {
        self.leading_zeros()
    }

    const BITS: u32 = Self::BITS;
    const LIMB_ONE: Self::Limb = 1;
    const LIMB_TWO: Self::Limb = 2;
    const LIMB_THREE: Self::Limb = 3;
}

impl Integer for i64 {
    type Limb = Self;

    #[inline]
    fn count_ones(self) -> u32 {
        self.count_ones()
    }
    #[inline]
    fn leading_zeros(self) -> u32 {
        self.leading_zeros()
    }

    const BITS: u32 = Self::BITS;
    const LIMB_ONE: Self::Limb = 1;
    const LIMB_TWO: Self::Limb = 2;
    const LIMB_THREE: Self::Limb = 3;
}

impl<const N: usize> Integer for BigInt<N> {
    type Limb = u64;

    #[inline]
    fn count_ones(self) -> u32 {
        self.count_ones()
    }
    #[inline]
    fn leading_zeros(self) -> u32 {
        self.leading_zeros()
    }

    const BITS: u32 = Self::BITS;
    const LIMB_ONE: Self::Limb = 1;
    const LIMB_TWO: Self::Limb = 2;
    const LIMB_THREE: Self::Limb = 3;
}
