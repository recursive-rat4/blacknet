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

use crate::algebra::Inv;
use crate::algebra::{AdditiveMonoid, MultiplicativeMonoid};
use core::ops::{Div, DivAssign, Neg, Sub, SubAssign};

#[rustfmt::skip]
pub trait AdditiveGroup
    : AdditiveMonoid
    + Neg<Output = Self>
    + Sub<Output = Self>
    + SubAssign
    + for<'a> Sub<&'a Self, Output = Self>
    + for<'a> SubAssign<&'a Self>
{
}

#[rustfmt::skip]
impl<T
    : AdditiveMonoid
    + Neg<Output = Self>
    + Sub<Output = Self>
    + SubAssign
    + for<'a> Sub<&'a Self, Output = Self>
    + for<'a> SubAssign<&'a Self>
> AdditiveGroup for T {}

#[rustfmt::skip]
pub trait MultiplicativeGroup
    : MultiplicativeMonoid
    + Inv<Output = Self>
    + Div<Output = Self>
    + DivAssign
    + for<'a> Div<&'a Self, Output = Self>
    + for<'a> DivAssign<&'a Self>
{
}

#[rustfmt::skip]
impl<T
    : MultiplicativeMonoid
    + Inv<Output = Self>
    + Div<Output = Self>
    + DivAssign
    + for<'a> Div<&'a Self, Output = Self>
    + for<'a> DivAssign<&'a Self>
> MultiplicativeGroup for T {}
