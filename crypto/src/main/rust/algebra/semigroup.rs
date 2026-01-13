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
    AdditiveCommutativeMagma, AdditiveMagma, LeftOne, LeftZero, MultiplicativeCommutativeMagma,
    MultiplicativeMagma,
};

/// A magma with associative addition.
#[rustfmt::skip]
pub trait AdditiveSemigroup
    : AdditiveMagma
{
}

/// Multiply `g` by a `scalar`.
#[rustfmt::skip]
pub fn double_and_add<
    G: AdditiveSemigroup + LeftZero,
    Scalar: IntoIterator<Item = bool>,
>(
    g: G,
    scalar: Scalar,
) -> G {
    let mut r = G::LEFT_ZERO;
    let mut t = g;
    for bit in scalar {
        if bit {
            r += &t
        }
        t = t.double()
    }
    r
}

/// A marker for semigroups with commutative addition.
#[rustfmt::skip]
pub trait AdditiveCommutativeSemigroup
    : AdditiveSemigroup
    + AdditiveCommutativeMagma
{
}

impl<G: AdditiveSemigroup + AdditiveCommutativeMagma> AdditiveCommutativeSemigroup for G {}

/// A magma with associative multiplication.
#[rustfmt::skip]
pub trait MultiplicativeSemigroup
    : MultiplicativeMagma
{
}

/// Raise `g` to a `scalar` power.
#[rustfmt::skip]
pub fn square_and_multiply<
    G: MultiplicativeSemigroup + LeftOne,
    Scalar: IntoIterator<Item = bool>,
>(
    g: G,
    scalar: Scalar,
) -> G {
    let mut r = G::LEFT_ONE;
    let mut t = g;
    for bit in scalar {
        if bit {
            r *= &t
        }
        t = t.square()
    }
    r
}

/// A marker for semigroups with commutative multiplication.
#[rustfmt::skip]
pub trait MultiplicativeCommutativeSemigroup
    : MultiplicativeSemigroup
    + MultiplicativeCommutativeMagma
{
}

impl<G: MultiplicativeSemigroup + MultiplicativeCommutativeMagma> MultiplicativeCommutativeSemigroup
    for G
{
}
