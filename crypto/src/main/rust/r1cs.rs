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
use crate::matrix::{DenseVector, SparseMatrix};
use crate::semiring::Semiring;
use core::iter::zip;
use serde::{Deserialize, Serialize};

/// Rank-1 constraint system over semirings consists of matrices `a, b, c`.
/// It asks for a vector `z` such that `(a * z) * (b * z) = (c * z)`.
#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct R1CS<R: Semiring> {
    a: SparseMatrix<R>,
    b: SparseMatrix<R>,
    c: SparseMatrix<R>,
}

impl<R: Semiring> R1CS<R> {
    /// Construct a new R1CS given the three matrices.
    pub const fn new(a: SparseMatrix<R>, b: SparseMatrix<R>, c: SparseMatrix<R>) -> Self {
        Self { a, b, c }
    }
}

impl<R: Semiring + Eq + Send + Sync> R1CS<R> {
    pub fn assigment(&self) -> Assigment<R> {
        let z = Assigment::new(self.variables());
        z.push(R::ONE);
        z
    }
}

impl<R: Semiring> From<R1CS<R>> for (SparseMatrix<R>, SparseMatrix<R>, SparseMatrix<R>) {
    fn from(r1cs: R1CS<R>) -> Self {
        (r1cs.a, r1cs.b, r1cs.c)
    }
}

impl<R: Semiring + Eq + Send + Sync> ConstraintSystem<R> for R1CS<R> {
    fn degree(&self) -> usize {
        2
    }

    fn constraints(&self) -> usize {
        self.a.rows()
    }

    fn variables(&self) -> usize {
        self.a.columns()
    }

    fn is_satisfied(&self, z: &DenseVector<R>) -> Result<R> {
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
