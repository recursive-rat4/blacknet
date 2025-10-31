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

use crate::amount::Amount;
use crate::blake2b::{Blake2b256, Hash};
use crate::ed25519::{PublicKey, Signature};
use crate::transaction::TxKind;
use alloc::boxed::Box;
use core::mem::size_of;
use digest::Digest;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Transaction {
    signature: Signature,
    from: PublicKey,
    seq: u32,
    anchor: Hash,
    fee: Amount,
    kind: TxKind,
    data: Box<[u8]>,
}

impl Transaction {
    pub fn new(
        from: PublicKey,
        seq: u32,
        anchor: Hash,
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

    pub fn generated(from: PublicKey, height: u32, anchor: Hash, amount: Amount) -> Self {
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

    pub fn hash(bytes: &[u8]) -> Hash {
        Blake2b256::digest(&bytes[size_of::<Signature>()..]).into()
    }

    pub const fn anchor(&self) -> Hash {
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

    pub const fn raw_data(&self) -> &[u8] {
        &self.data
    }
}
