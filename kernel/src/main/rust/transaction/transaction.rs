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
    amount::Amount,
    blake2b::Hash256,
    ed25519::{PublicKey, SecretKey, Signature, sign, verify},
    error::Result,
    transaction::TxKind,
};
use alloc::{boxed::Box, vec::Vec};
use blacknet_crypto::symmetric::Blake2b256;
use blacknet_serialization::format::to_bytes;
use serde::{Deserialize, Serialize};

const HEADER_SIZE_BYTES: usize = size_of::<Signature>()
    + size_of::<PublicKey>()
    + size_of::<u32>()
    + size_of::<Hash256>()
    + size_of::<Amount>()
    + size_of::<TxKind>();

#[derive(Deserialize, Serialize)]
pub struct Transaction {
    signature: Signature,
    from: PublicKey,
    seq: u32,
    anchor: Hash256,
    fee: Amount,
    kind: TxKind,
    data: Box<[u8]>,
}

impl Transaction {
    pub fn new(
        from: PublicKey,
        seq: u32,
        anchor: Hash256,
        fee: Amount,
        kind: TxKind,
        data: Box<[u8]>,
    ) -> Self {
        Self {
            signature: Default::default(),
            from,
            seq,
            anchor,
            fee,
            kind,
            data,
        }
    }

    pub fn generated(from: PublicKey, height: u32, anchor: Hash256, amount: Amount) -> Self {
        Self {
            signature: Default::default(),
            from,
            seq: height,
            anchor,
            fee: amount,
            kind: TxKind::Generated,
            data: Default::default(),
        }
    }

    pub fn compute_hash(bytes: &[u8]) -> Option<Hash256> {
        if bytes.len() > HEADER_SIZE_BYTES {
            Some(Blake2b256::digest(&bytes[size_of::<Signature>()..]).into())
        } else {
            None
        }
    }

    pub fn sign(&mut self, secret_key: &SecretKey) -> (Hash256, Vec<u8>) {
        let mut bytes = to_bytes(&self).expect("Transaction serialization");
        let hash = Self::compute_hash(&bytes).expect("Transaction serialized");
        self.signature = sign(hash, secret_key);
        bytes[0..64].copy_from_slice(self.signature.as_bytes());
        (hash, bytes)
    }

    pub fn verify_signature(&self, hash: Hash256) -> Result<()> {
        verify(self.signature, hash, self.from)
    }

    pub const fn anchor(&self) -> Hash256 {
        self.anchor
    }

    pub const fn fee(&self) -> Amount {
        self.fee
    }

    pub const fn from(&self) -> PublicKey {
        self.from
    }

    pub const fn kind(&self) -> TxKind {
        self.kind
    }

    pub const fn seq(&self) -> u32 {
        self.seq
    }

    pub const fn signature(&self) -> Signature {
        self.signature
    }

    pub const fn data_bytes(&self) -> &[u8] {
        &self.data
    }
}
