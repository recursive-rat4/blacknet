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

use crate::semigroup::{AdditiveSemigroup, MultiplicativeSemigroup};

#[rustfmt::skip]
pub trait AdditiveMonoid
    : Default
    + AdditiveSemigroup
{
    const IDENTITY: Self;
}

impl<M: AdditiveMonoid> AdditiveSemigroup for M {
    const LEFT_IDENTITY: Self = M::IDENTITY;
    const RIGHT_IDENTITY: Self = M::IDENTITY;
}

#[rustfmt::skip]
pub trait MultiplicativeMonoid
    : Default
    + MultiplicativeSemigroup
{
    const IDENTITY: Self;
}
