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

//! Matrix is a rectangular array of semiring entries.

mod densematrix;
mod densevector;
mod sparsematrix;
mod sparsevector;
mod symmetricmatrix;
mod symmetrictridiagonalmatrix;

pub use densematrix::DenseMatrix;
pub use densevector::DenseVector;
pub use sparsematrix::{SparseMatrix, SparseMatrixBuilder};
pub use sparsevector::SparseVector;
pub use symmetricmatrix::SymmetricMatrix;
pub use symmetrictridiagonalmatrix::SymmetricTridiagonalMatrix;
