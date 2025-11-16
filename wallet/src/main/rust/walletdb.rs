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

use crate::address::AddressCodec;
use crate::wallet::Wallet;
use blacknet_compat::Mode;
use blacknet_kernel::blake2b::Hash;
use blacknet_kernel::ed25519::PublicKey;
use blacknet_log::{LogManager, Logger, info};
use core::error::Error;
use std::collections::HashMap;

pub struct WalletDB {
    logger: Logger,
    address_codec: AddressCodec,
    wallets: HashMap<PublicKey, Wallet>,
}

impl WalletDB {
    pub fn new(mode: &Mode, log_manager: &LogManager) -> Result<Self, Box<dyn Error>> {
        let logger = log_manager.logger("WalletDB")?;
        info!(logger, "Driving SQLite {}", rusqlite::version());
        Ok(Self {
            logger,
            address_codec: AddressCodec::new(mode)?,
            wallets: HashMap::new(), //TODO
        })
    }

    pub const fn address_codec(&self) -> &AddressCodec {
        &self.address_codec
    }

    pub fn sequence(&self, public_key: PublicKey) -> Option<u32> {
        self.wallets.get(&public_key).map(Wallet::sequence)
    }

    pub fn anchor(&self) -> Hash {
        todo!();
    }
}

impl Drop for WalletDB {
    fn drop(&mut self) {
        info!(self.logger, "Braking SQLite");
    }
}
