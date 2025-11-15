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
use crate::eqextension::EqExtension;
use crate::matrixdense::MatrixDense;
use crate::operation::Double;
use crate::point::Point;
use crate::polynomial::Polynomial;
use crate::ring::UnitalRing;
use crate::vectordense::VectorDense;
use alloc::vec::Vec;
use core::iter::zip;
use core::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MultilinearExtension<R: UnitalRing> {
    coefficients: Vec<R>,
}

impl<R: UnitalRing> MultilinearExtension<R> {
    pub const fn hypercube(&self) -> &Vec<R> {
        &self.coefficients
    }

    pub const fn coefficients(&self) -> &Vec<R> {
        &self.coefficients
    }
}

impl<R: UnitalRing, const N: usize> From<[R; N]> for MultilinearExtension<R> {
    fn from(coefficients: [R; N]) -> Self {
        Self {
            coefficients: coefficients.into(),
        }
    }
}

impl<R: UnitalRing> From<Vec<R>> for MultilinearExtension<R> {
    fn from(coefficients: Vec<R>) -> Self {
        Self { coefficients }
    }
}

impl<R: UnitalRing> FromIterator<R> for MultilinearExtension<R> {
    fn from_iter<I: IntoIterator<Item = R>>(iter: I) -> Self {
        Self {
            coefficients: iter.into_iter().collect(),
        }
    }
}

impl<R: UnitalRing> From<MatrixDense<R>> for MultilinearExtension<R> {
    fn from(matrix: MatrixDense<R>) -> Self {
        Self {
            coefficients: matrix.steal(),
        }
    }
}

impl<R: UnitalRing> From<VectorDense<R>> for MultilinearExtension<R> {
    fn from(vector: VectorDense<R>) -> Self {
        Self {
            coefficients: vector.steal(),
        }
    }
}

impl<R: UnitalRing + From<u8>> Polynomial<R> for MultilinearExtension<R> {
    fn bind(&mut self, e: R) {
        let new_len = self.coefficients.len() >> 1;
        let (left, right) = self.coefficients.split_at_mut(new_len);
        zip(left, right).for_each(|(l, r)| *l += (*r - *l) * e);
        self.coefficients.truncate(new_len);
    }

    fn point(&self, point: &Point<R>) -> R {
        let basis = EqExtension::basis(point.coordinates());
        zip(self.coefficients.iter(), basis)
            .map(|(&c, b)| c * b)
            .sum()
    }

    fn hypercube_with_var<const VAL: i8>(&self) -> VectorDense<R> {
        let (left, right) = self.coefficients.split_at(self.coefficients.len() >> 1);
        match VAL {
            -2 => zip(left, right)
                .map(|(&l, &r)| l + l.double() - r.double())
                .collect::<Vec<R>>(),
            -1 => zip(left, right)
                .map(|(&l, &r)| l.double() - r)
                .collect::<Vec<R>>(),
            0 => left.to_vec(),
            1 => right.to_vec(),
            2 => zip(left, right)
                .map(|(&l, &r)| r.double() - l)
                .collect::<Vec<R>>(),
            3 => zip(left, right)
                .map(|(&l, &r)| r + r.double() - l.double())
                .collect::<Vec<R>>(),
            4 => zip(left, right)
                .map(|(&l, &r)| r.double().double() - l.double() - l)
                .collect::<Vec<R>>(),
            _ => unimplemented!("hypercube_with_var for val = {VAL}"),
        }
        .into()
    }

    fn degree(&self) -> usize {
        1
    }

    fn variables(&self) -> usize {
        self.coefficients.len().trailing_zeros() as usize
    }
}

impl<R: UnitalRing> Add for MultilinearExtension<R> {
    type Output = Self;

    fn add(self, rps: Self) -> Self::Output {
        Self {
            coefficients: zip(self.coefficients, rps.coefficients)
                .map(|(l, r)| l + r)
                .collect(),
        }
    }
}

impl<R: UnitalRing> AddAssign for MultilinearExtension<R> {
    fn add_assign(&mut self, rps: Self) {
        zip(self.coefficients.iter_mut(), rps.coefficients).for_each(|(l, r)| *l += r);
    }
}

impl<R: UnitalRing> Double for MultilinearExtension<R> {
    type Output = Self;

    fn double(self) -> Self::Output {
        Self {
            coefficients: self.coefficients.into_iter().map(Double::double).collect(),
        }
    }
}

impl<R: UnitalRing> Neg for MultilinearExtension<R> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            coefficients: self.coefficients.into_iter().map(|e| -e).collect(),
        }
    }
}

impl<R: UnitalRing> Sub for MultilinearExtension<R> {
    type Output = Self;

    fn sub(self, rps: Self) -> Self::Output {
        Self {
            coefficients: zip(self.coefficients, rps.coefficients)
                .map(|(l, r)| l - r)
                .collect(),
        }
    }
}

impl<R: UnitalRing> SubAssign for MultilinearExtension<R> {
    fn sub_assign(&mut self, rps: Self) {
        zip(self.coefficients.iter_mut(), rps.coefficients).for_each(|(l, r)| *l -= r);
    }
}

impl<R: UnitalRing> Add<R> for MultilinearExtension<R> {
    type Output = Self;

    fn add(self, rps: R) -> Self::Output {
        Self {
            coefficients: self.coefficients.into_iter().map(|l| l + rps).collect(),
        }
    }
}

impl<R: UnitalRing> AddAssign<R> for MultilinearExtension<R> {
    fn add_assign(&mut self, rps: R) {
        self.coefficients.iter_mut().for_each(|l| *l += rps);
    }
}

impl<R: UnitalRing> Sub<R> for MultilinearExtension<R> {
    type Output = Self;

    fn sub(self, rps: R) -> Self::Output {
        Self {
            coefficients: self.coefficients.into_iter().map(|l| l - rps).collect(),
        }
    }
}

impl<R: UnitalRing> SubAssign<R> for MultilinearExtension<R> {
    fn sub_assign(&mut self, rps: R) {
        self.coefficients.iter_mut().for_each(|l| *l -= rps);
    }
}

impl<R: UnitalRing> Mul<R> for MultilinearExtension<R> {
    type Output = Self;

    fn mul(self, rps: R) -> Self::Output {
        Self {
            coefficients: self.coefficients.into_iter().map(|l| l * rps).collect(),
        }
    }
}

impl<R: UnitalRing> MulAssign<R> for MultilinearExtension<R> {
    fn mul_assign(&mut self, rps: R) {
        self.coefficients.iter_mut().for_each(|l| *l *= rps);
    }
}

impl<R: UnitalRing + Absorb<R>> Absorb<R> for MultilinearExtension<R> {
    fn absorb_into(&self, duplex: &mut (impl Duplex<R> + ?Sized)) {
        duplex.absorb(&self.coefficients)
    }
}

impl<R: UnitalRing + Squeeze<R>> SqueezeWithSize<R> for MultilinearExtension<R> {
    fn squeeze_from(duplex: &mut (impl Duplex<R> + ?Sized), size: usize) -> Self {
        duplex.squeeze_with_size::<Vec<R>>(size).into()
    }
}
