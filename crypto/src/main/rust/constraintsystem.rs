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

use crate::semiring::Presemiring;
use crate::vectordense::VectorDense;
use thiserror::Error;

pub trait ConstraintSystem<R: Presemiring> {
    fn degree(&self) -> usize;
    fn constraints(&self) -> usize;
    fn variables(&self) -> usize;

    fn is_satisfied(&self, z: &VectorDense<R>) -> Result<R>;
}

#[derive(Debug, Error)]
pub enum Error<R: Presemiring> {
    #[error("Assigned {0} variables instead of {1} required")]
    Length(usize, usize),
    #[error("Mismatch at position {0}")]
    Mismatch(usize, R, R),
}

pub type Result<R> = core::result::Result<(), Error<R>>;
