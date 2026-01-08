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

use crate::algebra::Set;
use thiserror::Error;

pub trait ConstraintSystem<S: Set> {
    type Assigment;

    fn degree(&self) -> usize;
    fn constraints(&self) -> usize;
    fn variables(&self) -> usize;

    fn is_satisfied(&self, z: &Self::Assigment) -> Result<S>;
}

#[derive(Debug, Error)]
pub enum Error<S: Set> {
    #[error("Assigned {0} variables instead of {1} required")]
    Length(usize, usize),
    #[error("Mismatch at position {0}")]
    Mismatch(usize, S, S),
}

pub type Result<S> = core::result::Result<(), Error<S>>;
