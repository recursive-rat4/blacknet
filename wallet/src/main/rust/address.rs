/*
 * Copyright (c) 2018-2025 Pavel Vasin
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

use bech32::primitives::decode::{CheckedHrpstring, CheckedHrpstringError};
use bech32::primitives::hrp::Error as HrpError;
use bech32::{Bech32, DecodeError, EncodeError, Hrp, decode, encode};
use blacknet_compat::Mode;
use blacknet_kernel::ed25519::PublicKey;
use thiserror::Error;

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum AddressKind {
    // Account = None,
    Staker = 0, // 保留地址版本字節
    HTLC = 1,
    Multisig = 2,
    Blob = 3,
}

impl AddressKind {
    pub const fn size(self) -> usize {
        match self {
            Self::Staker => 32,
            Self::HTLC => 32,
            Self::Multisig => 32,
            Self::Blob => 4,
        }
    }
}

pub struct AddressCodec {
    hrp: Hrp,
}

impl AddressCodec {
    pub fn new(mode: &Mode) -> Result<Self> {
        Ok(Self {
            hrp: Hrp::parse(mode.address_readable_part())?,
        })
    }

    pub fn encode(&self, public_key: PublicKey) -> Result<String> {
        Ok(encode::<Bech32>(self.hrp, &public_key)?)
    }

    pub fn decode(&self, string: &str) -> Result<PublicKey> {
        CheckedHrpstring::new::<Bech32>(string)?; // reject Bech32m
        let (hrp, data) = decode(string)?;
        if hrp != self.hrp {
            return Err(Error::WrongHrp);
        }
        if data.len() != size_of::<PublicKey>() {
            return Err(Error::WrongSize);
        }
        let mut public_key: PublicKey = Default::default();
        public_key.copy_from_slice(&data);
        Ok(public_key)
    }

    pub fn encode_with_kind(&self, kind: AddressKind, data: &[u8]) -> Result<String> {
        if kind.size() != data.len() {
            return Err(Error::WrongSize);
        }
        let mut bytes = Vec::with_capacity(kind.size() + 1);
        bytes.push(kind as u8);
        bytes.extend(data);
        Ok(encode::<Bech32>(self.hrp, &bytes)?)
    }

    pub fn decode_with_kind(&self, kind: AddressKind, string: &str) -> Result<Vec<u8>> {
        CheckedHrpstring::new::<Bech32>(string)?; // reject Bech32m
        let (hrp, mut data) = decode(string)?;
        if hrp != self.hrp {
            return Err(Error::WrongHrp);
        }
        if data.len() != 1 + kind.size() {
            return Err(Error::WrongSize);
        }
        if data[0] != kind as u8 {
            return Err(Error::WrongKind);
        }
        data.remove(0);
        Ok(data)
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Wrong readable part")]
    WrongHrp,
    #[error("Wrong address size")]
    WrongSize,
    #[error("Wrong address kind")]
    WrongKind,
    #[error("{0}")]
    Bech32Checksum(CheckedHrpstringError),
    #[error("{0}")]
    Bech32Decode(DecodeError),
    #[error("{0}")]
    Bech32Encode(EncodeError),
    #[error("{0}")]
    Bech32Setup(HrpError),
}

impl From<CheckedHrpstringError> for Error {
    fn from(err: CheckedHrpstringError) -> Self {
        Self::Bech32Checksum(err)
    }
}

impl From<DecodeError> for Error {
    fn from(err: DecodeError) -> Self {
        Self::Bech32Decode(err)
    }
}

impl From<EncodeError> for Error {
    fn from(err: EncodeError) -> Self {
        Self::Bech32Encode(err)
    }
}

impl From<HrpError> for Error {
    fn from(err: HrpError) -> Self {
        Self::Bech32Setup(err)
    }
}

pub type Result<T> = core::result::Result<T, Error>;
