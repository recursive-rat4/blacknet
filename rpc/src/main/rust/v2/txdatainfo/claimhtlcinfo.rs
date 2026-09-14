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

use crate::v2::{ByteArrayInfo, error::Result};
use blacknet_kernel::transaction::ClaimHTLC;
use blacknet_network::wallet::{AddressCodec, AddressKind};
use blacknet_serialization::format::from_bytes;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ClaimHTLCInfo {
    id: String,
    preimage: ByteArrayInfo,
}

impl ClaimHTLCInfo {
    pub fn new(data: &[u8], address_codec: &AddressCodec) -> Result<Self> {
        let claim_htlc = from_bytes::<ClaimHTLC>(data, false)?;
        Ok(Self {
            id: address_codec.encode_with_kind(AddressKind::HTLC, &claim_htlc.id())?,
            preimage: claim_htlc.preimage().into(),
        })
    }
}
