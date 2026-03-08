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

use crate::algebra::{
    CommutativeRing, DivisionRing, Module, Ring, RingOps, SemimoduleOps, UnitalRing,
};

#[rustfmt::skip]
pub trait AlgebraOps<R, A>
    : SemimoduleOps<R, A>
    + RingOps<A>
{
}

#[rustfmt::skip]
impl<R, A, T
    : SemimoduleOps<R, A>
    + RingOps<A>
> AlgebraOps<R, A> for T {}

/// Associative algebra over a ring.
#[rustfmt::skip]
pub trait Algebra<R: Ring>
    : Module<R>
    + Ring
    + From<R>
    + AlgebraOps<R, Self>
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
    : Algebra<R>
    + CommutativeRing
{
}

#[rustfmt::skip]
impl<
    R: CommutativeRing,
    A: Algebra<R> + CommutativeRing
> CommutativeAlgebra<R> for A {}

#[rustfmt::skip]
pub trait DivisionAlgebra<R: DivisionRing>
    : Algebra<R>
    + DivisionRing
{
}
