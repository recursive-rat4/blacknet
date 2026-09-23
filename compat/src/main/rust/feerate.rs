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

use bytemuck::NoUninit;
use core::{fmt, num::ParseIntError, str::FromStr};
use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{Error as DeError, Unexpected, Visitor},
};

#[derive(Clone, Copy, Debug, Eq, NoUninit, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct FeeRate(u64);

impl FeeRate {
    pub const MIN: Self = Self(u64::MIN);
    pub const MAX: Self = Self(u64::MAX);

    /// # Panics
    ///
    /// If `bytes == 0`.
    pub const fn floor(satoshis: u64, bytes: u32) -> Self {
        Self(satoshis / bytes as u64)
    }

    pub fn parse(string: &str) -> Result<Self, Error> {
        let value_end = string
            .chars()
            .position(|c| !c.is_ascii_digit())
            .unwrap_or(string.len());
        let value = u64::from_str(&string[..value_end])?;
        let unit = string[value_end..].trim();
        match unit {
            "" | "sat/B" => Ok(Self(value)),
            _ => Err(Error::Unit),
        }
    }

    pub fn to_string_sat(&self) -> String {
        let symbol = "sat/B";
        format!("{} {symbol}", self.0)
    }
}

impl From<u64> for FeeRate {
    fn from(n: u64) -> Self {
        Self(n)
    }
}

impl From<FeeRate> for u64 {
    fn from(fee_rate: FeeRate) -> Self {
        fee_rate.0
    }
}

impl Serialize for FeeRate {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        if !serializer.is_human_readable() {
            self.0.serialize(serializer)
        } else {
            self.to_string_sat().serialize(serializer)
        }
    }
}

impl<'de> Deserialize<'de> for FeeRate {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct FeeRateVisitor;
        impl Visitor<'_> for FeeRateVisitor {
            type Value = FeeRate;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("fee rate")
            }
            fn visit_u64<E: DeError>(self, v: u64) -> Result<Self::Value, E> {
                Ok(FeeRate(v))
            }
            fn visit_str<E: DeError>(self, v: &str) -> Result<Self::Value, E> {
                FeeRate::parse(v).map_err(E::custom)
            }
            fn visit_i64<E: DeError>(self, v: i64) -> Result<Self::Value, E> {
                if v >= 0 {
                    Ok(FeeRate(v as u64))
                } else {
                    Err(E::invalid_value(Unexpected::Signed(v), &"non-negative"))
                }
            }
        }
        if !deserializer.is_human_readable() {
            Ok(Self(u64::deserialize(deserializer)?))
        } else {
            deserializer.deserialize_unit_struct("FeeRate", FeeRateVisitor)
        }
    }
}

#[derive(Debug)]
pub enum Error {
    Int(ParseIntError),
    Unit,
}

impl From<ParseIntError> for Error {
    fn from(err: ParseIntError) -> Self {
        Self::Int(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Int(err) => write!(f, "{err}"),
            Self::Unit => f.write_str("Unknown unit symbol"),
        }
    }
}

impl core::error::Error for Error {}
