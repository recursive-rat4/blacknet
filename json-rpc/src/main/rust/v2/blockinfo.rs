/*
 * Copyright (c) 2019-2025 Pavel Vasin
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

use crate::v2::{HashInfo, PublicKeyInfo, Result, SignatureInfo, TransactionInfo};
use blacknet_kernel::blake2b::Hash;
use blacknet_kernel::block::Block;
use blacknet_kernel::transaction::Transaction;
use blacknet_serialization::format::from_bytes;
use blacknet_wallet::address::AddressCodec;
use serde::{Deserialize, Serialize};
use serde_json::{Value, to_value};

#[derive(Deserialize, Serialize)]
pub struct BlockInfo {
    hash: HashInfo,
    size: u32,
    version: u32,
    previous: HashInfo,
    time: i64,
    generator: PublicKeyInfo,
    contentHash: HashInfo,
    signature: SignatureInfo,
    transactions: Value,
}

impl BlockInfo {
    pub fn new(
        block: &Block,
        hash: Hash,
        size: u32,
        tx_detail: bool,
        address_codec: &AddressCodec,
    ) -> Result<Self> {
        let transactions = if tx_detail {
            let mut txs = Vec::with_capacity(block.raw_transactions().len());
            for bytes in block.raw_transactions() {
                let tx = from_bytes::<Transaction>(bytes, false)?;
                let tx_hash = Transaction::hash(bytes);
                let info = TransactionInfo::new(&tx, tx_hash, bytes.len() as u32, address_codec)?;
                txs.push(to_value(info)?);
            }
            Value::Array(txs)
        } else {
            Value::Number(block.raw_transactions().len().into())
        };
        Ok(Self {
            hash: hash.into(),
            size,
            version: block.version(),
            previous: block.previous().into(),
            time: block.time().into(),
            generator: PublicKeyInfo::new(block.generator(), address_codec)?,
            contentHash: block.content_hash().into(),
            signature: block.signature().into(),
            transactions,
        })
    }
}
