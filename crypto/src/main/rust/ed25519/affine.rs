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
    AdditiveCommutativeMagma, AdditiveSemigroup, Double, IntegerModRing, LeftZero, One, RightZero,
    Set, Sqrt, Square, Zero, add_sub_chain, bl_double_and_add,
};
use crate::bigint::UInt256;
use crate::branchless::BlSelect;
use crate::ed25519::{E25519_D, Field25519, is_on_curve25519};
use core::fmt;
use core::iter::Sum;
use core::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error};
use zeroize::DefaultIsZeroes as ZeroizeIsDefault;

#[derive(Clone, Copy)]
pub struct Edwards25519Affine {
    x: Field25519,
    y: Field25519,
}

impl Edwards25519Affine {
    pub fn new(x: Field25519, y: Field25519) -> Option<Self> {
        if is_on_curve25519(x, y) {
            Some(Self { x, y })
        } else {
            None
        }
    }

    /// # Safety
    /// Point `(x, y)` is on the curve.
    pub const unsafe fn from_unchecked(x: Field25519, y: Field25519) -> Self {
        Self { x, y }
    }

    pub fn try_from_y(x_is_odd: bool, y: Field25519) -> Option<Self> {
        let yy = y.square();
        let xx = ((yy - Field25519::ONE) / (E25519_D * yy + Field25519::ONE))
            .expect("−d is not a square");
        let x = xx.sqrt()?;
        let n_is_odd = x.canonical().is_odd();
        if x_is_odd == n_is_odd {
            Some(Self { x, y })
        } else {
            Some(Self { x: -x, y })
        }
    }

    /// The x-coordinate.
    pub const fn x(&self) -> &Field25519 {
        &self.x
    }

    /// The y-coordinate.
    pub const fn y(&self) -> &Field25519 {
        &self.y
    }

    pub fn bl_mul<Scalar: IntoIterator<Item = bool>>(self, rps: Scalar) -> Self {
        bl_double_and_add::<Self, Scalar>(self, rps)
    }

    pub fn encode(&self) -> [u8; 32] {
        let mut bytes: [u8; 32] = self.y().canonical().to_le_bytes();
        let x_is_odd = self.x().canonical().is_odd();
        bytes[31] |= (x_is_odd as u8) << 7;
        bytes
    }

    pub fn decode(mut bytes: [u8; 32]) -> Option<Self> {
        let x_is_odd = (bytes[31] & 0x80) != 0;
        bytes[31] &= 0x7F;
        let n = UInt256::from_le_bytes(bytes);
        let y = Field25519::new(n);
        Self::try_from_y(x_is_odd, y)
    }
}

impl From<Edwards25519Affine> for (Field25519, Field25519) {
    fn from(point: Edwards25519Affine) -> Self {
        (point.x, point.y)
    }
}

impl fmt::Debug for Edwards25519Affine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:?}, {:?})", self.x, self.y)
    }
}

impl Default for Edwards25519Affine {
    #[inline]
    fn default() -> Self {
        Self::ZERO
    }
}

impl PartialEq for Edwards25519Affine {
    fn eq(&self, rps: &Self) -> bool {
        self.x == rps.x && self.y == rps.y
    }
}

impl Eq for Edwards25519Affine {}

impl Add for Edwards25519Affine {
    type Output = Self;

    fn add(self, rps: Self) -> Self::Output {
        let x1x2 = self.x * rps.x;
        let y1y2 = self.y * rps.y;
        let k = E25519_D * x1x2 * y1y2;
        let xr = (self.x * rps.y + self.y * rps.x) / (Field25519::ONE + k);
        let yr = (y1y2 + x1x2) / (Field25519::ONE - k);
        Self {
            x: xr.expect("Elliptic curve arithmetic"),
            y: yr.expect("Elliptic curve arithmetic"),
        }
    }
}

impl Add<&Self> for Edwards25519Affine {
    type Output = Self;

    #[inline]
    fn add(self, rps: &Self) -> Self::Output {
        self + *rps
    }
}

impl Add<Edwards25519Affine> for &Edwards25519Affine {
    type Output = Edwards25519Affine;

    #[inline]
    fn add(self, rps: Edwards25519Affine) -> Self::Output {
        *self + rps
    }
}

impl<'a> Add<&'a Edwards25519Affine> for &Edwards25519Affine {
    type Output = Edwards25519Affine;

    #[inline]
    fn add(self, rps: &'a Edwards25519Affine) -> Self::Output {
        *self + *rps
    }
}

impl AddAssign for Edwards25519Affine {
    #[inline]
    fn add_assign(&mut self, rps: Self) {
        *self = *self + rps
    }
}

impl AddAssign<&Self> for Edwards25519Affine {
    #[inline]
    fn add_assign(&mut self, rps: &Self) {
        *self = *self + rps
    }
}

impl Double for Edwards25519Affine {
    type Output = Self;

    fn double(self) -> Self {
        let xx = self.x.square();
        let yy = self.y.square();
        let k = E25519_D * xx * yy;
        let xr = (self.x * self.y).double() / (Field25519::ONE + k);
        let yr = (yy + xx) / (Field25519::ONE - k);
        Self {
            x: xr.expect("Elliptic curve arithmetic"),
            y: yr.expect("Elliptic curve arithmetic"),
        }
    }
}

impl Double for &Edwards25519Affine {
    type Output = Edwards25519Affine;

    #[inline]
    fn double(self) -> Self::Output {
        (*self).double()
    }
}

impl Neg for Edwards25519Affine {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: self.y,
        }
    }
}

impl Neg for &Edwards25519Affine {
    type Output = Edwards25519Affine;

    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: self.y,
        }
    }
}

impl Sub for Edwards25519Affine {
    type Output = Self;

    fn sub(self, rps: Self) -> Self::Output {
        let x1x2 = self.x * rps.x;
        let y1y2 = self.y * rps.y;
        let k = E25519_D * x1x2 * y1y2;
        let xr = (self.x * rps.y - self.y * rps.x) / (Field25519::ONE - k);
        let yr = (y1y2 - x1x2) / (Field25519::ONE + k);
        Self {
            x: xr.expect("Elliptic curve arithmetic"),
            y: yr.expect("Elliptic curve arithmetic"),
        }
    }
}

impl Sub<&Self> for Edwards25519Affine {
    type Output = Self;

    #[inline]
    fn sub(self, rps: &Self) -> Self::Output {
        self - *rps
    }
}

impl Sub<Edwards25519Affine> for &Edwards25519Affine {
    type Output = Edwards25519Affine;

    #[inline]
    fn sub(self, rps: Edwards25519Affine) -> Self::Output {
        *self - rps
    }
}

impl<'a> Sub<&'a Edwards25519Affine> for &Edwards25519Affine {
    type Output = Edwards25519Affine;

    #[inline]
    fn sub(self, rps: &'a Edwards25519Affine) -> Self::Output {
        *self - *rps
    }
}

impl SubAssign for Edwards25519Affine {
    #[inline]
    fn sub_assign(&mut self, rps: Self) {
        *self = *self - rps
    }
}

impl SubAssign<&Self> for Edwards25519Affine {
    #[inline]
    fn sub_assign(&mut self, rps: &Self) {
        *self = *self - rps
    }
}

impl<Scalar: IntoIterator<Item = bool>> Mul<Scalar> for Edwards25519Affine {
    type Output = Self;

    #[inline]
    fn mul(self, rps: Scalar) -> Self::Output {
        add_sub_chain(self, rps)
    }
}

impl<Scalar: IntoIterator<Item = bool>> MulAssign<Scalar> for Edwards25519Affine {
    #[inline]
    fn mul_assign(&mut self, rps: Scalar) {
        *self = *self * rps
    }
}

impl Sum for Edwards25519Affine {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|lps, rps| lps + rps).unwrap_or(Self::ZERO)
    }
}

impl<'a> Sum<&'a Self> for Edwards25519Affine {
    #[inline]
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.copied().sum()
    }
}

impl LeftZero for Edwards25519Affine {
    const LEFT_ZERO: Self = Self {
        x: Field25519::ZERO,
        y: Field25519::ONE,
    };
}

impl RightZero for Edwards25519Affine {
    const RIGHT_ZERO: Self = Self {
        x: Field25519::ZERO,
        y: Field25519::ONE,
    };
}

impl Zero for Edwards25519Affine {
    const ZERO: Self = Self {
        x: Field25519::ZERO,
        y: Field25519::ONE,
    };
}

impl Set for Edwards25519Affine {}

impl AdditiveCommutativeMagma for Edwards25519Affine {}

impl AdditiveSemigroup for Edwards25519Affine {}

impl BlSelect for Edwards25519Affine {
    type Output = Self;

    fn bl_select(self, rps: Self, condition: bool) -> Self {
        Self {
            x: self.x.bl_select(rps.x, condition),
            y: self.y.bl_select(rps.y, condition),
        }
    }
}

impl BlSelect<&Self> for Edwards25519Affine {
    type Output = Self;

    fn bl_select(self, rps: &Self, condition: bool) -> Self {
        Self {
            x: self.x.bl_select(&rps.x, condition),
            y: self.y.bl_select(&rps.y, condition),
        }
    }
}

impl BlSelect<Edwards25519Affine> for &Edwards25519Affine {
    type Output = Edwards25519Affine;

    fn bl_select(self, rps: Edwards25519Affine, condition: bool) -> Self::Output {
        Self::Output {
            x: (&self.x).bl_select(rps.x, condition),
            y: (&self.y).bl_select(rps.y, condition),
        }
    }
}

impl BlSelect for &Edwards25519Affine {
    type Output = Edwards25519Affine;

    fn bl_select(self, rps: Self, condition: bool) -> Self::Output {
        Self::Output {
            x: (&self.x).bl_select(&rps.x, condition),
            y: (&self.y).bl_select(&rps.y, condition),
        }
    }
}

impl Serialize for Edwards25519Affine {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let bytes: [u8; 32] = self.encode();
        bytes.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Edwards25519Affine {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let bytes = <[u8; 32]>::deserialize(deserializer)?;
        Self::decode(bytes).ok_or_else(|| D::Error::custom("Not a point on the elliptic curve"))
    }
}

impl ZeroizeIsDefault for Edwards25519Affine {}
