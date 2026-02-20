/*
 * Copyright (c) 2024-2026 Pavel Vasin
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
use crate::matrix::DenseVector;

/// Polynomial is an expression of indeterminates and coefficients with a finite number of terms.
pub trait Polynomial {
    /// Type of coefficients.
    type Coefficient: Semiring;
    /// Type of points at which the polynomial can be evaluated.
    type Point;

    /// Evaluate at a point.
    fn point(&self, point: &Self::Point) -> Self::Coefficient;
}

/// A polynomial in many indeterminates.
pub trait MultivariatePolynomial: Polynomial {
    /// Substitute an indeterminate for the given value.
    fn bind(&mut self, value: &Self::Coefficient);

    /// Sum over the unit hypercube with one indeterminate substituted for a small value.
    fn sum_with_var<const VAL: i8>(&self) -> Self::Coefficient;

    /// The individual degree.
    fn degree(&self) -> usize;
    /// The number of indeterminates.
    fn variables(&self) -> usize;
}

/// A polynomial in certain basis.
pub trait InBasis: Polynomial {
    /// Basis coordinates of a point.
    fn basis(&self, point: &Self::Point) -> DenseVector<Self::Coefficient>;
}

/// Tensor structured basis.
pub trait TensorBasis: InBasis {
    /// Tensor basis coordinates of a point.
    fn tensor_basis(
        &self,
        point: &Self::Point,
    ) -> (
        DenseVector<Self::Coefficient>,
        DenseVector<Self::Coefficient>,
    );
}
