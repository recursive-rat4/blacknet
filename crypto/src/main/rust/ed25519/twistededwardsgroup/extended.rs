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
    AdditiveAbelianGroup, AdditiveCommutativeMagma, AdditiveMonoid, AdditiveSemigroup, Double, Inv,
    LeftZero, One, RightZero, Set, Square, Zero,
};
use crate::ed25519::{TwistedEdwardsGroupParams, is_on_curve};
use core::fmt::{Debug, Formatter, Result};
use core::iter::Sum;
use core::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub struct TwistedEdwardsGroupExtended<P: TwistedEdwardsGroupParams> {
    x: P::F,
    y: P::F,
    z: P::F,
    t: P::F,
}

impl<P: TwistedEdwardsGroupParams> TwistedEdwardsGroupExtended<P> {
    pub fn new(x: P::F, y: P::F) -> Option<Self>
    where
        P: TwistedEdwardsGroupParams<F: Eq>,
    {
        if is_on_curve::<P>(x, y) {
            Some(Self {
                x,
                y,
                z: P::F::ONE,
                t: x * y,
            })
        } else {
            None
        }
    }

    /// # Safety
    /// Point `(x, y)` is on the curve.
    pub unsafe fn from_unchecked(x: P::F, y: P::F) -> Self {
        Self {
            x,
            y,
            z: P::F::ONE,
            t: x * y,
        }
    }

    pub fn scale(self) -> Self {
        let a = self.z.inv().expect("Elliptic curve arithmetic");
        Self {
            x: self.x * a,
            y: self.y * a,
            z: P::F::ONE,
            t: self.t * a,
        }
    }
}

impl<P: TwistedEdwardsGroupParams<F: Clone>> Clone for TwistedEdwardsGroupExtended<P> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<P: TwistedEdwardsGroupParams<F: Copy>> Copy for TwistedEdwardsGroupExtended<P> {}

impl<P: TwistedEdwardsGroupParams<F: Debug>> Debug for TwistedEdwardsGroupExtended<P> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "({:?}, {:?}, {:?}, {:?})",
            self.x, self.y, self.z, self.t
        )
    }
}

impl<P: TwistedEdwardsGroupParams> Default for TwistedEdwardsGroupExtended<P> {
    fn default() -> Self {
        Self::ZERO
    }
}

impl<P: TwistedEdwardsGroupParams<F: PartialEq>> PartialEq for TwistedEdwardsGroupExtended<P> {
    fn eq(&self, rps: &Self) -> bool {
        (self.x * rps.z == self.z * rps.x) && (self.y * rps.z == self.z * rps.y)
    }
}

impl<P: TwistedEdwardsGroupParams<F: Eq>> Eq for TwistedEdwardsGroupExtended<P> {}

impl<P: TwistedEdwardsGroupParams> Add for TwistedEdwardsGroupExtended<P> {
    type Output = Self;

    fn add(self, rps: Self) -> Self::Output {
        // add-2008-hwcd-2
        let x1x2 = self.x * rps.x;
        let y1y2 = self.y * rps.y;
        let z1t2 = self.z * rps.t;
        let z2t1 = self.t * rps.z;
        let e = z2t1 + z1t2;
        let f = (self.x - self.y) * (rps.x + rps.y) + y1y2 - x1x2;
        let g = if P::A_IS_MINUS_ONE {
            y1y2 - x1x2
        } else {
            y1y2 + P::A * x1x2
        };
        let h = z2t1 - z1t2;
        let xr = e * f;
        let yr = g * h;
        let zr = f * g;
        let tr = e * h;
        Self {
            x: xr,
            y: yr,
            z: zr,
            t: tr,
        }
    }
}

impl<P: TwistedEdwardsGroupParams> Add<&Self> for TwistedEdwardsGroupExtended<P> {
    type Output = Self;

    #[inline]
    fn add(self, rps: &Self) -> Self::Output {
        self + *rps
    }
}

impl<P: TwistedEdwardsGroupParams> AddAssign for TwistedEdwardsGroupExtended<P> {
    #[inline]
    fn add_assign(&mut self, rps: Self) {
        *self = *self + rps
    }
}

impl<P: TwistedEdwardsGroupParams> AddAssign<&Self> for TwistedEdwardsGroupExtended<P> {
    #[inline]
    fn add_assign(&mut self, rps: &Self) {
        *self = *self + *rps
    }
}

impl<P: TwistedEdwardsGroupParams> Double for TwistedEdwardsGroupExtended<P> {
    type Output = Self;

    fn double(self) -> Self {
        // dbl-2008-hwcd
        let xx = self.x.square();
        let yy = self.y.square();
        let zz2 = self.z.square().double();
        let d = if P::A_IS_MINUS_ONE { -xx } else { P::A * xx };
        let e = (self.x + self.y).square() - xx - yy;
        let g = d + yy;
        let f = g - zz2;
        let h = d - yy;
        let xr = e * f;
        let yr = g * h;
        let zr = f * g;
        let tr = e * h;
        Self {
            x: xr,
            y: yr,
            z: zr,
            t: tr,
        }
    }
}

impl<P: TwistedEdwardsGroupParams> Neg for TwistedEdwardsGroupExtended<P> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: self.y,
            z: self.z,
            t: -self.t,
        }
    }
}

impl<P: TwistedEdwardsGroupParams> Sub for TwistedEdwardsGroupExtended<P> {
    type Output = Self;

    fn sub(self, rps: Self) -> Self::Output {
        // sub-2025-v
        let x1x2 = self.x * rps.x;
        let y1y2 = self.y * rps.y;
        let z1t2 = self.z * rps.t;
        let z2t1 = self.t * rps.z;
        let e = z2t1 - z1t2;
        let f = (self.x - self.y) * (rps.y - rps.x) + y1y2 + x1x2;
        let g = if P::A_IS_MINUS_ONE {
            y1y2 + x1x2
        } else {
            y1y2 - P::A * x1x2
        };
        let h = z2t1 + z1t2;
        let xr = e * f;
        let yr = g * h;
        let zr = f * g;
        let tr = e * h;
        Self {
            x: xr,
            y: yr,
            z: zr,
            t: tr,
        }
    }
}

impl<P: TwistedEdwardsGroupParams> Sub<&Self> for TwistedEdwardsGroupExtended<P> {
    type Output = Self;

    #[inline]
    fn sub(self, rps: &Self) -> Self::Output {
        self - *rps
    }
}

impl<P: TwistedEdwardsGroupParams> SubAssign for TwistedEdwardsGroupExtended<P> {
    #[inline]
    fn sub_assign(&mut self, rps: Self) {
        *self = *self - rps
    }
}

impl<P: TwistedEdwardsGroupParams> SubAssign<&Self> for TwistedEdwardsGroupExtended<P> {
    #[inline]
    fn sub_assign(&mut self, rps: &Self) {
        *self = *self - *rps
    }
}

impl<P: TwistedEdwardsGroupParams, Scalar: IntoIterator<Item = bool>> Mul<Scalar>
    for TwistedEdwardsGroupExtended<P>
{
    type Output = Self;

    #[inline]
    fn mul(self, rps: Scalar) -> Self::Output {
        self.add_sub_chain(rps)
    }
}

impl<P: TwistedEdwardsGroupParams, Scalar: IntoIterator<Item = bool>> MulAssign<Scalar>
    for TwistedEdwardsGroupExtended<P>
{
    #[inline]
    fn mul_assign(&mut self, rps: Scalar) {
        *self = *self * rps;
    }
}

impl<P: TwistedEdwardsGroupParams> Sum for TwistedEdwardsGroupExtended<P> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|lps, rps| lps + rps).unwrap_or(Self::ZERO)
    }
}

impl<'a, P: TwistedEdwardsGroupParams> Sum<&'a Self> for TwistedEdwardsGroupExtended<P> {
    #[inline]
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.copied().sum()
    }
}

impl<P: TwistedEdwardsGroupParams> LeftZero for TwistedEdwardsGroupExtended<P> {
    const LEFT_ZERO: Self = Self {
        x: P::F::ZERO,
        y: P::F::ONE,
        z: P::F::ONE,
        t: P::F::ZERO,
    };
}

impl<P: TwistedEdwardsGroupParams> RightZero for TwistedEdwardsGroupExtended<P> {
    const RIGHT_ZERO: Self = Self {
        x: P::F::ZERO,
        y: P::F::ONE,
        z: P::F::ONE,
        t: P::F::ZERO,
    };
}

impl<P: TwistedEdwardsGroupParams> Zero for TwistedEdwardsGroupExtended<P> {
    const ZERO: Self = Self {
        x: P::F::ZERO,
        y: P::F::ONE,
        z: P::F::ONE,
        t: P::F::ZERO,
    };
}

impl<P: TwistedEdwardsGroupParams> Set for TwistedEdwardsGroupExtended<P> {}

impl<P: TwistedEdwardsGroupParams> AdditiveCommutativeMagma for TwistedEdwardsGroupExtended<P> {}

impl<P: TwistedEdwardsGroupParams> AdditiveSemigroup for TwistedEdwardsGroupExtended<P> {}

impl<P: TwistedEdwardsGroupParams> AdditiveMonoid for TwistedEdwardsGroupExtended<P> {}
