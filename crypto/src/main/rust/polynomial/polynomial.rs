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
use crate::polynomial::Point;

/// Polynomial is an expression of indeterminates and coefficients with a finite number of terms.
pub trait Polynomial {
    /// Type of points at which the polynomial can be evaluated.
    type Point;
}

/// A polynomial in many indeterminates.
pub trait MultivariatePolynomial<R: Semiring>: Polynomial<Point = Point<R>> {
    /// Substitute an indeterminate for the given value.
    fn bind(&mut self, value: R);

    /// Evaluate at a point.
    fn point(&self, point: &Self::Point) -> R;
    /// Evaluate over the unit hypercube with one indeterminate substituted for a small value.
    fn hypercube_with_var<const VAL: i8>(&self) -> DenseVector<R>;

    /// The individual degree.
    fn degree(&self) -> usize;
    /// The number of indeterminates.
    fn variables(&self) -> usize;
}
