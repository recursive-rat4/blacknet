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

use crate::module::Module;
use crate::ring::{CommutativeRing, DivisionRing, Ring};

#[rustfmt::skip]
pub trait Algebra<R: Ring, const N: usize>
    : Module<R, N>
    + Ring<BaseRing = R>
    + From<R>
{
}

#[rustfmt::skip]
pub trait CommutativeAlgebra<R: CommutativeRing, const N: usize>
    : Algebra<R, N>
    + CommutativeRing
{
}

#[rustfmt::skip]
pub trait DivisionAlgebra<R: DivisionRing, const N: usize>
    : Algebra<R, N>
    + DivisionRing
{
}
