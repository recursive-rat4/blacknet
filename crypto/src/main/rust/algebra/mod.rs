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

//! Abstract algebra

mod abeliangroup;
mod algebra;
mod cyclicgroup;
mod field;
mod freemodule;
mod group;
mod magma;
mod matrixring;
mod module;
mod monoid;
mod nttring;
mod operation;
mod ring;
mod semigroup;
mod semimodule;
mod semiring;
mod set;
mod univariatering;

pub use abeliangroup::*;
pub use algebra::*;
pub use cyclicgroup::*;
pub use field::*;
pub use freemodule::*;
pub use group::*;
pub use magma::*;
pub use matrixring::*;
pub use module::*;
pub use monoid::*;
pub use nttring::*;
pub use operation::*;
pub use ring::*;
pub use semigroup::*;
pub use semimodule::*;
pub use semiring::*;
pub use set::*;
pub use univariatering::*;
