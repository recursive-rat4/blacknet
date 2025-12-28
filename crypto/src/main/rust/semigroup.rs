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

use crate::magma::{
    AdditiveCommutativeMagma, AdditiveMagma, MultiplicativeCommutativeMagma, MultiplicativeMagma,
};

/// One-sided identity.
pub trait LeftZero {
    /// The left additive identity.
    const LEFT_ZERO: Self;
}

/// One-sided identity.
pub trait RightZero {
    /// The right additive identity.
    const RIGHT_ZERO: Self;
}

/// A magma with associative addition.
#[rustfmt::skip]
pub trait AdditiveSemigroup
    : AdditiveMagma
    + Copy
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
            r += t
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

/// One-sided identity.
pub trait LeftOne {
    /// The left multiplicative identity.
    const LEFT_ONE: Self;
}

/// One-sided identity.
pub trait RightOne {
    /// The right multiplicative identity.
    const RIGHT_ONE: Self;
}

/// A magma with associative multiplication.
#[rustfmt::skip]
pub trait MultiplicativeSemigroup
    : MultiplicativeMagma
    + Copy
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
            r *= t
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
