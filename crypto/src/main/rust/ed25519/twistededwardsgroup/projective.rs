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
    AdditiveSemigroup, Double, Inv, LeftZero, MultiplicativeMonoid, RightZero, Square,
};
use crate::ed25519::TwistedEdwardsGroupParams;
use core::fmt::{Debug, Formatter, Result};
use core::iter::Sum;
use core::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub struct TwistedEdwardsGroupProjective<P: TwistedEdwardsGroupParams> {
    x: P::F,
    y: P::F,
    z: P::F,
}

impl<P: TwistedEdwardsGroupParams> TwistedEdwardsGroupProjective<P> {
    pub const fn new(x: P::F, y: P::F) -> Self {
        Self { x, y, z: P::F::ONE }
    }

    pub fn scale(self) -> Self {
        let a = self.z.inv().expect("Elliptic curve arithmetic");
        Self {
            x: self.x * a,
            y: self.y * a,
            z: P::F::ONE,
        }
    }
}

impl<P: TwistedEdwardsGroupParams<F: Clone>> Clone for TwistedEdwardsGroupProjective<P> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<P: TwistedEdwardsGroupParams<F: Copy>> Copy for TwistedEdwardsGroupProjective<P> {}

impl<P: TwistedEdwardsGroupParams<F: Debug>> Debug for TwistedEdwardsGroupProjective<P> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({:?}, {:?}, {:?}", self.x, self.y, self.z)
    }
}

impl<P: TwistedEdwardsGroupParams> Default for TwistedEdwardsGroupProjective<P> {
    fn default() -> Self {
        Self::ZERO
    }
}

impl<P: TwistedEdwardsGroupParams<F: PartialEq>> PartialEq for TwistedEdwardsGroupProjective<P> {
    fn eq(&self, rps: &Self) -> bool {
        (self.x * rps.z == self.z * rps.x) && (self.y * rps.z == self.z * rps.y)
    }
}

impl<P: TwistedEdwardsGroupParams<F: Eq>> Eq for TwistedEdwardsGroupProjective<P> {}

impl<P: TwistedEdwardsGroupParams> Add for TwistedEdwardsGroupProjective<P> {
    type Output = Self;

    fn add(self, rps: Self) -> Self::Output {
        // add-2008-bbjlp
        let x1x2 = self.x * rps.x;
        let y1y2 = self.y * rps.y;
        let z1z2 = self.z * rps.z;
        let b = z1z2.square();
        let e = P::D * x1x2 * y1y2;
        let f = b - e;
        let g = b + e;
        let j = if P::A_IS_MINUS_ONE {
            y1y2 + x1x2
        } else {
            y1y2 - P::A * x1x2
        };
        let xr = z1z2 * f * ((self.x + self.y) * (rps.x + rps.y) - (y1y2 + x1x2));
        let yr = z1z2 * g * j;
        let zr = f * g;
        Self {
            x: xr,
            y: yr,
            z: zr,
        }
    }
}

impl<P: TwistedEdwardsGroupParams> Add<&Self> for TwistedEdwardsGroupProjective<P> {
    type Output = Self;

    #[inline]
    fn add(self, rps: &Self) -> Self::Output {
        self + *rps
    }
}

impl<P: TwistedEdwardsGroupParams> AddAssign for TwistedEdwardsGroupProjective<P> {
    #[inline]
    fn add_assign(&mut self, rps: Self) {
        *self = *self + rps
    }
}

impl<P: TwistedEdwardsGroupParams> AddAssign<&Self> for TwistedEdwardsGroupProjective<P> {
    #[inline]
    fn add_assign(&mut self, rps: &Self) {
        *self = *self + *rps
    }
}

impl<P: TwistedEdwardsGroupParams> Double for TwistedEdwardsGroupProjective<P> {
    type Output = Self;

    fn double(self) -> Self {
        // dbl-2008-bbjlp
        let b = (self.x + self.y).square();
        let xx = self.x.square();
        let yy = self.y.square();
        let zz = self.z.square();
        let e = if P::A_IS_MINUS_ONE { -xx } else { P::A * xx };
        let f = e + yy;
        let j = f - zz.double();
        let xr = (b - xx - yy) * j;
        let yr = f * (e - yy);
        let zr = f * j;
        Self {
            x: xr,
            y: yr,
            z: zr,
        }
    }
}

impl<P: TwistedEdwardsGroupParams> Neg for TwistedEdwardsGroupProjective<P> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: self.y,
            z: self.z,
        }
    }
}

impl<P: TwistedEdwardsGroupParams> Sub for TwistedEdwardsGroupProjective<P> {
    type Output = Self;

    fn sub(self, rps: Self) -> Self::Output {
        // sub-2025-v
        let x1x2 = self.x * rps.x;
        let y1y2 = self.y * rps.y;
        let z1z2 = self.z * rps.z;
        let b = z1z2.square();
        let e = P::D * x1x2 * y1y2;
        let f = b + e;
        let g = b - e;
        let j = if P::A_IS_MINUS_ONE {
            y1y2 - x1x2
        } else {
            y1y2 + P::A * x1x2
        };
        let xr = z1z2 * f * ((self.x + self.y) * (rps.y - rps.x) - (y1y2 - x1x2));
        let yr = z1z2 * g * j;
        let zr = f * g;
        Self {
            x: xr,
            y: yr,
            z: zr,
        }
    }
}

impl<P: TwistedEdwardsGroupParams> Sub<&Self> for TwistedEdwardsGroupProjective<P> {
    type Output = Self;

    #[inline]
    fn sub(self, rps: &Self) -> Self::Output {
        self - *rps
    }
}

impl<P: TwistedEdwardsGroupParams> SubAssign for TwistedEdwardsGroupProjective<P> {
    #[inline]
    fn sub_assign(&mut self, rps: Self) {
        *self = *self - rps
    }
}

impl<P: TwistedEdwardsGroupParams> SubAssign<&Self> for TwistedEdwardsGroupProjective<P> {
    #[inline]
    fn sub_assign(&mut self, rps: &Self) {
        *self = *self - *rps
    }
}

impl<P: TwistedEdwardsGroupParams, Scalar: IntoIterator<Item = bool>> Mul<Scalar>
    for TwistedEdwardsGroupProjective<P>
{
    type Output = Self;

    #[inline]
    fn mul(self, rps: Scalar) -> Self::Output {
        self.add_sub_chain(rps)
    }
}

impl<P: TwistedEdwardsGroupParams, Scalar: IntoIterator<Item = bool>> MulAssign<Scalar>
    for TwistedEdwardsGroupProjective<P>
{
    #[inline]
    fn mul_assign(&mut self, rps: Scalar) {
        *self = *self * rps;
    }
}

impl<P: TwistedEdwardsGroupParams> Sum for TwistedEdwardsGroupProjective<P> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|lps, rps| lps + rps).unwrap_or(Self::ZERO)
    }
}

impl<'a, P: TwistedEdwardsGroupParams> Sum<&'a Self> for TwistedEdwardsGroupProjective<P> {
    #[inline]
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.copied().sum()
    }
}

impl<P: TwistedEdwardsGroupParams> LeftZero for TwistedEdwardsGroupProjective<P> {
    const LEFT_ZERO: Self = Self {
        x: P::F::ZERO,
        y: P::F::ONE,
        z: P::F::ONE,
    };
}

impl<P: TwistedEdwardsGroupParams> RightZero for TwistedEdwardsGroupProjective<P> {
    const RIGHT_ZERO: Self = Self {
        x: P::F::ZERO,
        y: P::F::ONE,
        z: P::F::ONE,
    };
}

impl<P: TwistedEdwardsGroupParams> AdditiveMagma for TwistedEdwardsGroupProjective<P> {}

impl<P: TwistedEdwardsGroupParams> AdditiveCommutativeMagma for TwistedEdwardsGroupProjective<P> {}

impl<P: TwistedEdwardsGroupParams> AdditiveSemigroup for TwistedEdwardsGroupProjective<P> {}

impl<P: TwistedEdwardsGroupParams> AdditiveMonoid for TwistedEdwardsGroupProjective<P> {
    const ZERO: Self = Self {
        x: P::F::ZERO,
        y: P::F::ONE,
        z: P::F::ONE,
    };
}
