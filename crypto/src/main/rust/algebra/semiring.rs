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
    AdditiveCommutativeMonoid, AdditiveMagmaOps, Inv, MultiplicativeCommutativeSemigroup,
    MultiplicativeMagmaOps, MultiplicativeMonoid, MultiplicativeSemigroup,
};
use core::fmt;

#[rustfmt::skip]
pub trait SemiringOps<R>
    : AdditiveMagmaOps<R>
    + MultiplicativeMagmaOps<R>
{
}

#[rustfmt::skip]
impl<R, T
    : AdditiveMagmaOps<R>
    + MultiplicativeMagmaOps<R>
> SemiringOps<R> for T {}

#[rustfmt::skip]
pub trait SemifieldOps<R>
    : SemiringOps<R>
    + Inv<Output = Option<R>>
{
}

#[rustfmt::skip]
impl<R, T
    : SemiringOps<R>
    + Inv<Output = Option<R>>
> SemifieldOps<R> for T {}

/// A generalization of [nonunital ring][`crate::algebra::Ring`]
/// that doesn't require subtraction.
#[rustfmt::skip]
pub trait Presemiring
    : AdditiveCommutativeMonoid
    + MultiplicativeSemigroup
    + SemiringOps<Self>
{
}

#[rustfmt::skip]
impl<R
    : AdditiveCommutativeMonoid
    + MultiplicativeSemigroup
    + SemiringOps<Self>
> Presemiring for R
{
}

/// A generalization of [unital ring][`crate::algebra::UnitalRing`]
/// that doesn't require subtraction.
#[rustfmt::skip]
pub trait Semiring
    : Presemiring
    + MultiplicativeMonoid
{
}

#[rustfmt::skip]
impl<R
    : Presemiring
    + MultiplicativeMonoid
> Semiring for R {}

/// A marker for semirings with commutative multiplication.
/// Semiring elements commute under addition by definition.
#[rustfmt::skip]
pub trait CommutativeSemiring
    : Presemiring
    + MultiplicativeCommutativeSemigroup
{
}

#[rustfmt::skip]
impl<R
    : Presemiring
    + MultiplicativeCommutativeSemigroup
> CommutativeSemiring for R {}

/// A generalization of [division ring][`crate::algebra::DivisionRing`]
/// that doesn't require subtraction.
#[rustfmt::skip]
pub trait Semifield
    : Presemiring
    + SemifieldOps<Self>
{
}

/// Invert multiple elements.
///
/// Either all elements are inverted or the opaque error is returned.
pub fn batched_inv<R: Semifield + Clone, const N: usize>(
    a: &[R; N],
    b: &mut [R; N],
) -> Result<(), BatchedInvError>
where
    for<'a> &'a R: SemifieldOps<R>,
{
    // Montgomery trick
    const {
        assert!(N > 1);
    }
    b[1] = a[0].clone();
    for i in 2..N {
        b[i] = &b[i - 1] * &a[i - 1]
    }
    let p = &b[N - 1] * &a[N - 1];
    let mut v = p.inv().ok_or(BatchedInvError {})?;
    for i in (1..N).rev() {
        b[i] = &v * &b[i];
        v = &a[i] * &v;
    }
    b[0] = v;
    Ok(())
}

#[derive(Debug)]
pub struct BatchedInvError {}

impl fmt::Display for BatchedInvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Batch contains noninvertible elements")
    }
}

impl core::error::Error for BatchedInvError {}
