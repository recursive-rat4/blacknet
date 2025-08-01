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
use crate::ring::Ring;
use crate::vectordense::VectorDense;

#[derive(Debug, Eq, PartialEq)]
pub struct R1CS<R: Ring> {
    a: MatrixSparse<R>,
    b: MatrixSparse<R>,
    c: MatrixSparse<R>,
}

impl<R: Ring> R1CS<R> {
    pub const fn new(a: MatrixSparse<R>, b: MatrixSparse<R>, c: MatrixSparse<R>) -> Self {
        Self { a, b, c }
    }

    pub const fn constraints(&self) -> usize {
        self.a.rows()
    }

    pub const fn variables(&self) -> usize {
        self.a.columns()
    }

    pub fn is_satisfied(&self, z: &VectorDense<R>) -> bool {
        debug_assert!(
            z.dimension() == self.variables(),
            "Assigned {} variables instead of {} required",
            z.dimension(),
            self.variables()
        );
        (&self.a * z) * (&self.b * z) == &self.c * z
    }

    pub fn assigment(&self) -> VectorDense<R> {
        let mut z = Vec::<R>::with_capacity(self.variables());
        z.push(R::UNITY);
        z.into()
    }
}
