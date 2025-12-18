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

use crate::semigroup::{
    AdditiveCommutativeSemigroup, AdditiveSemigroup, MultiplicativeCommutativeSemigroup,
    MultiplicativeSemigroup,
};

#[rustfmt::skip]
pub trait AdditiveMonoid
    : Default
    + AdditiveSemigroup
{
    const IDENTITY: Self;
}

/// A marker for monoids with commutative addition.
#[rustfmt::skip]
pub trait AdditiveCommutativeMonoid
    : AdditiveMonoid
    + AdditiveCommutativeSemigroup
{
}

impl<G: AdditiveMonoid + AdditiveCommutativeSemigroup> AdditiveCommutativeMonoid for G {}

#[rustfmt::skip]
pub trait MultiplicativeMonoid
    : Default
    + MultiplicativeSemigroup
{
    const IDENTITY: Self;
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
