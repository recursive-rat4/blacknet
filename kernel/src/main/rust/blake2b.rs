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

use alloc::string::ToString;
use const_hex::FromHexError;
use core::{borrow::Borrow, fmt, str::FromStr};
use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error as _};

#[derive(Clone, Copy, Default, Eq, Hash, PartialEq)]
#[repr(transparent)]
pub struct Hash256([u8; 32]);

impl Hash256 {
    pub const ZERO: Self = Self([0; 32]);
}

impl AsRef<[u8]> for Hash256 {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl Borrow<[u8; 32]> for Hash256 {
    #[inline]
    fn borrow(&self) -> &[u8; 32] {
        &self.0
    }
}

impl fmt::Debug for Hash256 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl fmt::Display for Hash256 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", const_hex::encode_upper(self.0))
    }
}

impl From<[u8; 32]> for Hash256 {
    #[inline]
    fn from(array: [u8; 32]) -> Self {
        Self(array)
    }
}

impl From<Hash256> for [u8; 32] {
    #[inline]
    fn from(hash: Hash256) -> Self {
        hash.0
    }
}

impl FromStr for Hash256 {
    type Err = FromHexError;

    fn from_str(hex: &str) -> Result<Self, Self::Err> {
        Ok(Self(const_hex::decode_to_array(hex.as_bytes())?))
    }
}

impl Serialize for Hash256 {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        if !serializer.is_human_readable() {
            self.0.serialize(serializer)
        } else {
            self.to_string().serialize(serializer)
        }
    }
}

impl<'de> Deserialize<'de> for Hash256 {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(if !deserializer.is_human_readable() {
            Self(<[u8; 32]>::deserialize(deserializer)?)
        } else {
            Self::from_str(<&str>::deserialize(deserializer)?).map_err(D::Error::custom)?
        })
    }
}
