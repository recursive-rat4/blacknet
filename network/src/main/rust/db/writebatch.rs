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

use crate::db::DBView;
use blacknet_serialization::format::to_bytes;
use fjall::OwnedWriteBatch;
use serde::Serialize;

pub struct WriteBatch {
    inner: OwnedWriteBatch,
}

impl WriteBatch {
    pub const fn new(inner: OwnedWriteBatch) -> Self {
        Self { inner }
    }

    pub fn insert<K: AsRef<[u8]>, V: Serialize>(&mut self, view: &DBView<K, V>, key: K, value: &V) {
        self.inner
            .insert(&view.keyspace, key.as_ref(), to_bytes(value).unwrap())
    }

    pub fn insert_bytes<K: AsRef<[u8]>, V>(&mut self, view: &DBView<K, V>, key: K, bytes: &[u8]) {
        self.inner.insert(&view.keyspace, key.as_ref(), bytes)
    }

    pub fn remove<K: AsRef<[u8]>, V>(&mut self, view: &DBView<K, V>, key: K) {
        self.inner.remove(&view.keyspace, key.as_ref())
    }

    pub fn commit(self) {
        self.inner.commit().unwrap()
    }
}
