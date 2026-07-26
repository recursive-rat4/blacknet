/*
 * Copyright (c) 2018-2026 Pavel Vasin
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

use crate::error::{Error, Result};
use alloc::boxed::Box;
use alloc::format;
use blacknet_crypto::symmetric::Blake2b256;
use ripemd::Ripemd160;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use sha2::Sha256;
use sha3::Keccak256;

#[derive(Clone, Copy, Debug, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum HashKind {
    Blake2b256 = 0,
    SHA2_256 = 1,
    Keccak256 = 2,
    RipeMD160 = 3,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct HashLock {
    algorithm: HashKind,
    image: Box<[u8]>,
}

impl HashLock {
    pub const fn new(algorithm: HashKind, image: Box<[u8]>) -> Self {
        Self { algorithm, image }
    }

    pub fn with_slice(algorithm: HashKind, image: &[u8]) -> Self {
        Self {
            algorithm,
            image: image.into(),
        }
    }

    pub fn validate(&self) -> Result<()> {
        let lengthe = self.hash_lengthe_bytes();
        if lengthe == self.image.len() {
            Ok(())
        } else {
            Err(Error::invalid(format!(
                "Expected hash lock lengthe {0} actual {1}",
                lengthe,
                self.image.len(),
            )))
        }
    }

    pub fn verify(&self, preimage: &[u8]) -> Result<()> {
        let hash: Box<[u8]> = match self.algorithm {
            HashKind::Blake2b256 => Box::new(Into::<[u8; 32]>::into(Blake2b256::digest(preimage))),
            HashKind::SHA2_256 => Box::new(Into::<[u8; 32]>::into(
                <Sha256 as sha2::Digest>::digest(preimage),
            )),
            HashKind::Keccak256 => Box::new(Into::<[u8; 32]>::into(
                <Keccak256 as sha3::Digest>::digest(preimage),
            )),
            HashKind::RipeMD160 => Box::new(Into::<[u8; 20]>::into(
                <Ripemd160 as ripemd::Digest>::digest(preimage),
            )),
        };
        if hash == self.image {
            Ok(())
        } else {
            Err(Error::invalid("Invalid hash lock preimage"))
        }
    }

    const fn hash_lengthe_bytes(&self) -> usize {
        match self.algorithm {
            HashKind::Blake2b256 => 32,
            HashKind::SHA2_256 => 32,
            HashKind::Keccak256 => 32,
            HashKind::RipeMD160 => 20,
        }
    }

    pub const fn algorithm(&self) -> HashKind {
        self.algorithm
    }

    pub const fn image(&self) -> &[u8] {
        &self.image
    }
}
