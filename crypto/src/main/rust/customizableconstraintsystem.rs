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

use crate::matrixsparse::MatrixSparse;
use crate::r1cs::R1CS;
use crate::ring::Ring;
use crate::vectordense::VectorDense;

// https://eprint.iacr.org/2023/552

#[derive(Debug, Eq, PartialEq)]
pub struct CustomizableConstraintSystem<R: Ring> {
    constraints: usize,
    variables: usize,
    matrices: Vec<MatrixSparse<R>>,
    multisets: Vec<Vec<usize>>,
    constants: Vec<R>,
}

impl<R: Ring> CustomizableConstraintSystem<R> {
    pub const fn new(
        constraints: usize,
        variables: usize,
        matrices: Vec<MatrixSparse<R>>,
        multisets: Vec<Vec<usize>>,
        constants: Vec<R>,
    ) -> Self {
        Self {
            constraints,
            variables,
            matrices,
            multisets,
            constants,
        }
    }

    pub const fn constraints(&self) -> usize {
        self.constraints
    }

    pub const fn variables(&self) -> usize {
        self.variables
    }

    pub fn is_satisfied(&self, z: &VectorDense<R>) -> bool {
        debug_assert!(
            z.dimension() == self.variables(),
            "Assigned {} variables instead of {} required",
            z.dimension(),
            self.variables()
        );
        let mut sigma = VectorDense::fill(self.constraints, R::ZERO);
        for (i, &c) in self.constants.iter().enumerate() {
            let mut circle = VectorDense::fill(self.constraints, c);
            for &j in &self.multisets[i] {
                circle *= &self.matrices[j] * z;
            }
            sigma += circle;
        }
        sigma.steal().into_iter().all(|e| e == R::ZERO)
    }

    pub fn assigment(&self) -> VectorDense<R> {
        let mut z = Vec::<R>::with_capacity(self.variables());
        z.push(R::UNITY);
        z.into()
    }
}

impl<R: Ring> From<R1CS<R>> for CustomizableConstraintSystem<R> {
    fn from(r1cs: R1CS<R>) -> Self {
        let constraints = r1cs.constraints();
        let variables = r1cs.variables();
        let (a, b, c) = r1cs.steal();
        Self {
            constraints,
            variables,
            matrices: vec![a, b, c],
            multisets: vec![vec![0, 1], vec![2]],
            constants: vec![R::UNITY, -R::UNITY],
        }
    }
}
