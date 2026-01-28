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

use crate::algebra::{
    FreeModule, IntegerRing, MatrixRing, NTTRing, PolynomialRing, Ring, UnitalRing, UnivariateRing,
};
use crate::convolution::Convolution;
use crate::float::FloatOn;
use crate::matrix::{DenseMatrix, DenseVector, SparseMatrix, SparseVector};
use crate::numbertheoretictransform::Twiddles;
use core::marker::PhantomData;

pub enum L2 {}
pub enum LInf {}

pub struct NormBound<Lp, Length> {
    bound: Length,
    phantom: PhantomData<Lp>,
}

impl NormBound<L2, f64> {
    pub const fn new(bound: f64) -> Self {
        Self {
            bound,
            phantom: PhantomData,
        }
    }

    pub fn check<Object: EuclideanNorm>(&self, object: &Object) -> bool {
        object.euclidean_norm() < self.bound
    }
}

impl<Length: Ord> NormBound<LInf, Length> {
    pub const fn new(bound: Length) -> Self {
        Self {
            bound,
            phantom: PhantomData,
        }
    }

    pub fn check<Object: InfinityNorm<Length>>(&self, object: &Object) -> bool {
        object.check_infinity_norm(&self.bound)
    }
}

pub trait EuclideanNorm {
    fn euclidean_norm(&self) -> f64;
}

impl<Z: IntegerRing<Int: FloatOn<f64>>> EuclideanNorm for Z {
    fn euclidean_norm(&self) -> f64 {
        self.absolute().float_on()
    }
}

//RUST currently requires std for sqrt, https://github.com/rust-lang/rust/issues/137578

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

pub trait InfinityNorm<Length: Ord> {
    fn check_infinity_norm(&self, bound: &Length) -> bool;

    fn infinity_norm(&self) -> Length
    where
        Length: Default;
}

impl<Z: IntegerRing> InfinityNorm<Z::Int> for Z {
    fn check_infinity_norm(&self, bound: &Z::Int) -> bool {
        self.absolute() < *bound
    }

    fn infinity_norm(&self) -> Z::Int {
        self.absolute()
    }
}

impl<Length: Ord, R: Ring + InfinityNorm<Length>, const N: usize> InfinityNorm<Length>
    for FreeModule<R, N>
{
    fn check_infinity_norm(&self, bound: &Length) -> bool {
        self.components()
            .iter()
            .all(|i| i.check_infinity_norm(bound))
    }

    fn infinity_norm(&self) -> Length
    where
        Length: Default,
    {
        self.components()
            .iter()
            .map(InfinityNorm::infinity_norm)
            .max()
            .unwrap_or_default()
    }
}

impl<Length: Ord, R: Ring + InfinityNorm<Length>> InfinityNorm<Length> for DenseMatrix<R> {
    fn check_infinity_norm(&self, bound: &Length) -> bool {
        self.elements().iter().all(|i| i.check_infinity_norm(bound))
    }

    fn infinity_norm(&self) -> Length
    where
        Length: Default,
    {
        self.elements()
            .iter()
            .map(InfinityNorm::infinity_norm)
            .max()
            .unwrap_or_default()
    }
}

impl<Length: Ord, R: Ring + InfinityNorm<Length>> InfinityNorm<Length> for SparseMatrix<R> {
    fn check_infinity_norm(&self, bound: &Length) -> bool {
        self.elements().iter().all(|i| i.check_infinity_norm(bound))
    }

    fn infinity_norm(&self) -> Length
    where
        Length: Default,
    {
        self.elements()
            .iter()
            .map(InfinityNorm::infinity_norm)
            .max()
            .unwrap_or_default()
    }
}

impl<Length: Ord, R: Ring + InfinityNorm<Length>, const N: usize, const NN: usize>
    InfinityNorm<Length> for MatrixRing<R, N, NN>
{
    fn check_infinity_norm(&self, bound: &Length) -> bool {
        self.elements().iter().all(|i| i.check_infinity_norm(bound))
    }

    fn infinity_norm(&self) -> Length
    where
        Length: Default,
    {
        self.elements()
            .iter()
            .map(InfinityNorm::infinity_norm)
            .max()
            .unwrap_or_default()
    }
}

impl<Z: Twiddles<M> + InfinityNorm<Z::Int>, const M: usize, const N: usize> InfinityNorm<Z::Int>
    for NTTRing<Z, M, N>
{
    fn check_infinity_norm(&self, bound: &Z::Int) -> bool {
        self.coefficients().check_infinity_norm(bound)
    }

    fn infinity_norm(&self) -> Z::Int {
        self.coefficients().infinity_norm()
    }
}

impl<Length: Ord, R: Ring + InfinityNorm<Length>> InfinityNorm<Length> for DenseVector<R> {
    fn check_infinity_norm(&self, bound: &Length) -> bool {
        self.elements().iter().all(|i| i.check_infinity_norm(bound))
    }

    fn infinity_norm(&self) -> Length
    where
        Length: Default,
    {
        self.elements()
            .iter()
            .map(InfinityNorm::infinity_norm)
            .max()
            .unwrap_or_default()
    }
}

impl<Length: Ord, R: Ring + InfinityNorm<Length>> InfinityNorm<Length> for SparseVector<R> {
    fn check_infinity_norm(&self, bound: &Length) -> bool {
        self.elements().iter().all(|i| i.check_infinity_norm(bound))
    }

    fn infinity_norm(&self) -> Length
    where
        Length: Default,
    {
        self.elements()
            .iter()
            .map(InfinityNorm::infinity_norm)
            .max()
            .unwrap_or_default()
    }
}

impl<Length: Ord, R: UnitalRing + InfinityNorm<Length>, const N: usize, C: Convolution<R, N>>
    InfinityNorm<Length> for UnivariateRing<R, N, C>
{
    fn check_infinity_norm(&self, bound: &Length) -> bool {
        self.coefficients().check_infinity_norm(bound)
    }

    fn infinity_norm(&self) -> Length
    where
        Length: Default,
    {
        self.coefficients().infinity_norm()
    }
}
