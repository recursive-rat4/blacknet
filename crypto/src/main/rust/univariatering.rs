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

use crate::algebra::{Algebra, CommutativeAlgebra, UnitalAlgebra};
use crate::convolution::Convolution;
use crate::duplex::{Absorb, Duplex, Squeeze};
use crate::magma::{AdditiveMagma, MultiplicativeMagma};
use crate::module::{FreeModule, Module};
use crate::monoid::{AdditiveMonoid, MultiplicativeMonoid};
use crate::ring::{CommutativeRing, PolynomialRing, Ring, UnitalRing};
use crate::semigroup::MultiplicativeSemigroup;
use core::fmt::{Debug, Formatter, Result};
use core::iter::{Product, Sum};
use core::marker::PhantomData;
use core::ops::{Add, AddAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign};

// Univariate polynomial ring in monomial basis

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct UnivariateRing<R: UnitalRing, const N: usize, C: Convolution<R, N>> {
    coefficients: FreeModule<R, N>,
    phantom: PhantomData<C>,
}

impl<R: UnitalRing, const N: usize, C: Convolution<R, N>> Debug for UnivariateRing<R, N, C> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self.coefficients)
    }
}

impl<R: UnitalRing, const N: usize, C: Convolution<R, N>> Default for UnivariateRing<R, N, C> {
    #[inline]
    fn default() -> Self {
        Self::ZERO
    }
}

impl<R: UnitalRing, const N: usize, C: Convolution<R, N>> From<[R; N]> for UnivariateRing<R, N, C> {
    #[inline]
    fn from(coefficients: [R; N]) -> Self {
        Self {
            coefficients: coefficients.into(),
            phantom: PhantomData,
        }
    }
}

impl<R: UnitalRing, const N: usize, C: Convolution<R, N>> From<FreeModule<R, N>>
    for UnivariateRing<R, N, C>
{
    #[inline]
    fn from(coefficients: FreeModule<R, N>) -> Self {
        Self {
            coefficients,
            phantom: PhantomData,
        }
    }
}

impl<R: UnitalRing, const N: usize, C: Convolution<R, N>> From<R> for UnivariateRing<R, N, C> {
    fn from(scalar: R) -> Self {
        let mut t = [R::ZERO; N];
        t[0] = scalar;
        Self {
            coefficients: FreeModule::<R, N>::const_new(t),
            phantom: PhantomData,
        }
    }
}

impl<R: UnitalRing, const N: usize, C: Convolution<R, N>> Index<usize> for UnivariateRing<R, N, C> {
    type Output = R;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.coefficients[index]
    }
}

impl<R: UnitalRing, const N: usize, C: Convolution<R, N>> IndexMut<usize>
    for UnivariateRing<R, N, C>
{
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.coefficients[index]
    }
}

impl<R: UnitalRing, const N: usize, C: Convolution<R, N>> IntoIterator for UnivariateRing<R, N, C> {
    type Item = R;
    type IntoIter = core::array::IntoIter<R, N>;

    fn into_iter(self) -> Self::IntoIter {
        self.coefficients.into_iter()
    }
}

impl<R: UnitalRing, const N: usize, C: Convolution<R, N>> Add for UnivariateRing<R, N, C> {
    type Output = Self;

    fn add(self, rps: Self) -> Self::Output {
        Self::from(self.coefficients + rps.coefficients)
    }
}

impl<R: UnitalRing, const N: usize, C: Convolution<R, N>> AddAssign for UnivariateRing<R, N, C> {
    #[inline]
    fn add_assign(&mut self, rps: Self) {
        *self = *self + rps
    }
}

impl<R: UnitalRing, const N: usize, C: Convolution<R, N>> Neg for UnivariateRing<R, N, C> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::from(-self.coefficients)
    }
}

impl<R: UnitalRing, const N: usize, C: Convolution<R, N>> Sub for UnivariateRing<R, N, C> {
    type Output = Self;

    fn sub(self, rps: Self) -> Self::Output {
        Self::from(self.coefficients - rps.coefficients)
    }
}

impl<R: UnitalRing, const N: usize, C: Convolution<R, N>> SubAssign for UnivariateRing<R, N, C> {
    #[inline]
    fn sub_assign(&mut self, rps: Self) {
        *self = *self - rps
    }
}

impl<R: UnitalRing, const N: usize, C: Convolution<R, N>> Mul for UnivariateRing<R, N, C> {
    type Output = Self;

    fn mul(self, rps: Self) -> Self::Output {
        Self::from(C::convolute(
            self.coefficients.components(),
            rps.coefficients.components(),
        ))
    }
}

impl<R: UnitalRing, const N: usize, C: Convolution<R, N>> MulAssign for UnivariateRing<R, N, C> {
    #[inline]
    fn mul_assign(&mut self, rps: Self) {
        *self = *self * rps
    }
}

impl<R: UnitalRing, const N: usize, C: Convolution<R, N>> Mul<R> for UnivariateRing<R, N, C> {
    type Output = Self;

    fn mul(self, rps: R) -> Self::Output {
        Self::from(self.coefficients * rps)
    }
}

impl<R: UnitalRing, const N: usize, C: Convolution<R, N>> MulAssign<R> for UnivariateRing<R, N, C> {
    #[inline]
    fn mul_assign(&mut self, rps: R) {
        *self = *self * rps
    }
}

impl<R: UnitalRing, const N: usize, C: Convolution<R, N>> Sum for UnivariateRing<R, N, C> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|lps, rps| lps + rps).unwrap_or(Self::ZERO)
    }
}

impl<R: UnitalRing, const N: usize, C: Convolution<R, N>> Product for UnivariateRing<R, N, C> {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|lps, rps| lps * rps).unwrap_or(Self::UNITY)
    }
}

impl<R: UnitalRing, const N: usize, C: Convolution<R, N>> AdditiveMagma
    for UnivariateRing<R, N, C>
{
    fn double(self) -> Self {
        Self::from(self.coefficients.double())
    }
}

impl<R: UnitalRing, const N: usize, C: Convolution<R, N>> AdditiveMonoid
    for UnivariateRing<R, N, C>
{
    const IDENTITY: Self = Self {
        coefficients: FreeModule::<R, N>::IDENTITY,
        phantom: PhantomData,
    };
}

impl<R: UnitalRing, const N: usize, C: Convolution<R, N>> MultiplicativeMagma
    for UnivariateRing<R, N, C>
{
    #[inline]
    fn square(self) -> Self {
        self * self
    }
}

impl<R: UnitalRing, const N: usize, C: Convolution<R, N>> MultiplicativeSemigroup
    for UnivariateRing<R, N, C>
{
    const LEFT_IDENTITY: Self = {
        let mut t = [R::ZERO; N];
        t[0] = R::UNITY;
        Self {
            coefficients: FreeModule::<R, N>::const_new(t),
            phantom: PhantomData,
        }
    };
    const RIGHT_IDENTITY: Self = {
        let mut t = [R::ZERO; N];
        t[0] = R::UNITY;
        Self {
            coefficients: FreeModule::<R, N>::const_new(t),
            phantom: PhantomData,
        }
    };
}

impl<R: UnitalRing, const N: usize, C: Convolution<R, N>> MultiplicativeMonoid
    for UnivariateRing<R, N, C>
{
    const IDENTITY: Self = {
        let mut t = [R::ZERO; N];
        t[0] = R::UNITY;
        Self {
            coefficients: FreeModule::<R, N>::const_new(t),
            phantom: PhantomData,
        }
    };
}

impl<R: UnitalRing, const N: usize, C: Convolution<R, N>> Module<R> for UnivariateRing<R, N, C> {}

impl<R: UnitalRing, const N: usize, C: Convolution<R, N>> Ring for UnivariateRing<R, N, C> {
    type Int = R::Int;
}

impl<R: CommutativeRing, const N: usize, C: Convolution<R, N>> CommutativeRing
    for UnivariateRing<R, N, C>
{
}

impl<R: UnitalRing, const N: usize, C: Convolution<R, N>> Algebra<R> for UnivariateRing<R, N, C> {}

impl<R: UnitalRing, const N: usize, C: Convolution<R, N>> UnitalAlgebra<R>
    for UnivariateRing<R, N, C>
{
}

impl<R: CommutativeRing, const N: usize, C: Convolution<R, N>> CommutativeAlgebra<R>
    for UnivariateRing<R, N, C>
{
}

impl<R: UnitalRing, const N: usize, C: Convolution<R, N>> PolynomialRing<R>
    for UnivariateRing<R, N, C>
{
    #[allow(refining_impl_trait_reachable)]
    #[inline]
    fn coefficients(self) -> FreeModule<R, N> {
        self.coefficients
    }

    #[inline]
    fn constant_term(self) -> R {
        self.coefficients[0]
    }

    fn evaluate(self, point: R) -> R {
        let mut sigma = self.coefficients[0];
        let mut power = point;
        for i in 1..N - 1 {
            sigma += self.coefficients[i] * power;
            power *= point;
        }
        if N > 1 {
            sigma += self.coefficients[N - 1] * power;
        }
        sigma
    }
}

impl<R: UnitalRing + Absorb<R>, const N: usize, C: Convolution<R, N>> Absorb<R>
    for UnivariateRing<R, N, C>
{
    fn absorb_into(&self, duplex: &mut (impl Duplex<R> + ?Sized)) {
        duplex.absorb(&self.coefficients)
    }
}

impl<R: UnitalRing + Squeeze<R>, const N: usize, C: Convolution<R, N>> Squeeze<R>
    for UnivariateRing<R, N, C>
{
    fn squeeze_from(duplex: &mut (impl Duplex<R> + ?Sized)) -> Self {
        duplex.squeeze::<FreeModule<R, N>>().into()
    }
}
