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

use crate::abeliangroup::AdditiveAbelianGroup;
use crate::field::Field;
use crate::magma::{AdditiveMagma, Inv, MultiplicativeMagma};
use crate::monoid::AdditiveMonoid;
use crate::ring::Ring;
use core::fmt::{Debug, Formatter, Result};
use core::iter::Sum;
use core::ops::{Add, AddAssign, Neg, Sub, SubAssign};

pub trait TwistedEdwardsGroupParams: 'static + Copy + Eq {
    type F: Field;

    const A: Self::F;
    const D: Self::F;

    const A_IS_MINUS_ONE: bool;
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct TwistedEdwardsGroupAffine<P: TwistedEdwardsGroupParams> {
    x: P::F,
    y: P::F,
}

impl<P: TwistedEdwardsGroupParams> TwistedEdwardsGroupAffine<P> {
    pub const fn new(x: P::F, y: P::F) -> Self {
        Self { x, y }
    }
}

impl<P: TwistedEdwardsGroupParams> Debug for TwistedEdwardsGroupAffine<P> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({:?}, {:?})", self.x, self.y)
    }
}

impl<P: TwistedEdwardsGroupParams> Default for TwistedEdwardsGroupAffine<P> {
    fn default() -> Self {
        Self::IDENTITY
    }
}

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

impl<P: TwistedEdwardsGroupParams> AddAssign for TwistedEdwardsGroupAffine<P> {
    #[inline]
    fn add_assign(&mut self, rps: Self) {
        *self = *self + rps
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

impl<P: TwistedEdwardsGroupParams> SubAssign for TwistedEdwardsGroupAffine<P> {
    #[inline]
    fn sub_assign(&mut self, rps: Self) {
        *self = *self - rps
    }
}

impl<P: TwistedEdwardsGroupParams> Sum for TwistedEdwardsGroupAffine<P> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|lps, rps| lps + rps).unwrap_or(Self::IDENTITY)
    }
}

impl<P: TwistedEdwardsGroupParams> AdditiveMagma for TwistedEdwardsGroupAffine<P> {
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

impl<P: TwistedEdwardsGroupParams> AdditiveMonoid for TwistedEdwardsGroupAffine<P> {
    const IDENTITY: Self = Self {
        x: P::F::ZERO,
        y: P::F::ONE,
    };
}

impl<P: TwistedEdwardsGroupParams> AdditiveAbelianGroup for TwistedEdwardsGroupAffine<P> {}

#[derive(Clone, Copy, Eq)]
pub struct TwistedEdwardsGroupExtended<P: TwistedEdwardsGroupParams> {
    x: P::F,
    y: P::F,
    z: P::F,
    t: P::F,
}

impl<P: TwistedEdwardsGroupParams> TwistedEdwardsGroupExtended<P> {
    pub fn new(x: P::F, y: P::F) -> Self {
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

impl<P: TwistedEdwardsGroupParams> Debug for TwistedEdwardsGroupExtended<P> {
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
        Self::IDENTITY
    }
}

impl<P: TwistedEdwardsGroupParams> PartialEq for TwistedEdwardsGroupExtended<P> {
    fn eq(&self, rps: &Self) -> bool {
        (self.x * rps.z == self.z * rps.x) && (self.y * rps.z == self.z * rps.y)
    }
}

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

impl<P: TwistedEdwardsGroupParams> AddAssign for TwistedEdwardsGroupExtended<P> {
    #[inline]
    fn add_assign(&mut self, rps: Self) {
        *self = *self + rps
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

impl<P: TwistedEdwardsGroupParams> SubAssign for TwistedEdwardsGroupExtended<P> {
    #[inline]
    fn sub_assign(&mut self, rps: Self) {
        *self = *self - rps
    }
}

impl<P: TwistedEdwardsGroupParams> Sum for TwistedEdwardsGroupExtended<P> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|lps, rps| lps + rps).unwrap_or(Self::IDENTITY)
    }
}

impl<P: TwistedEdwardsGroupParams> AdditiveMagma for TwistedEdwardsGroupExtended<P> {
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

impl<P: TwistedEdwardsGroupParams> AdditiveMonoid for TwistedEdwardsGroupExtended<P> {
    const IDENTITY: Self = Self {
        x: P::F::ZERO,
        y: P::F::ONE,
        z: P::F::ONE,
        t: P::F::ZERO,
    };
}

impl<P: TwistedEdwardsGroupParams> AdditiveAbelianGroup for TwistedEdwardsGroupExtended<P> {}
