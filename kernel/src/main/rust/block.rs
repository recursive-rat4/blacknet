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

use crate::{
    blake2b::Hash256,
    ed25519::{PublicKey, SecretKey, Signature, sign, verify},
    error::{Error, Result},
};
use alloc::{boxed::Box, vec::Vec};
use blacknet_crypto::symmetric::Blake2b256;
use blacknet_serialization::format::to_bytes;
use blacknet_time::Seconds;
use serde::{Deserialize, Serialize};

pub const BLOCK_VERSION: u32 = 2;
const CONTENT_HASH_POS: usize =
    size_of::<u32>() + size_of::<Hash256>() + size_of::<Seconds>() + size_of::<PublicKey>();
const SIGNATURE_POS: usize = CONTENT_HASH_POS + size_of::<Hash256>();
const HEADER_SIZE_BYTES: usize = SIGNATURE_POS + size_of::<Signature>();

#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Block {
    version: u32,
    previous: Hash256,
    time: Seconds,
    generator: PublicKey,
    content_hash: Hash256,
    signature: Signature,
    transactions: Vec<Box<[u8]>>,
}

impl Block {
    pub fn new(previous: Hash256, time: Seconds, generator: PublicKey) -> Self {
        Self {
            version: BLOCK_VERSION,
            previous,
            time,
            generator,
            content_hash: Hash256::ZERO,
            signature: Default::default(),
            transactions: Vec::new(),
        }
    }

    pub const fn with_all(
        version: u32,
        previous: Hash256,
        time: Seconds,
        generator: PublicKey,
        content_hash: Hash256,
        signature: Signature,
        transactions: Vec<Box<[u8]>>,
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

    pub fn compute_hash(bytes: &[u8]) -> Option<Hash256> {
        if bytes.len() > HEADER_SIZE_BYTES {
            Some(Blake2b256::digest(&bytes[..HEADER_SIZE_BYTES - size_of::<Signature>()]).into())
        } else {
            None
        }
    }

    pub fn compute_content_hash(bytes: &[u8]) -> Option<Hash256> {
        if bytes.len() > HEADER_SIZE_BYTES {
            Some(Blake2b256::digest(&bytes[HEADER_SIZE_BYTES..]).into())
        } else {
            None
        }
    }

    pub fn sign(&mut self, secret_key: &SecretKey) -> (Hash256, Vec<u8>) {
        let mut bytes = to_bytes(&self).expect("Block serialization");
        let content_hash = Self::compute_content_hash(&bytes).expect("Block serialized");
        self.content_hash = content_hash;
        bytes[CONTENT_HASH_POS..CONTENT_HASH_POS + size_of::<Hash256>()]
            .copy_from_slice(content_hash.as_ref());
        let hash = Self::compute_hash(&bytes).expect("Block serialized");
        self.signature = sign(hash, secret_key);
        bytes[SIGNATURE_POS..SIGNATURE_POS + 64].copy_from_slice(self.signature.as_bytes());
        (hash, bytes)
    }

    pub const fn version(&self) -> u32 {
        self.version
    }

    pub const fn previous(&self) -> Hash256 {
        self.previous
    }

    pub const fn time(&self) -> Seconds {
        self.time
    }

    pub const fn generator(&self) -> PublicKey {
        self.generator
    }

    pub const fn content_hash(&self) -> Hash256 {
        self.content_hash
    }

    pub const fn signature(&self) -> Signature {
        self.signature
    }

    pub const fn raw_transactions(&self) -> &[Box<[u8]>] {
        self.transactions.as_slice()
    }

    pub fn verify_content_hash(&self, bytes: &[u8]) -> Result<()> {
        match Self::compute_content_hash(bytes) {
            Some(content_hash) if self.content_hash == content_hash => Ok(()),
            _ => Err(Error::invalid("Invalid content hash")),
        }
    }

    pub fn verify_signature(&self, hash: Hash256) -> Result<()> {
        verify(self.signature, hash, self.generator)
    }

    pub const fn is_empty(&self) -> bool {
        self.transactions.is_empty()
    }

    pub fn push(&mut self, tx: Box<[u8]>) {
        self.transactions.push(tx)
    }

    pub fn clear(&mut self) {
        self.transactions.clear()
    }
}
