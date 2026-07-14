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

use crate::walletdb::Error;
use blacknet_compat::Mode;
use blacknet_kernel::account::Lease;
use blacknet_kernel::amount::Amount;
use blacknet_kernel::blake2b::Hash;
use blacknet_kernel::ed25519::PublicKey;
use blacknet_kernel::transaction::{HashTimeLockContractId, MultiSignatureLockContractId};
use blacknet_time::{Seconds, SystemClock};
use rusqlite::{Connection, OpenFlags};
use std::path::Path;
use std::sync::Mutex;

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

    pub fn create(path: &Path, public_key: PublicKey, mode: &Mode) -> Result<Self> {
        let connection = Connection::open_with_flags(path, Self::create_flags())?;
        Self::initialize(connection, public_key, mode)
    }

    pub fn open(path: &Path, mode: &Mode) -> Result<Self> {
        let connection = Connection::open_with_flags(path, Self::open_flags())?;
        Self::attach(connection, mode)
    }

    pub fn ephemeral(public_key: PublicKey, mode: &Mode) -> Result<Self> {
        let connection = Connection::open_in_memory_with_flags(Self::create_flags())?;
        Self::initialize(connection, public_key, mode)
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
            "CREATE TABLE wallet(\
                id INTEGER PRIMARY KEY CHECK (id = 0),\
                created_at INTEGER NOT NULL,\
                public_key BLOB NOT NULL,\
                sequence INTEGER NOT NULL\
             ) STRICT;",
            (),
        )?;
        connection.execute("CREATE TABLE htlcs(id BLOB PRIMARY KEY) STRICT;", ())?;
        connection.execute("CREATE TABLE multisigs(id BLOB PRIMARY KEY) STRICT;", ())?;
        connection.execute(
            "CREATE TABLE out_leases(\
                 public_key BLOB NOT NULL,\
                 height INTEGER NOT NULL,\
                 amount INTEGER NOT NULL\
             ) STRICT;",
            (),
        )?;
        connection.execute(
            "CREATE TABLE transactions(id BLOB PRIMARY KEY, bytes BLOB NOT NULL) STRICT;",
            (),
        )?;
        Ok(())
    }

    fn initialize(connection: Connection, public_key: PublicKey, mode: &Mode) -> Result<Self> {
        Self::configure(&connection)?;
        Self::set_magic(&connection, mode)?;
        Self::create_schema(&connection)?;

        let created_at = SystemClock::secs();

        connection.execute(
            "INSERT INTO wallet VALUES(?, ?, ?, ?);",
            (0, created_at.value(), public_key.as_ref(), 0),
        )?;

        Ok(Self {
            connection: Mutex::new(connection),
        })
    }

    pub fn created_at(&self) -> Result<Seconds> {
        let connection = self.connection.lock().unwrap();
        let mut statement = connection.prepare_cached("SELECT created_at FROM wallet;")?;
        let num: i64 = statement.query_one((), |row| row.get(0))?;
        Ok(Seconds::new(num))
    }

    pub fn public_key(&self) -> Result<PublicKey> {
        let connection = self.connection.lock().unwrap();
        let mut statement = connection.prepare_cached("SELECT public_key FROM wallet;")?;
        let bytes: [u8; 32] = statement.query_one((), |row| row.get(0))?;
        Ok(PublicKey::from(bytes))
    }

    pub fn sequence(&self) -> Result<u32> {
        let connection = self.connection.lock().unwrap();
        let mut statement = connection.prepare_cached("SELECT sequence FROM wallet;")?;
        let sequence = statement.query_one((), |row| row.get(0))?;
        Ok(sequence)
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

    pub fn has_htlc(&self, id: HashTimeLockContractId) -> Result<bool> {
        let connection = self.connection.lock().unwrap();
        let mut statement =
            connection.prepare_cached("SELECT EXISTS(SELECT 1 FROM htlcs WHERE id = ?);")?;
        let exists = statement.query_one((id,), |row| row.get(0))?;
        Ok(exists)
    }

    pub fn put_htlc(&self, id: HashTimeLockContractId) -> Result<()> {
        let connection = self.connection.lock().unwrap();
        let mut statement = connection.prepare_cached("INSERT INTO htlcs VALUES(?);")?;
        statement.execute((id,))?;
        Ok(())
    }

    pub fn remove_htlc(&self, id: HashTimeLockContractId) -> Result<()> {
        let connection = self.connection.lock().unwrap();
        let mut statement = connection.prepare_cached("DELETE FROM htlcs WHERE id = ?;")?;
        statement.execute((id,))?;
        Ok(())
    }

    pub fn has_multisig(&self, id: MultiSignatureLockContractId) -> Result<bool> {
        let connection = self.connection.lock().unwrap();
        let mut statement =
            connection.prepare_cached("SELECT EXISTS(SELECT 1 FROM multisigs WHERE id = ?);")?;
        let exists = statement.query_one((id,), |row| row.get(0))?;
        Ok(exists)
    }

    pub fn put_multisig(&self, id: MultiSignatureLockContractId) -> Result<()> {
        let connection = self.connection.lock().unwrap();
        let mut statement = connection.prepare_cached("INSERT INTO multisigs VALUES(?);")?;
        statement.execute((id,))?;
        Ok(())
    }

    pub fn remove_multisig(&self, id: MultiSignatureLockContractId) -> Result<()> {
        let connection = self.connection.lock().unwrap();
        let mut statement = connection.prepare_cached("DELETE FROM multisigs WHERE id = ?;")?;
        statement.execute((id,))?;
        Ok(())
    }

    pub fn put_out_lease(&self, lease: Lease) -> Result<()> {
        let connection = self.connection.lock().unwrap();
        let mut statement = connection.prepare_cached("INSERT INTO out_leases VALUES(?, ?, ?);")?;
        statement.execute((
            lease.public_key().as_ref(),
            lease.height(),
            lease.balance().value(),
        ))?;
        Ok(())
    }

    pub fn remove_out_lease(&self, lease: Lease) -> Result<()> {
        let connection = self.connection.lock().unwrap();
        let mut statement = connection.prepare_cached(
            "DELETE FROM out_leases \
             WHERE ROWID = (\
                 SELECT ROWID FROM out_leases \
                 WHERE public_key = ? AND height = ? AND amount = ? \
                 LIMIT 1\
             );",
        )?;
        statement.execute((
            lease.public_key().as_ref(),
            lease.height(),
            lease.balance().value(),
        ))?;
        Ok(())
    }

    pub fn set_out_lease_height(&self, lease: Lease, height: u32) -> Result<()> {
        let connection = self.connection.lock().unwrap();
        let mut statement = connection.prepare_cached(
            "UPDATE out_leases \
             SET height = ? \
             WHERE ROWID = (\
                SELECT ROWID FROM out_leases \
                WHERE public_key = ? AND height = ? AND amount = ? \
                LIMIT 1\
             );",
        )?;
        statement.execute((
            height,
            lease.public_key().as_ref(),
            lease.height(),
            lease.balance().value(),
        ))?;
        Ok(())
    }

    pub fn withdraw_from_out_lease(&self, lease: Lease, withdraw: Amount) -> Result<()> {
        let connection = self.connection.lock().unwrap();
        let mut statement = connection.prepare_cached(
            "UPDATE out_leases \
             SET amount = amount - ? \
             WHERE ROWID = (\
                SELECT ROWID FROM out_leases \
                WHERE public_key = ? AND height = ? AND amount = ? \
                LIMIT 1\
             );",
        )?;
        statement.execute((
            withdraw.value(),
            lease.public_key().as_ref(),
            lease.height(),
            lease.balance().value(),
        ))?;
        Ok(())
    }
}

pub type Result<T> = core::result::Result<T, Error>;
