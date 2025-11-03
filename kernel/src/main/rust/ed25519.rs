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

use crate::blake2b::Hash;
use crate::error::Error;
use alloc::vec::Vec;
use core::array::TryFromSliceError;
use core::mem::transmute;
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

pub type PrivateKey = [u8; 32];

pub fn verify(_signature: Signature, _hash: Hash, _public_key: PublicKey) -> Result<(), Error> {
    todo!();
}
