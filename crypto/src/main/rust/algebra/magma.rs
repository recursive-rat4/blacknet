/*
 * Copyright (c) 2024-2026 Pavel Vasin
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

use crate::algebra::{Double, Set, Square};
use core::ops::{Add, AddAssign, Mul, MulAssign};

/// One-sided identity.
pub trait LeftZero {
    /// The left additive identity.
    const LEFT_ZERO: Self;
}

/// One-sided identity.
pub trait RightZero {
    /// The right additive identity.
    const RIGHT_ZERO: Self;
}

/// Two-sided identity.
pub trait Zero: LeftZero + RightZero {
    /// The additive identity.
    const ZERO: Self;
}

#[rustfmt::skip]
pub trait AdditiveMagmaOps<M>
    : Add<M, Output = M>
    + for<'a> Add<&'a M, Output = M>
    + Double<Output = M>
{
}

#[rustfmt::skip]
impl<M, T
    : Add<M, Output = M>
    + for<'a> Add<&'a M, Output = M>
    + Double<Output = M>
> AdditiveMagmaOps<M> for T {}

/// A set that is closed under addition.
#[rustfmt::skip]
pub trait AdditiveMagma
    : Set
    + Sized
    + AdditiveMagmaOps<Self>
    + AddAssign
    + for<'a> AddAssign<&'a Self>
{
}

#[rustfmt::skip]
impl<M
    : Set
    + Sized
    + AdditiveMagmaOps<M>
    + AddAssign
    + for<'a> AddAssign<&'a Self>
> AdditiveMagma for M
{}

/// A marker for magmata with commutative addition.
pub trait AdditiveCommutativeMagma: AdditiveMagma {}

/// One-sided identity.
pub trait LeftOne {
    /// The left multiplicative identity.
    const LEFT_ONE: Self;
}

/// One-sided identity.
pub trait RightOne {
    /// The right multiplicative identity.
    const RIGHT_ONE: Self;
}

/// Two-sided identity.
pub trait One: LeftOne + RightOne {
    /// The multiplicative identity.
    const ONE: Self;
}

#[rustfmt::skip]
pub trait MultiplicativeMagmaOps<M>
    : Mul<M, Output = M>
    + for<'a> Mul<&'a M, Output = M>
    + Square<Output = M>
{
}

#[rustfmt::skip]
impl<M, T
    : Mul<M, Output = M>
    + for<'a> Mul<&'a M, Output = M>
    + Square<Output = M>
> MultiplicativeMagmaOps<M> for T {}

/// A set that is closed under multiplication.
#[rustfmt::skip]
pub trait MultiplicativeMagma
    : Set
    + Sized
    + MultiplicativeMagmaOps<Self>
    + MulAssign
    + for<'a> MulAssign<&'a Self>
{
}

#[rustfmt::skip]
impl<M
    : Set
    + Sized
    + MultiplicativeMagmaOps<Self>
    + MulAssign
    + for<'a> MulAssign<&'a Self>
> MultiplicativeMagma for M
{}

/// A marker for magmata with commutative multiplication.
pub trait MultiplicativeCommutativeMagma: MultiplicativeMagma {}
