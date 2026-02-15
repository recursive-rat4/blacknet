/*
 * Copyright (c) 2026 Pavel Vasin
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

use crate::algebra::{AdditiveCommutativeMonoid, AdditiveMagmaOps, Presemiring};
use core::ops::{Mul, MulAssign};

#[rustfmt::skip]
pub trait SemimoduleOps<R, M>
    : AdditiveMagmaOps<M>
    + Mul<R, Output = M>
    + for<'a> Mul<&'a R, Output = M>
{
}

#[rustfmt::skip]
impl<R, M, T
    : AdditiveMagmaOps<M>
    + Mul<R, Output = M>
    + for<'a> Mul<&'a R, Output = M>
> SemimoduleOps<R, M> for T {}

/// A generalization of [module][`crate::algebra::Module`] to semirings.
#[rustfmt::skip]
pub trait Semimodule<R: Presemiring>
    : AdditiveCommutativeMonoid
    + SemimoduleOps<R, Self>
    + MulAssign<R>
    + for<'a> MulAssign<&'a R>
{
}

/// Any semiring is a semimodule over itself.
impl<R: Presemiring> Semimodule<R> for R {}
