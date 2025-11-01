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

    pub const fn with_all(
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

    pub fn compute_hash(bytes: &[u8]) -> Option<Hash> {
        if bytes.len() > HEADER_SIZE_BYTES {
            Some(Blake2b256::digest(&bytes[..HEADER_SIZE_BYTES - size_of::<Signature>()]).into())
        } else {
            None
        }
    }

    pub fn compute_content_hash(bytes: &[u8]) -> Option<Hash> {
        if bytes.len() > HEADER_SIZE_BYTES {
            Some(Blake2b256::digest(&bytes[HEADER_SIZE_BYTES..]).into())
        } else {
            None
        }
    }

    pub const fn version(&self) -> u32 {
        self.version
    }

    pub const fn previous(&self) -> Hash {
        self.previous
    }

    pub const fn time(&self) -> Seconds {
        self.time
    }

    pub const fn generator(&self) -> PublicKey {
        self.generator
    }

    pub const fn content_hash(&self) -> Hash {
        self.content_hash
    }

    pub const fn signature(&self) -> Signature {
        self.signature
    }

    pub fn raw_transactions(&self) -> &[Box<[u8]>] {
        &self.transactions
    }
}
