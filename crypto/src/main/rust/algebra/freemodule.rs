/*
 * Copyright (c) 2025 Pavel Vasin
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

use crate::algebra::AdditiveMonoid;
use crate::algebra::Double;
use crate::algebra::Module;
use crate::algebra::Ring;
use crate::algebra::{AdditiveCommutativeMagma, AdditiveMagma};
use crate::algebra::{AdditiveSemigroup, LeftZero, RightZero};
use crate::duplex::{Absorb, Duplex, Squeeze};
use core::array;
use core::borrow::{Borrow, BorrowMut};
use core::fmt::{Debug, Formatter, Result};
use core::iter::Sum;
use core::ops::{Add, AddAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Deserialize, Eq, PartialEq, Serialize)]
#[serde(bound(
    deserialize = "[R; N]: Deserialize<'de>",
    serialize = "[R; N]: Serialize"
))]
pub struct FreeModule<R: Ring, const N: usize> {
    components: [R; N],
}

impl<R: Ring, const N: usize> FreeModule<R, N> {
    pub const fn new(components: [R; N]) -> Self {
        Self { components }
    }

    pub const fn components(self) -> [R; N] {
        self.components
    }

    #[inline]
    pub fn from_fn<F: FnMut(usize) -> R>(f: F) -> Self {
        Self::from(array::from_fn(f))
    }

    pub const fn swap(&mut self, a: usize, b: usize) {
        self.components.swap(a, b)
    }
}

impl<R: Ring, const N: usize> Default for FreeModule<R, N> {
    #[inline]
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl<R: Ring, const N: usize> From<[R; N]> for FreeModule<R, N> {
    #[inline]
    fn from(components: [R; N]) -> Self {
        Self { components }
    }
}

impl<R: Ring + Debug, const N: usize> Debug for FreeModule<R, N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:?}", self.components)
    }
}

impl<R: Ring, const N: usize> AsRef<[R]> for FreeModule<R, N> {
    #[inline]
    fn as_ref(&self) -> &[R] {
        &self.components
    }
}

impl<R: Ring, const N: usize> AsMut<[R]> for FreeModule<R, N> {
    #[inline]
    fn as_mut(&mut self) -> &mut [R] {
        &mut self.components
    }
}

impl<R: Ring, const N: usize> Borrow<[R]> for FreeModule<R, N> {
    #[inline]
    fn borrow(&self) -> &[R] {
        &self.components
    }
}

impl<R: Ring, const N: usize> BorrowMut<[R]> for FreeModule<R, N> {
    #[inline]
    fn borrow_mut(&mut self) -> &mut [R] {
        &mut self.components
    }
}

impl<R: Ring, const N: usize> Index<usize> for FreeModule<R, N> {
    type Output = R;

    #[inline]
    fn index(&self, i: usize) -> &Self::Output {
        &self.components[i]
    }
}

impl<R: Ring, const N: usize> IndexMut<usize> for FreeModule<R, N> {
    #[inline]
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.components[i]
    }
}

impl<R: Ring, const N: usize> IntoIterator for FreeModule<R, N> {
    type Item = R;
    type IntoIter = core::array::IntoIter<R, N>;

    fn into_iter(self) -> Self::IntoIter {
        self.components.into_iter()
    }
}

impl<R: Ring, const N: usize> Add for FreeModule<R, N> {
    type Output = Self;

    fn add(self, rps: Self) -> Self::Output {
        Self {
            components: array::from_fn(|i| self.components[i] + rps.components[i]),
        }
    }
}

impl<R: Ring, const N: usize> Add<&Self> for FreeModule<R, N> {
    type Output = Self;

    #[inline]
    fn add(self, rps: &Self) -> Self::Output {
        self + *rps
    }
}

impl<R: Ring, const N: usize> AddAssign for FreeModule<R, N> {
    #[inline]
    fn add_assign(&mut self, rps: Self) {
        *self = *self + rps
    }
}

impl<R: Ring, const N: usize> AddAssign<&Self> for FreeModule<R, N> {
    #[inline]
    fn add_assign(&mut self, rps: &Self) {
        *self = *self + *rps
    }
}

impl<R: Ring, const N: usize> Double for FreeModule<R, N> {
    type Output = Self;

    fn double(self) -> Self {
        Self {
            components: array::from_fn(|i| self.components[i].double()),
        }
    }
}

impl<R: Ring, const N: usize> Neg for FreeModule<R, N> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            components: array::from_fn(|i| -self.components[i]),
        }
    }
}

impl<R: Ring, const N: usize> Sub for FreeModule<R, N> {
    type Output = Self;

    fn sub(self, rps: Self) -> Self::Output {
        Self {
            components: array::from_fn(|i| self.components[i] - rps.components[i]),
        }
    }
}

impl<R: Ring, const N: usize> Sub<&Self> for FreeModule<R, N> {
    type Output = Self;

    #[inline]
    fn sub(self, rps: &Self) -> Self::Output {
        self - *rps
    }
}

impl<R: Ring, const N: usize> SubAssign for FreeModule<R, N> {
    #[inline]
    fn sub_assign(&mut self, rps: Self) {
        *self = *self - rps
    }
}

impl<R: Ring, const N: usize> SubAssign<&Self> for FreeModule<R, N> {
    #[inline]
    fn sub_assign(&mut self, rps: &Self) {
        *self = *self - *rps
    }
}

impl<R: Ring, const N: usize> Mul<R> for FreeModule<R, N> {
    type Output = Self;

    fn mul(self, rps: R) -> Self::Output {
        Self {
            components: array::from_fn(|i| self.components[i] * rps),
        }
    }
}

impl<R: Ring, const N: usize> Mul<&R> for FreeModule<R, N> {
    type Output = Self;

    #[inline]
    fn mul(self, rps: &R) -> Self::Output {
        self * *rps
    }
}

impl<R: Ring, const N: usize> MulAssign<R> for FreeModule<R, N> {
    #[inline]
    fn mul_assign(&mut self, rps: R) {
        *self = *self * rps
    }
}

impl<R: Ring, const N: usize> MulAssign<&R> for FreeModule<R, N> {
    #[inline]
    fn mul_assign(&mut self, rps: &R) {
        *self = *self * *rps
    }
}

impl<R: Ring, const N: usize> Sum for FreeModule<R, N> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|lps, rps| lps + rps).unwrap_or(Self::IDENTITY)
    }
}

impl<'a, R: Ring, const N: usize> Sum<&'a Self> for FreeModule<R, N> {
    #[inline]
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.copied().sum()
    }
}

impl<R: Ring, const N: usize> LeftZero for FreeModule<R, N> {
    const LEFT_ZERO: Self = Self {
        components: [R::LEFT_ZERO; N],
    };
}

impl<R: Ring, const N: usize> RightZero for FreeModule<R, N> {
    const RIGHT_ZERO: Self = Self {
        components: [R::RIGHT_ZERO; N],
    };
}

impl<R: Ring, const N: usize> AdditiveMagma for FreeModule<R, N> {}

impl<R: Ring, const N: usize> AdditiveCommutativeMagma for FreeModule<R, N> {}

impl<R: Ring, const N: usize> AdditiveSemigroup for FreeModule<R, N> {}

impl<R: Ring, const N: usize> AdditiveMonoid for FreeModule<R, N> {
    const IDENTITY: Self = Self {
        components: [R::ZERO; N],
    };
}

impl<R: Ring, const N: usize> Module<R> for FreeModule<R, N> {}

impl<R: Ring + Absorb<R>, const N: usize> Absorb<R> for FreeModule<R, N> {
    fn absorb_into(self, duplex: &mut (impl Duplex<R> + ?Sized)) {
        duplex.absorb(self.components)
    }
}

impl<R: Ring + Squeeze<R>, const N: usize> Squeeze<R> for FreeModule<R, N> {
    fn squeeze_from(duplex: &mut (impl Duplex<R> + ?Sized)) -> Self {
        duplex.squeeze::<[R; N]>().into()
    }
}
