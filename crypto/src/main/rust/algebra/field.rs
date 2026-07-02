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

use crate::algebra::{CommutativeRing, DivisionRing, DivisionRingOps, IntegerModRing, UnitalRing};
use crate::branchless::BlOption;
use core::ops::Div;

#[rustfmt::skip]
pub trait FieldOps<F>
    : DivisionRingOps<F>
    + Div<F, Output = BlOption<F>>
    + for<'a> Div<&'a F, Output = BlOption<F>>
{
}

#[rustfmt::skip]
impl<F, T
    : DivisionRingOps<F>
    + Div<F, Output = BlOption<F>>
    + for<'a> Div<&'a F, Output = BlOption<F>>
> FieldOps<F> for T {}

/// A unital commutative division ring.
#[rustfmt::skip]
pub trait Field
    : UnitalRing
    + CommutativeRing
    + DivisionRing
    + Div<Output = BlOption<Self>>
    + for<'a> Div<&'a Self, Output = BlOption<Self>>
{
}

#[rustfmt::skip]
impl<R
    : UnitalRing
    + CommutativeRing
    + DivisionRing
    + Div<Output = BlOption<Self>>
    + for<'a> Div<&'a Self, Output = BlOption<Self>>
> Field for R
{
}

/// A ring `ℤ/q` where `q` is a prime number.
#[rustfmt::skip]
pub trait PrimeField
    : Field
    + IntegerModRing
{
}

impl<F: Field + IntegerModRing> PrimeField for F {}
