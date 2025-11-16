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

use core::fmt::{Debug, Display, Formatter, Result as FmtResult};
use data_encoding::{DecodeError, DecodeKind, HEXUPPER};
use serde::{Deserialize, Serialize};

pub type Blake2b256 = blake2::Blake2b<digest::consts::U32>;

#[derive(Clone, Copy, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Hash([u8; 32]);

impl Hash {
    pub const ZERO: Self = Self([0; 32]);
}

impl AsRef<[u8]> for Hash {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl Debug for Hash {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self)
    }
}

impl Display for Hash {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", HEXUPPER.encode(&self.0))
    }
}

impl From<[u8; 32]> for Hash {
    fn from(array: [u8; 32]) -> Self {
        Self(array)
    }
}

impl From<digest::generic_array::GenericArray<u8, digest::consts::U32>> for Hash {
    fn from(array: digest::generic_array::GenericArray<u8, digest::consts::U32>) -> Self {
        Self(array.into())
    }
}

impl From<Hash> for [u8; 32] {
    fn from(hash: Hash) -> Self {
        hash.0
    }
}

impl TryFrom<&str> for Hash {
    type Error = DecodeError;

    fn try_from(hex: &str) -> Result<Self, Self::Error> {
        if hex.len() == 64 {
            let mut buf: [u8; 32] = Default::default();
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

pub type Blake2b512 = blake2::Blake2b512;
