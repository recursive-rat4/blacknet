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
use crate::ed25519::{E25519_D_TWICE, Edwards25519Affine, Field25519, is_on_curve25519};
use core::fmt::{Debug, Formatter, Result};
use core::iter::Sum;
use core::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use zeroize::DefaultIsZeroes as ZeroizeIsDefault;

#[derive(Clone, Copy)]
pub struct Edwards25519Extended {
    x: Field25519,
    y: Field25519,
    z: Field25519,
    t: Field25519,
}

impl Edwards25519Extended {
    pub fn new(x: Field25519, y: Field25519) -> Option<Self> {
        if is_on_curve25519(x, y) {
            Some(Self {
                x,
                y,
                z: Field25519::ONE,
                t: x * y,
            })
        } else {
            None
        }
    }

    /// # Safety
    /// Point `(x, y)` is on the curve.
    pub unsafe fn from_unchecked(x: Field25519, y: Field25519) -> Self {
        Self {
            x,
            y,
            z: Field25519::ONE,
            t: x * y,
        }
    }

    /// # Safety
    /// Point `(x, y, z, t)` is on the curve.
    pub const unsafe fn const_from_unchecked(
        x: Field25519,
        y: Field25519,
        z: Field25519,
        t: Field25519,
    ) -> Self {
        Self { x, y, z, t }
    }

    pub fn scale(self) -> Self {
        let a = self.z.inv().expect("Elliptic curve arithmetic");
        Self {
            x: self.x * a,
            y: self.y * a,
            z: Field25519::ONE,
            t: self.t * a,
        }
    }

    pub fn bl_mul<Scalar: IntoIterator<Item = bool>>(self, rps: Scalar) -> Self {
        bl_double_and_add::<Self, Scalar>(self, rps)
    }
}

impl From<Edwards25519Affine> for Edwards25519Extended {
    fn from(affine: Edwards25519Affine) -> Self {
        let (x, y) = affine.into();
        unsafe { Self::from_unchecked(x, y) }
    }
}

impl From<Edwards25519Extended> for Edwards25519Affine {
    fn from(extended: Edwards25519Extended) -> Self {
        let a = extended.z.inv().expect("Elliptic curve arithmetic");
        unsafe { Self::from_unchecked(extended.x * a, extended.y * a) }
    }
}

impl Debug for Edwards25519Extended {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "({:?}, {:?}, {:?}, {:?})",
            self.x, self.y, self.z, self.t
        )
    }
}

impl Default for Edwards25519Extended {
    fn default() -> Self {
        Self::ZERO
    }
}

impl PartialEq for Edwards25519Extended {
    fn eq(&self, rps: &Self) -> bool {
        (self.x * rps.z == self.z * rps.x) && (self.y * rps.z == self.z * rps.y)
    }
}

impl Eq for Edwards25519Extended {}

impl Add for Edwards25519Extended {
    type Output = Self;

    fn add(self, rps: Self) -> Self::Output {
        // add-2008-hwcd-3
        let a = (self.y.sub_lazy(self.x)) * (rps.y.sub_lazy(rps.x));
        let b = (self.y.add_lazy(self.x)) * (rps.y.add_lazy(rps.x));
        let c = self.t * E25519_D_TWICE * rps.t;
        let d = self.z.double_lazy() * rps.z;
        let e = b.sub_lazy(a);
        let f = d.sub_lazy(c);
        let g = d.add_lazy(c);
        let h = b.add_lazy(a);
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

impl Add<&Self> for Edwards25519Extended {
    type Output = Self;

    #[inline]
    fn add(self, rps: &Self) -> Self::Output {
        self + *rps
    }
}

impl Add<Edwards25519Extended> for &Edwards25519Extended {
    type Output = Edwards25519Extended;

    #[inline]
    fn add(self, rps: Edwards25519Extended) -> Self::Output {
        *self + rps
    }
}

impl<'a> Add<&'a Edwards25519Extended> for &Edwards25519Extended {
    type Output = Edwards25519Extended;

    #[inline]
    fn add(self, rps: &'a Edwards25519Extended) -> Self::Output {
        *self + *rps
    }
}

impl AddAssign for Edwards25519Extended {
    #[inline]
    fn add_assign(&mut self, rps: Self) {
        *self = *self + rps
    }
}

impl AddAssign<&Self> for Edwards25519Extended {
    #[inline]
    fn add_assign(&mut self, rps: &Self) {
        *self = *self + rps
    }
}

impl Double for Edwards25519Extended {
    type Output = Self;

    fn double(self) -> Self {
        // dbl-2008-hwcd
        let xx = self.x.square();
        let yy = self.y.square();
        let zz2 = self.z.square().double();
        let d = xx + yy;
        let e = (self.x.add_lazy(self.y)).square().sub_lazy(d);
        let g = yy - xx;
        let f = g.sub_lazy(zz2);
        let h = d.neg_lazy();
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

impl Double for &Edwards25519Extended {
    type Output = Edwards25519Extended;

    #[inline]
    fn double(self) -> Self::Output {
        (*self).double()
    }
}

impl Neg for Edwards25519Extended {
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

impl Neg for &Edwards25519Extended {
    type Output = Edwards25519Extended;

    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: self.y,
            z: self.z,
            t: -self.t,
        }
    }
}

impl Sub for Edwards25519Extended {
    type Output = Self;

    fn sub(self, rps: Self) -> Self::Output {
        let a = (self.y.sub_lazy(self.x)) * (rps.y.add_lazy(rps.x));
        let b = (self.y.add_lazy(self.x)) * (rps.y.sub_lazy(rps.x));
        let c = self.t * E25519_D_TWICE * rps.t;
        let d = self.z.double_lazy() * rps.z;
        let e = b.sub_lazy(a);
        let f = d.add_lazy(c);
        let g = d.sub_lazy(c);
        let h = b.add_lazy(a);
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

impl Sub<&Self> for Edwards25519Extended {
    type Output = Self;

    #[inline]
    fn sub(self, rps: &Self) -> Self::Output {
        self - *rps
    }
}

impl Sub<Edwards25519Extended> for &Edwards25519Extended {
    type Output = Edwards25519Extended;

    #[inline]
    fn sub(self, rps: Edwards25519Extended) -> Self::Output {
        *self - rps
    }
}

impl<'a> Sub<&'a Edwards25519Extended> for &Edwards25519Extended {
    type Output = Edwards25519Extended;

    #[inline]
    fn sub(self, rps: &'a Edwards25519Extended) -> Self::Output {
        *self - *rps
    }
}

impl SubAssign for Edwards25519Extended {
    #[inline]
    fn sub_assign(&mut self, rps: Self) {
        *self = *self - rps
    }
}

impl SubAssign<&Self> for Edwards25519Extended {
    #[inline]
    fn sub_assign(&mut self, rps: &Self) {
        *self = *self - rps
    }
}

impl<Scalar: IntoIterator<Item = bool>> Mul<Scalar> for Edwards25519Extended {
    type Output = Self;

    #[inline]
    fn mul(self, rps: Scalar) -> Self::Output {
        add_sub_chain(self, rps)
    }
}

impl<Scalar: IntoIterator<Item = bool>> MulAssign<Scalar> for Edwards25519Extended {
    #[inline]
    fn mul_assign(&mut self, rps: Scalar) {
        *self = *self * rps
    }
}

impl Sum for Edwards25519Extended {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|lps, rps| lps + rps).unwrap_or(Self::ZERO)
    }
}

impl<'a> Sum<&'a Self> for Edwards25519Extended {
    #[inline]
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.copied().sum()
    }
}

impl LeftZero for Edwards25519Extended {
    const LEFT_ZERO: Self = Self {
        x: Field25519::ZERO,
        y: Field25519::ONE,
        z: Field25519::ONE,
        t: Field25519::ZERO,
    };
}

impl RightZero for Edwards25519Extended {
    const RIGHT_ZERO: Self = Self {
        x: Field25519::ZERO,
        y: Field25519::ONE,
        z: Field25519::ONE,
        t: Field25519::ZERO,
    };
}

impl Zero for Edwards25519Extended {
    const ZERO: Self = Self {
        x: Field25519::ZERO,
        y: Field25519::ONE,
        z: Field25519::ONE,
        t: Field25519::ZERO,
    };
}

impl Set for Edwards25519Extended {}

impl AdditiveCommutativeMagma for Edwards25519Extended {}

impl AdditiveSemigroup for Edwards25519Extended {}

impl BlSelect for Edwards25519Extended {
    type Output = Self;

    fn bl_select(self, rps: Self, condition: bool) -> Self {
        Self {
            x: self.x.bl_select(rps.x, condition),
            y: self.y.bl_select(rps.y, condition),
            z: self.z.bl_select(rps.z, condition),
            t: self.t.bl_select(rps.t, condition),
        }
    }
}

impl BlSelect<&Self> for Edwards25519Extended {
    type Output = Self;

    fn bl_select(self, rps: &Self, condition: bool) -> Self {
        Self {
            x: self.x.bl_select(&rps.x, condition),
            y: self.y.bl_select(&rps.y, condition),
            z: self.z.bl_select(&rps.z, condition),
            t: self.t.bl_select(&rps.t, condition),
        }
    }
}

impl BlSelect<Edwards25519Extended> for &Edwards25519Extended {
    type Output = Edwards25519Extended;

    fn bl_select(self, rps: Edwards25519Extended, condition: bool) -> Self::Output {
        Self::Output {
            x: (&self.x).bl_select(rps.x, condition),
            y: (&self.y).bl_select(rps.y, condition),
            z: (&self.z).bl_select(rps.z, condition),
            t: (&self.t).bl_select(rps.t, condition),
        }
    }
}

impl BlSelect for &Edwards25519Extended {
    type Output = Edwards25519Extended;

    fn bl_select(self, rps: Self, condition: bool) -> Self::Output {
        Self::Output {
            x: (&self.x).bl_select(&rps.x, condition),
            y: (&self.y).bl_select(&rps.y, condition),
            z: (&self.z).bl_select(&rps.z, condition),
            t: (&self.t).bl_select(&rps.t, condition),
        }
    }
}

impl ZeroizeIsDefault for Edwards25519Extended {}
