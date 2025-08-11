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

use crate::ring::{Ring, UnitalRing};
use core::ops::Add;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnivariatePolynomial<R: Ring> {
    coefficients: Vec<R>,
}

impl<R: UnitalRing> UnivariatePolynomial<R> {
    pub fn evaluate(&self, point: R) -> R {
        let mut sigma = self.coefficients[0];
        let mut power = point;
        for i in 1..self.coefficients.len() - 1 {
            sigma += self.coefficients[i] * power;
            power *= point;
        }
        if self.coefficients.len() > 1 {
            sigma += self.coefficients[self.coefficients.len() - 1] * power;
        }
        sigma
    }

    pub fn at_0_plus_1(&self) -> R {
        self.coefficients
            .iter()
            .copied()
            .fold(self.coefficients[0], Add::add)
    }

    pub const fn degree(&self) -> usize {
        self.coefficients.len() - 1
    }

    pub const fn variables(&self) -> usize {
        1
    }
}

impl<R: Ring, const N: usize> From<[R; N]> for UnivariatePolynomial<R> {
    fn from(coefficients: [R; N]) -> Self {
        Self {
            coefficients: coefficients.into(),
        }
    }
}

impl<R: Ring> From<Vec<R>> for UnivariatePolynomial<R> {
    fn from(coefficients: Vec<R>) -> Self {
        Self { coefficients }
    }
}
