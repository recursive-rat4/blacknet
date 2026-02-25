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

use crate::algebra::{Double, Semiring};
use crate::assigner::assigment::Assigment;
use crate::duplex::{Absorb, Duplex};
use alloc::vec::Vec;
use core::iter::zip;
use core::ops::{Add, AddAssign};

pub struct UnivariatePolynomial<'a, R: Semiring> {
    coefficients: Vec<R>,
    assigment: &'a Assigment<R>,
}

impl<'a, R: Semiring> UnivariatePolynomial<'a, R> {
    pub const fn new(coefficients: Vec<R>, assigment: &'a Assigment<R>) -> Self {
        Self {
            coefficients,
            assigment,
        }
    }

    #[allow(clippy::clone_on_copy, clippy::op_ref)]
    pub fn evaluate(&self, point: &R) -> R {
        // Horner method
        if self.coefficients.is_empty() {
            return R::ZERO;
        }
        let mut accum = self.coefficients[self.coefficients.len() - 1].clone();
        for i in (0..self.coefficients.len() - 1).rev() {
            let ap = accum * point;
            self.assigment.push(ap.clone());
            accum = ap + &self.coefficients[i];
        }
        accum
    }

    pub fn at_0_plus_1(&self) -> R {
        match self.coefficients.len() {
            0 => R::ZERO,
            1 => self.coefficients[0].double(),
            _ => self.coefficients[0].double() + self.coefficients.iter().skip(1).sum::<R>(),
        }
    }

    pub const fn degree(&self) -> usize {
        self.coefficients.len() - 1
    }

    pub const fn variables(&self) -> usize {
        1
    }
}

impl<'a, R: Semiring> IntoIterator for UnivariatePolynomial<'a, R> {
    type Item = R;
    type IntoIter = alloc::vec::IntoIter<R>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.coefficients.into_iter()
    }
}

impl<'a, R: Semiring> Add for UnivariatePolynomial<'a, R> {
    type Output = Self;

    fn add(self, rps: Self) -> Self::Output {
        debug_assert_eq!(self.coefficients.len(), rps.coefficients.len());
        Self {
            coefficients: zip(self.coefficients, rps.coefficients)
                .map(|(l, r)| l + r)
                .collect(),
            assigment: self.assigment,
        }
    }
}

impl<'a, R: Semiring> AddAssign for UnivariatePolynomial<'a, R> {
    fn add_assign(&mut self, rps: Self) {
        debug_assert_eq!(self.coefficients.len(), rps.coefficients.len());
        zip(self.coefficients.iter_mut(), rps.coefficients).for_each(|(l, r)| *l += r);
    }
}

impl<'a, R: Semiring> Double for UnivariatePolynomial<'a, R> {
    type Output = Self;

    fn double(self) -> Self::Output {
        Self {
            coefficients: self.coefficients.into_iter().map(Double::double).collect(),
            assigment: self.assigment,
        }
    }
}

impl<'a, R: Semiring + Absorb<R>> Absorb<R> for UnivariatePolynomial<'a, R> {
    fn absorb_into(self, duplex: &mut (impl Duplex<R> + ?Sized)) {
        duplex.absorb_iter(self.coefficients.into_iter())
    }
}

impl<'a, R: Semiring + Absorb<R>> Absorb<R> for &UnivariatePolynomial<'a, R> {
    fn absorb_into(self, duplex: &mut (impl Duplex<R> + ?Sized)) {
        duplex.absorb_iter(self.coefficients.iter().cloned())
    }
}
