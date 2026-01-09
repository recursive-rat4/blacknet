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

use crate::algebra::{AdditiveCommutativeMonoid, Presemiring};
use core::ops::{Mul, MulAssign};

/// A generalization of [module][`crate::algebra::Module`] to semirings.
#[rustfmt::skip]
pub trait Semimodule<R: Presemiring>
    : AdditiveCommutativeMonoid
    + Mul<R, Output = Self>
    + MulAssign<R>
    + for<'a> Mul<&'a R, Output = Self>
    + for<'a> MulAssign<&'a R>
{
}

/// Any semiring is a semimodule over itself.
impl<R: Presemiring> Semimodule<R> for R {}
