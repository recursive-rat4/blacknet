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

use crate::magma::{AdditiveMagma, MultiplicativeMagma};
use core::iter::{Product, Sum};

#[rustfmt::skip]
pub trait AdditiveSemigroup
    : AdditiveMagma
    + Sum
{
    const LEFT_IDENTITY: Self;
    const RIGHT_IDENTITY: Self;

    fn double_and_add<Scalar: IntoIterator<Item = bool>>(self, scalar: Scalar) -> Self {
        let mut r = Self::LEFT_IDENTITY;
        let mut t = self;
        for bit in scalar {
            if bit {
                r += t
            }
            t = t.double()
        }
        r
    }
}

#[rustfmt::skip]
pub trait MultiplicativeSemigroup
    : MultiplicativeMagma
    + Product
{
    const LEFT_IDENTITY: Self;
    const RIGHT_IDENTITY: Self;

    fn square_and_multiply<Scalar: IntoIterator<Item = bool>>(self, scalar: Scalar) -> Self {
        let mut r = Self::LEFT_IDENTITY;
        let mut t = self;
        for bit in scalar {
            if bit {
                r *= t
            }
            t = t.square()
        }
        r
    }
}
