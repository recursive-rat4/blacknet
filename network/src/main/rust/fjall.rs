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

use crate::settings::Settings;
use blacknet_compat::{XDGDirectories, ulimit};
use core::cmp::max;
use fjall::{
    CompressionType, Database, KeyspaceCreateOptions, KvSeparationOptions,
    OwnedWriteBatch as WriteBatch, Result, config::CompressionPolicy,
};
use std::sync::Arc;

pub struct Fjall {
    database: Database,
}

impl Fjall {
    fn max_open_files(settings: &Arc<Settings>) -> usize {
        let max_open_files = max(
            ulimit().unwrap() as isize
                - settings.incoming_connections as isize
                - settings.outgoing_connections as isize
                - 20_isize,
            64,
        );
        max_open_files as usize
    }

    pub fn open(dirs: &XDGDirectories, settings: &Arc<Settings>) -> Result<Arc<Fjall>> {
        let path = dirs.data().join("fjall");
        let database = Database::builder(path)
            .max_cached_files(Some(Self::max_open_files(settings)))
            .cache_size(settings.db_cache)
            .journal_compression(CompressionType::None)
            .open()?;
        Ok(Arc::new(Fjall { database }))
    }

    pub fn kv_options() -> KeyspaceCreateOptions {
        KeyspaceCreateOptions::default()
            .data_block_compression_policy(CompressionPolicy::disabled())
            .index_block_compression_policy(CompressionPolicy::disabled())
    }

    pub fn blob_options() -> KeyspaceCreateOptions {
        Self::kv_options().with_kv_separation(Some(
            KvSeparationOptions::default().compression(CompressionType::None),
        ))
    }

    pub const fn database(&self) -> &Database {
        &self.database
    }

    pub fn create_write_batch(&self) -> WriteBatch {
        self.database.batch()
    }
}
