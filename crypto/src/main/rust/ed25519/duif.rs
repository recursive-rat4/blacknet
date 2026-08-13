/*
 * Copyright (c) 2026 Pavel Vasin
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

use crate::algebra::Set;
use crate::branchless::BlSelect;
use crate::ed25519::{E25519_D_TWICE, Edwards25519Affine, Edwards25519Extended, Field25519};
use core::fmt::{Debug, Formatter, Result};
use core::ops::{Add, AddAssign, Neg, Sub, SubAssign};

#[derive(Clone, Copy)]
pub struct Edwards25519Duif {
    ypx: Field25519,
    ymx: Field25519,
    t2d: Field25519,
}

impl Edwards25519Duif {
    pub fn new(x: Field25519, y: Field25519) -> Option<Self> {
        let affine = Edwards25519Affine::new(x, y)?;
        Some(affine.into())
    }

    /// # Safety
    /// Point is on the curve.
    pub const unsafe fn from_unchecked(ypx: Field25519, ymx: Field25519, t2d: Field25519) -> Self {
        Self { ypx, ymx, t2d }
    }
}

impl From<Edwards25519Affine> for Edwards25519Duif {
    fn from(affine: Edwards25519Affine) -> Self {
        let (x, y) = affine.into();
        Self {
            ypx: y + x,
            ymx: y - x,
            t2d: y * x * E25519_D_TWICE,
        }
    }
}

impl From<Edwards25519Duif> for Edwards25519Affine {
    fn from(duif: Edwards25519Duif) -> Self {
        let x = (duif.ypx - duif.ymx).halve();
        let y = (duif.ypx + duif.ymx).halve();
        unsafe { Self::from_unchecked(x, y) }
    }
}

impl Debug for Edwards25519Duif {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({:?}, {:?}, {:?}", self.ypx, self.ymx, self.t2d)
    }
}

impl PartialEq for Edwards25519Duif {
    fn eq(&self, rps: &Self) -> bool {
        (self.ypx == rps.ypx) && (self.ymx == rps.ymx)
    }
}

impl Eq for Edwards25519Duif {}

impl Add<Edwards25519Duif> for Edwards25519Extended {
    type Output = Self;

    fn add(self, rps: Edwards25519Duif) -> Self::Output {
        // add-2008-hwcd-3
        let (x, y, z, t) = self.into();
        let a = (y.sub_lazy(x)) * rps.ymx;
        let b = (y.add_lazy(x)) * rps.ypx;
        let c = t * rps.t2d;
        let d = z.double_lazy();
        let e = b.sub_lazy(a);
        let f = d.sub_lazy(c);
        let g = d.add_lazy(c);
        let h = b.add_lazy(a);
        let xr = e * f;
        let yr = g * h;
        let zr = f * g;
        let tr = e * h;
        unsafe { Self::const_from_unchecked(xr, yr, zr, tr) }
    }
}

impl Add<&Edwards25519Duif> for Edwards25519Extended {
    type Output = Self;

    #[inline]
    fn add(self, rps: &Edwards25519Duif) -> Self::Output {
        self + *rps
    }
}

impl Add<Edwards25519Duif> for &Edwards25519Extended {
    type Output = Edwards25519Extended;

    #[inline]
    fn add(self, rps: Edwards25519Duif) -> Self::Output {
        *self + rps
    }
}

impl<'a> Add<&'a Edwards25519Duif> for &Edwards25519Extended {
    type Output = Edwards25519Extended;

    #[inline]
    fn add(self, rps: &'a Edwards25519Duif) -> Self::Output {
        *self + *rps
    }
}

impl AddAssign<Edwards25519Duif> for Edwards25519Extended {
    #[inline]
    fn add_assign(&mut self, rps: Edwards25519Duif) {
        *self = *self + rps
    }
}

impl AddAssign<&Edwards25519Duif> for Edwards25519Extended {
    #[inline]
    fn add_assign(&mut self, rps: &Edwards25519Duif) {
        *self = *self + rps
    }
}

impl Neg for Edwards25519Duif {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            ypx: self.ymx,
            ymx: self.ypx,
            t2d: -self.t2d,
        }
    }
}

impl Neg for &Edwards25519Duif {
    type Output = Edwards25519Duif;

    fn neg(self) -> Self::Output {
        Self::Output {
            ypx: self.ymx,
            ymx: self.ypx,
            t2d: -self.t2d,
        }
    }
}

impl Sub<Edwards25519Duif> for Edwards25519Extended {
    type Output = Self;

    fn sub(self, rps: Edwards25519Duif) -> Self::Output {
        let (x, y, z, t) = self.into();
        let a = (y.sub_lazy(x)) * rps.ypx;
        let b = (y.add_lazy(x)) * rps.ymx;
        let c = t * rps.t2d;
        let d = z.double_lazy();
        let e = b.sub_lazy(a);
        let f = d.add_lazy(c);
        let g = d.sub_lazy(c);
        let h = b.add_lazy(a);
        let xr = e * f;
        let yr = g * h;
        let zr = f * g;
        let tr = e * h;
        unsafe { Self::const_from_unchecked(xr, yr, zr, tr) }
    }
}

impl Sub<&Edwards25519Duif> for Edwards25519Extended {
    type Output = Self;

    #[inline]
    fn sub(self, rps: &Edwards25519Duif) -> Self::Output {
        self - *rps
    }
}

impl Sub<Edwards25519Duif> for &Edwards25519Extended {
    type Output = Edwards25519Extended;

    #[inline]
    fn sub(self, rps: Edwards25519Duif) -> Self::Output {
        *self - rps
    }
}

impl<'a> Sub<&'a Edwards25519Duif> for &Edwards25519Extended {
    type Output = Edwards25519Extended;

    #[inline]
    fn sub(self, rps: &'a Edwards25519Duif) -> Self::Output {
        *self - *rps
    }
}

impl SubAssign<Edwards25519Duif> for Edwards25519Extended {
    #[inline]
    fn sub_assign(&mut self, rps: Edwards25519Duif) {
        *self = *self - rps
    }
}

impl SubAssign<&Edwards25519Duif> for Edwards25519Extended {
    #[inline]
    fn sub_assign(&mut self, rps: &Edwards25519Duif) {
        *self = *self - rps
    }
}

impl Set for Edwards25519Duif {}

impl BlSelect for Edwards25519Duif {
    type Output = Self;

    fn bl_select(self, rps: Self, condition: bool) -> Self {
        Self {
            ypx: self.ypx.bl_select(rps.ypx, condition),
            ymx: self.ymx.bl_select(rps.ymx, condition),
            t2d: self.t2d.bl_select(rps.t2d, condition),
        }
    }
}

impl BlSelect<&Self> for Edwards25519Duif {
    type Output = Self;

    fn bl_select(self, rps: &Self, condition: bool) -> Self {
        Self {
            ypx: self.ypx.bl_select(&rps.ypx, condition),
            ymx: self.ymx.bl_select(&rps.ymx, condition),
            t2d: self.t2d.bl_select(&rps.t2d, condition),
        }
    }
}

impl BlSelect<Edwards25519Duif> for &Edwards25519Duif {
    type Output = Edwards25519Duif;

    fn bl_select(self, rps: Edwards25519Duif, condition: bool) -> Self::Output {
        Self::Output {
            ypx: (&self.ypx).bl_select(rps.ypx, condition),
            ymx: (&self.ymx).bl_select(rps.ymx, condition),
            t2d: (&self.t2d).bl_select(rps.t2d, condition),
        }
    }
}

impl BlSelect for &Edwards25519Duif {
    type Output = Edwards25519Duif;

    fn bl_select(self, rps: Self, condition: bool) -> Self::Output {
        Self::Output {
            ypx: (&self.ypx).bl_select(&rps.ypx, condition),
            ymx: (&self.ymx).bl_select(&rps.ymx, condition),
            t2d: (&self.t2d).bl_select(&rps.t2d, condition),
        }
    }
}
