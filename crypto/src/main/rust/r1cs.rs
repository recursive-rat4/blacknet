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

use crate::assigner::assigment::Assigment;
use crate::constraintsystem::{ConstraintSystem, Error, Result};
use crate::matrixsparse::MatrixSparse;
use crate::ring::UnitalRing;
use crate::vectordense::VectorDense;
use core::iter::zip;

#[derive(Debug, Eq, PartialEq)]
pub struct R1CS<R: UnitalRing> {
    a: MatrixSparse<R>,
    b: MatrixSparse<R>,
    c: MatrixSparse<R>,
}

impl<R: UnitalRing> R1CS<R> {
    pub const fn new(a: MatrixSparse<R>, b: MatrixSparse<R>, c: MatrixSparse<R>) -> Self {
        Self { a, b, c }
    }

    pub fn steal(self) -> (MatrixSparse<R>, MatrixSparse<R>, MatrixSparse<R>) {
        (self.a, self.b, self.c)
    }

    pub fn assigment(&self) -> Assigment<R> {
        let z = Assigment::new(self.variables());
        z.push(R::UNITY);
        z
    }
}

impl<R: UnitalRing> ConstraintSystem<R> for R1CS<R> {
    fn degree(&self) -> usize {
        2
    }

    fn constraints(&self) -> usize {
        self.a.rows()
    }

    fn variables(&self) -> usize {
        self.a.columns()
    }

    fn is_satisfied(&self, z: &VectorDense<R>) -> Result<R> {
        if z.dimension() != self.variables() {
            return Err(Error::Length(z.dimension(), self.variables()));
        }
        let az_bz = (&self.a * z) * (&self.b * z);
        let cz = &self.c * z;
        match zip(az_bz, cz).enumerate().find(|(_, (a, e))| a != e) {
            Some((i, (a, e))) => Err(Error::Mismatch(i, a, e)),
            None => Ok(()),
        }
    }
}
