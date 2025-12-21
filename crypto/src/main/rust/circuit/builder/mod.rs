/*
 * Copyright (c) 2025 Pavel Vasin
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

//! Arithmetic circuit builder.
//!
//! It can output [R1CS][crate::r1cs::R1CS] over semirings,
//! and [CCS][crate::customizableconstraintsystem::CustomizableConstraintSystem] over rings.

mod circuitbuilder;
mod constant;
mod linearcombination;
mod linearmonoid;
mod linearspan;
mod linearterm;
mod variable;

pub use circuitbuilder::{CircuitBuilder, Constraint, Expression, Scope};
pub use constant::Constant;
pub use linearcombination::LinearCombination;
pub use linearmonoid::LinearMonoid;
pub use linearspan::LinearSpan;
pub use linearterm::LinearTerm;
pub use variable::{Variable, VariableKind};
