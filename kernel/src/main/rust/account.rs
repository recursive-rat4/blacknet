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
use crate::error::{Error, Result};
use crate::proofofstake::{MATURITY, MIN_LEASE};
use alloc::borrow::ToOwned;
use alloc::format;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize)]
pub struct Account {
    seq: u32,
    stake: Amount,
    immature: Vec<Input>,
    leases: Vec<Lease>,
}

impl Account {
    pub fn balance(&self) -> Amount {
        self.stake + self.immature.iter().copied().map(Input::balance).sum()
    }

    pub fn confirmed_balance(&self, height: u32, confirmations: u32) -> Amount {
        self.stake
            + self
                .immature
                .iter()
                .copied()
                .map(|i| i.confirmed_balance(height, confirmations))
                .sum()
    }

    pub fn staking_balance(&self, height: u32) -> Amount {
        self.stake
            + self
                .immature
                .iter()
                .copied()
                .map(|i| i.mature_balance(height))
                .sum()
            + self
                .leases
                .iter()
                .copied()
                .map(|i| i.mature_balance(height))
                .sum()
    }

    pub fn total_balance(&self) -> Amount {
        self.stake
            + self.immature.iter().copied().map(Input::balance).sum()
            + self.leases.iter().copied().map(Lease::balance).sum()
    }

    pub fn credit(&mut self, amount: Amount) -> Result<()> {
        if amount <= self.stake {
            self.stake -= amount;
            return Ok(());
        }

        if self.balance() < amount {
            return Err(Error::Invalid("Insufficient funds".to_owned()));
        }

        let mut r = amount - self.stake;
        self.stake = Amount::ZERO;
        while r > Amount::ZERO {
            if r < self.immature[0].amount {
                self.immature[0].amount -= r;
                break;
            } else {
                r -= self.immature[0].amount;
                self.immature.remove(0);
            }
        }

        Ok(())
    }

    pub fn debit(&mut self, height: u32, amount: Amount) {
        if amount != Amount::ZERO {
            self.immature.push(Input { height, amount })
        }
    }

    pub fn add_lease(&mut self, public_key: PublicKey, height: u32, amount: Amount) {
        self.leases.push(Lease {
            public_key,
            height,
            amount,
        })
    }

    pub fn remove_lease(
        &mut self,
        public_key: PublicKey,
        height: u32,
        amount: Amount,
    ) -> Result<()> {
        let lease = Lease {
            public_key,
            height,
            amount,
        };
        let position = self
            .leases
            .iter()
            .copied()
            .position(|i| i == lease)
            .ok_or(Error::Invalid("Lease not found".to_owned()))?;
        self.leases.remove(position);
        Ok(())
    }

    pub fn withdraw_from_lease(
        &mut self,
        withdraw: Amount,
        amount: Amount,
        public_key: PublicKey,
        height: u32,
    ) -> Result<()> {
        let lease = Lease {
            public_key,
            height,
            amount,
        };
        let position = self
            .leases
            .iter()
            .copied()
            .position(|i| i == lease)
            .ok_or(Error::Invalid("Lease not found".to_owned()))?;
        self.leases[position].withdraw(withdraw)?;
        Ok(())
    }

    pub fn prune(&mut self, height: u32) -> bool {
        let mature = self
            .immature
            .iter()
            .copied()
            .map(|i| i.mature_balance(height))
            .sum();
        if mature == Amount::ZERO {
            false
        } else {
            self.stake += mature;
            self.immature = self
                .immature
                .iter()
                .copied()
                .filter(|i| !i.is_mature(height))
                .collect();
            true
        }
    }
}

#[derive(Clone, Copy, Deserialize, Serialize)]
pub struct Input {
    height: u32,
    amount: Amount,
}

impl Input {
    fn is_confirmed(self, height: u32, confirmations: u32) -> bool {
        height > self.height + confirmations
    }
    fn is_mature(self, height: u32) -> bool {
        height > self.height + MATURITY
    }
    fn balance(self) -> Amount {
        self.amount
    }
    fn confirmed_balance(self, height: u32, confirmations: u32) -> Amount {
        if self.is_confirmed(height, confirmations) {
            self.amount
        } else {
            Amount::ZERO
        }
    }
    fn mature_balance(self, height: u32) -> Amount {
        if self.is_mature(height) {
            self.amount
        } else {
            Amount::ZERO
        }
    }
}

#[derive(Clone, Copy, Deserialize, Eq, PartialEq, Serialize)]
pub struct Lease {
    public_key: PublicKey,
    height: u32,
    amount: Amount,
}

impl Lease {
    fn is_mature(self, height: u32) -> bool {
        height > self.height + MATURITY
    }
    fn balance(self) -> Amount {
        self.amount
    }
    fn mature_balance(self, height: u32) -> Amount {
        if self.is_mature(height) {
            self.amount
        } else {
            Amount::ZERO
        }
    }
    fn withdraw(&mut self, withdraw: Amount) -> Result<()> {
        if withdraw > self.amount - MIN_LEASE {
            return Err(Error::Invalid(format!(
                "Can not withdraw more than {0}",
                self.amount - MIN_LEASE
            )));
        }
        self.amount -= withdraw;
        Ok(())
    }
}
