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

use crate::db::Fjall;
use core::marker::PhantomData;
use fjall::{Keyspace, Result};

pub struct View<K, V> {
    keyspace: Keyspace,
    phantom_k: PhantomData<K>,
    phantom_v: PhantomData<V>,
}

impl<K, V> View<K, V> {
    pub(super) fn new(fjall: &Fjall, name: &str) -> Result<Self> {
        Ok(Self {
            keyspace: fjall.database().keyspace(name, Fjall::kv_options)?,
            phantom_k: PhantomData,
            phantom_v: PhantomData,
        })
    }

    pub(super) fn with_blob(fjall: &Fjall, name: &str) -> Result<Self> {
        Ok(Self {
            keyspace: fjall.database().keyspace(name, Fjall::blob_options)?,
            phantom_k: PhantomData,
            phantom_v: PhantomData,
        })
    }
}

impl<K, V> AsRef<Keyspace> for View<K, V> {
    fn as_ref(&self) -> &Keyspace {
        &self.keyspace
    }
}
