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

use crate::convolution::Convolution;
use crate::densematrix::DenseMatrix;
use crate::densevector::DenseVector;
use crate::float::FloatOn;
use crate::freemodule::FreeModule;
use crate::integer::Integer;
use crate::matrixring::MatrixRing;
use crate::nttring::NTTRing;
use crate::numbertheoretictransform::Twiddles;
use crate::ring::{IntegerRing, PolynomialRing, Ring, UnitalRing};
use crate::sparsevector::SparseVector;
use crate::univariatering::UnivariateRing;

pub trait EuclideanNorm {
    fn euclidean_norm(&self) -> f64;
}

impl<Z: IntegerRing<Int: FloatOn<f64>>> EuclideanNorm for Z {
    fn euclidean_norm(&self) -> f64 {
        self.absolute().float_on()
    }
}

//RUST currently requires std for sqrt

#[cfg(feature = "std")]
impl<R: Ring + EuclideanNorm, const N: usize> EuclideanNorm for FreeModule<R, N> {
    fn euclidean_norm(&self) -> f64 {
        self.into_iter()
            .map(|i| i.euclidean_norm())
            .map(|i| i * i)
            .sum::<f64>()
            .sqrt()
    }
}

#[cfg(feature = "std")]
impl<Z: Twiddles<M> + EuclideanNorm, const M: usize, const N: usize> EuclideanNorm
    for NTTRing<Z, M, N>
{
    fn euclidean_norm(&self) -> f64 {
        self.coefficients().euclidean_norm()
    }
}

#[cfg(feature = "std")]
impl<R: Ring + EuclideanNorm> EuclideanNorm for DenseVector<R> {
    fn euclidean_norm(&self) -> f64 {
        self.elements()
            .iter()
            .map(|i| i.euclidean_norm())
            .map(|i| i * i)
            .sum::<f64>()
            .sqrt()
    }
}

#[cfg(feature = "std")]
impl<R: Ring + EuclideanNorm> EuclideanNorm for SparseVector<R> {
    fn euclidean_norm(&self) -> f64 {
        self.elements()
            .iter()
            .map(|i| i.euclidean_norm())
            .map(|i| i * i)
            .sum::<f64>()
            .sqrt()
    }
}

#[cfg(feature = "std")]
impl<R: UnitalRing + EuclideanNorm, const N: usize, C: Convolution<R, N>> EuclideanNorm
    for UnivariateRing<R, N, C>
{
    fn euclidean_norm(&self) -> f64 {
        self.coefficients().euclidean_norm()
    }
}

pub trait InfinityNorm<N: Integer> {
    fn check_infinity_norm(&self, bound: N) -> bool;
}

impl<Z: IntegerRing> InfinityNorm<Z::Int> for Z {
    fn check_infinity_norm(&self, bound: Z::Int) -> bool {
        self.absolute() < bound
    }
}

impl<R: Ring + InfinityNorm<R::Int>, const N: usize> InfinityNorm<R::Int> for FreeModule<R, N> {
    fn check_infinity_norm(&self, bound: R::Int) -> bool {
        self.components()
            .iter()
            .all(|i| i.check_infinity_norm(bound))
    }
}

impl<R: Ring + InfinityNorm<R::Int>> InfinityNorm<R::Int> for DenseMatrix<R> {
    fn check_infinity_norm(&self, bound: R::Int) -> bool {
        self.elements().iter().all(|i| i.check_infinity_norm(bound))
    }
}

impl<R: Ring + InfinityNorm<R::Int>, const N: usize, const NN: usize> InfinityNorm<R::Int>
    for MatrixRing<R, N, NN>
{
    fn check_infinity_norm(&self, bound: R::Int) -> bool {
        self.elements().iter().all(|i| i.check_infinity_norm(bound))
    }
}

impl<Z: Twiddles<M> + InfinityNorm<Z::Int>, const M: usize, const N: usize> InfinityNorm<Z::Int>
    for NTTRing<Z, M, N>
{
    fn check_infinity_norm(&self, bound: Z::Int) -> bool {
        self.coefficients().check_infinity_norm(bound)
    }
}

impl<R: Ring + InfinityNorm<R::Int>> InfinityNorm<R::Int> for DenseVector<R> {
    fn check_infinity_norm(&self, bound: R::Int) -> bool {
        self.elements().iter().all(|i| i.check_infinity_norm(bound))
    }
}

impl<R: Ring + InfinityNorm<R::Int>> InfinityNorm<R::Int> for SparseVector<R> {
    fn check_infinity_norm(&self, bound: R::Int) -> bool {
        self.elements().iter().all(|i| i.check_infinity_norm(bound))
    }
}

impl<R: UnitalRing + InfinityNorm<R::Int>, const N: usize, C: Convolution<R, N>>
    InfinityNorm<R::Int> for UnivariateRing<R, N, C>
{
    fn check_infinity_norm(&self, bound: R::Int) -> bool {
        self.coefficients().check_infinity_norm(bound)
    }
}
