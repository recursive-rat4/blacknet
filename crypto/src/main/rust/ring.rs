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
use crate::algebra::{CommutativeAlgebra, UnitalAlgebra};
use crate::cyclicgroup::AdditiveCyclicGroup;
use crate::integer::Integer;
use crate::magma::Inv;
use crate::monoid::{AdditiveMonoid, MultiplicativeMonoid};
use crate::semigroup::MultiplicativeSemigroup;
use alloc::vec::Vec;
use core::ops::{Index, IndexMut};

#[rustfmt::skip]
pub trait Ring
    : AdditiveAbelianGroup
    + MultiplicativeSemigroup
{
    type Int: Integer;

    const ZERO: Self = <Self as AdditiveMonoid>::IDENTITY;
}

impl<R: Ring> AdditiveAbelianGroup for R {}

#[rustfmt::skip]
pub trait UnitalRing
    : Ring
    + MultiplicativeMonoid
{
    const UNITY: Self = <Self as MultiplicativeMonoid>::IDENTITY;
}

impl<R: Ring + MultiplicativeMonoid> UnitalRing for R {}

pub trait CommutativeRing: UnitalRing {}

#[rustfmt::skip]
pub trait CyclotomicRing<Z: IntegerRing>
    : PolynomialRing<Z>
    + CommutativeAlgebra<Z>
{
    fn conjugate(self) -> Self;

    const CYCLOTOMIC_INDEX: usize;
}

#[rustfmt::skip]
pub trait DivisionRing
    : Ring
    + Inv<Output = Option<Self>>
{
}

#[rustfmt::skip]
pub trait IntegerRing
    : AdditiveCyclicGroup
    + CommutativeRing
{
    fn new(n: Self::Int) -> Self;
    fn with_limb(n: <Self::Int as Integer>::Limb) -> Self;

    fn canonical(self) -> Self::Int;
    fn absolute(self) -> Self::Int;

    const BITS: u32;
    const MODULUS: Self::Int;

    fn gadget(self) -> Vec<Self> {
        let mut representative = self.canonical();
        let mut bits = Vec::<Self>::with_capacity(Self::BITS as usize);
        for _ in 0..Self::BITS {
            let bit = representative & Self::Int::LIMB_ONE;
            bits.push(Self::with_limb(bit));
            representative >>= Self::Int::LIMB_ONE;
        }
        bits
    }
}

#[rustfmt::skip]
pub trait PolynomialRing<R: UnitalRing>
    : UnitalAlgebra<R>
    + Index<usize, Output = R>
    + IndexMut<usize, Output = R>
    + IntoIterator<Item = R>
{
    fn constant_term(self) -> R;
    fn evaluate(self, point: R) -> R;
}
