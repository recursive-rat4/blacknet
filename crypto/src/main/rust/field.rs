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

use crate::algebra::{CommutativeAlgebra, DivisionAlgebra};
use crate::ring::{CommutativeRing, DivisionRing, IntegerRing, PolynomialRing};
use core::ops::Div;

#[rustfmt::skip]
pub trait Field
    : CommutativeRing
    + DivisionRing
    + Div<Output = Option<Self>>
{
    const ONE: Self = Self::UNITY;
}

impl<F: Field> DivisionRing for F {}

#[rustfmt::skip]
pub trait PrimeField
    : Field
    + IntegerRing
{
}

impl<F: PrimeField> Field for F {}

#[rustfmt::skip]
pub trait AlgebraicExtension<F: Field, const N: usize>
    : Field
    + PolynomialRing<F, N>
    + CommutativeAlgebra<F, N>
    + DivisionAlgebra<F, N>
    + Div<F, Output = Option<Self>>
{
}
