/*
 * Copyright (c) 2025-2026 Pavel Vasin
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

use crate::Milliseconds;
use bytemuck::NoUninit;
use core::fmt::{Debug, Display, Formatter, Result as FmtResult};
use core::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};
use serde::{Deserialize, Serialize};

/// A timestamp or a time interval measured in seconds. The value may be negative.
#[derive(Clone, Copy, Deserialize, Eq, NoUninit, Ord, PartialEq, PartialOrd, Serialize)]
#[repr(transparent)]
pub struct Seconds(i64);

impl Seconds {
    pub const fn new(n: i64) -> Self {
        Self(n)
    }

    pub const fn with_minutes(n: i64) -> Self {
        Self(n * 60)
    }

    pub const fn with_hours(n: i64) -> Self {
        Self(n * 3600)
    }

    pub const fn with_days(n: i64) -> Self {
        Self(n * 86400)
    }

    pub const fn value(self) -> i64 {
        self.0
    }

    pub const fn to_millis(self) -> Milliseconds {
        Milliseconds::new(self.0 * 1000)
    }

    pub const fn to_be_bytes(self) -> [u8; 8] {
        self.0.to_be_bytes()
    }

    pub const fn to_le_bytes(self) -> [u8; 8] {
        self.0.to_le_bytes()
    }

    /// The maximum value that can be represented by this type.
    pub const MAX: Self = Self(i64::MAX);
    /// The minimum value that can be represented by this type.
    pub const MIN: Self = Self(i64::MIN);
    pub const ZERO: Self = Self(0);
}

impl Debug for Seconds {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.0)
    }
}

impl Default for Seconds {
    #[inline]
    fn default() -> Self {
        Self::ZERO
    }
}

impl Display for Seconds {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.0)
    }
}

impl From<i64> for Seconds {
    fn from(n: i64) -> Self {
        Self(n)
    }
}

impl From<Seconds> for i64 {
    fn from(secs: Seconds) -> Self {
        secs.0
    }
}

impl TryFrom<Seconds> for core::time::Duration {
    type Error = core::num::TryFromIntError;

    fn try_from(secs: Seconds) -> Result<Self, Self::Error> {
        Ok(Self::from_secs(secs.0.try_into()?))
    }
}

impl Add for Seconds {
    type Output = Self;

    fn add(self, rps: Self) -> Self::Output {
        Self(self.0 + rps.0)
    }
}

impl AddAssign for Seconds {
    #[inline]
    fn add_assign(&mut self, rps: Self) {
        *self = *self + rps
    }
}

impl Neg for Seconds {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl Sub for Seconds {
    type Output = Self;

    fn sub(self, rps: Self) -> Self::Output {
        Self(self.0 - rps.0)
    }
}

impl SubAssign for Seconds {
    #[inline]
    fn sub_assign(&mut self, rps: Self) {
        *self = *self - rps
    }
}

impl Mul<i64> for Seconds {
    type Output = Self;

    fn mul(self, rps: i64) -> Self::Output {
        Self(self.0 * rps)
    }
}

impl MulAssign<i64> for Seconds {
    #[inline]
    fn mul_assign(&mut self, rps: i64) {
        *self = *self * rps
    }
}

impl Mul<Seconds> for i64 {
    type Output = Seconds;

    fn mul(self, rps: Seconds) -> Self::Output {
        Seconds(self * rps.0)
    }
}

impl Div for Seconds {
    type Output = i64;

    fn div(self, rps: Self) -> Self::Output {
        self.0 / rps.0
    }
}

impl Div<i64> for Seconds {
    type Output = Seconds;

    fn div(self, rps: i64) -> Self::Output {
        Self(self.0 / rps)
    }
}

impl DivAssign<i64> for Seconds {
    #[inline]
    fn div_assign(&mut self, rps: i64) {
        *self = *self / rps
    }
}

impl Rem for Seconds {
    type Output = Self;

    fn rem(self, rps: Self) -> Self::Output {
        Self(self.0 % rps.0)
    }
}

impl RemAssign for Seconds {
    #[inline]
    fn rem_assign(&mut self, rps: Self) {
        *self = *self % rps
    }
}

impl Rem<i64> for Seconds {
    type Output = Self;

    fn rem(self, rps: i64) -> Self::Output {
        Self(self.0 % rps)
    }
}

impl RemAssign<i64> for Seconds {
    #[inline]
    fn rem_assign(&mut self, rps: i64) {
        *self = *self % rps
    }
}
