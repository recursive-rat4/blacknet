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
use crate::algebra::{Algebra, CommutativeAlgebra};
use crate::integer::Integer;
use crate::magma::Inv;
use crate::monoid::{AdditiveMonoid, MultiplicativeMonoid};
use crate::semigroup::MultiplicativeSemigroup;

#[rustfmt::skip]
pub trait Ring
    : AdditiveAbelianGroup
    + MultiplicativeSemigroup
{
    type BaseRing: Ring;
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

impl<Z: IntegerRing> CommutativeRing for Z {}

#[rustfmt::skip]
pub trait CyclotomicRing<Z: IntegerRing, const N: usize>
    : PolynomialRing<Z, N>
    + CommutativeAlgebra<Z, N>
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

pub trait IntegerRing: UnitalRing {
    fn canonical(self) -> Self::Int;
    fn absolute(self) -> Self::Int;

    const BITS: usize;
    const MODULUS: Self::Int;
}

#[rustfmt::skip]
pub trait PolynomialRing<R: Ring, const N: usize>
    : Algebra<R, N>
{
    fn constant_term(self) -> R;
    fn evaluate(self, point: R) -> R;
}
