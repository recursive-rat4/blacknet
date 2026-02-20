/*
 * Copyright (c) 2026 Pavel Vasin
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

use crate::algebra::UnitalRing;
use crate::matrix::DenseVector;
use crate::polynomial::{InBasis, MultivariatePolynomial, Point, Polynomial};
use alloc::vec::Vec;
use core::iter::zip;

/// A multivariate polynomial that is a sum of univariate polynomials.
///
/// For zero-knowledge sum-check from Libra: <https://eprint.iacr.org/2019/317>.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MaskingPolynomial<R: UnitalRing> {
    coefficients: Vec<R>,
    degree: usize,
    variables: usize,
}

impl<R: UnitalRing> MaskingPolynomial<R> {
    /// Construct a new polynomial.
    pub const fn new(coefficients: Vec<R>, degree: usize, variables: usize) -> Self {
        debug_assert!(1 + degree * variables == coefficients.len());
        Self {
            coefficients,
            degree,
            variables,
        }
    }

    /// Sum over the unit hypercube.
    pub fn sum(&self) -> R {
        let sigma = self.coefficients[0];
        if self.variables == 0 {
            return sigma;
        }
        let sigmas = self.coefficients[1..].iter().sum::<R>();
        let mut k = R::ONE;
        for _ in 1..self.variables {
            k = k.double();
        }
        k * (sigma.double() + sigmas)
    }

    fn eval(univariate: &[R], point: &R) -> R {
        let mut power = *point;
        let mut sigma = univariate[0] * power;
        for &coefficient in univariate.iter().skip(1) {
            power *= point;
            sigma += coefficient * power;
        }
        sigma
    }
}

impl<R: UnitalRing> From<MaskingPolynomial<R>> for (Vec<R>, usize, usize) {
    fn from(polynomial: MaskingPolynomial<R>) -> Self {
        (
            polynomial.coefficients,
            polynomial.degree,
            polynomial.variables,
        )
    }
}

impl<R: UnitalRing> Polynomial for MaskingPolynomial<R> {
    type Coefficient = R;
    type Point = Point<R>;

    fn point(&self, point: &Point<R>) -> R {
        let univariates = self.coefficients[1..].chunks_exact(self.degree).rev();
        self.coefficients[0]
            + zip(univariates, point)
                .map(|(univariate, coordinate)| Self::eval(univariate, coordinate))
                .sum::<R>()
    }
}

impl<R: UnitalRing> MultivariatePolynomial for MaskingPolynomial<R> {
    fn bind(&mut self, value: &R) {
        self.variables -= 1;
        let new_len = self.coefficients.len() - self.degree;
        let univariate = &self.coefficients[new_len..];
        let sigma = Self::eval(univariate, value);
        self.coefficients[0] += sigma;
        self.coefficients.truncate(new_len);
    }

    fn sum_with_var<const VAL: i8>(&self) -> R {
        let len = self.coefficients.len() - self.degree;
        let univariate = &self.coefficients[len..];

        let sigma = match VAL {
            -2 => {
                let mut sigma = -univariate[0].double();
                for (i, coefficient) in univariate.iter().enumerate().skip(1) {
                    let mut cp = coefficient.double();
                    for _ in 0..i {
                        cp = cp.double();
                    }
                    if i & 1 == 0 {
                        sigma -= cp;
                    } else {
                        sigma += cp;
                    }
                }
                self.coefficients[0] + sigma
            }
            -1 => univariate.iter().enumerate().fold(
                self.coefficients[0],
                |accum, (i, coefficient)| {
                    if i & 1 == 0 {
                        accum - coefficient
                    } else {
                        accum + coefficient
                    }
                },
            ),
            0 => self.coefficients[0],
            1 => self.coefficients[0] + univariate.iter().sum::<R>(),
            2 => {
                let mut sigma = univariate[0].double();
                for (i, coefficient) in univariate.iter().enumerate().skip(1) {
                    let mut cp = coefficient.double();
                    for _ in 0..i {
                        cp = cp.double();
                    }
                    sigma += cp;
                }
                self.coefficients[0] + sigma
            }
            3 => {
                let mut sigma = univariate[0].double() + univariate[0];
                for (i, coefficient) in univariate.iter().enumerate().skip(1) {
                    let mut cp = coefficient.double() + coefficient;
                    for _ in 0..i {
                        cp = cp.double() + cp;
                    }
                    sigma += cp;
                }
                self.coefficients[0] + sigma
            }
            4 => {
                let mut sigma = univariate[0].double().double();
                for (i, coefficient) in univariate.iter().enumerate().skip(1) {
                    let mut cp = coefficient.double().double();
                    for _ in 0..i {
                        cp = cp.double().double();
                    }
                    sigma += cp;
                }
                self.coefficients[0] + sigma
            }
            _ => unimplemented!("sum_with_var for val = {VAL}"),
        };

        if self.variables == 1 {
            return sigma;
        }

        let sigmas = self.coefficients[1..len].iter().sum::<R>();

        let mut k = R::ONE;
        for _ in 2..self.variables {
            k = k.double();
        }

        k * (sigma.double() + sigmas)
    }

    fn degree(&self) -> usize {
        self.degree
    }

    fn variables(&self) -> usize {
        self.variables
    }
}

impl<R: UnitalRing> InBasis for MaskingPolynomial<R> {
    fn basis(&self, point: &Point<R>) -> DenseVector<R> {
        debug_assert!(self.variables == point.dimension());
        let n = self.coefficients.len();
        let mut powers = Vec::<R>::with_capacity(n);
        powers.push(R::ONE);
        if n == 1 {
            return powers.into();
        }
        for i in (0..self.variables).rev() {
            let mut power = point[i];
            powers.push(power);
            for _ in 1..self.degree {
                power *= point[i];
                powers.push(power);
            }
        }
        powers.into()
    }
}
