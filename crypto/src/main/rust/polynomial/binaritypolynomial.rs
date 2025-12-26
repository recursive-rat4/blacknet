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

use crate::matrix::DenseVector;
use crate::operation::Square;
use crate::polynomial::{MultilinearExtension, Point, Polynomial};
use crate::ring::UnitalRing;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

/// A multivariate polynomial that evaluates to `0` over the unit hypercube,
/// if all coefficients are from subset `{0, 1}`.
#[derive(Clone, Deserialize, Serialize)]
pub struct BinarityPolynomial<R: UnitalRing> {
    coefficients: MultilinearExtension<R>,
}

impl<R: UnitalRing> BinarityPolynomial<R> {
    /// Construct a new polynomial.
    pub const fn new(coefficients: MultilinearExtension<R>) -> Self {
        Self { coefficients }
    }
}

impl<R: UnitalRing, const N: usize> From<[R; N]> for BinarityPolynomial<R> {
    fn from(coefficients: [R; N]) -> Self {
        Self {
            coefficients: coefficients.into(),
        }
    }
}

impl<R: UnitalRing> From<Vec<R>> for BinarityPolynomial<R> {
    fn from(coefficients: Vec<R>) -> Self {
        Self {
            coefficients: coefficients.into(),
        }
    }
}

impl<R: UnitalRing> From<DenseVector<R>> for BinarityPolynomial<R> {
    fn from(coefficients: DenseVector<R>) -> Self {
        Self {
            coefficients: coefficients.into(),
        }
    }
}

impl<R: UnitalRing + From<u8>> Polynomial<R> for BinarityPolynomial<R> {
    fn bind(&mut self, e: R) {
        self.coefficients.bind(e);
    }

    fn point(&self, point: &Point<R>) -> R {
        let t = self.coefficients.point(point);
        t.square() - t
    }

    fn hypercube_with_var<const VAL: i8>(&self) -> DenseVector<R> {
        let t = self.coefficients.hypercube_with_var::<VAL>();
        (&t).square() - t
    }

    fn degree(&self) -> usize {
        2
    }

    fn variables(&self) -> usize {
        self.coefficients.variables()
    }
}
