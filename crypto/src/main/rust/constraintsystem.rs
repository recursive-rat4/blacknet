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

use crate::ring::Ring;
use crate::vectordense::VectorDense;
use core::fmt;

pub trait ConstraintSystem<R: Ring> {
    fn degree(&self) -> usize;
    fn constraints(&self) -> usize;
    fn variables(&self) -> usize;

    fn is_satisfied(&self, z: &VectorDense<R>) -> Result<R>;
}

#[derive(Debug, Eq, PartialEq)]
pub enum Error<R: Ring> {
    Length(usize, usize),
    Mismatch(usize, R, R),
}

impl<R: Ring> core::error::Error for Error<R> {}

impl<R: Ring> fmt::Display for Error<R> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Length(actual, expected) => {
                write!(
                    formatter,
                    "Assigned {actual} variables instead of {expected} required",
                )
            }
            Error::Mismatch(position, _, _) => {
                write!(formatter, "Mismatch at position {position}")
            }
        }
    }
}

pub type Result<R> = core::result::Result<(), Error<R>>;
