/*
 * Copyright (c) 2019-2026 Pavel Vasin
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

use crate::v2::{HashInfo, PublicKeyInfo, Result};
use blacknet_network::{db::BlockNotification as Notification, wallet::AddressCodec};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct BlockNotification {
    hash: HashInfo,
    height: u32,
    size: u32,
    version: u32,
    previous: HashInfo,
    time: i64,
    generator: PublicKeyInfo,
    transactions: u32,
}

impl BlockNotification {
    pub fn new(notification: &Notification, address_codec: &AddressCodec) -> Result<Self> {
        Ok(Self {
            hash: notification.1.into(),
            height: notification.2,
            size: notification.3,
            version: notification.0.version(),
            previous: notification.0.previous().into(),
            time: notification.0.time().into(),
            generator: PublicKeyInfo::new(notification.0.generator(), address_codec)?,
            transactions: notification.0.raw_transactions().len() as u32,
        })
    }
}
