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

use crate::abeliangroup::AdditiveAbelianGroup;
use crate::magma::Inv;
use crate::monoid::{AdditiveMonoid, MultiplicativeMonoid};

#[rustfmt::skip]
pub trait Ring
    : AdditiveAbelianGroup
    + MultiplicativeMonoid
{
    type BaseRing: Ring;

    const UNITY: Self = <Self as MultiplicativeMonoid>::IDENTITY;
    const ZERO: Self = <Self as AdditiveMonoid>::IDENTITY;
}

impl<R: Ring> AdditiveAbelianGroup for R {}

pub trait CommutativeRing: Ring {}

impl<Z: IntegerRing> CommutativeRing for Z {}

pub trait CyclotomicRing: Ring {
    fn conjugate(self) -> Self;

    const CYCLOTOMIC_INDEX: usize;
}

#[rustfmt::skip]
pub trait DivisionRing
    : Ring
    + Inv<Output = Option<Self>>
{
}

pub trait IntegerRing: Ring {
    const BITS: usize;
}
