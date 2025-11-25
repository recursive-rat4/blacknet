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

use crate::monoid::{
    AdditiveCommutativeMonoid, AdditiveMonoid, MultiplicativeCommutativeMonoid,
    MultiplicativeMonoid,
};
use crate::semigroup::MultiplicativeSemigroup;

#[rustfmt::skip]
pub trait Presemiring
    : AdditiveCommutativeMonoid
    + MultiplicativeSemigroup
{
    const ZERO: Self = <Self as AdditiveMonoid>::IDENTITY;
}

impl<R: AdditiveCommutativeMonoid + MultiplicativeSemigroup> Presemiring for R {}

#[rustfmt::skip]
pub trait Semiring
    : Presemiring
    + MultiplicativeMonoid
{
    const ONE: Self = <Self as MultiplicativeMonoid>::IDENTITY;
}

impl<R: Presemiring + MultiplicativeMonoid> Semiring for R {}

#[rustfmt::skip]
pub trait CommutativeSemiring
    : Semiring
    + MultiplicativeCommutativeMonoid
{
}

impl<R: Semiring + MultiplicativeCommutativeMonoid> CommutativeSemiring for R {}
