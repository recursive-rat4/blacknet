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

use crate::amount::Amount;
use crate::ed25519::{PublicKey, Signature};
use alloc::boxed::Box;
use serde::{Deserialize, Serialize};

pub type MultiSignatureLockContractId = [u8; 32];

#[derive(Clone, Copy, Deserialize, Serialize)]
pub struct Deposit {
    from: PublicKey,
    amount: Amount,
}

impl Deposit {
    pub fn from(self) -> PublicKey {
        self.from
    }

    pub fn amount(self) -> Amount {
        self.amount
    }
}

#[derive(Clone, Copy, Deserialize, Serialize)]
pub struct Sig {
    index: u8,
    signature: Signature,
}

impl Sig {
    pub fn index(self) -> u8 {
        self.index
    }

    pub fn signature(self) -> Signature {
        self.signature
    }
}

#[derive(Deserialize, Serialize)]
pub struct CreateMultisig {
    n: u8,
    deposits: Box<[Deposit]>,
    signatures: Box<[Sig]>,
}

impl CreateMultisig {
    pub fn n(&self) -> u8 {
        self.n
    }

    pub fn deposits(&self) -> &[Deposit] {
        &self.deposits
    }

    pub fn signatures(&self) -> &[Sig] {
        &self.signatures
    }
}

//TODO
