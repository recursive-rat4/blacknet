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

use bytemuck::NoUninit;
use core::fmt::{Debug, Display, Formatter, Result as FmtResult};
use core::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};
use serde::{Deserialize, Serialize};

// A timestamp or a time interval measured in milliseconds. The value may be negative.

#[derive(Clone, Copy, Deserialize, Eq, NoUninit, Ord, PartialEq, PartialOrd, Serialize)]
#[repr(transparent)]
pub struct Milliseconds {
    n: i64,
}

impl Milliseconds {
    pub const fn new(n: i64) -> Self {
        Self { n }
    }

    pub const fn from_seconds(n: i64) -> Self {
        Self { n: n * 1000 }
    }

    pub const fn from_minutes(n: i64) -> Self {
        Self { n: n * 60000 }
    }

    pub const fn from_hours(n: i64) -> Self {
        Self { n: n * 3600000 }
    }

    pub const fn from_days(n: i64) -> Self {
        Self { n: n * 86400000 }
    }

    pub const MAX: Self = Self { n: i64::MAX };
    pub const MIN: Self = Self { n: i64::MIN };
    pub const ZERO: Self = Self { n: 0 };
}

impl Debug for Milliseconds {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.n)
    }
}

impl Default for Milliseconds {
    #[inline]
    fn default() -> Self {
        Self::ZERO
    }
}

impl Display for Milliseconds {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.n)
    }
}

impl From<i64> for Milliseconds {
    fn from(n: i64) -> Self {
        Self { n }
    }
}

impl From<Milliseconds> for i64 {
    fn from(millis: Milliseconds) -> Self {
        millis.n
    }
}

impl TryFrom<Milliseconds> for core::time::Duration {
    type Error = core::num::TryFromIntError;

    fn try_from(millis: Milliseconds) -> Result<Self, Self::Error> {
        Ok(Self::from_millis(millis.n.try_into()?))
    }
}

impl Add for Milliseconds {
    type Output = Self;

    fn add(self, rps: Self) -> Self::Output {
        Self { n: self.n + rps.n }
    }
}

impl AddAssign for Milliseconds {
    #[inline]
    fn add_assign(&mut self, rps: Self) {
        *self = *self + rps
    }
}

impl Neg for Milliseconds {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self { n: -self.n }
    }
}

impl Sub for Milliseconds {
    type Output = Self;

    fn sub(self, rps: Self) -> Self::Output {
        Self { n: self.n - rps.n }
    }
}

impl SubAssign for Milliseconds {
    #[inline]
    fn sub_assign(&mut self, rps: Self) {
        *self = *self - rps
    }
}

impl Mul<i64> for Milliseconds {
    type Output = Self;

    fn mul(self, rps: i64) -> Self::Output {
        Self { n: self.n * rps }
    }
}

impl MulAssign<i64> for Milliseconds {
    #[inline]
    fn mul_assign(&mut self, rps: i64) {
        *self = *self * rps
    }
}

impl Mul<Milliseconds> for i64 {
    type Output = Milliseconds;

    fn mul(self, rps: Milliseconds) -> Self::Output {
        Milliseconds { n: self * rps.n }
    }
}

impl Div for Milliseconds {
    type Output = i64;

    fn div(self, rps: Self) -> Self::Output {
        self.n / rps.n
    }
}

impl Div<i64> for Milliseconds {
    type Output = Milliseconds;

    fn div(self, rps: i64) -> Self::Output {
        Self { n: self.n / rps }
    }
}

impl DivAssign<i64> for Milliseconds {
    #[inline]
    fn div_assign(&mut self, rps: i64) {
        *self = *self / rps
    }
}

impl Rem for Milliseconds {
    type Output = Self;

    fn rem(self, rps: Self) -> Self::Output {
        Self { n: self.n % rps.n }
    }
}

impl RemAssign for Milliseconds {
    #[inline]
    fn rem_assign(&mut self, rps: Self) {
        *self = *self % rps
    }
}

impl Rem<i64> for Milliseconds {
    type Output = Self;

    fn rem(self, rps: i64) -> Self::Output {
        Self { n: self.n % rps }
    }
}

impl RemAssign<i64> for Milliseconds {
    #[inline]
    fn rem_assign(&mut self, rps: i64) {
        *self = *self % rps
    }
}
