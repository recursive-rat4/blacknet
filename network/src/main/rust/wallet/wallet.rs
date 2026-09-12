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

use crate::wallet::{MasterSecret, Mnemonic};
use blacknet_compat::Mode;
use blacknet_kernel::{
    account::Lease,
    amount::Amount,
    blake2b::Hash256,
    ed25519::{PublicKey, SecretKey, to_public_key, to_secret_key},
    transaction::{HashTimeLockContractId, MultiSignatureLockContractId},
};
use blacknet_time::{Seconds, SystemClock};
use core::fmt;
use rusqlite::{Connection, OpenFlags};
use std::{path::Path, sync::Mutex};

pub use rusqlite::Error;

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

    pub fn open(path: &Path, mode: &Mode) -> Result<Self, OpenError> {
        let connection = Connection::open_with_flags(path, Self::open_flags())?;
        Self::attach(connection, mode)
    }

    pub fn ephemeral(mode: &Mode) -> Result<Self> {
        let connection = Connection::open_in_memory_with_flags(Self::create_flags())?;
        Self::initialize(connection, mode)
    }

    pub fn attach(connection: Connection, mode: &Mode) -> Result<Self, OpenError> {
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

    fn check_magic(connection: &Connection, mode: &Mode) -> Result<(), OpenError> {
        let magic: u32 = connection.query_one("PRAGMA application_id;", (), |row| row.get(0))?;
        if magic == mode.network_magic() {
            Ok(())
        } else {
            Err(OpenError::Magic(mode.agent_name().to_owned()))
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
                is_staking INTEGER NOT NULL CHECK (is_staking IN (FALSE, TRUE)),\
                sequence INTEGER NOT NULL\
             ) STRICT;",
            (),
        )?;
        connection.execute(
            "CREATE TABLE keys(\
                 path TEXT NOT NULL UNIQUE,\
                 secret BLOB,\
                 public BLOB\
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

    fn initialize(connection: Connection, mode: &Mode) -> Result<Self> {
        Self::configure(&connection)?;
        Self::set_magic(&connection, mode)?;
        Self::create_schema(&connection)?;

        let created_at = SystemClock::secs();

        connection.execute(
            "INSERT INTO wallet VALUES(?, ?, ?, ?);",
            (0, created_at.value(), true, 0),
        )?;

        Ok(Self {
            connection: Mutex::new(connection),
        })
    }

    pub fn set_mnemonic(&self, mnemonic: Mnemonic) -> Result<()> {
        let master = MasterSecret::from(mnemonic);
        let connection = self.connection.lock().unwrap();
        connection.execute(
            "INSERT INTO keys VALUES(?, ?, ?);",
            ("master", master.as_bytes(), Option::<&[u8]>::None),
        )?;
        Ok(())
    }

    pub fn derive_account(&self) -> Result<(), DeriveAccountError> {
        let connection = self.connection.lock().unwrap();
        let mut statement = connection.prepare_cached("SELECT secret FROM keys WHERE path = ?;")?;
        let master: Vec<u8> = statement.query_one(("master",), |row| row.get(0))?;
        let master = MasterSecret::from(master);
        let secret_key = to_secret_key(master).ok_or(DeriveAccountError::Version)?;
        let public_key = to_public_key(&secret_key);
        let mut statement = connection.prepare_cached("INSERT INTO keys VALUES(?, ?, ?);")?;
        statement.execute(("", secret_key.as_ref(), public_key.as_ref()))?;
        Ok(())
    }

    pub fn created_at(&self) -> Result<Seconds> {
        let connection = self.connection.lock().unwrap();
        let mut statement = connection.prepare_cached("SELECT created_at FROM wallet;")?;
        let num: i64 = statement.query_one((), |row| row.get(0))?;
        Ok(Seconds::new(num))
    }

    pub fn is_staking(&self) -> Result<bool> {
        let connection = self.connection.lock().unwrap();
        let mut statement = connection.prepare_cached("SELECT is_staking FROM wallet;")?;
        let b: bool = statement.query_one((), |row| row.get(0))?;
        Ok(b)
    }

    pub fn public_key(&self) -> Result<PublicKey> {
        let connection = self.connection.lock().unwrap();
        let mut statement = connection.prepare_cached("SELECT public FROM keys WHERE path = ?;")?;
        let bytes: [u8; 32] = statement.query_one(("",), |row| row.get(0))?;
        Ok(PublicKey::from(bytes))
    }

    pub fn secret_key(&self) -> Result<SecretKey> {
        let connection = self.connection.lock().unwrap();
        let mut statement = connection.prepare_cached("SELECT secret FROM keys WHERE path = ?;")?;
        let bytes: [u8; 32] = statement.query_one(("",), |row| row.get(0))?;
        Ok(SecretKey::from(bytes))
    }

    pub fn sequence(&self) -> Result<u32> {
        let connection = self.connection.lock().unwrap();
        let mut statement = connection.prepare_cached("SELECT sequence FROM wallet;")?;
        let sequence = statement.query_one((), |row| row.get(0))?;
        Ok(sequence)
    }

    pub fn get_transaction(&self, id: Hash256) -> Result<Box<[u8]>> {
        let id: [u8; _] = id.into();
        let connection = self.connection.lock().unwrap();
        let mut statement =
            connection.prepare_cached("SELECT bytes FROM transactions WHERE id = ?;")?;
        let bytes = statement.query_one((id,), |row| row.get(0))?;
        Ok(bytes)
    }

    pub fn put_transaction(&self, id: Hash256, bytes: &[u8]) -> Result<()> {
        let id: [u8; _] = id.into();
        let connection = self.connection.lock().unwrap();
        let mut statement = connection.prepare_cached("INSERT INTO transactions VALUES(?, ?);")?;
        statement.execute((id, bytes))?;
        Ok(())
    }

    pub fn count_transactions(&self) -> Result<usize> {
        let connection = self.connection.lock().unwrap();
        let mut statement = connection.prepare_cached("SELECT COUNT(*) FROM transactions;")?;
        let n = statement.query_one((), |row| row.get(0))?;
        Ok(n)
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

#[derive(Debug)]
pub enum OpenError {
    Magic(String),
    Sqlite(Error),
}

impl From<Error> for OpenError {
    fn from(error: Error) -> Self {
        Self::Sqlite(error)
    }
}

impl fmt::Display for OpenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Magic(name) => {
                write!(f, "This SQLite database doesn't look like {name} wallet")
            }
            Self::Sqlite(err) => write!(f, "{err}"),
        }
    }
}

impl core::error::Error for OpenError {}

#[derive(Debug)]
pub enum DeriveAccountError {
    Version,
    Sqlite(Error),
}

impl From<Error> for DeriveAccountError {
    fn from(error: Error) -> Self {
        Self::Sqlite(error)
    }
}

impl fmt::Display for DeriveAccountError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Version => write!(
                f,
                "Cannot derive pre-quantum account from this master secret"
            ),
            Self::Sqlite(err) => write!(f, "{err}"),
        }
    }
}

impl core::error::Error for DeriveAccountError {}

pub type Result<T, E = Error> = core::result::Result<T, E>;
