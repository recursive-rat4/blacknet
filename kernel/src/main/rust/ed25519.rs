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

use crate::blake2b::{Blake2b256, Hash};
use crate::error::Error;
use alloc::vec::Vec;
use core::array::TryFromSliceError;
use core::mem::transmute;
use digest::Digest;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Signature {
    r: [u8; 32],
    s: [u8; 32],
}

impl Signature {
    pub const fn raw_r(self) -> [u8; 32] {
        self.r
    }

    pub const fn raw_s(self) -> [u8; 32] {
        self.s
    }
}

impl TryFrom<Vec<u8>> for Signature {
    type Error = TryFromSliceError;

    fn try_from(vec: Vec<u8>) -> Result<Self, Self::Error> {
        let bytes = <[u8; 64]>::try_from(vec.as_slice())?;
        let rs: [[u8; 32]; 2] = unsafe { transmute(bytes) };
        Ok(Self { r: rs[0], s: rs[1] })
    }
}

pub type PublicKey = [u8; 32];

pub fn to_public_key(_private_key: PrivateKey) -> PublicKey {
    todo!();
}

pub type PrivateKey = [u8; 32];

const fn check_version(bytes: [u8; 32]) -> bool {
    bytes[0] & 0xF0 == 0x10
}

pub fn to_private_key(mnemonic: &str) -> Option<PrivateKey> {
    let hash: [u8; 32] = Blake2b256::digest(mnemonic).into();
    if check_version(hash) {
        Some(hash)
    } else {
        None
    }
}

pub fn sign(_hash: Hash, _private_key: PrivateKey) -> Signature {
    todo!();
}

pub fn verify(_signature: Signature, _hash: Hash, _public_key: PublicKey) -> Result<(), Error> {
    todo!();
}
