/*
 * Copyright (c) 2025-2026 Pavel Vasin
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
    AdditiveCommutativeMonoid, AdditiveMagmaOps, MultiplicativeCommutativeMonoid,
    MultiplicativeMagmaOps, MultiplicativeMonoid, MultiplicativeSemigroup,
};

#[rustfmt::skip]
pub trait SemiringOps<R>
    : AdditiveMagmaOps<R>
    + MultiplicativeMagmaOps<R>
{
}

#[rustfmt::skip]
impl<R, T
    : AdditiveMagmaOps<R>
    + MultiplicativeMagmaOps<R>
> SemiringOps<R> for T {}

/// A generalization of [nonunital ring][`crate::algebra::Ring`]
/// that doesn't require subtraction.
#[rustfmt::skip]
pub trait Presemiring
    : AdditiveCommutativeMonoid
    + MultiplicativeSemigroup
    + SemiringOps<Self>
{
}

#[rustfmt::skip]
impl<R
    : AdditiveCommutativeMonoid
    + MultiplicativeSemigroup
    + SemiringOps<Self>
> Presemiring for R
{
}

/// A generalization of [unital ring][`crate::algebra::UnitalRing`]
/// that doesn't require subtraction.
#[rustfmt::skip]
pub trait Semiring
    : Presemiring
    + MultiplicativeMonoid
    + Copy
{
}

#[rustfmt::skip]
impl<R
    : Presemiring
    + MultiplicativeMonoid
    + Copy
> Semiring for R {}

/// A marker for semirings with commutative multiplication.
/// Semiring elements commute under addition by definition.
#[rustfmt::skip]
pub trait CommutativeSemiring
    : Semiring
    + MultiplicativeCommutativeMonoid
{
}

impl<R: Semiring + MultiplicativeCommutativeMonoid> CommutativeSemiring for R {}
