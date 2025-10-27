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

use crate::v2::ByteArrayInfo;
use crate::v2::error::Result;
use blacknet_kernel::transaction::Blob;
use blacknet_serialization::format::from_bytes;
use blacknet_wallet::address::{AddressCodec, AddressKind};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct BlobInfo {
    tag: String,
    data: ByteArrayInfo,
}

impl BlobInfo {
    pub fn new(data: &[u8], address_codec: &AddressCodec) -> Result<Self> {
        let blob = from_bytes::<Blob>(data, false)?;
        Ok(Self {
            tag: address_codec.encode_with_kind(AddressKind::Blob, &blob.tag())?,
            data: blob.data().into(),
        })
    }
}
