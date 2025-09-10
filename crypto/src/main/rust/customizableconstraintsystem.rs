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
use crate::r1cs::R1CS;
use crate::ring::UnitalRing;
use crate::vectordense::VectorDense;
use alloc::vec;
use alloc::vec::Vec;

// https://eprint.iacr.org/2023/552

#[derive(Debug, Eq, PartialEq)]
pub struct CustomizableConstraintSystem<R: UnitalRing> {
    matrices: Vec<MatrixSparse<R>>,
    multisets: Vec<Vec<usize>>,
    constants: Vec<R>,
}

impl<R: UnitalRing> CustomizableConstraintSystem<R> {
    pub const fn new(
        matrices: Vec<MatrixSparse<R>>,
        multisets: Vec<Vec<usize>>,
        constants: Vec<R>,
    ) -> Self {
        Self {
            matrices,
            multisets,
            constants,
        }
    }

    pub fn assigment(&self) -> Assigment<R> {
        let z = Assigment::new(self.variables());
        z.push(R::UNITY);
        z
    }
}

impl<R: UnitalRing> From<R1CS<R>> for CustomizableConstraintSystem<R> {
    fn from(r1cs: R1CS<R>) -> Self {
        let (a, b, c) = r1cs.steal();
        Self {
            matrices: vec![a, b, c],
            multisets: vec![vec![0, 1], vec![2]],
            constants: vec![R::UNITY, -R::UNITY],
        }
    }
}

impl<R: UnitalRing> ConstraintSystem<R> for CustomizableConstraintSystem<R> {
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
            .map(MatrixSparse::rows)
            .expect("Valid CCS")
    }

    fn variables(&self) -> usize {
        self.matrices
            .first()
            .map(MatrixSparse::columns)
            .expect("Valid CCS")
    }

    fn is_satisfied(&self, z: &VectorDense<R>) -> Result<R> {
        let constraints = self.constraints();
        let variables = self.variables();
        if z.dimension() != variables {
            return Err(Error::Length(z.dimension(), variables));
        }
        let mut sigma = VectorDense::fill(constraints, R::ZERO);
        for (i, &c) in self.constants.iter().enumerate() {
            let mut circle = VectorDense::fill(constraints, c);
            for &j in &self.multisets[i] {
                circle *= &self.matrices[j] * z;
            }
            sigma += circle;
        }
        match sigma
            .steal()
            .into_iter()
            .enumerate()
            .find(|(_, e)| *e != R::ZERO)
        {
            Some((i, e)) => Err(Error::Mismatch(i, e, R::ZERO)),
            None => Ok(()),
        }
    }
}
