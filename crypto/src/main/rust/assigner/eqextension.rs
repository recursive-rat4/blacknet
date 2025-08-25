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
use crate::point::Point;
use crate::ring::UnitalRing;
use crate::vectordense::VectorDense;
use core::iter::zip;

pub struct EqExtension<'a, R: UnitalRing> {
    coefficients: Vec<R>,
    assigment: &'a Assigment<R>,
}

impl<'a, R: UnitalRing> EqExtension<'a, R> {
    pub const fn new(coefficients: Vec<R>, assigment: &'a Assigment<R>) -> Self {
        Self {
            coefficients,
            assigment,
        }
    }

    pub fn point(&self, point: &Point<R>) -> R {
        let mut pi = R::UNITY;
        zip(self.coefficients.iter(), point.coordinates()).for_each(|(&c, &p)| {
            let cp = c * p;
            self.assigment.push(cp);
            let t = pi * (cp.double() - c - p + R::UNITY);
            self.assigment.push(t);
            pi = t;
        });
        pi
    }

    pub fn hypercube(&self) -> VectorDense<R> {
        let mut r = vec![R::ZERO; 1 << self.coefficients.len()];
        r[0] = R::UNITY;
        let mut j = 1;
        for i in (0..self.coefficients.len()).rev() {
            let mut l = j;
            for k in 0..j {
                let t = self.coefficients[i] * r[k];
                self.assigment.push(t);
                r[l] = t;
                r[k] -= t;
                l += 1;
            }
            j <<= 1;
        }
        r.into()
    }
}
