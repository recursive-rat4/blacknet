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

use crate::algebra::{
    AdditiveAbelianGroup, AdditiveCommutativeMagma, AdditiveMagma, AdditiveMonoid,
    AdditiveSemigroup, Double, LeftZero, MultiplicativeMonoid, RightZero, Square,
};
use crate::ed25519::TwistedEdwardsGroupParams;
use core::fmt::{Debug, Formatter, Result};
use core::iter::Sum;
use core::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub struct TwistedEdwardsGroupAffine<P: TwistedEdwardsGroupParams> {
    x: P::F,
    y: P::F,
}

impl<P: TwistedEdwardsGroupParams> TwistedEdwardsGroupAffine<P> {
    pub const fn new(x: P::F, y: P::F) -> Self {
        Self { x, y }
    }
}

impl<P: TwistedEdwardsGroupParams<F: Clone>> Clone for TwistedEdwardsGroupAffine<P> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<P: TwistedEdwardsGroupParams<F: Copy>> Copy for TwistedEdwardsGroupAffine<P> {}

impl<P: TwistedEdwardsGroupParams<F: Debug>> Debug for TwistedEdwardsGroupAffine<P> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({:?}, {:?})", self.x, self.y)
    }
}

impl<P: TwistedEdwardsGroupParams> Default for TwistedEdwardsGroupAffine<P> {
    fn default() -> Self {
        Self::ZERO
    }
}

impl<P: TwistedEdwardsGroupParams<F: PartialEq>> PartialEq for TwistedEdwardsGroupAffine<P> {
    fn eq(&self, rps: &Self) -> bool {
        self.x == rps.x && self.y == rps.y
    }
}

impl<P: TwistedEdwardsGroupParams<F: Eq>> Eq for TwistedEdwardsGroupAffine<P> {}

impl<P: TwistedEdwardsGroupParams> Add for TwistedEdwardsGroupAffine<P> {
    type Output = Self;

    fn add(self, rps: Self) -> Self::Output {
        let x1x2 = self.x * rps.x;
        let y1y2 = self.y * rps.y;
        let k = P::D * x1x2 * y1y2;
        let xr = (self.x * rps.y + self.y * rps.x) / (P::F::ONE + k);
        let yr = if P::A_IS_MINUS_ONE {
            (y1y2 + x1x2) / (P::F::ONE - k)
        } else {
            (y1y2 - P::A * x1x2) / (P::F::ONE - k)
        };
        Self {
            x: xr.expect("Elliptic curve arithmetic"),
            y: yr.expect("Elliptic curve arithmetic"),
        }
    }
}

impl<P: TwistedEdwardsGroupParams> Add<&Self> for TwistedEdwardsGroupAffine<P> {
    type Output = Self;

    #[inline]
    fn add(self, rps: &Self) -> Self::Output {
        self + *rps
    }
}

impl<P: TwistedEdwardsGroupParams> AddAssign for TwistedEdwardsGroupAffine<P> {
    #[inline]
    fn add_assign(&mut self, rps: Self) {
        *self = *self + rps
    }
}

impl<P: TwistedEdwardsGroupParams> AddAssign<&Self> for TwistedEdwardsGroupAffine<P> {
    #[inline]
    fn add_assign(&mut self, rps: &Self) {
        *self = *self + *rps
    }
}

impl<P: TwistedEdwardsGroupParams> Double for TwistedEdwardsGroupAffine<P> {
    type Output = Self;

    fn double(self) -> Self {
        let xx = self.x.square();
        let yy = self.y.square();
        let k = P::D * xx * yy;
        let xr = (self.x * self.y).double() / (P::F::ONE + k);
        let yr = if P::A_IS_MINUS_ONE {
            (yy + xx) / (P::F::ONE - k)
        } else {
            (yy - P::A * xx) / (P::F::ONE - k)
        };
        Self {
            x: xr.expect("Elliptic curve arithmetic"),
            y: yr.expect("Elliptic curve arithmetic"),
        }
    }
}

impl<P: TwistedEdwardsGroupParams> Neg for TwistedEdwardsGroupAffine<P> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: self.y,
        }
    }
}

impl<P: TwistedEdwardsGroupParams> Sub for TwistedEdwardsGroupAffine<P> {
    type Output = Self;

    fn sub(self, rps: Self) -> Self::Output {
        // sub-2025-v
        let x1x2 = self.x * rps.x;
        let y1y2 = self.y * rps.y;
        let k = P::D * x1x2 * y1y2;
        let xr = (self.x * rps.y - self.y * rps.x) / (P::F::ONE - k);
        let yr = if P::A_IS_MINUS_ONE {
            (y1y2 - x1x2) / (P::F::ONE + k)
        } else {
            (y1y2 + P::A * x1x2) / (P::F::ONE + k)
        };
        Self {
            x: xr.expect("Elliptic curve arithmetic"),
            y: yr.expect("Elliptic curve arithmetic"),
        }
    }
}

impl<P: TwistedEdwardsGroupParams> Sub<&Self> for TwistedEdwardsGroupAffine<P> {
    type Output = Self;

    #[inline]
    fn sub(self, rps: &Self) -> Self::Output {
        self - *rps
    }
}

impl<P: TwistedEdwardsGroupParams> SubAssign for TwistedEdwardsGroupAffine<P> {
    #[inline]
    fn sub_assign(&mut self, rps: Self) {
        *self = *self - rps
    }
}

impl<P: TwistedEdwardsGroupParams> SubAssign<&Self> for TwistedEdwardsGroupAffine<P> {
    #[inline]
    fn sub_assign(&mut self, rps: &Self) {
        *self = *self - *rps
    }
}

impl<P: TwistedEdwardsGroupParams, Scalar: IntoIterator<Item = bool>> Mul<Scalar>
    for TwistedEdwardsGroupAffine<P>
{
    type Output = Self;

    #[inline]
    fn mul(self, rps: Scalar) -> Self::Output {
        self.add_sub_chain(rps)
    }
}

impl<P: TwistedEdwardsGroupParams, Scalar: IntoIterator<Item = bool>> MulAssign<Scalar>
    for TwistedEdwardsGroupAffine<P>
{
    #[inline]
    fn mul_assign(&mut self, rps: Scalar) {
        *self = *self * rps;
    }
}

impl<P: TwistedEdwardsGroupParams> Sum for TwistedEdwardsGroupAffine<P> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|lps, rps| lps + rps).unwrap_or(Self::ZERO)
    }
}

impl<'a, P: TwistedEdwardsGroupParams> Sum<&'a Self> for TwistedEdwardsGroupAffine<P> {
    #[inline]
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.copied().sum()
    }
}

impl<P: TwistedEdwardsGroupParams> LeftZero for TwistedEdwardsGroupAffine<P> {
    const LEFT_ZERO: Self = Self {
        x: P::F::ZERO,
        y: P::F::ONE,
    };
}

impl<P: TwistedEdwardsGroupParams> RightZero for TwistedEdwardsGroupAffine<P> {
    const RIGHT_ZERO: Self = Self {
        x: P::F::ZERO,
        y: P::F::ONE,
    };
}

impl<P: TwistedEdwardsGroupParams> AdditiveMagma for TwistedEdwardsGroupAffine<P> {}

impl<P: TwistedEdwardsGroupParams> AdditiveCommutativeMagma for TwistedEdwardsGroupAffine<P> {}

impl<P: TwistedEdwardsGroupParams> AdditiveSemigroup for TwistedEdwardsGroupAffine<P> {}

impl<P: TwistedEdwardsGroupParams> AdditiveMonoid for TwistedEdwardsGroupAffine<P> {
    const ZERO: Self = Self {
        x: P::F::ZERO,
        y: P::F::ONE,
    };
}
