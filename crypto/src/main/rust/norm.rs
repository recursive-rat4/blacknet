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

//! Generalized length.

use crate::algebra::{
    IntegerModRing, MatrixSpace, QuaternionAlgebra, Ring, UnitalRing, UnivariateRing, VectorRing,
};
use crate::convolution::Convolution;
use crate::float::Cast;
use crate::matrix::{DenseMatrix, DenseVector, SparseMatrix, SparseVector};
use core::marker::PhantomData;

/// ℓ₂ space.
pub enum L2 {}
/// ℓ-∞ space.
pub enum LInf {}

/// ℓₚ norm bound.
pub struct NormBound<Lp, Length> {
    bound: Length,
    phantom: PhantomData<Lp>,
}

impl NormBound<L2, f64> {
    /// Construct a Euclidean norm bound.
    pub const fn new(bound: f64) -> Self {
        Self {
            bound,
            phantom: PhantomData,
        }
    }

    /// Check whether the norm of an object is less than bound.
    pub fn check<Object: EuclideanNorm>(&self, object: &Object) -> bool {
        object.euclidean_norm() < self.bound
    }
}

impl<Length: Ord> NormBound<LInf, Length> {
    /// Construct an infinity norm bound.
    pub const fn new(bound: Length) -> Self {
        Self {
            bound,
            phantom: PhantomData,
        }
    }

    /// Check whether the norm of an object is less than bound.
    pub fn check<Object: InfinityNorm<Length>>(&self, object: &Object) -> bool {
        object.check_infinity_norm(&self.bound)
    }
}

/// ℓ₂ norm.
pub trait EuclideanNorm {
    /// Compute Euclidean norm.
    fn euclidean_norm(&self) -> f64;
}

impl<Z: IntegerModRing<Int: Cast<f64>>> EuclideanNorm for Z {
    fn euclidean_norm(&self) -> f64 {
        self.absolute().cast()
    }
}

//RUST https://github.com/rust-lang/rust/issues/137578

impl<R: Ring + EuclideanNorm, const N: usize> EuclideanNorm for VectorRing<R, N> {
    fn euclidean_norm(&self) -> f64 {
        libm::sqrt(
            self.into_iter()
                .map(EuclideanNorm::euclidean_norm)
                .map(|i| i * i)
                .sum::<f64>(),
        )
    }
}

impl<R: Ring + EuclideanNorm> EuclideanNorm for DenseVector<R> {
    fn euclidean_norm(&self) -> f64 {
        libm::sqrt(
            self.iter()
                .map(EuclideanNorm::euclidean_norm)
                .map(|i| i * i)
                .sum::<f64>(),
        )
    }
}

impl<R: Ring + EuclideanNorm> EuclideanNorm for SparseVector<R> {
    fn euclidean_norm(&self) -> f64 {
        libm::sqrt(
            self.as_ref()
                .iter()
                .map(EuclideanNorm::euclidean_norm)
                .map(|i| i * i)
                .sum::<f64>(),
        )
    }
}

impl<R: UnitalRing + EuclideanNorm> EuclideanNorm for QuaternionAlgebra<R> {
    fn euclidean_norm(&self) -> f64 {
        libm::sqrt(
            self.as_ref()
                .iter()
                .map(EuclideanNorm::euclidean_norm)
                .map(|i| i * i)
                .sum::<f64>(),
        )
    }
}

impl<R: UnitalRing + EuclideanNorm, const N: usize, C: Convolution<R, N>> EuclideanNorm
    for UnivariateRing<R, N, C>
{
    fn euclidean_norm(&self) -> f64 {
        libm::sqrt(
            self.as_ref()
                .iter()
                .map(EuclideanNorm::euclidean_norm)
                .map(|i| i * i)
                .sum::<f64>(),
        )
    }
}

/// ℓ-∞ norm.
pub trait InfinityNorm<Length: Ord> {
    /// Check whether the norm is less than bound.
    fn check_infinity_norm(&self, bound: &Length) -> bool;

    /// Compute infinity norm.
    ///
    /// For 0-dimensional objects returns default.
    fn infinity_norm(&self) -> Length
    where
        Length: Default;
}

impl<Z: IntegerModRing> InfinityNorm<Z::Int> for Z {
    fn check_infinity_norm(&self, bound: &Z::Int) -> bool {
        self.absolute() < *bound
    }

    fn infinity_norm(&self) -> Z::Int {
        self.absolute()
    }
}

impl<Length: Ord, R: Ring + InfinityNorm<Length>, const N: usize> InfinityNorm<Length>
    for VectorRing<R, N>
{
    fn check_infinity_norm(&self, bound: &Length) -> bool {
        self.into_iter().all(|i| i.check_infinity_norm(bound))
    }

    fn infinity_norm(&self) -> Length
    where
        Length: Default,
    {
        self.into_iter()
            .map(InfinityNorm::infinity_norm)
            .max()
            .unwrap_or_default()
    }
}

impl<Length: Ord, R: Ring + InfinityNorm<Length>> InfinityNorm<Length> for DenseMatrix<R> {
    fn check_infinity_norm(&self, bound: &Length) -> bool {
        self.as_ref().iter().all(|i| i.check_infinity_norm(bound))
    }

    fn infinity_norm(&self) -> Length
    where
        Length: Default,
    {
        self.as_ref()
            .iter()
            .map(InfinityNorm::infinity_norm)
            .max()
            .unwrap_or_default()
    }
}

impl<Length: Ord, R: Ring + InfinityNorm<Length>> InfinityNorm<Length> for SparseMatrix<R> {
    fn check_infinity_norm(&self, bound: &Length) -> bool {
        self.as_ref().iter().all(|i| i.check_infinity_norm(bound))
    }

    fn infinity_norm(&self) -> Length
    where
        Length: Default,
    {
        self.as_ref()
            .iter()
            .map(InfinityNorm::infinity_norm)
            .max()
            .unwrap_or_default()
    }
}

impl<Length: Ord, R: Ring + InfinityNorm<Length>, const M: usize, const N: usize, const MN: usize>
    InfinityNorm<Length> for MatrixSpace<R, M, N, MN>
{
    fn check_infinity_norm(&self, bound: &Length) -> bool {
        self.as_ref().iter().all(|i| i.check_infinity_norm(bound))
    }

    fn infinity_norm(&self) -> Length
    where
        Length: Default,
    {
        self.as_ref()
            .iter()
            .map(InfinityNorm::infinity_norm)
            .max()
            .unwrap_or_default()
    }
}

impl<Length: Ord, R: Ring + InfinityNorm<Length>> InfinityNorm<Length> for DenseVector<R> {
    fn check_infinity_norm(&self, bound: &Length) -> bool {
        self.iter().all(|i| i.check_infinity_norm(bound))
    }

    fn infinity_norm(&self) -> Length
    where
        Length: Default,
    {
        self.iter()
            .map(InfinityNorm::infinity_norm)
            .max()
            .unwrap_or_default()
    }
}

impl<Length: Ord, R: Ring + InfinityNorm<Length>> InfinityNorm<Length> for SparseVector<R> {
    fn check_infinity_norm(&self, bound: &Length) -> bool {
        self.as_ref().iter().all(|i| i.check_infinity_norm(bound))
    }

    fn infinity_norm(&self) -> Length
    where
        Length: Default,
    {
        self.as_ref()
            .iter()
            .map(InfinityNorm::infinity_norm)
            .max()
            .unwrap_or_default()
    }
}

impl<Length: Ord, R: UnitalRing + InfinityNorm<Length>> InfinityNorm<Length>
    for QuaternionAlgebra<R>
{
    fn check_infinity_norm(&self, bound: &Length) -> bool {
        self.as_ref().iter().all(|i| i.check_infinity_norm(bound))
    }

    fn infinity_norm(&self) -> Length
    where
        Length: Default,
    {
        self.as_ref()
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
        self.as_ref().iter().all(|i| i.check_infinity_norm(bound))
    }

    fn infinity_norm(&self) -> Length
    where
        Length: Default,
    {
        self.as_ref()
            .iter()
            .map(InfinityNorm::infinity_norm)
            .max()
            .unwrap_or_default()
    }
}
