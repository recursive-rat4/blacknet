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

use crate::point::Point;
use crate::polynomial::Polynomial;
use crate::ring::UnitalRing;
use crate::vectordense::VectorDense;
use core::iter::zip;
use core::ops::{Mul, MulAssign};

#[derive(Clone)]
pub struct EqExtension<R: UnitalRing> {
    coefficients: Vec<R>,
    z: R,
}

impl<R: UnitalRing> EqExtension<R> {
    pub const fn new(coefficients: Vec<R>, z: R) -> Self {
        Self { coefficients, z }
    }

    pub fn basis(coefficients: &[R]) -> Vec<R> {
        Self::evaluate(coefficients, R::UNITY)
    }

    pub fn hypercube(&self) -> VectorDense<R> {
        Self::evaluate(&self.coefficients, self.z).into()
    }

    fn evaluate(coefficients: &[R], z: R) -> Vec<R> {
        let mut r = vec![R::ZERO; 1 << coefficients.len()];
        r[0] = z;
        let mut j = 1;
        for i in (0..coefficients.len()).rev() {
            let mut l = j;
            for k in 0..j {
                let launder = coefficients[i] * r[k];
                r[l] = launder;
                r[k] -= launder;
                l += 1;
            }
            j <<= 1;
        }
        r
    }
}

impl<R: UnitalRing, const N: usize> From<[R; N]> for EqExtension<R> {
    fn from(coefficients: [R; N]) -> Self {
        Self {
            coefficients: coefficients.into(),
            z: R::UNITY,
        }
    }
}

impl<R: UnitalRing> From<Vec<R>> for EqExtension<R> {
    fn from(coefficients: Vec<R>) -> Self {
        Self {
            coefficients,
            z: R::UNITY,
        }
    }
}

impl<R: UnitalRing + From<u8>> Polynomial<R> for EqExtension<R> {
    fn bind(&mut self, e: R) {
        self.z *= (self.coefficients[0] * e).double() - self.coefficients[0] - e + R::UNITY;
        self.coefficients.remove(0);
    }

    fn point(&self, point: &Point<R>) -> R {
        self.z
            * zip(self.coefficients.iter(), point.coordinates())
                .map(|(&c, &p)| (c * p).double() - c - p + R::UNITY)
                .product()
    }

    #[rustfmt::skip]
    fn hypercube_with_var<const VAL: i8>(&self) -> VectorDense<R> {
        let z = match VAL {
            -2 => self.z * (R::from(3) - self.coefficients[0] - self.coefficients[0].double().double()),
            -1 => self.z * (R::from(2) - self.coefficients[0] - self.coefficients[0].double()),
            0 => self.z * (R::UNITY - self.coefficients[0]),
            1 => self.z * self.coefficients[0],
            2 => self.z * (self.coefficients[0].double() + self.coefficients[0] - R::UNITY),
            3 => self.z * (self.coefficients[0].double().double() + self.coefficients[0] - R::from(2)),
            4 => self.z * (self.coefficients[0].double().double().double() - self.coefficients[0] - R::from(3)),
            _ => unimplemented!("hypercube_with_var for val = {VAL}"),
        };
        Self::evaluate(&self.coefficients[1..], z).into()
    }

    fn degree(&self) -> usize {
        1
    }

    fn variables(&self) -> usize {
        self.coefficients.len()
    }
}

impl<R: UnitalRing> Mul<R> for EqExtension<R> {
    type Output = Self;

    fn mul(self, rps: R) -> Self::Output {
        Self {
            coefficients: self.coefficients,
            z: self.z * rps,
        }
    }
}

impl<R: UnitalRing> MulAssign<R> for EqExtension<R> {
    fn mul_assign(&mut self, rps: R) {
        self.z *= rps
    }
}
