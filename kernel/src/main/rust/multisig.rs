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
use crate::ed25519::PublicKey;
use alloc::boxed::Box;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Deserialize, Serialize)]
pub struct Deposit {
    from: PublicKey,
    amount: Amount,
}

impl Deposit {
    pub const fn new(from: PublicKey, amount: Amount) -> Self {
        Self { from, amount }
    }

    pub const fn from(self) -> PublicKey {
        self.from
    }

    pub const fn amount(self) -> Amount {
        self.amount
    }
}

#[derive(Deserialize, Serialize)]
pub struct Multisig {
    n: u8,
    deposits: Box<[Deposit]>,
}

impl Multisig {
    pub const fn new(n: u8, deposits: Box<[Deposit]>) -> Self {
        Self { n, deposits }
    }

    pub const fn n(&self) -> u8 {
        self.n
    }

    pub const fn deposits(&self) -> &[Deposit] {
        &self.deposits
    }

    pub fn amount(&self) -> Amount {
        self.deposits.iter().copied().map(Deposit::amount).sum()
    }
}
