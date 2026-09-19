/*
 * Copyright (c) 2025-2026 Pavel Vasin
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

use blacknet_kernel::ed25519::Signature;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct SignatureInfo(
    #[serde(
        deserialize_with = "const_hex::serde::deserialize",
        serialize_with = "const_hex::serde::no_prefix::serialize_upper"
    )]
    [u8; 64],
);

impl From<Signature> for SignatureInfo {
    fn from(signature: Signature) -> Self {
        Self(signature.into())
    }
}

impl From<SignatureInfo> for Signature {
    fn from(info: SignatureInfo) -> Self {
        Self::from(info.0)
    }
}
