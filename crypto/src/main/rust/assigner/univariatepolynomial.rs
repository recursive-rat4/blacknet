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
use crate::duplex::{Absorb, Duplex};
use crate::ring::{Ring, UnitalRing};
use alloc::vec::Vec;
use core::ops::Add;

pub struct UnivariatePolynomial<'a, R: Ring> {
    coefficients: Vec<R>,
    assigment: &'a Assigment<R>,
}

impl<'a, R: UnitalRing> UnivariatePolynomial<'a, R> {
    pub fn new(coefficients: Vec<R>, assigment: &'a Assigment<R>) -> Self {
        Self {
            coefficients,
            assigment,
        }
    }

    pub fn evaluate(&self, point: R) -> R {
        let mut sigma = self.coefficients[0];
        let mut power = point;
        for i in 1..self.coefficients.len() - 1 {
            let cp = self.coefficients[i] * power;
            self.assigment.push(cp);
            sigma += cp;
            let pp = power * point;
            self.assigment.push(pp);
            power = pp;
        }
        if self.coefficients.len() > 1 {
            let cp = self.coefficients[self.coefficients.len() - 1] * power;
            self.assigment.push(cp);
            sigma += cp;
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

impl<'a, R: Ring> IntoIterator for UnivariatePolynomial<'a, R> {
    type Item = R;
    type IntoIter = alloc::vec::IntoIter<R>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.coefficients.into_iter()
    }
}

impl<'a, R: Ring + Absorb<R>> Absorb<R> for UnivariatePolynomial<'a, R> {
    fn absorb_into(&self, duplex: &mut (impl Duplex<R> + ?Sized)) {
        duplex.absorb(&self.coefficients)
    }
}
