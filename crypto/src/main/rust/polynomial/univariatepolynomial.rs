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

use crate::algebra::{DivisionRing, Double, Semiring, SemiringOps, Set, Square, UnitalRing};
use crate::duplex::{Absorb, Duplex, Squeeze, SqueezeWithSize};
use crate::matrix::DenseVector;
use crate::polynomial::{InBasis, Polynomial, TensorBasis};
use alloc::borrow::{Borrow, BorrowMut};
use alloc::vec::Vec;
use core::iter::zip;
use core::ops::{Add, AddAssign, Deref, DerefMut, Div, Index, IndexMut, Mul, MulAssign, Neg};
use serde::{Deserialize, Serialize};

/// A polynomial in one indeterminate.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct UnivariatePolynomial<R: Semiring> {
    coefficients: Vec<R>,
}

impl<R: Semiring> UnivariatePolynomial<R> {
    pub fn at_0_plus_1(&self) -> R
    where
        for<'a> &'a R: SemiringOps<R>,
    {
        match self.coefficients.len() {
            0 => R::ZERO,
            1 => (&self.coefficients[0]).double(),
            _ => (&self.coefficients[0]).double() + self.coefficients.iter().skip(1).sum::<R>(),
        }
    }

    pub const fn degree(&self) -> usize {
        self.coefficients.len() - 1
    }

    pub const fn variables(&self) -> usize {
        1
    }
}

impl<R: Semiring, const N: usize> From<[R; N]> for UnivariatePolynomial<R> {
    fn from(coefficients: [R; N]) -> Self {
        Self {
            coefficients: coefficients.into(),
        }
    }
}

impl<R: Semiring> From<Vec<R>> for UnivariatePolynomial<R> {
    #[inline]
    fn from(coefficients: Vec<R>) -> Self {
        Self { coefficients }
    }
}

impl<R: Semiring> From<UnivariatePolynomial<R>> for Vec<R> {
    #[inline]
    fn from(polynomial: UnivariatePolynomial<R>) -> Self {
        polynomial.coefficients
    }
}

impl<R: Semiring> AsRef<[R]> for UnivariatePolynomial<R> {
    #[inline]
    fn as_ref(&self) -> &[R] {
        &self.coefficients
    }
}

impl<R: Semiring> AsMut<[R]> for UnivariatePolynomial<R> {
    #[inline]
    fn as_mut(&mut self) -> &mut [R] {
        self
    }
}

impl<R: Semiring> Borrow<[R]> for UnivariatePolynomial<R> {
    #[inline]
    fn borrow(&self) -> &[R] {
        &self.coefficients
    }
}

impl<R: Semiring> BorrowMut<[R]> for UnivariatePolynomial<R> {
    #[inline]
    fn borrow_mut(&mut self) -> &mut [R] {
        &mut self.coefficients
    }
}

impl<R: Semiring> Deref for UnivariatePolynomial<R> {
    type Target = [R];

    #[inline]
    fn deref(&self) -> &[R] {
        &self.coefficients
    }
}

impl<R: Semiring> DerefMut for UnivariatePolynomial<R> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.coefficients
    }
}

impl<R: Semiring> Index<usize> for UnivariatePolynomial<R> {
    type Output = R;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.coefficients[index]
    }
}

impl<R: Semiring> IndexMut<usize> for UnivariatePolynomial<R> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.coefficients[index]
    }
}

impl<R: Semiring> IntoIterator for UnivariatePolynomial<R> {
    type Item = R;
    type IntoIter = alloc::vec::IntoIter<R>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.coefficients.into_iter()
    }
}

impl<'a, R: Semiring> IntoIterator for &'a UnivariatePolynomial<R> {
    type Item = &'a R;
    type IntoIter = core::slice::Iter<'a, R>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.coefficients.iter()
    }
}

impl<R: Semiring> Polynomial for UnivariatePolynomial<R>
where
    for<'a> &'a R: SemiringOps<R>,
{
    type Coefficient = R;
    type Point = R;

    fn point(&self, point: &R) -> R {
        if self.coefficients.is_empty() {
            return R::ZERO;
        }
        let mut sigma = self.coefficients[0].clone();
        let mut power = point.clone();
        for i in 1..self.coefficients.len() - 1 {
            sigma += &self.coefficients[i] * &power;
            power *= point;
        }
        if self.coefficients.len() > 1 {
            sigma += &self.coefficients[self.coefficients.len() - 1] * power;
        }
        sigma
    }
}

/// In monomial basis.
impl<R: Semiring> InBasis for UnivariatePolynomial<R>
where
    for<'a> &'a R: SemiringOps<R>,
{
    fn basis(&self, point: &R) -> DenseVector<R> {
        let n = self.coefficients.len();
        let mut powers = Vec::<R>::with_capacity(n);
        if n == 0 {
            return powers.into();
        }
        powers.push(R::ONE);
        if n == 1 {
            return powers.into();
        }
        let point = point.clone();
        powers.push(point.clone());
        let mut power = point.clone();
        for _ in 2..n {
            power *= &point;
            powers.push(power.clone());
        }
        powers.into()
    }

    fn coefficients(&self) -> &[R] {
        &self.coefficients
    }
}

impl<R: Semiring> TensorBasis for UnivariatePolynomial<R>
where
    for<'a> &'a R: SemiringOps<R>,
{
    fn tensor_basis(&self, point: &R) -> (DenseVector<R>, DenseVector<R>) {
        let n = self.coefficients.len().isqrt();
        debug_assert!(self.coefficients.len() == n * n);
        debug_assert!(n > 1);
        let mut point = point.clone();

        let mut power = point.clone();
        let mut right = Vec::<R>::with_capacity(n);
        right.push(R::ONE);
        right.push(point.clone());
        for _ in 2..n {
            power *= &point;
            right.push(power.clone());
        }

        point *= power;
        power = point.clone();

        let mut left = Vec::<R>::with_capacity(n);
        left.push(R::ONE);
        left.push(point.clone());
        for _ in 2..n {
            power *= &point;
            left.push(power.clone());
        }

        (left.into(), right.into())
    }
}

impl<R: Semiring> Add for UnivariatePolynomial<R> {
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

impl<R: Semiring> AddAssign for UnivariatePolynomial<R> {
    fn add_assign(&mut self, rps: Self) {
        debug_assert_eq!(self.coefficients.len(), rps.coefficients.len());
        zip(self.coefficients.iter_mut(), rps.coefficients).for_each(|(l, r)| *l += r);
    }
}

impl<R: Semiring> Double for UnivariatePolynomial<R> {
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

impl<R: Semiring> Mul for UnivariatePolynomial<R>
where
    for<'a> &'a R: SemiringOps<R>,
{
    type Output = Self;

    fn mul(self, rps: Self) -> Self::Output {
        &self * &rps
    }
}

impl<R: Semiring> MulAssign for UnivariatePolynomial<R>
where
    for<'a> &'a R: SemiringOps<R>,
{
    fn mul_assign(&mut self, rps: Self) {
        *self = &*self * &rps
    }
}

impl<R: Semiring> Square for UnivariatePolynomial<R>
where
    for<'a> &'a R: SemiringOps<R>,
{
    type Output = Self;

    fn square(self) -> Self::Output {
        &self * &self
    }
}

impl<R: Semiring> Square for &UnivariatePolynomial<R>
where
    for<'a> &'a R: SemiringOps<R>,
{
    type Output = UnivariatePolynomial<R>;

    fn square(self) -> Self::Output {
        self * self
    }
}

impl<R: Semiring> Mul<&UnivariatePolynomial<R>> for &UnivariatePolynomial<R>
where
    for<'a> &'a R: SemiringOps<R>,
{
    type Output = UnivariatePolynomial<R>;

    fn mul(self, rps: &UnivariatePolynomial<R>) -> Self::Output {
        // Long method
        let mut coefficients = Vec::new();
        coefficients.resize(
            self.coefficients.len() + rps.coefficients.len() - 1,
            R::ZERO,
        );
        for i in 0..self.coefficients.len() {
            for j in 0..rps.coefficients.len() {
                coefficients[i + j] += &self.coefficients[i] * &rps.coefficients[j];
            }
        }
        Self::Output { coefficients }
    }
}

impl<R: Semiring> Mul<R> for UnivariatePolynomial<R>
where
    for<'a> &'a R: SemiringOps<R>,
{
    type Output = Self;

    fn mul(self, rps: R) -> Self::Output {
        self * &rps
    }
}

impl<R: Semiring> Mul<&R> for UnivariatePolynomial<R>
where
    for<'a> &'a R: SemiringOps<R>,
{
    type Output = Self;

    fn mul(self, rps: &R) -> Self::Output {
        Self {
            coefficients: self.coefficients.into_iter().map(|l| l * rps).collect(),
        }
    }
}

impl<R: UnitalRing + DivisionRing> Div<R> for UnivariatePolynomial<R>
where
    for<'a> &'a R: SemiringOps<R>,
{
    type Output = Option<Self>;

    fn div(self, rps: R) -> Self::Output {
        rps.inv().map(|v| self * v)
    }
}

impl<S: Set, R: Semiring + Absorb<S>> Absorb<S> for UnivariatePolynomial<R> {
    fn absorb_into(self, duplex: &mut (impl Duplex<S> + ?Sized)) {
        duplex.absorb_iter(self.coefficients.into_iter())
    }
}

impl<S: Set, R: Semiring + Absorb<S>> Absorb<S> for &UnivariatePolynomial<R> {
    fn absorb_into(self, duplex: &mut (impl Duplex<S> + ?Sized)) {
        duplex.absorb_iter(self.coefficients.iter().cloned())
    }
}

impl<S: Set, R: Semiring + Squeeze<S>> SqueezeWithSize<S> for UnivariatePolynomial<R> {
    fn squeeze_from(duplex: &mut (impl Duplex<S> + ?Sized), size: usize) -> Self {
        (0..size)
            .map(|_| duplex.squeeze::<R>())
            .collect::<Vec<R>>()
            .into()
    }
}
