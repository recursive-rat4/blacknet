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

use crate::duplex::{Absorb, Duplex, Squeeze, SqueezeWithSize};
use crate::operation::Double;
use crate::ring::{Ring, UnitalRing};
use alloc::vec::Vec;
use core::iter::zip;
use core::ops::{Add, AddAssign, Deref, Neg};

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

    pub const fn coefficients(&self) -> &Vec<R> {
        &self.coefficients
    }

    #[inline]
    pub fn steal(self) -> Vec<R> {
        self.coefficients
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

impl<R: Ring> AsRef<[R]> for UnivariatePolynomial<R> {
    #[inline]
    fn as_ref(&self) -> &[R] {
        &self.coefficients
    }
}

impl<R: Ring> Deref for UnivariatePolynomial<R> {
    type Target = [R];

    #[inline]
    fn deref(&self) -> &[R] {
        &self.coefficients
    }
}

impl<R: Ring> IntoIterator for UnivariatePolynomial<R> {
    type Item = R;
    type IntoIter = alloc::vec::IntoIter<R>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.coefficients.into_iter()
    }
}

impl<R: UnitalRing> Add for UnivariatePolynomial<R> {
    type Output = Self;

    fn add(self, rps: Self) -> Self::Output {
        debug_assert_eq!(self.coefficients.len(), rps.coefficients.len());
        Self {
            coefficients: zip(self.coefficients, rps.coefficients)
                .map(|(l, r)| l + r)
                .collect(),
        }
    }
}

impl<R: UnitalRing> AddAssign for UnivariatePolynomial<R> {
    fn add_assign(&mut self, rps: Self) {
        debug_assert_eq!(self.coefficients.len(), rps.coefficients.len());
        zip(self.coefficients.iter_mut(), rps.coefficients).for_each(|(l, r)| *l += r);
    }
}

impl<R: UnitalRing> Double for UnivariatePolynomial<R> {
    type Output = Self;

    fn double(self) -> Self::Output {
        Self {
            coefficients: self.coefficients.into_iter().map(Double::double).collect(),
        }
    }
}

impl<R: UnitalRing> Neg for UnivariatePolynomial<R> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            coefficients: self.coefficients.into_iter().map(Neg::neg).collect(),
        }
    }
}

impl<R: Ring + Absorb<R>> Absorb<R> for UnivariatePolynomial<R> {
    fn absorb_into(&self, duplex: &mut (impl Duplex<R> + ?Sized)) {
        duplex.absorb(&self.coefficients)
    }
}

impl<R: Ring + Squeeze<R>> SqueezeWithSize<R> for UnivariatePolynomial<R> {
    fn squeeze_from(duplex: &mut (impl Duplex<R> + ?Sized), size: usize) -> Self {
        duplex.squeeze_with_size::<Vec<R>>(size).into()
    }
}
