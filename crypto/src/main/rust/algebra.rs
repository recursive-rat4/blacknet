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
use crate::ring::{CommutativeRing, DivisionRing, Ring, UnitalRing};

/// Associative algebra over a ring.
#[rustfmt::skip]
pub trait Algebra<R: Ring>
    : Module<R>
    + Ring
    + From<R>
{
}

/// Any ring is an algebra over itself.
impl<R: Ring> Algebra<R> for R {}

/// An algebra with multiplicative identity.
#[rustfmt::skip]
pub trait UnitalAlgebra<R: UnitalRing>
    : Algebra<R>
    + UnitalRing
{
}

/// Any unital ring is a unital algebra over itself.
impl<R: UnitalRing> UnitalAlgebra<R> for R {}

/// A marker for algebras with commutative multiplication.
#[rustfmt::skip]
pub trait CommutativeAlgebra<R: CommutativeRing>
    : UnitalAlgebra<R>
    + CommutativeRing
{
}

#[rustfmt::skip]
impl<
    R: CommutativeRing,
    A: UnitalAlgebra<R> + CommutativeRing
> CommutativeAlgebra<R> for A {}

#[rustfmt::skip]
pub trait DivisionAlgebra<R: DivisionRing>
    : Algebra<R>
    + DivisionRing
{
}
