/*
 * Copyright (c) 2025 Pavel Vasin
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

use blacknet_compat::Mode;
use blacknet_kernel::blake2b::Hash;
use rusqlite::{Connection, Error as SqliteError, OpenFlags};
use std::path::Path;
use std::sync::Mutex;
use thiserror::Error;

#[derive(Debug)]
pub struct Wallet {
    connection: Mutex<Connection>,
}

impl Wallet {
    fn open_flags() -> OpenFlags {
        // SQLITE_OPEN_FULLMUTEX is not properly supported in rusqlite
        OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_EXRESCODE
    }

    fn create_flags() -> OpenFlags {
        Self::open_flags() | OpenFlags::SQLITE_OPEN_CREATE
    }

    pub fn create(path: &Path, mode: &Mode) -> Result<Self> {
        let connection = Connection::open_with_flags(path, Self::create_flags())?;
        Self::initialize(connection, mode)
    }

    pub fn open(path: &Path, mode: &Mode) -> Result<Self> {
        let connection = Connection::open_with_flags(path, Self::open_flags())?;
        Self::attach(connection, mode)
    }

    pub fn ephemeral(mode: &Mode) -> Result<Self> {
        let connection = Connection::open_in_memory_with_flags(Self::create_flags())?;
        Self::initialize(connection, mode)
    }

    pub fn attach(connection: Connection, mode: &Mode) -> Result<Self> {
        Self::check_magic(&connection, mode)?;
        Self::configure(&connection)?;
        Ok(Self {
            connection: Mutex::new(connection),
        })
    }

    fn configure(connection: &Connection) -> Result<()> {
        connection.pragma_update(None, "locking_mode", "EXCLUSIVE")?;
        #[cfg(target_os = "macos")]
        connection.pragma_update(None, "fullfsync", "TRUE")?;
        connection.pragma_update(None, "synchronous", "FULL")?;
        connection.pragma_update(None, "journal_mode", "DELETE")?;
        Ok(())
    }

    fn check_magic(connection: &Connection, mode: &Mode) -> Result<()> {
        let magic: u32 = connection.query_one("PRAGMA application_id;", (), |row| row.get(0))?;
        if magic == mode.network_magic() {
            Ok(())
        } else {
            Err(Error::WrongMagic(mode.agent_name().to_owned()))
        }
    }

    fn set_magic(connection: &Connection, mode: &Mode) -> Result<()> {
        connection.pragma_update(None, "application_id", mode.network_magic())?;
        connection.pragma_update(None, "user_version", 1)?;
        Ok(())
    }

    fn create_schema(connection: &Connection) -> Result<()> {
        connection.execute(
            "CREATE TABLE transactions(id BLOB PRIMARY KEY, bytes BLOB NOT NULL) STRICT;",
            (),
        )?;
        Ok(())
    }

    fn initialize(connection: Connection, mode: &Mode) -> Result<Self> {
        Self::configure(&connection)?;
        Self::set_magic(&connection, mode)?;
        Self::create_schema(&connection)?;
        Ok(Self {
            connection: Mutex::new(connection),
        })
    }

    pub fn sequence(&self) -> u32 {
        todo!();
    }

    pub fn get_transaction(&self, id: Hash) -> Result<Box<[u8]>> {
        let id: [u8; _] = id.into();
        let connection = self.connection.lock().unwrap();
        let mut statement =
            connection.prepare_cached("SELECT bytes FROM transactions WHERE id = ?;")?;
        let bytes = statement.query_one((id,), |row| row.get(0))?;
        Ok(bytes)
    }

    pub fn put_transaction(&self, id: Hash, bytes: &[u8]) -> Result<()> {
        let id: [u8; _] = id.into();
        let connection = self.connection.lock().unwrap();
        let mut statement = connection.prepare_cached("INSERT INTO transactions VALUES(?, ?);")?;
        statement.execute((id, bytes))?;
        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("This SQLite database doesn't look like {0} wallet")]
    WrongMagic(String),
    #[error("{0}")]
    Sqlite(#[from] SqliteError),
}

pub type Result<T> = core::result::Result<T, Error>;
