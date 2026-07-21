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
    AdditiveCommutativeMagma, AdditiveSemigroup, Double, Inv, LeftZero, One, RightZero, Set,
    Square, Zero, add_sub_chain, bl_double_and_add,
};
use crate::branchless::BlSelect;
use crate::ed25519::{E25519_D, Edwards25519Affine, Field25519, is_on_curve25519};
use core::fmt::{Debug, Formatter, Result};
use core::iter::Sum;
use core::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use zeroize::DefaultIsZeroes as ZeroizeIsDefault;

#[derive(Clone, Copy)]
pub struct Edwards25519Projective {
    x: Field25519,
    y: Field25519,
    z: Field25519,
}

impl Edwards25519Projective {
    pub fn new(x: Field25519, y: Field25519) -> Option<Self> {
        if is_on_curve25519(x, y) {
            Some(Self {
                x,
                y,
                z: Field25519::ONE,
            })
        } else {
            None
        }
    }

    /// # Safety
    /// Point `(x, y)` is on the curve.
    pub const unsafe fn from_unchecked(x: Field25519, y: Field25519) -> Self {
        Self {
            x,
            y,
            z: Field25519::ONE,
        }
    }

    pub fn scale(self) -> Self {
        let a = self.z.inv().expect("Elliptic curve arithmetic");
        Self {
            x: self.x * a,
            y: self.y * a,
            z: Field25519::ONE,
        }
    }

    pub fn bl_mul<Scalar: IntoIterator<Item = bool>>(self, rps: Scalar) -> Self {
        bl_double_and_add::<Self, Scalar>(self, rps)
    }
}

impl From<Edwards25519Projective> for Edwards25519Affine {
    fn from(projective: Edwards25519Projective) -> Self {
        let a = projective.z.inv().expect("Elliptic curve arithmetic");
        unsafe { Self::from_unchecked(projective.x * a, projective.y * a) }
    }
}

impl Debug for Edwards25519Projective {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({:?}, {:?}, {:?}", self.x, self.y, self.z)
    }
}

impl Default for Edwards25519Projective {
    fn default() -> Self {
        Self::ZERO
    }
}

impl PartialEq for Edwards25519Projective {
    fn eq(&self, rps: &Self) -> bool {
        (self.x * rps.z == self.z * rps.x) && (self.y * rps.z == self.z * rps.y)
    }
}

impl Eq for Edwards25519Projective {}

impl Add for Edwards25519Projective {
    type Output = Self;

    fn add(self, rps: Self) -> Self::Output {
        // add-2008-bbjlp
        let x1x2 = self.x * rps.x;
        let y1y2 = self.y * rps.y;
        let z1z2 = self.z * rps.z;
        let b = z1z2.square();
        let e = E25519_D * x1x2 * y1y2;
        let f = b.sub_lazy(e);
        let g = b.add_lazy(e);
        let j = y1y2.add_lazy(x1x2);
        let xr = z1z2 * f * ((self.x.add_lazy(self.y)) * (rps.x.add_lazy(rps.y)) - (y1y2 + x1x2));
        let yr = z1z2 * g * j;
        let zr = f * g;
        Self {
            x: xr,
            y: yr,
            z: zr,
        }
    }
}

impl Add<&Self> for Edwards25519Projective {
    type Output = Self;

    #[inline]
    fn add(self, rps: &Self) -> Self::Output {
        self + *rps
    }
}

impl Add<Edwards25519Projective> for &Edwards25519Projective {
    type Output = Edwards25519Projective;

    #[inline]
    fn add(self, rps: Edwards25519Projective) -> Self::Output {
        *self + rps
    }
}

impl<'a> Add<&'a Edwards25519Projective> for &Edwards25519Projective {
    type Output = Edwards25519Projective;

    #[inline]
    fn add(self, rps: &'a Edwards25519Projective) -> Self::Output {
        *self + *rps
    }
}

impl AddAssign for Edwards25519Projective {
    #[inline]
    fn add_assign(&mut self, rps: Self) {
        *self = *self + rps
    }
}

impl AddAssign<&Self> for Edwards25519Projective {
    #[inline]
    fn add_assign(&mut self, rps: &Self) {
        *self = *self + rps
    }
}

impl Double for Edwards25519Projective {
    type Output = Self;

    fn double(self) -> Self {
        // dbl-2008-bbjlp
        let b = (self.x.add_lazy(self.y)).square();
        let xx = self.x.square();
        let yy = self.y.square();
        let zz = self.z.square();
        let e = xx.neg_lazy() - yy;
        let f = yy - xx;
        let j = f.sub_lazy(zz.double());
        let xr = (b.add_lazy(e)) * j;
        let yr = f * e;
        let zr = f * j;
        Self {
            x: xr,
            y: yr,
            z: zr,
        }
    }
}

impl Double for &Edwards25519Projective {
    type Output = Edwards25519Projective;

    #[inline]
    fn double(self) -> Self::Output {
        (*self).double()
    }
}

impl Neg for Edwards25519Projective {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: self.y,
            z: self.z,
        }
    }
}

impl Neg for &Edwards25519Projective {
    type Output = Edwards25519Projective;

    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: self.y,
            z: self.z,
        }
    }
}

impl Sub for Edwards25519Projective {
    type Output = Self;

    fn sub(self, rps: Self) -> Self::Output {
        let x1x2 = self.x * rps.x;
        let y1y2 = self.y * rps.y;
        let z1z2 = self.z * rps.z;
        let b = z1z2.square();
        let e = E25519_D * x1x2 * y1y2;
        let f = b.add_lazy(e);
        let g = b.sub_lazy(e);
        let j = y1y2.sub_lazy(x1x2);
        let xr = z1z2 * f * ((self.x.add_lazy(self.y)) * (rps.y.sub_lazy(rps.x)) - (y1y2 - x1x2));
        let yr = z1z2 * g * j;
        let zr = f * g;
        Self {
            x: xr,
            y: yr,
            z: zr,
        }
    }
}

impl Sub<&Self> for Edwards25519Projective {
    type Output = Self;

    #[inline]
    fn sub(self, rps: &Self) -> Self::Output {
        self - *rps
    }
}

impl Sub<Edwards25519Projective> for &Edwards25519Projective {
    type Output = Edwards25519Projective;

    #[inline]
    fn sub(self, rps: Edwards25519Projective) -> Self::Output {
        *self - rps
    }
}

impl<'a> Sub<&'a Edwards25519Projective> for &Edwards25519Projective {
    type Output = Edwards25519Projective;

    #[inline]
    fn sub(self, rps: &'a Edwards25519Projective) -> Self::Output {
        *self - *rps
    }
}

impl SubAssign for Edwards25519Projective {
    #[inline]
    fn sub_assign(&mut self, rps: Self) {
        *self = *self - rps
    }
}

impl SubAssign<&Self> for Edwards25519Projective {
    #[inline]
    fn sub_assign(&mut self, rps: &Self) {
        *self = *self - rps
    }
}

impl<Scalar: IntoIterator<Item = bool>> Mul<Scalar> for Edwards25519Projective {
    type Output = Self;

    #[inline]
    fn mul(self, rps: Scalar) -> Self::Output {
        add_sub_chain(self, rps)
    }
}

impl<Scalar: IntoIterator<Item = bool>> MulAssign<Scalar> for Edwards25519Projective {
    #[inline]
    fn mul_assign(&mut self, rps: Scalar) {
        *self = *self * rps
    }
}

impl Sum for Edwards25519Projective {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|lps, rps| lps + rps).unwrap_or(Self::ZERO)
    }
}

impl<'a> Sum<&'a Self> for Edwards25519Projective {
    #[inline]
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.copied().sum()
    }
}

impl LeftZero for Edwards25519Projective {
    const LEFT_ZERO: Self = Self {
        x: Field25519::ZERO,
        y: Field25519::ONE,
        z: Field25519::ONE,
    };
}

impl RightZero for Edwards25519Projective {
    const RIGHT_ZERO: Self = Self {
        x: Field25519::ZERO,
        y: Field25519::ONE,
        z: Field25519::ONE,
    };
}

impl Zero for Edwards25519Projective {
    const ZERO: Self = Self {
        x: Field25519::ZERO,
        y: Field25519::ONE,
        z: Field25519::ONE,
    };
}

impl Set for Edwards25519Projective {}

impl AdditiveCommutativeMagma for Edwards25519Projective {}

impl AdditiveSemigroup for Edwards25519Projective {}

impl BlSelect for Edwards25519Projective {
    type Output = Self;

    fn bl_select(self, rps: Self, condition: bool) -> Self {
        Self {
            x: self.x.bl_select(rps.x, condition),
            y: self.y.bl_select(rps.y, condition),
            z: self.z.bl_select(rps.z, condition),
        }
    }
}

impl BlSelect<&Self> for Edwards25519Projective {
    type Output = Self;

    fn bl_select(self, rps: &Self, condition: bool) -> Self {
        Self {
            x: self.x.bl_select(&rps.x, condition),
            y: self.y.bl_select(&rps.y, condition),
            z: self.z.bl_select(&rps.z, condition),
        }
    }
}

impl BlSelect<Edwards25519Projective> for &Edwards25519Projective {
    type Output = Edwards25519Projective;

    fn bl_select(self, rps: Edwards25519Projective, condition: bool) -> Self::Output {
        Self::Output {
            x: (&self.x).bl_select(rps.x, condition),
            y: (&self.y).bl_select(rps.y, condition),
            z: (&self.z).bl_select(rps.z, condition),
        }
    }
}

impl BlSelect for &Edwards25519Projective {
    type Output = Edwards25519Projective;

    fn bl_select(self, rps: Self, condition: bool) -> Self::Output {
        Self::Output {
            x: (&self.x).bl_select(&rps.x, condition),
            y: (&self.y).bl_select(&rps.y, condition),
            z: (&self.z).bl_select(&rps.z, condition),
        }
    }
}

impl ZeroizeIsDefault for Edwards25519Projective {}
