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

//! Univariate interpolation.

use crate::algebra::{Inv, SemifieldOps, SemiringOps, UnitalRing};
use crate::matrix::{DenseMatrix, DenseVector};
use crate::polynomial::UnivariatePolynomial;
use alloc::vec::Vec;

pub struct Interpolator<R: UnitalRing> {
    m: DenseMatrix<R>,
}

impl<R: UnitalRing + From<i8> + Clone> Interpolator<R>
where
    for<'a> &'a R: SemifieldOps<R>,
{
    pub fn degree_1() -> Option<Self> {
        Self::sequential(R::from(0), 2)
    }

    pub fn degree_2() -> Option<Self> {
        Self::sequential(R::from(-1), 3)
    }

    pub fn degree_3() -> Option<Self> {
        Self::sequential(R::from(-1), 4)
    }

    pub fn degree_4() -> Option<Self> {
        Self::sequential(R::from(-2), 5)
    }

    pub fn degree_5() -> Option<Self> {
        Self::sequential(R::from(-2), 6)
    }
}

impl<R: UnitalRing + Clone> Interpolator<R>
where
    for<'a> &'a R: SemifieldOps<R>,
{
    pub fn sequential(start: R, length: usize) -> Option<Self> {
        debug_assert!(length >= 1);
        let mut xs = Vec::<R>::with_capacity(length);
        xs.push(start);
        for i in 1..length {
            xs.push(&xs[i - 1] + R::ONE);
        }
        Self::with_xs(&xs)
    }

    pub fn with_xs(xs: &[R]) -> Option<Self> {
        let m = DenseMatrix::<R>::vandermonde(xs);
        let m = m.inv()?;
        Some(Self { m })
    }
}

impl<R: UnitalRing + Clone> Interpolator<R>
where
    for<'a> &'a R: SemiringOps<R>,
{
    pub fn interpolate(&self, ys: &[R]) -> UnivariatePolynomial<R> {
        let y = DenseVector::from(ys);
        let coefficients: Vec<R> = (&self.m * y).into();
        coefficients.into()
    }
}
