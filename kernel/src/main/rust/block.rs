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
use crate::ed25519::{PublicKey, Signature};
use alloc::boxed::Box;
use blacknet_time::Seconds;
use core::mem::size_of;
use digest::Digest;
use serde::{Deserialize, Serialize};

const VERSION: u32 = 2;
const CONTENT_HASH_POS: usize =
    size_of::<u32>() + size_of::<Hash>() + size_of::<Seconds>() + size_of::<PublicKey>();
const SIGNATURE_POS: usize = CONTENT_HASH_POS + size_of::<Hash>();
const HEADER_SIZE_BYTES: usize = SIGNATURE_POS + size_of::<Signature>();

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Block {
    version: u32,
    previous: Hash,
    time: Seconds,
    generator: PublicKey,
    content_hash: Hash,
    signature: Signature,
    transactions: Box<[Box<[u8]>]>,
}

impl Block {
    pub fn new(previous: Hash, time: Seconds, generator: PublicKey) -> Self {
        Self {
            version: VERSION,
            previous,
            time,
            generator,
            content_hash: Default::default(),
            signature: Default::default(),
            transactions: Default::default(),
        }
    }

    pub fn with_all(
        version: u32,
        previous: Hash,
        time: Seconds,
        generator: PublicKey,
        content_hash: Hash,
        signature: Signature,
        transactions: Box<[Box<[u8]>]>,
    ) -> Self {
        Self {
            version,
            previous,
            time,
            generator,
            content_hash,
            signature,
            transactions,
        }
    }

    pub fn hash(bytes: &[u8]) -> Hash {
        Blake2b256::digest(&bytes[..HEADER_SIZE_BYTES - size_of::<Signature>()]).into()
    }

    pub fn content_hash(bytes: &[u8]) -> Hash {
        Blake2b256::digest(&bytes[HEADER_SIZE_BYTES..]).into()
    }
}
