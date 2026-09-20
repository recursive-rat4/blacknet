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

use crate::db::View;
use blacknet_serialization::from_bytes;
use core::fmt::Debug;
use fjall::Readable;
use serde::Deserialize;

#[derive(Clone)]
pub struct Snapshot {
    snapshot: fjall::Snapshot,
}

impl Snapshot {
    pub(super) const fn new(snapshot: fjall::Snapshot) -> Self {
        Self { snapshot }
    }

    pub fn get<K: AsRef<[u8]>, V: for<'de> Deserialize<'de>>(
        &self,
        view: &View<K, V>,
        key: K,
    ) -> Option<V> {
        self.snapshot
            .get(view, key)
            .unwrap()
            .map(|slice| from_bytes::<V>(&slice, false).unwrap())
    }

    pub fn get_with_size<K: AsRef<[u8]>, V: for<'de> Deserialize<'de>>(
        &self,
        view: &View<K, V>,
        key: K,
    ) -> Option<(V, usize)> {
        self.snapshot.get(view, key).unwrap().map(|slice| {
            let size = slice.len();
            let deserialized = from_bytes::<V>(&slice, false).unwrap();
            (deserialized, size)
        })
    }

    pub fn get_bytes<K: AsRef<[u8]>, V>(&self, view: &View<K, V>, key: K) -> Option<Box<[u8]>> {
        self.snapshot
            .get(view, key)
            .unwrap()
            .map(|slice| Box::from(&*slice))
    }

    pub fn contains<K: AsRef<[u8]>, V>(&self, view: &View<K, V>, key: K) -> bool {
        self.snapshot.contains_key(view, key).unwrap()
    }

    pub fn count<K, V>(&self, view: &View<K, V>) -> usize {
        self.snapshot.len(view).unwrap()
    }

    pub fn iter<K, V>(&self, view: &View<K, V>) -> impl Iterator<Item = (K, V)>
    where
        K: for<'a> TryFrom<&'a [u8], Error: Debug>,
        V: for<'de> Deserialize<'de>,
    {
        self.snapshot.iter(view).map(|item| {
            let (key, value) = item.into_inner().unwrap();
            (
                key.as_ref().try_into().unwrap(),
                from_bytes::<V>(&value, false).unwrap(),
            )
        })
    }
}
