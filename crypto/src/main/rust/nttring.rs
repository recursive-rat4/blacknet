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
use crate::convolution::{Convolution, Negacyclic};
use crate::duplex::{Absorb, Duplex, Squeeze};
use crate::magma::{AdditiveMagma, MultiplicativeMagma};
use crate::module::{FreeModule, Module};
use crate::monoid::{AdditiveMonoid, MultiplicativeMonoid};
use crate::numbertheoretictransform::{NTTConvolution, Twiddles, cooley_tukey, gentleman_sande};
use crate::ring::{CommutativeRing, PolynomialRing, PowerOfTwoCyclotomicRing, Ring, UnitalRing};
use crate::semigroup::MultiplicativeSemigroup;
use crate::univariatering::UnivariateRing;
use core::fmt::{Debug, Formatter, Result};
use core::iter::{Product, Sum};
use core::ops::{Add, AddAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign};

// Univariate polynomial ring in NTT form

type Iso<Z, const N: usize> = UnivariateRing<Z, N, Negacyclic>;

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct NTTRing<Z: Twiddles<M>, const M: usize, const N: usize> {
    spectrum: FreeModule<Z, N>,
}

impl<Z: Twiddles<M>, const M: usize, const N: usize> NTTRing<Z, M, N> {
    const INERTIA: usize = const {
        assert!(N % M == 0);
        N / M
    };

    pub const fn const_from(scalar: Z) -> Self {
        let mut t = [Z::ZERO; N];
        let mut i = 0;
        loop {
            if i % Self::INERTIA == 0 {
                t[i] = scalar;
            } else {
                t[i] = Z::ZERO;
            }
            i += 1;
            if i == N {
                break;
            }
        }
        Self {
            spectrum: FreeModule::<Z, N>::const_new(t),
        }
    }
}

impl<Z: Twiddles<M>, const M: usize, const N: usize> Debug for NTTRing<Z, M, N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self.spectrum)
    }
}

impl<Z: Twiddles<M>, const M: usize, const N: usize> Default for NTTRing<Z, M, N> {
    #[inline]
    fn default() -> Self {
        Self::ZERO
    }
}

impl<Z: Twiddles<M>, const M: usize, const N: usize> From<[Z; N]> for NTTRing<Z, M, N> {
    fn from(coefficients: [Z; N]) -> Self {
        Self {
            spectrum: cooley_tukey(coefficients).into(),
        }
    }
}

impl<Z: Twiddles<M>, const M: usize, const N: usize> From<FreeModule<Z, N>> for NTTRing<Z, M, N> {
    fn from(coefficients: FreeModule<Z, N>) -> Self {
        Self {
            spectrum: cooley_tukey(coefficients.components()).into(),
        }
    }
}

impl<Z: Twiddles<M>, const M: usize, const N: usize> From<Z> for NTTRing<Z, M, N> {
    fn from(scalar: Z) -> Self {
        Self::const_from(scalar)
    }
}

impl<Z: Twiddles<M>, const M: usize, const N: usize> From<Iso<Z, N>> for NTTRing<Z, M, N> {
    fn from(iso: Iso<Z, N>) -> Self {
        Self::from(iso.coefficients())
    }
}

impl<Z: Twiddles<M>, const M: usize, const N: usize> From<NTTRing<Z, M, N>> for Iso<Z, N> {
    fn from(ntt: NTTRing<Z, M, N>) -> Self {
        Self::from(ntt.coefficients())
    }
}

impl<Z: Twiddles<M>, const M: usize, const N: usize> Index<usize> for NTTRing<Z, M, N> {
    type Output = Z;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.spectrum[index]
    }
}

impl<Z: Twiddles<M>, const M: usize, const N: usize> IndexMut<usize> for NTTRing<Z, M, N> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.spectrum[index]
    }
}

impl<Z: Twiddles<M>, const M: usize, const N: usize> IntoIterator for NTTRing<Z, M, N> {
    type Item = Z;
    type IntoIter = core::array::IntoIter<Z, N>;

    fn into_iter(self) -> Self::IntoIter {
        self.spectrum.into_iter()
    }
}

impl<Z: Twiddles<M>, const M: usize, const N: usize> Add for NTTRing<Z, M, N> {
    type Output = Self;

    fn add(self, rps: Self) -> Self::Output {
        Self {
            spectrum: self.spectrum + rps.spectrum,
        }
    }
}

impl<Z: Twiddles<M>, const M: usize, const N: usize> AddAssign for NTTRing<Z, M, N> {
    #[inline]
    fn add_assign(&mut self, rps: Self) {
        *self = *self + rps
    }
}

impl<Z: Twiddles<M>, const M: usize, const N: usize> Neg for NTTRing<Z, M, N> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            spectrum: -self.spectrum,
        }
    }
}

impl<Z: Twiddles<M>, const M: usize, const N: usize> Sub for NTTRing<Z, M, N> {
    type Output = Self;

    fn sub(self, rps: Self) -> Self::Output {
        Self {
            spectrum: self.spectrum - rps.spectrum,
        }
    }
}

impl<Z: Twiddles<M>, const M: usize, const N: usize> SubAssign for NTTRing<Z, M, N> {
    #[inline]
    fn sub_assign(&mut self, rps: Self) {
        *self = *self - rps
    }
}

impl<Z: Twiddles<M>, const M: usize, const N: usize> Mul for NTTRing<Z, M, N> {
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

impl<Z: Twiddles<M>, const M: usize, const N: usize> MulAssign for NTTRing<Z, M, N> {
    #[inline]
    fn mul_assign(&mut self, rps: Self) {
        *self = *self * rps
    }
}

impl<Z: Twiddles<M>, const M: usize, const N: usize> Mul<Z> for NTTRing<Z, M, N> {
    type Output = Self;

    fn mul(self, rps: Z) -> Self::Output {
        Self {
            spectrum: self.spectrum * rps,
        }
    }
}

impl<Z: Twiddles<M>, const M: usize, const N: usize> MulAssign<Z> for NTTRing<Z, M, N> {
    #[inline]
    fn mul_assign(&mut self, rps: Z) {
        *self = *self * rps
    }
}

impl<Z: Twiddles<M>, const M: usize, const N: usize> Sum for NTTRing<Z, M, N> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|lps, rps| lps + rps).unwrap_or(Self::ZERO)
    }
}

impl<Z: Twiddles<M>, const M: usize, const N: usize> Product for NTTRing<Z, M, N> {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|lps, rps| lps * rps).unwrap_or(Self::UNITY)
    }
}

impl<Z: Twiddles<M>, const M: usize, const N: usize> AdditiveMagma for NTTRing<Z, M, N> {
    fn double(self) -> Self {
        Self {
            spectrum: self.spectrum.double(),
        }
    }
}

impl<Z: Twiddles<M>, const M: usize, const N: usize> AdditiveMonoid for NTTRing<Z, M, N> {
    const IDENTITY: Self = Self {
        spectrum: FreeModule::<Z, N>::IDENTITY,
    };
}

impl<Z: Twiddles<M>, const M: usize, const N: usize> MultiplicativeMagma for NTTRing<Z, M, N> {
    fn square(self) -> Self {
        if Self::INERTIA == 1 {
            Self {
                spectrum: FreeModule::<Z, N>::from_fn(|i| self.spectrum[i].square()),
            }
        } else {
            self * self
        }
    }
}

impl<Z: Twiddles<M>, const M: usize, const N: usize> MultiplicativeSemigroup for NTTRing<Z, M, N> {
    const LEFT_IDENTITY: Self = Self::const_from(Z::UNITY);
    const RIGHT_IDENTITY: Self = Self::const_from(Z::UNITY);
}

impl<Z: Twiddles<M>, const M: usize, const N: usize> MultiplicativeMonoid for NTTRing<Z, M, N> {
    const IDENTITY: Self = Self::const_from(Z::UNITY);
}

impl<Z: Twiddles<M>, const M: usize, const N: usize> Module<Z> for NTTRing<Z, M, N> {}

impl<Z: Twiddles<M>, const M: usize, const N: usize> Ring for NTTRing<Z, M, N> {
    type Int = Z::Int;
}

impl<Z: Twiddles<M>, const M: usize, const N: usize> CommutativeRing for NTTRing<Z, M, N> {}

impl<Z: Twiddles<M>, const M: usize, const N: usize> Algebra<Z> for NTTRing<Z, M, N> {}

impl<Z: Twiddles<M>, const M: usize, const N: usize> UnitalAlgebra<Z> for NTTRing<Z, M, N> {}

impl<Z: Twiddles<M>, const M: usize, const N: usize> CommutativeAlgebra<Z> for NTTRing<Z, M, N> {}

impl<Z: Twiddles<M>, const M: usize, const N: usize> PolynomialRing<Z> for NTTRing<Z, M, N> {
    #[allow(refining_impl_trait_reachable)]
    fn coefficients(self) -> FreeModule<Z, N> {
        gentleman_sande(self.spectrum.components()).into()
    }

    #[inline]
    fn constant_term(self) -> Z {
        self.coefficients()[0]
    }

    fn evaluate(self, point: Z) -> Z {
        let iso: Iso<Z, N> = self.into();
        iso.evaluate(point)
    }
}

impl<Z: Twiddles<M>, const M: usize, const N: usize> PowerOfTwoCyclotomicRing<Z>
    for NTTRing<Z, M, N>
{
    fn conjugate(self) -> Self {
        if Self::INERTIA == 1 {
            let mut spectrum = self.spectrum;
            for i in 0..N / 2 {
                spectrum.swap(i, N - 1 - i);
            }
            Self { spectrum }
        } else {
            let iso: Iso<Z, N> = self.into();
            Self::from(iso.conjugate().coefficients())
        }
    }
}

impl<Z: Twiddles<M> + Absorb<Z>, const M: usize, const N: usize> Absorb<Z> for NTTRing<Z, M, N> {
    fn absorb_into(&self, duplex: &mut (impl Duplex<Z> + ?Sized)) {
        duplex.absorb(&self.spectrum)
    }
}

impl<Z: Twiddles<M> + Squeeze<Z>, const M: usize, const N: usize> Squeeze<Z> for NTTRing<Z, M, N> {
    fn squeeze_from(duplex: &mut (impl Duplex<Z> + ?Sized)) -> Self {
        Self {
            spectrum: duplex.squeeze::<FreeModule<Z, N>>(),
        }
    }
}
