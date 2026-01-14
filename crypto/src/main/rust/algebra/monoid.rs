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
    AdditiveCommutativeSemigroup, AdditiveSemigroup, MultiplicativeCommutativeSemigroup,
    MultiplicativeSemigroup, One, Zero,
};
use core::iter::{Product, Sum};

/// A semigroup with a two-sided additive identity.
#[rustfmt::skip]
pub trait AdditiveMonoid
    : AdditiveSemigroup
    + Zero
    + Sum
    + for<'a> Sum<&'a Self>
{
}

/// A marker for monoids with commutative addition.
#[rustfmt::skip]
pub trait AdditiveCommutativeMonoid
    : AdditiveMonoid
    + AdditiveCommutativeSemigroup
{
}

impl<G: AdditiveMonoid + AdditiveCommutativeSemigroup> AdditiveCommutativeMonoid for G {}

/// A semigroup with a two-sided multiplicative identity.
#[rustfmt::skip]
pub trait MultiplicativeMonoid
    : MultiplicativeSemigroup
    + One
    + Product
    + for<'a> Product<&'a Self>
{
}

/// A marker for monoids with commutative multiplication.
#[rustfmt::skip]
pub trait MultiplicativeCommutativeMonoid
    : MultiplicativeMonoid
    + MultiplicativeCommutativeSemigroup
{
}

impl<G: MultiplicativeMonoid + MultiplicativeCommutativeSemigroup> MultiplicativeCommutativeMonoid
    for G
{
}
