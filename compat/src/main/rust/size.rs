/*
 * Copyright (c) 2020-2026 Pavel Vasin
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

use core::{fmt, num::ParseIntError, str::FromStr};
use serde::{
    Deserialize, Deserializer,
    de::{Error as DeError, Unexpected, Visitor},
};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Size(u64);

impl Size {
    pub fn parse(string: &str) -> Result<Self, Error> {
        let value_end = string
            .chars()
            .position(|c| !c.is_ascii_digit())
            .unwrap_or(string.len());
        let value = u64::from_str(&string[..value_end])?;
        let unit = string[value_end..].trim();
        let multiplier = match unit {
            "" => 1,
            "B" => 1,
            "kB" => 1000,
            "KiB" => 1024,
            // "KB" => 1000,
            // "KB" => 1024,
            "MB" => 1000000,
            "MiB" => 1048576,
            "GB" => 1000000000,
            "GiB" => 1073741824,
            _ => return Err(Error::Unit),
        };
        let n = value.checked_mul(multiplier).ok_or(Error::Overflow)?;
        Ok(Self(n))
    }

    pub fn to_string_lossy(&self, binary: bool) -> String {
        let (multiplier, symbol) = if binary {
            if self.0 >= 1073741824 {
                (1073741824, "GiB")
            } else if self.0 >= 1048576 {
                (1048576, "MiB")
            } else if self.0 >= 1024 {
                (1024, "KiB")
            } else {
                (1, "B")
            }
        } else {
            if self.0 >= 1000000000 {
                (1000000000, "GB")
            } else if self.0 >= 1000000 {
                (1000000, "MB")
            } else if self.0 >= 1000 {
                (1000, "kB")
            } else {
                (1, "B")
            }
        };
        let f = self.0 as f64 / multiplier as f64;
        let f = format!("{f:.2}");
        let f = f.trim_end_matches('0').trim_end_matches('.');
        format!("{f} {symbol}")
    }

    pub const fn as_bytes(self) -> u64 {
        self.0
    }

    pub const fn as_usize(self) -> usize {
        if usize::BITS >= u64::BITS {
            self.0 as usize
        } else {
            if self.0 <= usize::MAX as u64 {
                self.0 as usize
            } else {
                usize::MAX
            }
        }
    }

    pub const fn as_u32(self) -> u32 {
        if self.0 <= u32::MAX as u64 {
            self.0 as u32
        } else {
            u32::MAX
        }
    }
}

impl From<u64> for Size {
    fn from(n: u64) -> Self {
        Self(n)
    }
}

impl<'de> Deserialize<'de> for Size {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct SizeVisitor;
        impl Visitor<'_> for SizeVisitor {
            type Value = Size;
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("data size")
            }
            fn visit_u64<E: DeError>(self, v: u64) -> Result<Self::Value, E> {
                Ok(Size(v))
            }
            fn visit_str<E: DeError>(self, v: &str) -> Result<Self::Value, E> {
                Size::parse(v).map_err(|err| E::custom(err.to_string()))
            }
            fn visit_i64<E: DeError>(self, v: i64) -> Result<Self::Value, E> {
                if v >= 0 {
                    Ok(Size(v as u64))
                } else {
                    Err(E::invalid_value(Unexpected::Signed(v), &"non-negative"))
                }
            }
        }
        deserializer.deserialize_unit_struct("Size", SizeVisitor)
    }
}

#[derive(Debug)]
pub enum Error {
    Int(ParseIntError),
    Overflow,
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
            Self::Overflow => f.write_str("Size is too large"),
            Self::Unit => f.write_str("Unknown unit symbol"),
        }
    }
}

impl core::error::Error for Error {}
