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

use crate::algebra::{Double, Inv, RingOps, SemiringOps, Square, UnitalRing, UnitalSemiring};
use crate::branchless::BlOption;
use crate::matrix::DenseVector;
use crate::polynomial::{InBasis, Polynomial, TensorBasis};
use crate::symmetric::{Absorb, Duplexer, Squeeze, SqueezeWithSize};
use alloc::borrow::{Borrow, BorrowMut};
use alloc::vec;
use alloc::vec::Vec;
use core::iter::zip;
use core::ops::{Add, AddAssign, Deref, DerefMut, Div, Index, IndexMut, Mul, MulAssign, Neg};
#[cfg(feature = "rayon")]
use rayon::iter::IntoParallelIterator;
use serde::{Deserialize, Serialize};

/// A polynomial in one indeterminate.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct UnivariatePolynomial<R: UnitalSemiring> {
    coefficients: Vec<R>,
}

impl<R: UnitalSemiring> UnivariatePolynomial<R> {
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
}

impl<R: UnitalSemiring, const N: usize> From<[R; N]> for UnivariatePolynomial<R> {
    fn from(coefficients: [R; N]) -> Self {
        Self {
            coefficients: coefficients.into(),
        }
    }
}

impl<R: UnitalSemiring> From<Vec<R>> for UnivariatePolynomial<R> {
    #[inline]
    fn from(coefficients: Vec<R>) -> Self {
        Self { coefficients }
    }
}

impl<R: UnitalSemiring> From<UnivariatePolynomial<R>> for Vec<R> {
    #[inline]
    fn from(polynomial: UnivariatePolynomial<R>) -> Self {
        polynomial.coefficients
    }
}

impl<R: UnitalSemiring> AsRef<[R]> for UnivariatePolynomial<R> {
    #[inline]
    fn as_ref(&self) -> &[R] {
        &self.coefficients
    }
}

impl<R: UnitalSemiring> AsMut<[R]> for UnivariatePolynomial<R> {
    #[inline]
    fn as_mut(&mut self) -> &mut [R] {
        self
    }
}

impl<R: UnitalSemiring> Borrow<[R]> for UnivariatePolynomial<R> {
    #[inline]
    fn borrow(&self) -> &[R] {
        &self.coefficients
    }
}

impl<R: UnitalSemiring> BorrowMut<[R]> for UnivariatePolynomial<R> {
    #[inline]
    fn borrow_mut(&mut self) -> &mut [R] {
        &mut self.coefficients
    }
}

impl<R: UnitalSemiring> Deref for UnivariatePolynomial<R> {
    type Target = [R];

    #[inline]
    fn deref(&self) -> &[R] {
        &self.coefficients
    }
}

impl<R: UnitalSemiring> DerefMut for UnivariatePolynomial<R> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.coefficients
    }
}

impl<R: UnitalSemiring> Index<u32> for UnivariatePolynomial<R> {
    type Output = R;

    #[inline]
    fn index(&self, index: u32) -> &Self::Output {
        &self.coefficients[index as usize]
    }
}

impl<R: UnitalSemiring> IndexMut<u32> for UnivariatePolynomial<R> {
    #[inline]
    fn index_mut(&mut self, index: u32) -> &mut Self::Output {
        &mut self.coefficients[index as usize]
    }
}

impl<R: UnitalSemiring> IntoIterator for UnivariatePolynomial<R> {
    type Item = R;
    type IntoIter = alloc::vec::IntoIter<R>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.coefficients.into_iter()
    }
}

impl<'a, R: UnitalSemiring> IntoIterator for &'a UnivariatePolynomial<R> {
    type Item = &'a R;
    type IntoIter = core::slice::Iter<'a, R>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.coefficients.iter()
    }
}

impl<'a, R: UnitalSemiring> IntoIterator for &'a mut UnivariatePolynomial<R> {
    type Item = &'a mut R;
    type IntoIter = core::slice::IterMut<'a, R>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.coefficients.iter_mut()
    }
}

#[cfg(feature = "rayon")]
impl<R: UnitalSemiring + Send> IntoParallelIterator for UnivariatePolynomial<R> {
    type Item = R;
    type Iter = rayon::vec::IntoIter<R>;

    #[inline]
    fn into_par_iter(self) -> Self::Iter {
        self.coefficients.into_par_iter()
    }
}

#[cfg(feature = "rayon")]
impl<'a, R: UnitalSemiring + Sync> IntoParallelIterator for &'a UnivariatePolynomial<R> {
    type Item = &'a R;
    type Iter = rayon::slice::Iter<'a, R>;

    #[inline]
    fn into_par_iter(self) -> Self::Iter {
        (&self.coefficients).into_par_iter()
    }
}

#[cfg(feature = "rayon")]
impl<'a, R: UnitalSemiring + Send> IntoParallelIterator for &'a mut UnivariatePolynomial<R> {
    type Item = &'a mut R;
    type Iter = rayon::slice::IterMut<'a, R>;

    #[inline]
    fn into_par_iter(self) -> Self::Iter {
        (&mut self.coefficients).into_par_iter()
    }
}

impl<R: UnitalSemiring + Clone> Polynomial for UnivariatePolynomial<R>
where
    for<'a> &'a R: SemiringOps<R>,
{
    type Coefficient = R;
    type Point = R;

    fn point(&self, point: &R) -> R {
        // Horner method
        let mut coefficients = self.coefficients.iter().rev();
        let Some(mut accum) = coefficients.next().cloned() else {
            return R::ZERO;
        };
        for coefficient in coefficients {
            accum *= point;
            accum += coefficient;
        }
        accum
    }
}

/// In monomial basis.
impl<R: UnitalSemiring + Clone> InBasis for UnivariatePolynomial<R>
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
}

impl<R: UnitalSemiring + Clone> TensorBasis for UnivariatePolynomial<R>
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

impl<R: UnitalSemiring> Add for UnivariatePolynomial<R> {
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

impl<R: UnitalSemiring> AddAssign for UnivariatePolynomial<R> {
    fn add_assign(&mut self, rps: Self) {
        debug_assert_eq!(self.coefficients.len(), rps.coefficients.len());
        zip(self.coefficients.iter_mut(), rps.coefficients).for_each(|(l, r)| *l += r);
    }
}

impl<R: UnitalSemiring> Double for UnivariatePolynomial<R> {
    type Output = Self;

    fn double(self) -> Self::Output {
        Self {
            coefficients: self.coefficients.into_iter().map(Double::double).collect(),
        }
    }
}

impl<R: UnitalSemiring> Double for &UnivariatePolynomial<R>
where
    for<'a> &'a R: SemiringOps<R>,
{
    type Output = UnivariatePolynomial<R>;

    fn double(self) -> Self::Output {
        Self::Output {
            coefficients: self.coefficients.iter().map(Double::double).collect(),
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

impl<R: UnitalRing> Neg for &UnivariatePolynomial<R>
where
    for<'a> &'a R: RingOps<R>,
{
    type Output = UnivariatePolynomial<R>;

    fn neg(self) -> Self::Output {
        Self::Output {
            coefficients: self.coefficients.iter().map(Neg::neg).collect(),
        }
    }
}

impl<R: UnitalSemiring + Clone> Mul for UnivariatePolynomial<R>
where
    for<'a> &'a R: SemiringOps<R>,
{
    type Output = Self;

    fn mul(self, rps: Self) -> Self::Output {
        &self * &rps
    }
}

impl<R: UnitalSemiring + Clone> MulAssign for UnivariatePolynomial<R>
where
    for<'a> &'a R: SemiringOps<R>,
{
    fn mul_assign(&mut self, rps: Self) {
        *self = &*self * &rps
    }
}

impl<R: UnitalSemiring + Clone> Square for UnivariatePolynomial<R>
where
    for<'a> &'a R: SemiringOps<R>,
{
    type Output = Self;

    fn square(self) -> Self::Output {
        &self * &self
    }
}

impl<R: UnitalSemiring + Clone> Square for &UnivariatePolynomial<R>
where
    for<'a> &'a R: SemiringOps<R>,
{
    type Output = UnivariatePolynomial<R>;

    fn square(self) -> Self::Output {
        self * self
    }
}

impl<R: UnitalSemiring + Clone> Mul<&UnivariatePolynomial<R>> for &UnivariatePolynomial<R>
where
    for<'a> &'a R: SemiringOps<R>,
{
    type Output = UnivariatePolynomial<R>;

    fn mul(self, rps: &UnivariatePolynomial<R>) -> Self::Output {
        // Long method
        let mut coefficients = vec![R::ZERO; self.coefficients.len() + rps.coefficients.len() - 1];
        for i in 0..self.coefficients.len() {
            for j in 0..rps.coefficients.len() {
                coefficients[i + j] += &self.coefficients[i] * &rps.coefficients[j];
            }
        }
        Self::Output { coefficients }
    }
}

impl<R: UnitalSemiring> Mul<R> for UnivariatePolynomial<R>
where
    for<'a> &'a R: SemiringOps<R>,
{
    type Output = Self;

    fn mul(self, rps: R) -> Self::Output {
        self * &rps
    }
}

impl<R: UnitalSemiring> Mul<&R> for UnivariatePolynomial<R>
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

impl<R: UnitalSemiring + Inv<Output = BlOption<R>>> Div<R> for UnivariatePolynomial<R>
where
    for<'a> &'a R: SemiringOps<R>,
{
    type Output = BlOption<Self>;

    fn div(self, rps: R) -> Self::Output {
        rps.inv().map(|v| self * v)
    }
}

impl<Msg, R: UnitalSemiring + Absorb<Msg>> Absorb<Msg> for UnivariatePolynomial<R> {
    fn absorb_into<D: Duplexer<Msg = Msg>>(self, duplex: &mut D) {
        duplex.absorb_iter(self.coefficients)
    }
}

impl<Msg, R: UnitalSemiring + Absorb<Msg> + Clone> Absorb<Msg> for &UnivariatePolynomial<R> {
    fn absorb_into<D: Duplexer<Msg = Msg>>(self, duplex: &mut D) {
        duplex.absorb_iter(self.coefficients.iter().cloned())
    }
}

impl<Msg, R: UnitalSemiring + Squeeze<Msg>> SqueezeWithSize<Msg> for UnivariatePolynomial<R> {
    fn squeeze_from<D: Duplexer<Msg = Msg>>(duplex: &mut D, size: usize) -> Self {
        Self {
            coefficients: (0..size).map(|_| duplex.squeeze::<R>()).collect(),
        }
    }
}
