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
use crate::module::Module;
use crate::operation::Inv;
use crate::semiring::{CommutativeSemiring, Presemiring, Semiring};
use alloc::vec::Vec;
use core::ops::{Index, IndexMut};

#[rustfmt::skip]
pub trait Ring
    : Presemiring
    + AdditiveAbelianGroup
{
    type Int: Integer;
}

#[rustfmt::skip]
pub trait UnitalRing
    : Ring
    + Semiring
{
}

impl<R: Ring + Semiring> UnitalRing for R {}

#[rustfmt::skip]
pub trait CommutativeRing
    : UnitalRing
    + CommutativeSemiring
{
}

impl<R: UnitalRing + CommutativeSemiring> CommutativeRing for R {}

#[rustfmt::skip]
pub trait DivisionRing
    : Ring
    + Inv<Output = Option<Self>>
{
}

#[rustfmt::skip]
pub trait IntegerRing
    : CommutativeRing
    + AdditiveCyclicGroup
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

impl<Z: IntegerRing> AdditiveCyclicGroup for Z {}

#[rustfmt::skip]
pub trait PolynomialRing<R: UnitalRing>
    : UnitalAlgebra<R>
    + Index<usize, Output = R>
    + IndexMut<usize, Output = R>
    + IntoIterator<Item = R>
{
    fn coefficients(self) -> impl Module<R>;
    fn constant_term(self) -> R;
    fn evaluate(self, point: R) -> R;
}

#[rustfmt::skip]
pub trait PowerOfTwoCyclotomicRing<Z: IntegerRing>
    : PolynomialRing<Z>
    + CommutativeAlgebra<Z>
{
    fn conjugate(self) -> Self;
}
