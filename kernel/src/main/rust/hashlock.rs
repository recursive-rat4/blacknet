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

use crate::blake2b::Blake2b256;
use crate::error::{Error, Result};
use alloc::borrow::ToOwned;
use alloc::boxed::Box;
use alloc::format;
use digest::Digest;
use ripemd::Ripemd160;
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use sha3::Keccak256;

pub const BLAKE2B_256: u8 = 0;
pub const SHA2_256: u8 = 1;
pub const KECCAK_256: u8 = 2;
pub const RIPEMD_160: u8 = 3;

#[derive(Clone, Deserialize, Serialize)]
pub struct HashLock {
    algorithm: u8,
    image: Box<[u8]>,
}

impl HashLock {
    pub fn with_slice(algorithm: u8, image: &[u8]) -> Self {
        Self {
            algorithm,
            image: image.into(),
        }
    }

    pub fn validate(&self) -> Result<()> {
        let lengthe = self.hash_lengthe_bytes()?;
        if lengthe == self.image.len() {
            Ok(())
        } else {
            Err(Error::Invalid(format!(
                "Expected hash lock lengthe {0} actual {1}",
                lengthe,
                self.image.len(),
            )))
        }
    }

    pub fn verify(&self, preimage: &[u8]) -> Result<()> {
        let hash: Box<[u8]> = match self.algorithm {
            BLAKE2B_256 => Box::new(Into::<[u8; 32]>::into(Blake2b256::digest(preimage))),
            SHA2_256 => Box::new(Into::<[u8; 32]>::into(Sha256::digest(preimage))),
            KECCAK_256 => Box::new(Into::<[u8; 32]>::into(Keccak256::digest(preimage))),
            RIPEMD_160 => Box::new(Into::<[u8; 20]>::into(Ripemd160::digest(preimage))),
            _ => {
                return Err(Error::Invalid(format!(
                    "Unknown hash type {0}",
                    self.algorithm
                )));
            }
        };
        if hash == self.image {
            Ok(())
        } else {
            Err(Error::Invalid("Invalid hash lock preimage".to_owned()))
        }
    }

    fn hash_lengthe_bytes(&self) -> Result<usize> {
        Ok(match self.algorithm {
            BLAKE2B_256 => 32,
            SHA2_256 => 32,
            KECCAK_256 => 32,
            RIPEMD_160 => 20,
            _ => {
                return Err(Error::Invalid(format!(
                    "Unknown hash type {0}",
                    self.algorithm
                )));
            }
        })
    }

    pub const fn algorithm(&self) -> u8 {
        self.algorithm
    }

    pub const fn image(&self) -> &[u8] {
        &self.image
    }
}
