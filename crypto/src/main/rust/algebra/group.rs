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
    AdditiveMagmaOps, AdditiveMonoid, Inv, MultiplicativeMagmaOps, MultiplicativeMonoid,
};
use core::ops::{Div, DivAssign, Neg, Sub, SubAssign};

#[rustfmt::skip]
pub trait AdditiveGroupOps<G>
    : AdditiveMagmaOps<G>
    + Neg<Output = G>
    + Sub<G, Output = G>
    + for<'a> Sub<&'a G, Output = G>
{
}

#[rustfmt::skip]
impl<G, T
    : AdditiveMagmaOps<G>
    + Neg<Output = G>
    + Sub<G, Output = G>
    + for<'a> Sub<&'a G, Output = G>
> AdditiveGroupOps<G> for T {}

#[rustfmt::skip]
pub trait AdditiveGroup
    : AdditiveMonoid
    + AdditiveGroupOps<Self>
    + SubAssign
    + for<'a> SubAssign<&'a Self>
    + Clone
{
}

#[rustfmt::skip]
impl<G
    : AdditiveMonoid
    + AdditiveGroupOps<Self>
    + SubAssign
    + for<'a> SubAssign<&'a Self>
    + Clone
> AdditiveGroup for G {}

#[rustfmt::skip]
pub trait MultiplicativeGroupOps<G>
    : MultiplicativeMagmaOps<G>
    + Inv<Output = G>
    + Div<G, Output = G>
    + for<'a> Div<&'a G, Output = G>
{
}

#[rustfmt::skip]
impl<G, T
    : MultiplicativeMagmaOps<G>
    + Inv<Output = G>
    + Div<G, Output = G>
    + for<'a> Div<&'a G, Output = G>
> MultiplicativeGroupOps<G> for T {}

#[rustfmt::skip]
pub trait MultiplicativeGroup
    : MultiplicativeMonoid
    + MultiplicativeGroupOps<Self>
    + DivAssign
    + for<'a> DivAssign<&'a Self>
    + Clone
{
}

#[rustfmt::skip]
impl<G
    : MultiplicativeMonoid
    + MultiplicativeGroupOps<Self>
    + DivAssign
    + for<'a> DivAssign<&'a Self>
    + Clone
> MultiplicativeGroup for G {}
