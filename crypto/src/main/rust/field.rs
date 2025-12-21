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

use crate::ring::{CommutativeRing, DivisionRing, IntegerRing};
use core::ops::Div;

#[rustfmt::skip]
pub trait Field
    : CommutativeRing
    + DivisionRing
    + Div<Output = Option<Self>>
    + for<'a> Div<&'a Self, Output = Option<Self>>
{
}

#[rustfmt::skip]
impl<R
    : CommutativeRing
    + DivisionRing
    + Div<Output = Option<Self>>
    + for<'a> Div<&'a Self, Output = Option<Self>>
> Field for R
{
}

#[rustfmt::skip]
pub trait PrimeField
    : Field
    + IntegerRing
{
}

impl<F: Field + IntegerRing> PrimeField for F {}
