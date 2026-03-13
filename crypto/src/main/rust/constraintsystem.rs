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
use core::fmt;

pub trait ConstraintSystem<S: Set> {
    type Assigment;

    fn degree(&self) -> usize;
    fn constraints(&self) -> usize;
    fn variables(&self) -> usize;

    fn is_satisfied(&self, z: &Self::Assigment) -> Result<S>;
}

#[derive(Debug)]
pub enum Error<S: Set> {
    Length(usize, usize),
    Mismatch(usize, S, S),
}

impl<S: Set> fmt::Display for Error<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Length(actual, expected) => write!(
                f,
                "Assigned {actual} variables instead of {expected} required"
            ),
            Error::Mismatch(idx, _, _) => write!(f, "Mismatch at position {idx}"),
        }
    }
}

impl<S: Set + fmt::Debug> core::error::Error for Error<S> {}

pub type Result<S> = core::result::Result<(), Error<S>>;
