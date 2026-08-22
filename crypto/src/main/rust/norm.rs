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
    IntegerModRing, MatrixSpace, QuaternionAlgebra, Semiring, UnitalRing, UnitalSemiring,
    UnivariateRing, VectorRing, Zero,
};
use crate::convolution::Convolution;
use crate::float::Cast;
use crate::matrix::{DenseMatrix, DenseVector, SparseMatrix, SparseVector};

/// ℓ₂ space.
pub enum L2 {}
/// ℓ-∞ space.
pub enum LInf {}

/// ℓₚ norm.
pub trait Norm<Lp> {
    /// Non-negative numeric.
    type Length;

    /// Check whether the norm is less than bound.
    fn check_norm(&self, bound: &Self::Length) -> bool;
    /// Compute the norm.
    fn norm(&self) -> Self::Length;
}

impl<Z: IntegerModRing<Int: Cast<f64>>> Norm<L2> for Z {
    type Length = f64;

    fn check_norm(&self, bound: &f64) -> bool {
        self.absolute().cast() < *bound
    }

    fn norm(&self) -> f64 {
        self.absolute().cast()
    }
}

//RUST https://github.com/rust-lang/rust/issues/137578

impl<R: Semiring + Norm<L2, Length = f64>, const N: usize> Norm<L2> for VectorRing<R, N> {
    type Length = R::Length;

    fn check_norm(&self, bound: &Self::Length) -> bool {
        self.norm() < *bound
    }

    fn norm(&self) -> Self::Length {
        libm::sqrt(self.into_iter().map(Norm::<L2>::norm).map(|i| i * i).sum())
    }
}

impl<T: Norm<L2, Length = f64>> Norm<L2> for DenseVector<T> {
    type Length = T::Length;

    fn check_norm(&self, bound: &Self::Length) -> bool {
        self.norm() < *bound
    }

    fn norm(&self) -> Self::Length {
        libm::sqrt(self.into_iter().map(Norm::<L2>::norm).map(|i| i * i).sum())
    }
}

impl<T: Zero + Norm<L2, Length = f64>> Norm<L2> for SparseVector<T> {
    type Length = T::Length;

    fn check_norm(&self, bound: &Self::Length) -> bool {
        self.norm() < *bound
    }

    fn norm(&self) -> Self::Length {
        libm::sqrt(
            self.as_ref()
                .iter()
                .map(Norm::<L2>::norm)
                .map(|i| i * i)
                .sum(),
        )
    }
}

impl<R: UnitalRing + Norm<L2, Length = f64>> Norm<L2> for QuaternionAlgebra<R> {
    type Length = R::Length;

    fn check_norm(&self, bound: &Self::Length) -> bool {
        self.norm() < *bound
    }

    fn norm(&self) -> Self::Length {
        libm::sqrt(self.into_iter().map(Norm::<L2>::norm).map(|i| i * i).sum())
    }
}

impl<R: UnitalSemiring + Norm<L2, Length = f64>, const N: usize, C: Convolution<R, N>> Norm<L2>
    for UnivariateRing<R, N, C>
{
    type Length = R::Length;

    fn check_norm(&self, bound: &Self::Length) -> bool {
        self.norm() < *bound
    }

    fn norm(&self) -> Self::Length {
        libm::sqrt(self.into_iter().map(Norm::<L2>::norm).map(|i| i * i).sum())
    }
}

impl<Z: IntegerModRing> Norm<LInf> for Z {
    type Length = Z::Int;

    fn check_norm(&self, bound: &Z::Int) -> bool {
        self.absolute() < *bound
    }

    fn norm(&self) -> Z::Int {
        self.absolute()
    }
}

impl<R: Semiring + Norm<LInf, Length: Ord + Default>, const N: usize> Norm<LInf>
    for VectorRing<R, N>
{
    type Length = R::Length;

    fn check_norm(&self, bound: &Self::Length) -> bool {
        self.into_iter().all(|i| i.check_norm(bound))
    }

    fn norm(&self) -> Self::Length {
        self.into_iter()
            .map(Norm::<LInf>::norm)
            .max()
            .unwrap_or_default()
    }
}

impl<T: Norm<LInf, Length: Ord + Default>> Norm<LInf> for DenseVector<T> {
    type Length = T::Length;

    fn check_norm(&self, bound: &Self::Length) -> bool {
        self.into_iter().all(|i| i.check_norm(bound))
    }

    fn norm(&self) -> Self::Length {
        self.into_iter()
            .map(Norm::<LInf>::norm)
            .max()
            .unwrap_or_default()
    }
}

impl<T: Norm<LInf, Length: Ord + Default>> Norm<LInf> for DenseMatrix<T> {
    type Length = T::Length;

    fn check_norm(&self, bound: &Self::Length) -> bool {
        self.as_ref().iter().all(|i| i.check_norm(bound))
    }

    fn norm(&self) -> Self::Length {
        self.as_ref()
            .iter()
            .map(Norm::<LInf>::norm)
            .max()
            .unwrap_or_default()
    }
}

impl<T: Zero + Norm<LInf, Length: Ord + Default>> Norm<LInf> for SparseMatrix<T> {
    type Length = T::Length;

    fn check_norm(&self, bound: &Self::Length) -> bool {
        self.as_ref().iter().all(|i| i.check_norm(bound))
    }

    fn norm(&self) -> Self::Length {
        self.as_ref()
            .iter()
            .map(Norm::<LInf>::norm)
            .max()
            .unwrap_or_default()
    }
}

impl<
    R: Semiring + Norm<LInf, Length: Ord + Default>,
    const M: usize,
    const N: usize,
    const MN: usize,
> Norm<LInf> for MatrixSpace<R, M, N, MN>
{
    type Length = R::Length;

    fn check_norm(&self, bound: &Self::Length) -> bool {
        self.into_iter().all(|i| i.check_norm(bound))
    }

    fn norm(&self) -> Self::Length {
        self.as_ref()
            .iter()
            .map(Norm::<LInf>::norm)
            .max()
            .unwrap_or_default()
    }
}

impl<T: Zero + Norm<LInf, Length: Ord + Default>> Norm<LInf> for SparseVector<T> {
    type Length = T::Length;

    fn check_norm(&self, bound: &Self::Length) -> bool {
        self.as_ref().iter().all(|i| i.check_norm(bound))
    }

    fn norm(&self) -> Self::Length {
        self.as_ref()
            .iter()
            .map(Norm::<LInf>::norm)
            .max()
            .unwrap_or_default()
    }
}

impl<R: UnitalRing + Norm<LInf, Length: Ord + Default>> Norm<LInf> for QuaternionAlgebra<R> {
    type Length = R::Length;

    fn check_norm(&self, bound: &Self::Length) -> bool {
        self.into_iter().all(|i| i.check_norm(bound))
    }

    fn norm(&self) -> Self::Length {
        self.into_iter()
            .map(Norm::<LInf>::norm)
            .max()
            .unwrap_or_default()
    }
}

impl<R: UnitalSemiring + Norm<LInf, Length: Ord + Default>, const N: usize, C: Convolution<R, N>>
    Norm<LInf> for UnivariateRing<R, N, C>
{
    type Length = R::Length;

    fn check_norm(&self, bound: &Self::Length) -> bool {
        self.into_iter().all(|i| i.check_norm(bound))
    }

    fn norm(&self) -> Self::Length {
        self.into_iter()
            .map(Norm::<LInf>::norm)
            .max()
            .unwrap_or_default()
    }
}
