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

use crate::db::{DBVersion, DBVersionKey, View};
use blacknet_serialization::to_bytes;
use fjall::OwnedWriteBatch;
use serde::Serialize;

pub struct WriteBatch {
    batch: OwnedWriteBatch,
}

impl WriteBatch {
    pub const fn new(batch: OwnedWriteBatch) -> Self {
        Self { batch }
    }

    pub fn insert<K: AsRef<[u8]>, V: Serialize>(&mut self, view: &View<K, V>, key: K, value: &V) {
        self.batch
            .insert(view.as_ref(), key.as_ref(), to_bytes(value).unwrap())
    }

    pub fn insert_bytes<K: AsRef<[u8]>, V>(&mut self, view: &View<K, V>, key: K, bytes: &[u8]) {
        self.batch.insert(view.as_ref(), key.as_ref(), bytes)
    }

    pub fn verset<V: Serialize>(&mut self, db_version: &DBVersion, key: DBVersionKey, value: &V) {
        self.batch.insert(
            db_version.versions.as_ref(),
            DBVersion::key(key),
            to_bytes(value).unwrap(),
        )
    }

    pub fn remove<K: AsRef<[u8]>, V>(&mut self, view: &View<K, V>, key: K) {
        self.batch.remove(view.as_ref(), key.as_ref())
    }

    pub fn commit(self) {
        self.batch.commit().unwrap()
    }
}
