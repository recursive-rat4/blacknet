/*
 * Copyright (c) 2018-2025 Pavel Vasin
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

use core::fmt::{Display, Formatter, Result};
use core::iter::Sum;
use core::ops::{Add, AddAssign, Sub, SubAssign};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Default, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct Amount {
    value: u64,
}

impl Amount {
    pub const ZERO: Self = Self { value: 0 };
    /**
     * The satoshi is a monetary unit of many cryptocurrencies
     */
    pub const COIN: Self = Self { value: 100000000 };

    pub const fn new(value: u64) -> Self {
        Self { value }
    }

    pub const fn value(self) -> u64 {
        self.value
    }
}

impl Display for Amount {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.value)
    }
}

impl From<u64> for Amount {
    fn from(value: u64) -> Self {
        Self { value }
    }
}

impl From<Amount> for u64 {
    fn from(amount: Amount) -> Self {
        amount.value
    }
}

impl Add for Amount {
    type Output = Self;

    fn add(self, rps: Self) -> Self::Output {
        Self::from(self.value + rps.value)
    }
}

impl AddAssign for Amount {
    fn add_assign(&mut self, rps: Self) {
        self.value += rps.value;
    }
}

impl Sub for Amount {
    type Output = Self;

    fn sub(self, rps: Self) -> Self::Output {
        Self::from(self.value - rps.value)
    }
}

impl SubAssign for Amount {
    fn sub_assign(&mut self, rps: Self) {
        self.value -= rps.value;
    }
}

impl Sum for Amount {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|lps, rps| lps + rps).unwrap_or(Self::ZERO)
    }
}
