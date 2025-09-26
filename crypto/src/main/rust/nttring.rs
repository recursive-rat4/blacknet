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

#![allow(clippy::manual_is_multiple_of)]

use crate::algebra::{Algebra, CommutativeAlgebra, UnitalAlgebra};
use crate::convolution::Convolution;
use crate::duplex::{Absorb, Duplex, Squeeze};
use crate::magma::{AdditiveMagma, MultiplicativeMagma};
use crate::module::{FreeModule, Module};
use crate::monoid::{AdditiveMonoid, MultiplicativeMonoid};
use crate::numbertheoretictransform::{NTTConvolution, Twiddles, cooley_tukey, gentleman_sande};
use crate::ring::{CommutativeRing, PolynomialRing, Ring, UnitalRing};
use crate::semigroup::MultiplicativeSemigroup;
use core::fmt::{Debug, Formatter, Result};
use core::iter::{Product, Sum};
use core::ops::{Add, AddAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign};

// Univariate polynomial ring in NTT form

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct NTTRing<R: Twiddles<M>, const M: usize, const N: usize> {
    spectrum: FreeModule<R, N>,
}

impl<R: Twiddles<M>, const M: usize, const N: usize> NTTRing<R, M, N> {
    const INERTIA: usize = const {
        assert!(N % M == 0);
        N / M
    };

    pub const fn const_from(scalar: R) -> Self {
        let mut t = [R::ZERO; N];
        let mut i = 0;
        loop {
            if i % Self::INERTIA == 0 {
                t[i] = scalar;
            } else {
                t[i] = R::ZERO;
            }
            i += 1;
            if i == N {
                break;
            }
        }
        Self {
            spectrum: FreeModule::<R, N>::const_new(t),
        }
    }
}

impl<R: Twiddles<M>, const M: usize, const N: usize> Debug for NTTRing<R, M, N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self.spectrum)
    }
}

impl<R: Twiddles<M>, const M: usize, const N: usize> Default for NTTRing<R, M, N> {
    #[inline]
    fn default() -> Self {
        Self::ZERO
    }
}

impl<R: Twiddles<M>, const M: usize, const N: usize> From<[R; N]> for NTTRing<R, M, N> {
    #[inline]
    fn from(coefficients: [R; N]) -> Self {
        Self {
            spectrum: cooley_tukey(coefficients).into(),
        }
    }
}

impl<R: Twiddles<M>, const M: usize, const N: usize> From<FreeModule<R, N>> for NTTRing<R, M, N> {
    #[inline]
    fn from(coefficients: FreeModule<R, N>) -> Self {
        Self {
            spectrum: cooley_tukey(coefficients.components()).into(),
        }
    }
}

impl<R: Twiddles<M>, const M: usize, const N: usize> From<R> for NTTRing<R, M, N> {
    fn from(scalar: R) -> Self {
        Self::const_from(scalar)
    }
}

impl<R: Twiddles<M>, const M: usize, const N: usize> Index<usize> for NTTRing<R, M, N> {
    type Output = R;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.spectrum[index]
    }
}

impl<R: Twiddles<M>, const M: usize, const N: usize> IndexMut<usize> for NTTRing<R, M, N> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.spectrum[index]
    }
}

impl<R: Twiddles<M>, const M: usize, const N: usize> IntoIterator for NTTRing<R, M, N> {
    type Item = R;
    type IntoIter = core::array::IntoIter<R, N>;

    fn into_iter(self) -> Self::IntoIter {
        self.spectrum.into_iter()
    }
}

impl<R: Twiddles<M>, const M: usize, const N: usize> Add for NTTRing<R, M, N> {
    type Output = Self;

    fn add(self, rps: Self) -> Self::Output {
        Self {
            spectrum: self.spectrum + rps.spectrum,
        }
    }
}

impl<R: Twiddles<M>, const M: usize, const N: usize> AddAssign for NTTRing<R, M, N> {
    #[inline]
    fn add_assign(&mut self, rps: Self) {
        *self = *self + rps
    }
}

impl<R: Twiddles<M>, const M: usize, const N: usize> Neg for NTTRing<R, M, N> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            spectrum: -self.spectrum,
        }
    }
}

impl<R: Twiddles<M>, const M: usize, const N: usize> Sub for NTTRing<R, M, N> {
    type Output = Self;

    fn sub(self, rps: Self) -> Self::Output {
        Self {
            spectrum: self.spectrum - rps.spectrum,
        }
    }
}

impl<R: Twiddles<M>, const M: usize, const N: usize> SubAssign for NTTRing<R, M, N> {
    #[inline]
    fn sub_assign(&mut self, rps: Self) {
        *self = *self - rps
    }
}

impl<R: Twiddles<M>, const M: usize, const N: usize> Mul for NTTRing<R, M, N> {
    type Output = Self;

    fn mul(self, rps: Self) -> Self::Output {
        Self {
            spectrum: FreeModule::from(NTTConvolution::<M, N>::convolute(
                self.spectrum.components(),
                rps.spectrum.components(),
            )),
        }
    }
}

impl<R: Twiddles<M>, const M: usize, const N: usize> MulAssign for NTTRing<R, M, N> {
    #[inline]
    fn mul_assign(&mut self, rps: Self) {
        *self = *self * rps
    }
}

impl<R: Twiddles<M>, const M: usize, const N: usize> Mul<R> for NTTRing<R, M, N> {
    type Output = Self;

    fn mul(self, rps: R) -> Self::Output {
        Self {
            spectrum: self.spectrum * rps,
        }
    }
}

impl<R: Twiddles<M>, const M: usize, const N: usize> MulAssign<R> for NTTRing<R, M, N> {
    #[inline]
    fn mul_assign(&mut self, rps: R) {
        *self = *self * rps
    }
}

impl<R: Twiddles<M>, const M: usize, const N: usize> Sum for NTTRing<R, M, N> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|lps, rps| lps + rps).unwrap_or(Self::ZERO)
    }
}

impl<R: Twiddles<M>, const M: usize, const N: usize> Product for NTTRing<R, M, N> {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|lps, rps| lps * rps).unwrap_or(Self::UNITY)
    }
}

impl<R: Twiddles<M>, const M: usize, const N: usize> AdditiveMagma for NTTRing<R, M, N> {
    fn double(self) -> Self {
        Self {
            spectrum: self.spectrum.double(),
        }
    }
}

impl<R: Twiddles<M>, const M: usize, const N: usize> AdditiveMonoid for NTTRing<R, M, N> {
    const IDENTITY: Self = Self {
        spectrum: FreeModule::<R, N>::IDENTITY,
    };
}

impl<R: Twiddles<M>, const M: usize, const N: usize> MultiplicativeMagma for NTTRing<R, M, N> {
    fn square(self) -> Self {
        if Self::INERTIA == 1 {
            Self {
                spectrum: FreeModule::<R, N>::from_fn(|i| self.spectrum[i].square()),
            }
        } else {
            self * self
        }
    }
}

impl<R: Twiddles<M>, const M: usize, const N: usize> MultiplicativeSemigroup for NTTRing<R, M, N> {
    const LEFT_IDENTITY: Self = Self::const_from(R::UNITY);
    const RIGHT_IDENTITY: Self = Self::const_from(R::UNITY);
}

impl<R: Twiddles<M>, const M: usize, const N: usize> MultiplicativeMonoid for NTTRing<R, M, N> {
    const IDENTITY: Self = Self::const_from(R::UNITY);
}

impl<R: Twiddles<M>, const M: usize, const N: usize> Module<R> for NTTRing<R, M, N> {}

impl<R: Twiddles<M>, const M: usize, const N: usize> Ring for NTTRing<R, M, N> {
    type Int = R::Int;
}

impl<R: Twiddles<M> + CommutativeRing, const M: usize, const N: usize> CommutativeRing
    for NTTRing<R, M, N>
{
}

impl<R: Twiddles<M>, const M: usize, const N: usize> Algebra<R> for NTTRing<R, M, N> {}

impl<R: Twiddles<M>, const M: usize, const N: usize> UnitalAlgebra<R> for NTTRing<R, M, N> {}

impl<R: Twiddles<M> + CommutativeRing, const M: usize, const N: usize> CommutativeAlgebra<R>
    for NTTRing<R, M, N>
{
}

impl<R: Twiddles<M>, const M: usize, const N: usize> PolynomialRing<R> for NTTRing<R, M, N> {
    #[allow(refining_impl_trait_reachable)]
    fn coefficients(self) -> FreeModule<R, N> {
        gentleman_sande(self.spectrum.components()).into()
    }

    #[inline]
    fn constant_term(self) -> R {
        self.coefficients()[0]
    }

    fn evaluate(self, point: R) -> R {
        let coefficients = self.coefficients();
        let mut sigma = coefficients[0];
        let mut power = point;
        for i in 1..N - 1 {
            sigma += coefficients[i] * power;
            power *= point;
        }
        if N > 1 {
            sigma += coefficients[N - 1] * power;
        }
        sigma
    }
}

impl<R: Twiddles<M> + Absorb<R>, const M: usize, const N: usize> Absorb<R> for NTTRing<R, M, N> {
    fn absorb_into(&self, duplex: &mut (impl Duplex<R> + ?Sized)) {
        duplex.absorb(&self.spectrum)
    }
}

impl<R: Twiddles<M> + Squeeze<R>, const M: usize, const N: usize> Squeeze<R> for NTTRing<R, M, N> {
    fn squeeze_from(duplex: &mut (impl Duplex<R> + ?Sized)) -> Self {
        Self {
            spectrum: duplex.squeeze::<FreeModule<R, N>>(),
        }
    }
}
