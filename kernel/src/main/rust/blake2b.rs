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

use core::{borrow::Borrow, fmt, str::FromStr};
use data_encoding::{DecodeError, DecodeKind, HEXUPPER};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
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
        write!(f, "{}", HEXUPPER.encode(&self.0))
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
    type Err = DecodeError;

    fn from_str(hex: &str) -> Result<Self, Self::Err> {
        if hex.len() == 64 {
            let mut buf = [0_u8; 32];
            match HEXUPPER.decode_mut(hex.as_bytes(), &mut buf) {
                Ok(_) => Ok(Self(buf)),
                Err(err) => Err(err.error),
            }
        } else {
            Err(DecodeError {
                position: 0,
                kind: DecodeKind::Length,
            })
        }
    }
}
