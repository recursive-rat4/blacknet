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

use crate::algebra::Semiring;
use crate::algebra::UnitalRing;
use crate::assigner::assigment::Assigment;
use crate::constraintsystem::{ConstraintSystem, Error, Result};
use crate::matrix::{DenseVector, SparseMatrix};
use crate::r1cs::R1CS;
use alloc::vec;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

/// CCS <https://eprint.iacr.org/2023/552>
#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CustomizableConstraintSystem<R: Semiring> {
    matrices: Vec<SparseMatrix<R>>,
    multisets: Vec<Vec<usize>>,
    constants: Vec<R>,
}

impl<R: Semiring> CustomizableConstraintSystem<R> {
    pub const fn new(
        matrices: Vec<SparseMatrix<R>>,
        multisets: Vec<Vec<usize>>,
        constants: Vec<R>,
    ) -> Self {
        Self {
            matrices,
            multisets,
            constants,
        }
    }
}

impl<R: Semiring + Eq + Send + Sync> CustomizableConstraintSystem<R> {
    pub fn assigment(&self) -> Assigment<R> {
        let z = Assigment::new(self.variables());
        z.push(R::ONE);
        z
    }
}

impl<R: UnitalRing> From<R1CS<R>> for CustomizableConstraintSystem<R> {
    fn from(r1cs: R1CS<R>) -> Self {
        let (a, b, c) = r1cs.into();
        Self {
            matrices: vec![a, b, c],
            multisets: vec![vec![0, 1], vec![2]],
            constants: vec![R::ONE, -R::ONE],
        }
    }
}

impl<R: Semiring + Eq + Send + Sync> ConstraintSystem<R> for CustomizableConstraintSystem<R> {
    fn degree(&self) -> usize {
        self.multisets
            .iter()
            .map(Vec::len)
            .max()
            .expect("Valid CCS")
    }

    fn constraints(&self) -> usize {
        self.matrices
            .first()
            .map(SparseMatrix::rows)
            .expect("Valid CCS")
    }

    fn variables(&self) -> usize {
        self.matrices
            .first()
            .map(SparseMatrix::columns)
            .expect("Valid CCS")
    }

    fn is_satisfied(&self, z: &DenseVector<R>) -> Result<R> {
        let constraints = self.constraints();
        let variables = self.variables();
        if z.dimension() != variables {
            return Err(Error::Length(z.dimension(), variables));
        }
        let mut sigma = DenseVector::fill(constraints, R::ZERO);
        for (i, &c) in self.constants.iter().enumerate() {
            let mut circle = DenseVector::fill(constraints, c);
            for &j in &self.multisets[i] {
                circle *= &self.matrices[j] * z;
            }
            sigma += circle;
        }
        match sigma.into_iter().enumerate().find(|(_, e)| *e != R::ZERO) {
            Some((i, e)) => Err(Error::Mismatch(i, e, R::ZERO)),
            None => Ok(()),
        }
    }
}
