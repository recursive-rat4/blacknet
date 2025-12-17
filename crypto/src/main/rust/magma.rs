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

use crate::operation::{Double, Square};
use core::ops::{Add, AddAssign, Mul, MulAssign};

/// A set that is closed under addition.
#[rustfmt::skip]
pub trait AdditiveMagma
    : Copy
    + Eq
    + Add<Output = Self>
    + AddAssign
    + Double<Output = Self>
    + for<'a> Add<&'a Self, Output = Self>
    + for<'a> AddAssign<&'a Self>
{
}

/// A marker for magmata with commutative addition.
pub trait AdditiveCommutativeMagma: AdditiveMagma {}

/// A set that is closed under multiplication.
#[rustfmt::skip]
pub trait MultiplicativeMagma
    : Copy
    + Eq
    + Mul<Output = Self>
    + MulAssign
    + Square<Output = Self>
    + for<'a> Mul<&'a Self, Output = Self>
    + for<'a> MulAssign<&'a Self>
{
}

/// A marker for magmata with commutative multiplication.
pub trait MultiplicativeCommutativeMagma: MultiplicativeMagma {}
