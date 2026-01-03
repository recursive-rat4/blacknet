/*
 * Copyright (c) 2023-2026 Pavel Vasin
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

use crate::fjall::Fjall;
use blacknet_serialization::format::from_bytes;
use core::marker::PhantomData;
use core::ops::Deref;
use fjall::{Database, Keyspace, Result};
use serde::Deserialize;

pub struct DBView<K: AsRef<[u8]>, V: for<'de> Deserialize<'de>> {
    keyspace: Keyspace,
    phantom_k: PhantomData<K>,
    phantom_v: PhantomData<V>,
}

impl<K: AsRef<[u8]>, V: for<'de> Deserialize<'de>> DBView<K, V> {
    pub fn new(fjall: &Database, name: &str) -> Result<Self> {
        Ok(Self {
            keyspace: fjall.keyspace(name, Fjall::kv_options)?,
            phantom_k: PhantomData,
            phantom_v: PhantomData,
        })
    }

    pub fn with_blob(fjall: &Database, name: &str) -> Result<Self> {
        Ok(Self {
            keyspace: fjall.keyspace(name, Fjall::blob_options)?,
            phantom_k: PhantomData,
            phantom_v: PhantomData,
        })
    }

    pub fn contains(&self, key: K) -> bool {
        self.keyspace.contains_key(key).unwrap()
    }

    pub fn get(&self, key: K) -> Option<V> {
        self.keyspace
            .get(key)
            .unwrap()
            .map(|slice| from_bytes::<V>(&slice, false).unwrap())
    }

    pub fn get_with_size(&self, key: K) -> Option<(V, usize)> {
        self.keyspace.get(key).unwrap().map(|slice| {
            let size = slice.len();
            let deserealized = from_bytes::<V>(&slice, false).unwrap();
            (deserealized, size)
        })
    }

    pub fn get_bytes(&self, key: K) -> Option<Box<[u8]>> {
        self.keyspace
            .get(key)
            .unwrap()
            .map(|slice| Box::from(slice.deref()))
    }
}
