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

use crate::wallet::Mnemonic;
use blacknet_crypto::zeroize::zeroize_vec;
use core::fmt;

pub struct MasterSecret {
    bytes: Vec<u8>,
}

impl MasterSecret {
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }
}

impl From<Vec<u8>> for MasterSecret {
    #[inline]
    fn from(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }
}

impl From<Mnemonic> for MasterSecret {
    #[inline]
    fn from(mnemonic: Mnemonic) -> Self {
        let bytes = mnemonic.into_bytes();
        Self { bytes }
    }
}

impl fmt::Debug for MasterSecret {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MasterSecret").finish_non_exhaustive()
    }
}

impl AsRef<[u8]> for MasterSecret {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        &self.bytes
    }
}

impl Drop for MasterSecret {
    fn drop(&mut self) {
        zeroize_vec(&mut self.bytes)
    }
}
