/*
 * Copyright (c) 2026 Pavel Vasin
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

use crate::db::{DBView, Fjall};
use blacknet_serialization::{error::Result, format::from_bytes};
use fjall::Error as FjallError;
use serde::Deserialize;

#[repr(u32)]
pub enum DBVersionKey {
    BlockDB,
    CoinDB,
    CoinDBState,
}

pub struct DBVersion {
    pub(super) versions: DBView<[u8; 4], Box<[u8]>>,
}

impl DBVersion {
    pub fn new(fjall: &Fjall) -> Result<Self, FjallError> {
        Ok(Self {
            versions: DBView::new(fjall, "versions")?,
        })
    }

    pub fn get<V: for<'a> Deserialize<'a>>(&self, key: DBVersionKey) -> Option<V> {
        self.get_or_err(key).and_then(Result::ok)
    }

    pub fn get_or_err<V: for<'a> Deserialize<'a>>(&self, key: DBVersionKey) -> Option<Result<V>> {
        self.get_bytes(key)
            .map(|bytes| from_bytes::<V>(&bytes, false))
    }

    fn get_bytes(&self, key: DBVersionKey) -> Option<Box<[u8]>> {
        let key = Self::key(key);
        self.versions.get_bytes(key)
    }

    pub(super) const fn key(key: DBVersionKey) -> [u8; 4] {
        (key as u32).to_le_bytes()
    }
}
